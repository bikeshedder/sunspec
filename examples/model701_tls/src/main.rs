use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::Duration
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use sunspec::models::model701::Model701;
use sunspec::DEFAULT_DISCOVERY_ADDRESSES;
use sunspec::{
    client::{AsyncClient, Config},
    models::model1::Model1,
};
use tokio::time::sleep;
use tokio_modbus::client::{Context, tcp};
use tokio::net::TcpStream;
use rustls::crypto::{CryptoProvider, aws_lc_rs as provider};

#[derive(Parser)]
struct Args {
    addr: SocketAddr,
    device_id: u8,
    #[arg(
        long,
        short='d',
        help = "Discovery addresses",
        name = "ADDRESS",
        default_values_t = DEFAULT_DISCOVERY_ADDRESSES
    )]
    discovery_addresses: Vec<u16>,
    #[arg(
        long,
        short = 't',
        help = "Read timeout in seconds",
        name = "SECONDS",
        default_value_t = 1.0
    )]
    read_timeout: f32,
}

use pkcs8::der::Decode;
use pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls_pemfile::{certs, pkcs8_private_keys, ec_private_keys};
use tokio_rustls::TlsConnector;

fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}

fn load_keys(path: &Path, password: Option<&str>) -> io::Result<PrivateKeyDer<'static>> {
    let expected_tag = match &password {
        Some(_) => "ENCRYPTED PRIVATE KEY",
        None => "PRIVATE KEY",
    };

    if expected_tag.eq("PRIVATE KEY") {
        let content = std::fs::read(path)?;
        if content.starts_with("-----BEGIN EC PRIVATE KEY".as_bytes()) {
            ec_private_keys(&mut BufReader::new(File::open(path)?))
            .next()
            .unwrap()
            .map(Into::into)
        }else{
            pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
            .next()
            .unwrap()
            .map(Into::into)
        }
    } else {
        let content = std::fs::read(path)?;
        let mut iter = pem::parse_many(content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?
            .into_iter()
            .filter(|x| x.tag() == expected_tag)
            .map(|x| x.contents().to_vec());

        match iter.next() {
            Some(key) => match password {
                Some(password) => {
                    let encrypted =
                        pkcs8::EncryptedPrivateKeyInfo::from_der(&key).map_err(|err| {
                            io::Error::new(io::ErrorKind::InvalidData, err.to_string())
                        })?;
                    let decrypted = encrypted.decrypt(password).map_err(|err| {
                        io::Error::new(io::ErrorKind::InvalidData, err.to_string())
                    })?;
                    let key = decrypted.as_bytes().to_vec();
                    match rustls_pemfile::read_one_from_slice(&key).expect("cannot parse private key .pem file") {
                        Some((rustls_pemfile::Item::Pkcs1Key(key), _keys)) => io::Result::Ok(key.into()),
                        Some((rustls_pemfile::Item::Pkcs8Key(key), _keys)) => io::Result::Ok(key.into()),
                        Some((rustls_pemfile::Item::Sec1Key(key), _keys)) => io::Result::Ok(key.into()),
                        _ => io::Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key")),
                    }
                }
                None => io::Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key")),
            },
            None => io::Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key")),
        }
    }
}

async fn connect_tls(socket_addr: SocketAddr) -> io::Result<Context> {
    let mut root_cert_store = tokio_rustls::rustls::RootCertStore::empty();
    let ca_path = Path::new("./pki/cav3.pem");
    let mut pem = BufReader::new(File::open(ca_path)?);
    let certs = rustls_pemfile::certs(&mut pem).collect::<Result<Vec<_>, _>>()?;
    root_cert_store.add_parsable_certificates(certs);

    let domain = "localhost";
    let cert_path = Path::new("./pki/clientv3.pem");
    let key_path = Path::new("./pki/clientv3.key");
    let certs = load_certs(cert_path)?;
    let key = load_keys(key_path, None)?;

    let mut config = tokio_rustls::rustls::ClientConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites:
            vec![provider::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256],
            kx_groups: vec![provider::kx_group::SECP256R1],
            ..provider::default_provider()
        }
        .into()
    )
    .with_protocol_versions(&[&rustls::version::TLS12])
    .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?
    .with_root_certificates(root_cert_store)
    .with_client_auth_cert(certs, key)
    .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    config.resumption = config.resumption
        .tls12_resumption(rustls::client::Tls12Resumption::SessionIdOnly);

    let connector = TlsConnector::from(Arc::new(config));

    let stream = TcpStream::connect(&socket_addr).await?;
    stream.set_nodelay(true)?;

    let domain = ServerName::try_from(domain)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;

    let transport = connector.connect(domain, stream).await?;

    // Tokio modbus transport layer setup
    let ctx = tcp::attach(transport);
    Ok(ctx)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = AsyncClient::new(
        connect_tls(args.addr).await?,
        Config {
            discovery_addresses: args.discovery_addresses,
            read_timeout: (args.read_timeout != 0.0)
                .then(|| Duration::from_secs_f32(args.read_timeout)),
            ..Default::default()
        },
    );

    let device = client.device(args.device_id).await?;
    let m1: Model1 = device.read_model().await?;

    println!("Manufacturer: {}", m1.mn);
    println!("Model: {}", m1.md);
    println!("Version: {}", m1.vr.as_deref().unwrap_or("(unspecified)"));
    println!("Serial Number: {}", m1.sn);

    println!(
        "Supported models: {}",
        device
            .models
            .supported_model_ids()
            .iter()
            .map(|id| id.to_string())
            .join(", ")
    );

    loop {
        let m701: Model701 = device.read_model().await?;
        let w = m701.w.unwrap() as f32 * 10f32.powf(m701.w_sf.unwrap().into());
        let wh = m701.tot_wh_inj.unwrap() as f32 * 10f32.powf(m701.tot_wh_sf.unwrap().into());
        println!("{:12.3} kWh {:9.3} kW", wh / 1000.0, w / 1000.0,);
        sleep(Duration::from_secs(1)).await;
    }
}
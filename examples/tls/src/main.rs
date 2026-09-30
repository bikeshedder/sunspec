use std::{error::Error, path::PathBuf, sync::Arc};

use clap::Parser;
use itertools::Itertools;
use sunspec::{
    client::{AsyncClient, Config},
    models::model1::Model1,
};
use tokio::net::TcpStream;
use tokio_modbus::client::tcp::attach;
use tokio_rustls::{
    rustls::{
        pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName},
        ClientConfig, RootCertStore,
    },
    TlsConnector,
};

#[derive(Parser)]
struct Args {
    host: String,
    device_id: u8,
    ca: PathBuf,
    cert: PathBuf,
    key: PathBuf,
    #[arg(long, short = 'p', default_value_t = 802)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from_pem_file(&args.ca)?)?;
    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(
            vec![CertificateDer::from_pem_file(&args.cert)?],
            PrivateKeyDer::from_pem_file(&args.key)?,
        )?;
    let stream = TcpStream::connect((args.host.as_str(), args.port)).await?;
    let stream = TlsConnector::from(Arc::new(config))
        .connect(ServerName::try_from(args.host)?, stream)
        .await?;

    let client = AsyncClient::new(attach(stream), Config::default());
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
            .iter()
            .map(|info| info.id.to_string())
            .join(", ")
    );

    Ok(())
}

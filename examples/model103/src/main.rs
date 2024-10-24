use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::models::model103::Model103;
use sunspec::DEFAULT_DISCOVERY_ADDRESSES;
use sunspec::{
    client::{AsyncClient, Config},
    models::model1::Model1,
};
use tokio::time::sleep;
use tokio_modbus::{client::tcp::connect_slave, Slave};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut client = AsyncClient::new(
        connect_slave(args.addr, Slave(args.device_id)).await?,
        Config {
            discovery_addresses: args.discovery_addresses,
            read_timeout: (args.read_timeout != 0.0)
                .then(|| Duration::from_secs_f32(args.read_timeout)),
            ..Default::default()
        },
    )
    .await?;

    let m1: Model1 = client.read_model().await?;

    println!("Manufacturer: {}", m1.mn);
    println!("Model: {}", m1.md);
    println!("Version: {}", m1.vr.as_deref().unwrap_or("(unspecified)"));
    println!("Serial Number: {}", m1.sn);

    println!(
        "Supported models: {}",
        client
            .models
            .supported_model_ids()
            .iter()
            .map(|id| id.to_string())
            .join(", ")
    );

    loop {
        let m103: Model103 = client.read_model().await?;
        let w = m103.w as f32 * 10f32.powf(m103.w_sf.into());
        let wh = m103.wh as f32 * 10f32.powf(m103.wh_sf.into());
        println!("{:12.3} kWh {:9.3} kW", wh / 1000.0, w / 1000.0,);
        sleep(Duration::from_secs(1)).await;
    }
}

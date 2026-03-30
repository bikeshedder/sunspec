use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::{
    client::{AsyncClient, Config},
    models::{model1::Model1, model712::Model712},
    DEFAULT_DISCOVERY_ADDRESSES,
};
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

    let client = AsyncClient::new(
        connect_slave(args.addr, Slave(args.device_id)).await?,
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

    let m712: Model712 = device.read_model().await?;
    println!("{:?}", m712);

    Ok(())
}

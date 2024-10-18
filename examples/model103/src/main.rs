use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::{
    tokio_modbus::{discover_models, read_model},
    Config, DEFAULT_DISCOVERY_ADDRESSES,
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
    let mut ctx = connect_slave(args.addr, Slave(args.device_id)).await?;

    let read_timeout =
        (args.read_timeout != 0.0).then(|| Duration::from_secs_f32(args.read_timeout));

    let client_config = Config {
        discovery_addresses: args.discovery_addresses,
        read_timeout: read_timeout.clone(),
        ..Default::default()
    };

    let models = discover_models(&mut ctx, &client_config).await?.models;

    let m1 = read_model(&mut ctx, models.m1, &client_config).await?;

    println!("Manufacturer: {}", m1.mn);
    println!("Model: {}", m1.md);
    println!("Version: {}", m1.vr.as_deref().unwrap_or("(unspecified)"));
    println!("Serial Number: {}", m1.sn);

    println!(
        "Supported models: {}",
        models
            .supported_model_ids()
            .iter()
            .map(|id| id.to_string())
            .join(", ")
    );

    loop {
        let m103 = read_model(&mut ctx, models.m103, &client_config).await?;
        let w = m103.w as f32 * 10f32.powf(m103.w_sf.into());
        let wh = m103.wh as f32 * 10f32.powf(m103.wh_sf.into());
        println!("{:12.3} kWh {:9.3} kW", wh / 1000.0, w / 1000.0,);
        sleep(Duration::from_secs(1)).await;
    }
}

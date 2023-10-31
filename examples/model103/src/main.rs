use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::{discover_models, models::Model103, read_model, read_point, ReadPointError};
use tokio::time::sleep;
use tokio_modbus::{client::tcp::connect_slave, Slave};

#[derive(Parser)]
struct Args {
    addr: SocketAddr,
    device_id: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut ctx = connect_slave(args.addr, Slave(args.device_id)).await?;

    let models = discover_models(&mut ctx).await?;
    let m1 = read_model(&mut ctx, &models.m1).await?;

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

    let m103 = read_model(&mut ctx, &models.m103).await?;

    loop {
        // Read a single point rather than the complete model
        let m103_w = read_point(&mut ctx, &models.m103, &Model103::W)
            .await?
            .ok_or(ReadPointError::MissingMandatoryValue)?;
        let w = m103_w as f32 * 10f32.powi(m103.w_sf.into());
        println!("{:.1}W", w);
        sleep(Duration::from_secs(1)).await;
    }
}

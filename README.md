# SunSpec Rust Implementation

[![Latest Version](https://img.shields.io/crates/v/sunspec.svg)](https://crates.io/crates/sunspec)
![Unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg "Unsafe forbidden")

This [Rust](https://www.rust-lang.org) crate contains code for accessing [SunSpec](https://en.wikipedia.org/wiki/SunSpec) compliant devices
in a safe and convenient way.

## Highlights

- [x] Pure Rust library
- [x] No unsafe code
- [x] Panic free
- [x] All communication is done via
      [tokio-modbus](https://crates.io/crate/tokio-modbus)
      without wrapping it in some kind of abstraction layer.
- [x] Supports Modbus TCP and RTU (via `tokio-modbus`).
- [x] Implements "Device Information Model Discovery" as
      defined in the SunSpec specification.
- [x] Fully typed models generated from the SMDX files contained in the
      [SunSpec models repository](https://github.com/sunspec/models/)
- [x] Fully documented. Even the generated models.
- [ ] Repeating models

## Features

| Feature | Description | Extra dependencies | Default |
| ------- | ----------- | ------------------ | ------- |
| `tokio` | Enable `tokio_modbus` support | `tokio-modbus` | yes |

## Examples

The `examples` directory in the code repository contains the unabridged code.

### Example code for accessing data from a three phase inverter using the model 103

```rust,ignore
use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::tokio_modbus::{discover_models, read_model};
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

    let models = discover_models(&mut ctx).await?.models;
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

    loop {
        let m103 = read_model(&mut ctx, &models.m103).await?;
        let w = m103.w as f32 * 10f32.powf(m103.w_sf.into());
        let wh = m103.wh as f32 * 10f32.powf(m103.wh_sf.into());
        println!("{:12.3} kWh {:9.3} kW", wh / 1000.0, w / 1000.0,);
        sleep(Duration::from_secs(1)).await;
    }
}
```

## FAQ

How does this crate differ from crates like `tokio-sunspec`, `sunspec-models`, `sunspec_rs`?

- This crate generates all code using Rust code via the official
  [SunSpec models repository](https://github.com/sunspec/models/)
  with a code generator that was written in Rust, too.

- All generated models are plain Rust structs. A single Modbus call
  can return the complete data for a model rather than having to fetch
  points individually.

- All public types are documented. Even the generated models.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0)>
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT)>

at your option.

# SunSpec Rust Implementation

[![Latest Version](https://img.shields.io/crates/v/sunspec.svg)](https://crates.io/crates/sunspec)
[![CI](https://img.shields.io/github/actions/workflow/status/bikeshedder/sunspec/rust.yml?logo=github&label=CI)](https://github.com/bikeshedder/sunspec/actions?query=workflow%3ARust)
![Unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg "Unsafe forbidden")
[![Rust 1.75+](https://img.shields.io/badge/rustc-1.75+-lightgray.svg "Rust 1.75+")](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

This [Rust](https://www.rust-lang.org) crate contains code for accessing [SunSpec](https://en.wikipedia.org/wiki/SunSpec) compliant devices
in a safe and convenient way.

## Highlights

- [x] Pure Rust library
- [x] No unsafe code
- [x] Panic free
- [x] All communication is abstracted via traits making it runtime agnostic
- [x] Supports Modbus TCP and RTU (via [tokio-modbus](https://crates.io/crates/tokio-modbus)).
- [x] Implements "Device Information Model Discovery" as
      defined in the SunSpec specification.
- [x] Fully typed models generated from the JSON files contained in the
      [SunSpec models repository](https://github.com/sunspec/models/)
- [x] Fully typed enums
- [x] Fully typed bitfields
- [x] Fully documented. Even the generated models.
- [x] Reading of complete models in a single request.

| ⚠️ Nested and repeating groups are not supported, yet. |
| ---- |

## Features

| Feature | Description                   | Extra dependencies            | Default |
| ------- | ----------------------------- | ----------------------------- | ------- |
| `tokio` | Enable `tokio_modbus` support | `tokio-modbus`, `tokio/time`  | yes     |
| `serde` | Enable `serde` support        | `serde`, `bitflags/serde`     | yes     |

## Examples

The `examples` directory in the code repository contains the unabridged code.

### Example code for accessing data from a three phase inverter using the model 103

```rust,ignore
use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::{
    client::{AsyncClient, Config},
    models::{model1::Model1, model103::Model103},
};
use tokio::time::sleep;
use tokio_modbus::client::tcp::connect;

#[derive(Parser)]
struct Args {
    addr: SocketAddr,
    device_id: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = AsyncClient::new(connect(args.addr).await?, Config::default());
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
        let m103: Model103 = device.read_model().await?;
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

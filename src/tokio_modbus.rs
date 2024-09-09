//! This module contains the actual communication methods via
//! `tokio-modbus`.
use std::future::Future;
use std::io;
use std::time::Duration;

use crate::config::Config;
use crate::discovery::{DiscoveryError, ModelAddr, UnknownModel, SUNS_IDENTIFIER};
use crate::model::{Model, ReadModelError};
use crate::models::Models;
use crate::point::{PointDef, ReadPointError, WritePointError};
use crate::value::Value;
use crate::DiscoveryResult;

use tokio_modbus::client::{Context, Reader, Writer};

async fn read_holding_registers_array<const CNT: usize>(
    context: &mut Context,
    addr: u16,
) -> io::Result<[u16; CNT]> {
    // Unwrap is fine here as read_holding_registers is guaranteed to
    // return the right amount of words.
    context
        .read_holding_registers(addr, CNT as u16)
        .await
        .map(|words| {
            words
                .try_into()
                .expect("read_holding_registers returned the wrong amount of words")
        })
}

/// This function implements the "Device Information Model Discovery"
/// as explained in [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
pub async fn discover_models(
    context: &mut Context,
    config: &Config,
) -> Result<DiscoveryResult, DiscoveryError> {
    // Read addresses 0, 40000 and 50000 looking for the SunS identifier
    let mut info_model_addr: Option<u16> = None;
    for &addr in &config.discovery_addresses {
        // TODO add timeout
        match apply_timeout(
            read_holding_registers_array::<2>(context, addr),
            config.read_timeout,
        )
        .await
        {
            Ok(identifier) if identifier == SUNS_IDENTIFIER => {
                info_model_addr = Some(addr);
                break;
            }
            Ok(_) => continue,
            // TODO: Switch out string matching once tokio_modbus::frame::Exception
            //       is made public. See these PR's:
            //           - https://github.com/slowtec/tokio-modbus/pull/218
            //           - https://github.com/slowtec/tokio-modbus/pull/231
            Err(e) if e.to_string() == "Modbus function 3: Illegal data address" => continue,
            Err(e) => Err(e),
        }?;
    }
    let Some(mut addr) = info_model_addr else {
        return Err(DiscoveryError::SunsIdentifierNotFound);
    };

    addr += 2;

    let mut models = Models::default();
    let mut unknown_models: Vec<UnknownModel> = vec![];

    loop {
        let [model_id, len] = apply_timeout(
            read_holding_registers_array::<2>(context, addr),
            config.read_timeout,
        )
        .await?;
        if model_id == 0xFFFF {
            break;
        }
        addr = addr.checked_add(2).ok_or(DiscoveryError::AddressOverflow)?;
        if !models.set_addr(model_id, addr, len) {
            unknown_models.push(UnknownModel {
                id: model_id,
                addr,
                len,
            });
        }
        addr = addr
            .checked_add(len)
            .ok_or(DiscoveryError::AddressOverflow)?;
    }

    Ok(DiscoveryResult {
        models,
        unknown_models,
    })
}

/// Read model data from modbus
///
/// Note: Some models are too big to be fetched in a single request
///       and multiple read_holding_registers calls will be issued.
pub async fn read_model<M: Model>(
    ctx: &mut Context,
    addr: &ModelAddr<M>,
    config: &Config,
) -> Result<M, ReadModelError> {
    let data = if addr.len <= config.max_read_length {
        apply_timeout(
            ctx.read_holding_registers(addr.addr, addr.len),
            config.read_timeout,
        )
        .await?
    } else {
        let mut data: Vec<u16> = Vec::with_capacity(addr.len.into());
        let begin = addr.addr;
        let start = addr.addr + addr.len;
        let ranges = (begin..start)
            .step_by(config.max_read_length as usize)
            .map(|x| x..((x + config.max_read_length).min(start)));
        for range in ranges {
            let chunk = apply_timeout(
                ctx.read_holding_registers(range.start, range.len().try_into().unwrap()),
                config.read_timeout,
            )
            .await?;
            data.extend(chunk);
        }
        data
    };
    M::from_data(&data)
}

/// Read data for a single point. Please note that
/// `read_model` is more efficient when loading multiple
/// points from a single model.
pub async fn read_point<M: Model, T: Value>(
    ctx: &mut Context,
    model_addr: &ModelAddr<M>,
    point_def: &PointDef<M, T>,
    config: &Config,
) -> Result<Option<T>, ReadPointError> {
    let data = apply_timeout(
        ctx.read_holding_registers(model_addr.addr + point_def.offset, point_def.length),
        config.read_timeout,
    )
    .await?;
    Ok(Some(Value::decode(&data)?))
}

/// Write data for a single point
pub async fn write_point<M: Model, T: Value>(
    ctx: &mut Context,
    model_addr: &ModelAddr<M>,
    point_def: &PointDef<M, T>,
    value: T,
    config: &Config,
) -> Result<(), WritePointError> {
    let data = value.encode();
    if data.len() > point_def.length as usize {
        return Err(WritePointError::ValueTooLarge);
    }
    apply_timeout(
        ctx.write_multiple_registers(model_addr.addr + point_def.offset, &data),
        config.write_timeout,
    )
    .await?;
    Ok(())
}

async fn apply_timeout<T>(
    fut: impl Future<Output = io::Result<T>>,
    timeout: Option<Duration>,
) -> io::Result<T> {
    if let Some(timeout) = timeout {
        tokio::time::timeout(timeout, fut).await?
    } else {
        fut.await
    }
}

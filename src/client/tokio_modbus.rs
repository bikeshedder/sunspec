//! This module contains the actual communication methods via
//! `tokio-modbus`.
use std::future::Future;
use std::time::Duration;

use super::config::Config;
use super::discovery::DiscoveryResult;
use super::discovery::{DiscoveryError, UnknownModel};
use super::{CommunicationError, ReadModelError, ReadPointError, WritePointError};
use crate::constants::SUNS_IDENTIFIER;
use crate::model::{Model, ModelAddr};
use crate::models::Models;
use crate::point::PointDef;
use crate::value::Value;

use tokio_modbus::client::{Context, Reader, Writer};

async fn read_holding_registers_array<const CNT: usize>(
    context: &mut Context,
    addr: u16,
) -> tokio_modbus::Result<[u16; CNT]> {
    // Unwrap is fine here as read_holding_registers is guaranteed to
    // return the right amount of words.
    context
        .read_holding_registers(addr, CNT as u16)
        .await
        .map(|inner| {
            inner.map(|words| {
                words
                    .try_into()
                    .expect("read_holding_registers returned the wrong amount of words")
            })
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
        .await?
        .map_err(CommunicationError::from_modbus)?
        {
            Ok(identifier) if identifier == SUNS_IDENTIFIER => {
                info_model_addr = Some(addr);
                break;
            }
            Ok(_) => continue,
            Err(tokio_modbus::ExceptionCode::IllegalDataAddress) => continue,
            Err(e) => return Err(CommunicationError::from_modbus(e).into()),
        }
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
        .await?
        .map_err(CommunicationError::from_modbus)?
        .map_err(CommunicationError::from_modbus)?;
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
    addr: ModelAddr<M>,
    config: &Config,
) -> Result<M, ReadModelError> {
    let data = if addr.len <= config.max_read_length {
        apply_timeout(
            ctx.read_holding_registers(addr.addr, addr.len),
            config.read_timeout,
        )
        .await?
        .map_err(CommunicationError::from_modbus)?
        .map_err(CommunicationError::from_modbus)?
    } else {
        let mut data: Vec<u16> = Vec::with_capacity(addr.len.into());
        let begin = addr.addr;
        let start = addr.addr + addr.len;
        let ranges = (begin..start)
            .step_by(config.max_read_length as usize)
            .map(|x| x..((x + config.max_read_length).min(start)));
        for range in ranges {
            let chunk = apply_timeout(
                ctx.read_holding_registers(
                    range.start,
                    range
                        .len()
                        .try_into()
                        .expect("read_holding_registers returned the wrong amount of words"),
                ),
                config.read_timeout,
            )
            .await?
            .map_err(CommunicationError::from_modbus)?
            .map_err(CommunicationError::from_modbus)?;
            data.extend(chunk);
        }
        data
    };
    M::from_data(&data).map_err(|e| ReadModelError::Point(ReadPointError::DecodeError(e)))
}

/// Read data for a single point. Please note that
/// `read_model` is more efficient when loading multiple
/// points from a single model.
pub async fn read_point<M: Model, T: Value>(
    ctx: &mut Context,
    model_addr: ModelAddr<M>,
    point_def: PointDef<M, T>,
    config: &Config,
) -> Result<T, ReadPointError> {
    let data = apply_timeout(
        ctx.read_holding_registers(model_addr.addr + point_def.offset, point_def.length),
        config.read_timeout,
    )
    .await?
    .map_err(CommunicationError::from_modbus)?
    .map_err(CommunicationError::from_modbus)?;
    Ok(Value::decode(&data)?)
}

/// Write data for a single point
pub async fn write_point<M: Model, T: Value>(
    ctx: &mut Context,
    model_addr: ModelAddr<M>,
    point_def: PointDef<M, T>,
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
    .await?
    .map_err(CommunicationError::from_modbus)?
    .map_err(CommunicationError::from_modbus)?;
    Ok(())
}

async fn apply_timeout<T>(
    fut: impl Future<Output = tokio_modbus::Result<T>>,
    timeout: Option<Duration>,
) -> Result<tokio_modbus::Result<T>, CommunicationError> {
    Ok(if let Some(timeout) = timeout {
        tokio::time::timeout(timeout, fut)
            .await
            .map_err(|_| CommunicationError::Timeout)?
    } else {
        fut.await
    })
}

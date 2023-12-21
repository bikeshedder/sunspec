//! This module contains the actual communication methods via
//! `tokio-modbus`.
use std::io;

use crate::discovery::{DiscoveryError, ModelAddr, UnknownModel, SUNS_IDENTIFIER};
use crate::model::{Model, ReadModelError};
use crate::models::Models;
use crate::point::{PointDef, ReadPointError, WritePointError};
use crate::value::{FixedSize, Value};
use crate::DiscoveryResult;

use tokio_modbus::client::{Context, Reader, Writer};

async fn read_fixed_size<T: FixedSize>(context: &mut Context, addr: u16) -> io::Result<Option<T>> {
    match context.read_holding_registers(addr, T::SIZE).await {
        Ok(words) => Ok(Some(T::decode(&words).unwrap())),
        // TODO: Switch out string matching once tokio_modbus::frame::Exception
        //       is made public. See these PR's:
        //           - https://github.com/slowtec/tokio-modbus/pull/218
        //           - https://github.com/slowtec/tokio-modbus/pull/231
        Err(e) if e.to_string() == "Modbus function 3: Illegal data address" => Ok(None),
        Err(e) => Err(e),
    }
}

/// This function implements the "Device Information Model Discovery"
/// as explained in [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
pub async fn discover_models(context: &mut Context) -> Result<DiscoveryResult, DiscoveryError> {
    // Read addresses 0, 40000 and 50000 looking for the SunS identifier
    let mut info_model_addr: Option<u16> = None;
    for addr in [0, 40000, 50000] {
        let identifier = read_fixed_size::<u32>(context, addr).await?;
        if identifier == Some(SUNS_IDENTIFIER) {
            info_model_addr = Some(addr);
            break;
        }
    }
    let Some(mut addr) = info_model_addr else {
        return Err(DiscoveryError::SunsIdentifierNotFound);
    };

    addr += 2;

    let mut models = Models::default();
    let mut unknown_models: Vec<UnknownModel> = vec![];

    loop {
        let [model_id, len] = *context.read_holding_registers(addr, 2).await? else {
            unreachable!();
        };
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

// Modbus defines that a maximum of 125 registers can be read
// (6.4, Page 16) and 123 registers can be written (6.12, Page 30)
// to in a single request. See:
// https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf
const MAX_READ_LENGTH: u16 = 125;
//const MAX_WRITE_LENGTH: u16 = 123;

/// Read model data from modbus
///
/// Note: Some models are too big to be fetched in a single go.
pub async fn read_model<M: Model>(
    ctx: &mut Context,
    addr: &ModelAddr<M>,
) -> Result<M, ReadModelError> {
    let data = if addr.len <= MAX_READ_LENGTH {
        ctx.read_holding_registers(addr.addr, addr.len).await?
    } else {
        let mut data: Vec<u16> = Vec::with_capacity(addr.len.into());
        let begin = addr.addr;
        let start = addr.addr + addr.len;
        let ranges = (begin..start)
            .step_by(MAX_READ_LENGTH as usize)
            .map(|x| x..((x + MAX_READ_LENGTH).min(start)));
        for range in ranges {
            let chunk = ctx
                .read_holding_registers(range.start, range.len().try_into().unwrap())
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
) -> Result<Option<T>, ReadPointError> {
    let data = ctx
        .read_holding_registers(model_addr.addr + point_def.offset, point_def.length)
        .await?;
    Ok(Some(Value::decode(&data)?))
}

/// Write data for a single point
pub async fn write_point<M: Model, T: Value>(
    ctx: &mut Context,
    model_addr: &ModelAddr<M>,
    point_def: &PointDef<M, T>,
    value: T,
) -> Result<(), WritePointError> {
    let data = value.encode();
    if data.len() > point_def.length as usize {
        return Err(WritePointError::ValueTooLarge);
    }
    ctx.write_multiple_registers(model_addr.addr + point_def.offset, &data)
        .await?;
    Ok(())
}

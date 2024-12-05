use std::{future::Future, time::Duration};

use crate::{Model, ModelAddr, Models, Point, Value, SUNS_IDENTIFIER};

use super::{
    error::ModbusError, Config, DiscoveryError, DiscoveryResult, ReadModelError, ReadPointError,
    UnknownModel, WritePointError,
};

/// Async client
#[derive(Debug)]
pub struct AsyncClient<C: AsyncModbusClient> {
    /// This is the actual modbus client which implements the `AsyncModbusClient` trait.
    pub client: C,
    /// Model configuration
    pub config: Config,
    /// Discovered models
    pub models: Models,
    /// Unknown models
    pub unknown_models: Vec<UnknownModel>,
}

impl<C: AsyncModbusClient> AsyncClient<C> {
    /// Create new AsyncClient using a `AsyncModbusClient` and a `Config`
    /// and perform "Device Information Model Discovery" as explained in
    /// [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
    pub async fn new(mut client: C, config: Config) -> Result<Self, DiscoveryError> {
        let discovery_result = discover_models(
            &mut client,
            &config.discovery_addresses,
            config.read_timeout,
        )
        .await?;
        Ok(Self {
            client,
            config,
            models: discovery_result.models,
            unknown_models: discovery_result.unknown_models,
        })
    }
    /// Read model data from modbus
    ///
    /// Note: Some models are too big to be fetched in a single request
    ///       and multiple read_holding_registers calls will be issued.
    pub async fn read_model<M: Model>(&mut self) -> Result<M, ReadModelError> {
        let addr = M::addr(&self.models);
        read_model(
            &mut self.client,
            addr,
            self.config.max_read_length,
            self.config.read_timeout,
        )
        .await
    }
    /// Read data for a single point. Please note that
    /// `read_model` is more efficient when loading multiple
    /// points from a single model.
    pub async fn read_point<M: Model, T: Value>(
        &mut self,
        point: Point<M, T>,
    ) -> Result<T, ReadPointError> {
        let model_addr = M::addr(&self.models);
        read_point(
            &mut self.client,
            model_addr,
            point,
            self.config.read_timeout,
        )
        .await
    }
    /// Write data for a single point
    pub async fn write_point<M: Model, T: Value>(
        &mut self,
        point: Point<M, T>,
        value: T,
    ) -> Result<(), WritePointError> {
        let model_addr = M::addr(&self.models);
        write_point(
            &mut self.client,
            model_addr,
            point,
            value,
            self.config.write_timeout,
        )
        .await
    }
}

/// Async Modbus client
pub trait AsyncModbusClient {
    /// Read registers from Modbus device
    fn read_registers(
        &mut self,
        addr: u16,
        len: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ModbusError>> + Send;
    /// Write registers to Modbus device
    fn write_registers(
        &mut self,
        addr: u16,
        data: &[u16],
    ) -> impl Future<Output = Result<(), ModbusError>> + Send;
}

async fn read_holding_registers_array<const CNT: usize>(
    client: &mut impl AsyncModbusClient,
    addr: u16,
) -> Result<[u16; CNT], ModbusError> {
    // Unwrap is fine here as read_holding_registers is guaranteed to
    // return the right amount of words.
    client.read_registers(addr, CNT as u16).await.map(|words| {
        words
            .try_into()
            .expect("read_holding_registers returned the wrong amount of words")
    })
}

/// This function implements the "Device Information Model Discovery"
/// as explained in [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
async fn discover_models(
    client: &mut impl AsyncModbusClient,
    discovery_addresses: &[u16],
    read_timeout: Option<Duration>,
) -> Result<DiscoveryResult, DiscoveryError> {
    // Read addresses 0, 40000 and 50000 looking for the SunS identifier
    let mut info_model_addr: Option<u16> = None;
    for &addr in discovery_addresses.iter() {
        // TODO add timeout
        match apply_timeout(
            read_holding_registers_array::<2>(client, addr),
            read_timeout,
        )
        .await
        {
            Ok(identifier) if identifier == SUNS_IDENTIFIER => {
                info_model_addr = Some(addr);
                break;
            }
            Ok(_) => continue,
            Err(ModbusError::Timeout) => continue,
            Err(ModbusError::IllegalDataAddress) => continue,
            Err(e) => return Err(e.into()),
        }
    }
    let Some(mut addr) = info_model_addr else {
        return Err(DiscoveryError::SunsIdentifierNotFound);
    };

    addr += 2;

    let mut models = Models::default();
    let mut unknown_models: Vec<UnknownModel> = vec![];
    let mut model_count = 0;

    loop {
        let res = apply_timeout(read_holding_registers_array::<2>(client, addr), read_timeout).await;
        let [model_id, len] = match res {
            Ok(d) => d,
            Err(ModbusError::IllegalDataAddress) => {
                if model_count > 0 {
                    [0xFFFF, 0] //simulate end for devices without end model
                } else {
                    return Err(DiscoveryError::ModbusError(ModbusError::IllegalDataAddress));
                }
            }
            Err(e) => return Err(DiscoveryError::ModbusError(e)),
        };

        if model_id == 0xFFFF {
            break;
        }

        model_count += 1;

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
async fn read_model<M: Model>(
    client: &mut impl AsyncModbusClient,
    addr: ModelAddr<M>,
    max_read_length: u16,
    read_timeout: Option<Duration>,
) -> Result<M, ReadModelError> {
    let data = if addr.len <= max_read_length {
        apply_timeout(client.read_registers(addr.addr, addr.len), read_timeout).await?
    } else {
        let mut data: Vec<u16> = Vec::with_capacity(addr.len.into());
        let begin = addr.addr;
        let start = addr.addr + addr.len;
        let ranges = (begin..start)
            .step_by(max_read_length as usize)
            .map(|x| x..((x + max_read_length).min(start)));
        for range in ranges {
            let chunk = apply_timeout(
                client.read_registers(
                    range.start,
                    range
                        .len()
                        .try_into()
                        .expect("read_holding_registers returned the wrong amount of words"),
                ),
                read_timeout,
            )
            .await?;
            data.extend(chunk);
        }
        data
    };
    Ok(M::from_data(&data)?)
}

/// Read data for a single point. Please note that
/// `read_model` is more efficient when loading multiple
/// points from a single model.
async fn read_point<M: Model, T: Value>(
    client: &mut impl AsyncModbusClient,
    model_addr: ModelAddr<M>,
    point: Point<M, T>,
    read_timeout: Option<Duration>,
) -> Result<T, ReadPointError> {
    let data = apply_timeout(
        client.read_registers(model_addr.addr + point.offset, point.length),
        read_timeout,
    )
    .await?;
    Ok(Value::decode(&data)?)
}

/// Write data for a single point
async fn write_point<M: Model, T: Value>(
    client: &mut impl AsyncModbusClient,
    model_addr: ModelAddr<M>,
    point: Point<M, T>,
    value: T,
    write_timeout: Option<Duration>,
) -> Result<(), WritePointError> {
    let data = value.encode();
    if data.len() > point.length as usize {
        return Err(WritePointError::ValueTooLarge);
    }
    apply_timeout(
        client.write_registers(model_addr.addr + point.offset, &data),
        write_timeout,
    )
    .await?;
    Ok(())
}

async fn apply_timeout<T>(
    fut: impl Future<Output = Result<T, ModbusError>>,
    timeout: Option<Duration>,
) -> Result<T, ModbusError> {
    if let Some(timeout) = timeout {
        tokio::time::timeout(timeout, fut)
            .await
            .map_err(|_| ModbusError::Timeout)?
    } else {
        fut.await
    }
}

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
    /// Client configuration
    pub config: Config,
}

impl<C: AsyncModbusClient> AsyncClient<C> {
    /// Create new AsyncClient using a `AsyncModbusClient` and a `Config`
    pub fn new(client: impl IntoAsyncModbusClient<C>, config: Config) -> Self {
        Self {
            client: client.into_async_modbus_client(),
            config,
        }
    }
    /// Perform "Device Information Model Discovery" as explained in
    /// [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
    /// for all slave IDs (0..=255) and return a vector of discovered
    /// devices.
    pub async fn devices(&self) -> Vec<AsyncDevice<C>> {
        let mut devices = Vec::new();
        for slave_id in 0..=255 {
            if let Ok(device) = self.device(slave_id).await {
                devices.push(device);
            }
        }
        devices
    }
    /// Perform "Device Information Model Discovery" as explained in
    /// [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
    /// for a single slave ID and return the discovered device.
    pub async fn device(&self, slave_id: u8) -> Result<AsyncDevice<C>, DiscoveryError> {
        let discovery_result = discover_models(
            &self.client,
            slave_id,
            &self.config.discovery_addresses,
            self.config.read_timeout,
        )
        .await?;
        Ok(AsyncDevice {
            client: self.client.clone(),
            config: self.config.clone(),
            slave_id,
            models: discovery_result.models,
            unknown_models: discovery_result.unknown_models,
        })
    }
}

/// Client structure for a discovered device
#[derive(Debug)]
pub struct AsyncDevice<C: AsyncModbusClient> {
    /// This is the actual modbus client which implements the `AsyncModbusClient` trait.
    pub client: C,
    /// Client configuration
    pub config: Config,
    /// The Slave ID
    pub slave_id: u8,
    /// Discovered models
    pub models: Models,
    /// Unknown models
    pub unknown_models: Vec<UnknownModel>,
}

impl<C: AsyncModbusClient> AsyncDevice<C> {
    /// Read model data from modbus
    ///
    /// Note: Some models are too big to be fetched in a single request
    ///       and multiple read_holding_registers calls will be issued.
    pub async fn read_model<M: Model>(&self) -> Result<M, ReadModelError> {
        let addr = M::addr(&self.models);
        read_model(
            &self.client,
            self.slave_id,
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
        &self,
        point: Point<M, T>,
    ) -> Result<T, ReadPointError> {
        let model_addr = M::addr(&self.models);
        read_point(
            &self.client,
            self.slave_id,
            model_addr,
            point,
            self.config.read_timeout,
        )
        .await
    }
    /// Write data for a single point
    pub async fn write_point<M: Model, T: Value>(
        &self,
        point: Point<M, T>,
        value: T,
    ) -> Result<(), WritePointError> {
        let model_addr = M::addr(&self.models);
        write_point(
            &self.client,
            self.slave_id,
            model_addr,
            point,
            value,
            self.config.write_timeout,
        )
        .await
    }
}

/// Async Modbus client
pub trait AsyncModbusClient: Sync + Clone {
    /// Read registers from Modbus device
    fn read_registers(
        &self,
        slave_id: u8,
        addr: u16,
        len: u16,
    ) -> impl Future<Output = Result<Vec<u16>, ModbusError>> + Send;
    /// Write registers to Modbus device
    fn write_registers(
        &self,
        slave_id: u8,
        addr: u16,
        data: &[u16],
    ) -> impl Future<Output = Result<(), ModbusError>> + Send;
}

pub trait IntoAsyncModbusClient<C: AsyncModbusClient> {
    fn into_async_modbus_client(self) -> C;
}

impl<C: AsyncModbusClient> IntoAsyncModbusClient<C> for C {
    fn into_async_modbus_client(self) -> C {
        self
    }
}

async fn read_holding_registers_array<const CNT: usize>(
    client: &impl AsyncModbusClient,
    slave_id: u8,
    addr: u16,
) -> Result<[u16; CNT], ModbusError> {
    // Unwrap is fine here as read_holding_registers is guaranteed to
    // return the right amount of words.
    client
        .read_registers(slave_id, addr, CNT as u16)
        .await
        .map(|words| {
            words
                .try_into()
                .expect("read_holding_registers returned the wrong amount of words")
        })
}

/// This function implements the "Device Information Model Discovery"
/// as explained in [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
async fn discover_models(
    client: &impl AsyncModbusClient,
    slave_id: u8,
    discovery_addresses: &[u16],
    read_timeout: Option<Duration>,
) -> Result<DiscoveryResult, DiscoveryError> {
    // Read addresses 0, 40000 and 50000 looking for the SunS identifier
    let mut info_model_addr: Option<u16> = None;
    for &addr in discovery_addresses.iter() {
        // TODO add timeout
        match apply_timeout(
            read_holding_registers_array::<2>(client, slave_id, addr),
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
        let res = apply_timeout(
            read_holding_registers_array::<2>(client, slave_id, addr),
            read_timeout,
        )
        .await;

        let [model_id, len] = match res {
            // End model found. Exit the loop.
            Ok([0xFFFF, _]) => break,
            // Some devices like SMA STP10.0-3SE-40 do not have an end model
            // and discovery fails with an IllegalDataAddress modbus error.
            // Work around that by pretending an end model was found when an
            // IllegalDataAddress error occurs during discovery and at least
            // one valid model has been found before.
            Err(ModbusError::IllegalDataAddress) if model_count > 0 => break,
            x => x,
        }?;

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
    client: &impl AsyncModbusClient,
    slave_id: u8,
    addr: ModelAddr<M>,
    max_read_length: u16,
    read_timeout: Option<Duration>,
) -> Result<M, ReadModelError> {
    let data = if addr.len <= max_read_length {
        apply_timeout(
            client.read_registers(slave_id, addr.addr, addr.len),
            read_timeout,
        )
        .await?
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
                    slave_id,
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
    client: &impl AsyncModbusClient,
    slave_id: u8,
    model_addr: ModelAddr<M>,
    point: Point<M, T>,
    read_timeout: Option<Duration>,
) -> Result<T, ReadPointError> {
    let data = apply_timeout(
        client.read_registers(slave_id, model_addr.addr + point.offset, point.length),
        read_timeout,
    )
    .await?;
    Ok(Value::decode(&data)?)
}

/// Write data for a single point
async fn write_point<M: Model, T: Value>(
    client: &impl AsyncModbusClient,
    slave_id: u8,
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
        client.write_registers(slave_id, model_addr.addr + point.offset, &data),
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

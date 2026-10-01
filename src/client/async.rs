use std::{future::Future, num::NonZeroU16, time::Duration};

use crate::{DiscoveredModel, Model, ModelInfo, ModelKind, Models, SUNS_IDENTIFIER};

use super::{
    error::ModbusError, Config, DiscoveryError, DiscoveryResult, LookupError, ModelHandle,
    UnknownModel,
};

/// Async client
///
/// Devices created by this client copy its Modbus client and
/// configuration, so both can be changed between discoveries.
#[derive(Debug)]
#[non_exhaustive]
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
        let discovery = discover_models(
            &self.client,
            slave_id,
            &self.config.discovery_addresses,
            self.config.read_timeout,
        )
        .await?;
        Ok(self.device_from_discovery(slave_id, discovery))
    }
    /// Create a device from the result of a previous model discovery
    /// without performing the discovery again.
    ///
    /// The discovery result must have been returned by
    /// [`AsyncDevice::discovery`] for the same device and the register
    /// map of the device must not have changed since. Otherwise models
    /// are read from and written to the wrong registers. The SunSpec
    /// specification does not guarantee that the register map of a
    /// device stays the same.
    pub fn device_from_discovery(
        &self,
        slave_id: u8,
        discovery: DiscoveryResult,
    ) -> AsyncDevice<C> {
        AsyncDevice {
            client: self.client.clone(),
            config: self.config.clone(),
            slave_id,
            discovery,
        }
    }
}

/// Client structure for a discovered device
///
/// The client, slave id and discovery result are only readable as the
/// selected models rely on them describing the same device.
#[derive(Debug)]
pub struct AsyncDevice<C: AsyncModbusClient> {
    pub(super) client: C,
    /// Client configuration. It can be changed at any time, e.g. to adjust
    /// the timeouts for this device.
    pub config: Config,
    pub(super) slave_id: u8,
    discovery: DiscoveryResult,
}

impl<C: AsyncModbusClient> AsyncDevice<C> {
    /// The Modbus client used to communicate with this device
    pub fn client(&self) -> &C {
        &self.client
    }
    /// The slave id of this device
    pub fn slave_id(&self) -> u8 {
        self.slave_id
    }
    /// The result of the model discovery of this device
    pub fn discovery(&self) -> &DiscoveryResult {
        &self.discovery
    }
    /// Select the model of the given type.
    ///
    /// Returns an error if the model was not discovered or discovered
    /// more than once. Use [`models`](Self::models) for models which can
    /// be contained multiple times.
    pub fn model<M: Model>(&self) -> Result<ModelHandle<'_, C, M>, LookupError> {
        let mut models = self.models::<M>();
        let model = models
            .next()
            .ok_or(LookupError::ModelNotDiscovered { model_id: M::ID })?;
        if models.next().is_some() {
            return Err(LookupError::ModelNotUnique { model_id: M::ID });
        }
        Ok(model)
    }
    /// Select all models of the given type in the order they appear in
    /// the Modbus map of the device. Use [`AnyModel`](crate::AnyModel) to select all
    /// models.
    pub fn models<M: ModelKind>(&self) -> impl Iterator<Item = ModelHandle<'_, C, M>> + '_ {
        self.discovery
            .models
            .iter()
            .filter(|model| M::matches(model.info()))
            .map(move |model| ModelHandle::new(self, *model))
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
    read_timeout: Option<Duration>,
) -> Result<[u16; CNT], ModbusError> {
    let len = CNT as u16;
    let data = read_registers(client, slave_id, addr, len, read_timeout).await?;
    data.try_into()
        .map_err(|data: Vec<u16>| ModbusError::InvalidResponseLength {
            expected: len,
            actual: data.len(),
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
        match read_holding_registers_array::<2>(client, slave_id, addr, read_timeout).await {
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
    let Some(addr) = info_model_addr else {
        return Err(DiscoveryError::SunsIdentifierNotFound);
    };

    let mut addr = addr.checked_add(2).ok_or(DiscoveryError::AddressOverflow)?;

    let mut models = Models::default();
    let mut unknown_models: Vec<UnknownModel> = vec![];
    let mut model_count = 0;

    loop {
        let res = read_holding_registers_array::<2>(client, slave_id, addr, read_timeout).await;

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

        let model_addr = addr
            .checked_add(2)
            .and_then(NonZeroU16::new)
            .ok_or(DiscoveryError::AddressOverflow)?;
        addr = model_addr.get();
        match ModelInfo::by_id(model_id) {
            Some(info) => models.push(DiscoveredModel::new(info, model_addr, len)),
            None => unknown_models.push(UnknownModel {
                id: model_id,
                addr,
                len,
            }),
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

/// Read registers from modbus and check that the expected number of
/// registers was returned.
pub(super) async fn read_registers(
    client: &impl AsyncModbusClient,
    slave_id: u8,
    addr: u16,
    len: u16,
    read_timeout: Option<Duration>,
) -> Result<Vec<u16>, ModbusError> {
    let data = apply_timeout(client.read_registers(slave_id, addr, len), read_timeout).await?;
    if data.len() != usize::from(len) {
        return Err(ModbusError::InvalidResponseLength {
            expected: len,
            actual: data.len(),
        });
    }
    Ok(data)
}

/// Read the registers of a discovered model. If `len` exceeds
/// `max_read_length` multiple read_holding_registers calls will be issued.
pub(super) async fn read_registers_chunked(
    client: &impl AsyncModbusClient,
    slave_id: u8,
    model: &DiscoveredModel,
    max_read_length: u16,
    read_timeout: Option<Duration>,
) -> Result<Vec<u16>, ModbusError> {
    // A chunk size of 0 would never make any progress.
    let max_read_length = max_read_length.max(1);
    let len = model.len();
    let mut data: Vec<u16> = Vec::with_capacity(len.into());
    let mut offset = 0;
    while offset < len {
        let chunk_len = (len - offset).min(max_read_length);
        // A discovered model always ends within the address space, so
        // this can't overflow.
        let addr = model.addr() + offset;
        let chunk = read_registers(client, slave_id, addr, chunk_len, read_timeout).await?;
        data.extend(chunk);
        offset += chunk_len;
    }
    Ok(data)
}

pub(super) async fn apply_timeout<T>(
    fut: impl Future<Output = Result<T, ModbusError>>,
    timeout: Option<Duration>,
) -> Result<T, ModbusError> {
    #[cfg(feature = "tokio")]
    {
        if let Some(timeout) = timeout {
            tokio::time::timeout(timeout, fut)
                .await
                .map_err(|_| ModbusError::Timeout)?
        } else {
            fut.await
        }
    }

    #[cfg(not(feature = "tokio"))]
    {
        let _ = timeout;
        fut.await
    }
}

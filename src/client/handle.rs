use std::{fmt, marker::PhantomData};

use crate::{
    sealed::ModelKindImpl, AnyModel, DiscoveredModel, Model, ModelInfo, ModelKind, ParseError,
    Point, Value,
};

use super::{
    r#async::{apply_timeout, read_registers_chunked},
    AsyncDevice, AsyncModbusClient, ReadModelError, ReadPointError, WritePointError,
};

/// A model of a device which was found during model discovery.
///
/// It is returned by [`AsyncDevice::model`] and [`AsyncDevice::models`].
/// The type parameter `M` is either a specific model type or
/// [`AnyModel`] for accessing any model dynamically.
pub struct ModelHandle<'a, C: AsyncModbusClient, M = AnyModel> {
    device: &'a AsyncDevice<C>,
    model: DiscoveredModel,
    kind: PhantomData<fn() -> M>,
}

impl<'a, C: AsyncModbusClient, M> ModelHandle<'a, C, M> {
    pub(super) fn new(device: &'a AsyncDevice<C>, model: DiscoveredModel) -> Self {
        Self {
            device,
            model,
            kind: PhantomData,
        }
    }
    /// Information about the model.
    pub fn info(&self) -> &'static ModelInfo {
        self.model.info()
    }
    /// The discovered address of the first register following the
    /// model id and length registers.
    pub fn addr(&self) -> u16 {
        self.model.addr()
    }
    /// The discovered length of the model. This is the number of
    /// registers following the model id and length registers.
    // This is not a collection, so `is_empty` would make no sense.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u16 {
        self.model.len()
    }
    /// The discovered model.
    pub fn discovered(&self) -> &DiscoveredModel {
        &self.model
    }
}

impl<C: AsyncModbusClient, M: ModelKind> ModelHandle<'_, C, M> {
    /// Read the model data.
    ///
    /// Note: Some models are too big to be fetched in a single request
    ///       and multiple read_holding_registers calls will be issued.
    pub async fn read(&self) -> Result<M, ReadModelError> {
        let device = self.device;
        let data = read_registers_chunked(
            &device.client,
            device.slave_id,
            self.model.addr(),
            self.model.len(),
            device.config.max_read_length,
            device.config.read_timeout,
        )
        .await?;
        M::parse(self.model.info(), &data).map_err(|error| match error {
            ParseError::TooShort => ReadModelError::ModelTooShort {
                model_id: self.model.info().id,
                len: self.model.len(),
            },
            error => error.into(),
        })
    }
}

impl<'a, C: AsyncModbusClient, M: Model> ModelHandle<'a, C, M> {
    /// Read data for a single point. Please note that
    /// [`read`](Self::read) is more efficient when loading multiple
    /// points from a single model.
    pub async fn read_point<T: Value>(&self, point: Point<M, T>) -> Result<T, ReadPointError> {
        if !self.contains(&point) {
            return Err(ReadPointError::PointOutOfBounds);
        }
        let device = self.device;
        let data = apply_timeout(
            device.client.read_registers(
                device.slave_id,
                self.model.addr() + point.offset,
                point.length,
            ),
            device.config.read_timeout,
        )
        .await?;
        Ok(T::decode(&data)?)
    }
    /// Write data for a single point.
    pub async fn write_point<T: Value>(
        &self,
        point: Point<M, T>,
        value: T,
    ) -> Result<(), WritePointError> {
        if !self.contains(&point) {
            return Err(WritePointError::PointOutOfBounds);
        }
        let data = value.encode();
        if data.len() > point.length as usize {
            return Err(WritePointError::ValueTooLarge);
        }
        let device = self.device;
        apply_timeout(
            device
                .client
                .write_registers(device.slave_id, self.model.addr() + point.offset, &data),
            device.config.write_timeout,
        )
        .await?;
        Ok(())
    }
    /// Convert this handle into a handle for accessing the model
    /// dynamically as [`AnyModel`].
    pub fn untyped(self) -> ModelHandle<'a, C, AnyModel> {
        ModelHandle::new(self.device, self.model)
    }
    /// Check whether the point lies within the discovered model length.
    fn contains<T: Value>(&self, point: &Point<M, T>) -> bool {
        u32::from(point.offset) + u32::from(point.length) <= u32::from(self.model.len())
    }
}

impl<'a, C: AsyncModbusClient> ModelHandle<'a, C, AnyModel> {
    /// Convert this handle into a handle of the given model type.
    /// Returns `None` if the model is of a different type.
    pub fn downcast<M: Model>(self) -> Option<ModelHandle<'a, C, M>> {
        M::matches(self.model.info()).then(|| ModelHandle::new(self.device, self.model))
    }
}

// The following impls are written manually as `#[derive(...)]` would
// add `C: Clone`/`M: Clone`/... bounds which are not needed as the
// handle only contains a reference to the device and `M` is only used
// as a `PhantomData` marker.

impl<C: AsyncModbusClient, M> Clone for ModelHandle<'_, C, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: AsyncModbusClient, M> Copy for ModelHandle<'_, C, M> {}

impl<C: AsyncModbusClient, M> fmt::Debug for ModelHandle<'_, C, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModelHandle")
            .field("slave_id", &self.device.slave_id)
            .field("model", &self.model)
            .finish_non_exhaustive()
    }
}

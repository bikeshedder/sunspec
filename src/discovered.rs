use std::num::NonZeroU16;

use crate::ModelInfo;

/// A model which was found during model discovery.
///
/// The address and length are only exposed for debugging purposes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize),
    serde(
        try_from = "serde_impl::DiscoveredModel",
        into = "serde_impl::DiscoveredModel"
    )
)]
pub struct DiscoveredModel {
    info: &'static ModelInfo,
    // A model can never start at address 0 as it is always preceded by
    // the SunS identifier.
    addr: NonZeroU16,
    // `addr + len` never exceeds `u16::MAX`. This is ensured by the model
    // discovery and when deserializing.
    len: u16,
}

impl DiscoveredModel {
    pub(crate) const fn new(info: &'static ModelInfo, addr: NonZeroU16, len: u16) -> Self {
        Self { info, addr, len }
    }
    /// Information about the model.
    pub const fn info(&self) -> &'static ModelInfo {
        self.info
    }
    /// The discovered address of the first register following the
    /// model id and length registers.
    pub const fn addr(&self) -> u16 {
        self.addr.get()
    }
    /// The discovered length of the model. This is the number of
    /// registers following the model id and length registers.
    // This is not a collection, so `is_empty` would make no sense.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> u16 {
        self.len
    }
}

/// The models which were found during model discovery and are enabled
/// via Cargo features.
///
/// A model can be contained multiple times.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize),
    serde(transparent)
)]
pub struct Models {
    models: Vec<DiscoveredModel>,
}

impl Models {
    pub(crate) fn push(&mut self, model: DiscoveredModel) {
        self.models.push(model);
    }
    /// Returns an iterator over all discovered models in the order they
    /// appear in the Modbus map of the device.
    pub fn iter(&self) -> impl Iterator<Item = &DiscoveredModel> + '_ {
        self.models.iter()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use std::num::NonZeroU16;

    use crate::ModelInfo;

    /// Serialized form of [`super::DiscoveredModel`] which contains the
    /// model id instead of the model information.
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub(super) struct DiscoveredModel {
        id: u16,
        addr: NonZeroU16,
        len: u16,
    }

    impl From<super::DiscoveredModel> for DiscoveredModel {
        fn from(model: super::DiscoveredModel) -> Self {
            Self {
                id: model.info.id,
                addr: model.addr,
                len: model.len,
            }
        }
    }

    impl TryFrom<DiscoveredModel> for super::DiscoveredModel {
        type Error = String;
        fn try_from(model: DiscoveredModel) -> Result<Self, Self::Error> {
            let info = ModelInfo::by_id(model.id)
                .ok_or_else(|| format!("Unknown model id: {}", model.id))?;
            if model.addr.checked_add(model.len).is_none() {
                return Err(format!(
                    "Model {} at address {} with length {} exceeds the address space",
                    model.id, model.addr, model.len
                ));
            }
            Ok(Self::new(info, model.addr, model.len))
        }
    }
}

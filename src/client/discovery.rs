use thiserror::Error;

use crate::Models;

use super::ModbusError;

/// For every discovered but unknown model to this library
/// this structure is returned.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct UnknownModel {
    /// ID of the discovered model
    pub id: u16,
    /// Address of the discovered model
    pub addr: u16,
    /// Length of the discovered model
    pub len: u16,
}

/// The result of a SunSpec model discovery.
///
/// It can be passed to
/// [`AsyncClient::device_from_discovery`](super::AsyncClient::device_from_discovery)
/// to create a device without performing the discovery again.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DiscoveryResult {
    /// The addresses of the discovered models.
    pub models: Models,
    /// Unknown models with their addresses and lengths.
    pub unknown_models: Vec<UnknownModel>,
}

/// This error is returned when an error occurs during model discovery.
#[derive(Debug, Error)]
pub enum DiscoveryError {
    /// Communication error.
    #[error("Modbus error: {0}")]
    Modbus(#[from] ModbusError),
    /// The Modbus slave did not provide the "SunS" header at any of the
    /// discovery addresses (see
    /// [`Config::discovery_addresses`](super::Config::discovery_addresses)).
    #[error("SunS identifier not found")]
    SunsIdentifierNotFound,
    /// The addresses would overflow while discovering models. The slave
    /// device seems to be returning garbage data.
    #[error("Address overflow detected")]
    AddressOverflow,
}

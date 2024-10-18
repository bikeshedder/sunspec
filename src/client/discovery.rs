use thiserror::Error;

use crate::model::{Model, ModelAddr};
use crate::models::Models;

use super::error::CommunicationError;

impl<M: Model> Copy for ModelAddr<M> {}

impl<M: Model> Clone for ModelAddr<M> {
    fn clone(&self) -> Self {
        *self
    }
}

/// For every discovered but unknown model to this library
/// this structure is returned.
#[derive(Debug)]
pub struct UnknownModel {
    /// ID of the discovered model
    pub id: u16,
    /// Address of the discovered model
    pub addr: u16,
    /// Length of the discovered model
    pub len: u16,
}

/// The result of a SunSpec model discovery.
#[derive(Debug)]
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
    #[error("Communication: {0}")]
    Communication(#[from] CommunicationError),
    /// The Modbus slave did not provide the "SunS" header at the well known
    /// addresses 0, 40000 or 50000.
    #[error("SunS identifier not found")]
    SunsIdentifierNotFound,
    /// The addresses would overflow while discovering modules. The slave
    /// device seams to be returning garbage data.
    #[error("Address overflow detected")]
    AddressOverflow,
}

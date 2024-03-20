use std::{io, marker::PhantomData};

use thiserror::Error;

use crate::model::Model;
use crate::models::Models;

/// "SunS" identifier used when performing the
/// model discovery.
pub const SUNS_IDENTIFIER: u32 = 0x53756e53; // SunS

/// This structure is used to store the address of
/// models after a successful model discovery.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ModelAddr<M: Model> {
    /// The discovered address of this model.
    pub addr: u16,
    /// The discovered length of this model. A length of
    /// 0 indicates that the model is unsupported.
    pub len: u16,
    model: PhantomData<M>,
}

impl<M: Model> ModelAddr<M> {
    /// Set the address of a discovered model
    pub fn set_addr(&mut self, addr: u16, len: u16) {
        self.addr = addr;
        self.len = len;
    }
}

impl<M: Model> Default for ModelAddr<M> {
    fn default() -> Self {
        Self {
            addr: Default::default(),
            len: Default::default(),
            model: Default::default(),
        }
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
    /// I/O error occured. Please note that all errors returned by `tokio-modbus`
    /// are stored inside this I/O error.
    #[error("I/O Error")]
    IO(#[from] io::Error),
    /// The Modbus slave did not provide the "SunS" header at the well known
    /// addresses 0, 40000 or 50000.
    #[error("SunS identifier not found")]
    SunsIdentifierNotFound,
    /// The addresses would overflow while discovering modules. The slave
    /// device seams to be returning garbage data.
    #[error("Address overflow detected")]
    AddressOverflow,
}

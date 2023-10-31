use std::{io, marker::PhantomData};

use itertools::Itertools;
use thiserror::Error;
use tokio_modbus::{
    client::{Context, Reader},
    Address,
};
use tracing::debug;

use crate::{model::Model, models::Models, value::FixedSize};

async fn read_fixed_size<T: FixedSize>(context: &mut Context, addr: Address) -> io::Result<T> {
    let words = context.read_holding_registers(addr, T::SIZE).await?;
    Ok(T::decode(&words).unwrap())
}

const SUNS_IDENTIFIER: u32 = 0x53756e53; // SunS

/// This structure is used to store the address of
/// models after a successful model discovery.
#[derive(Debug)]
pub struct ModelAddr<M: Model> {
    pub(crate) addr: u16,
    pub(crate) len: u16,
    model: PhantomData<M>,
}

impl<M: Model> ModelAddr<M> {
    pub(crate) fn set_addr(&mut self, addr: u16, len: u16) {
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

/// This function implements the "Device Information Model Discovery"
/// as explained in [SunSpec Device Information Specification V1.1](https://sunspec.org/wp-content/uploads/2022/05/SunSpec-Device-Information-Model-Specificiation-V1-1-final.pdf)
pub async fn discover_models(context: &mut Context) -> Result<Models, DiscoverError> {
    // Read addresses 0, 40000 and 50000 looking for the SunS identifier
    let mut info_model_addr: Option<u16> = None;
    for addr in [0, 40000, 50000] {
        let identifier = read_fixed_size::<u32>(context, addr).await?;
        if identifier == SUNS_IDENTIFIER {
            info_model_addr = Some(addr);
            break;
        }
    }
    let Some(mut addr) = info_model_addr else {
        return Err(DiscoverError::SunsIdentifierNotFound);
    };

    addr += 2;

    let mut models = Models::default();
    let mut unknown_models: Vec<u16> = vec![];

    loop {
        let [model_id, len] = *context.read_holding_registers(addr, 2).await? else {
            unreachable!();
        };
        if model_id == 0xFFFF {
            break;
        }
        addr = addr.checked_add(2).ok_or(DiscoverError::AddressOverflow)?;
        if !models.set_addr(model_id, addr, len) {
            unknown_models.push(model_id);
        }
        addr = addr
            .checked_add(len)
            .ok_or(DiscoverError::AddressOverflow)?;
    }

    if !unknown_models.is_empty() {
        debug!(
            "Ignoring unknown models: {}",
            unknown_models.iter().map(|id| id.to_string()).join(", ")
        );
    }

    Ok(models)
}

/// This error is returned when an error occurs during model discovery.
#[derive(Debug, Error)]
pub enum DiscoverError {
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

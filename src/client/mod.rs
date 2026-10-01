mod r#async;
mod config;
mod discovery;
mod error;
mod handle;
#[cfg(feature = "tokio-modbus")]
pub mod tokio_modbus;

pub use config::{
    Config, DEFAULT_MAX_READ_LENGTH, DEFAULT_MAX_WRITE_LENGTH, DEFAULT_READ_TIMEOUT,
    DEFAULT_WRITE_TIMEOUT,
};
pub use discovery::{DiscoveryError, DiscoveryResult, UnknownModel};
pub use error::{LookupError, ModbusError, ReadModelError, ReadPointError, WritePointError};
pub use handle::ModelHandle;
pub use r#async::{AsyncClient, AsyncDevice, AsyncModbusClient};

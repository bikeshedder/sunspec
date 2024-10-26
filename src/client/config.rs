use std::time::Duration;

use crate::DEFAULT_DISCOVERY_ADDRESSES;

/// Default timeout when reading registers
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(1);

/// Default timeout when reading registers
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(1);

/// Modbus defines that a maximum of 125 registers can be read
/// in a single request. See 6.4, Page 16:
/// See: https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf
pub const DEFAULT_MAX_READ_LENGTH: u16 = 125;

/// Modbus defines that a maximum 123 registers can be written
/// in a single request. See 6.12, Page 30:
/// https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf
pub const DEFAULT_MAX_WRITE_LENGTH: u16 = 123;

/// Client configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Addresses to check for the SunS identifier (default: [0, 40000, 50000])
    ///
    /// Some devices don't work according to the specification and don't respond
    /// anything on address 0.
    pub discovery_addresses: Vec<u16>,
    /// Timeout when reading registers
    pub read_timeout: Option<Duration>,
    /// Maximum chunk size when reading registers
    pub max_read_length: u16,
    /// Timeout when writing registers
    pub write_timeout: Option<Duration>,
    /// Maximum chunk size when writing registers
    pub max_write_length: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discovery_addresses: DEFAULT_DISCOVERY_ADDRESSES.into(),
            read_timeout: Some(DEFAULT_READ_TIMEOUT),
            write_timeout: Some(DEFAULT_WRITE_TIMEOUT),
            max_read_length: DEFAULT_MAX_READ_LENGTH,
            max_write_length: DEFAULT_MAX_WRITE_LENGTH,
        }
    }
}

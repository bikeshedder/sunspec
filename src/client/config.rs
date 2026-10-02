use std::time::Duration;

use derive_builder::Builder;

use crate::DEFAULT_DISCOVERY_ADDRESSES;

/// Default timeout when reading registers
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(1);

/// Default timeout when writing registers
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(1);

/// Modbus defines that a maximum of 125 registers can be read
/// in a single request. See 6.4, Page 16:
/// See: <https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf>
pub const DEFAULT_MAX_READ_LENGTH: u16 = 125;

/// Modbus defines that a maximum 123 registers can be written
/// in a single request. See 6.12, Page 30:
/// <https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf>
pub const DEFAULT_MAX_WRITE_LENGTH: u16 = 123;

/// Client configuration
///
/// Use [`Config::default`] or [`Config::builder`] to create it:
///
/// ```
/// # use sunspec::client::Config;
/// let config = Config::builder()
///     .discovery_addresses(vec![40000])
///     .read_timeout(None)
///     .build();
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Builder)]
#[builder(pattern = "owned", default, build_fn(private, name = "fallible_build"))]
#[non_exhaustive]
pub struct Config {
    /// Addresses to check for the SunS identifier (default: [40000, 0, 50000])
    ///
    /// The addresses are checked in order. 40000 comes first because some
    /// devices don't work according to the specification and misbehave when
    /// address 0 is queried.
    pub discovery_addresses: Vec<u16>,
    /// Timeout when reading registers
    pub read_timeout: Option<Duration>,
    /// Maximum chunk size when reading registers. A value of 0 is treated
    /// as 1.
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

impl Config {
    /// Create a builder for the client configuration. All fields which
    /// are not set use the value of [`Config::default`].
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl ConfigBuilder {
    /// Build the client configuration.
    pub fn build(self) -> Config {
        // Every field falls back to the value of `Config::default`, so
        // building can't fail.
        self.fallible_build().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        assert_eq!(Config::builder().build(), Config::default());
        let config = Config::builder().max_read_length(10).build();
        assert_eq!(config.max_read_length, 10);
        assert_eq!(config.read_timeout, Some(DEFAULT_READ_TIMEOUT));
    }
}

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Update `strum` to version `0.27`

## [0.8.0] - 2024-12-10

### Added

- Add multi device support

### Changed

- Update `tokio-modbus` to version `0.16`
- Update `thiserror` to version `2.0`
- Increase MSRV to 1.76

## [0.7.2] - 2024-12-10

### Changed

- Add workaround to model discovery for devices without end model (e.g. some SMA inverters)

## [0.7.1] - 2024-11-06

### Fixed

- Fix timeout handling in model discovery

### Changed

- Change default order of discovery addresses to `[40000, 0, 50000]`. A lot
  of devices timeout at address `0` and the Python implementation from the
  sunspec Alliance uses the same order: [SunSpecModbusClientDevice.base\_addr\_list]

[SunSpecModbusClientDevice.base_addr_list]: https://github.com/sunspec/pysunspec2/blob/7d27273e8568c48e54186ce7bfea3f4573b21deb/sunspec2/modbus/client.py#L193

### [0.7.0] - 2024-10-26

### Added

- Add `AsyncClient` which provides a more ergonomic API

### Changed

- `ModelAddr` and `PointRef` now implement `Copy` and `Clone` and are passed
  by value and not by reference.
- Move all client specific code into `client` module.
- Rename `PointDef` to `Point`.

### Removed

- Remove old client functions

## [0.6.1] - 2024-10-18

### Fixed

- Add `+ Sync` to `CommunicationError::Modbus` variant

## [0.6.0] - 2024-10-17

### Fixed

- Add `+ Send` to `CommunicationError::Modbus` variant

### Changed

- Update `tokio-modbus` to version `0.15`

## [0.5.0] - 2024-09-11

### Changed

- Add Config struct for timeouts and discovery addresses
- Update `tokio_modbus` to version `0.14`

## [0.4.0] - 2024-03-20

### Added

- Add `serde` (de)serialization support

### Changed

- Update `strum` to version `0.26`
- Update `tokio-modbus` to version `0.11`
- Update sunspec models (2024-02-15)
  - Remove model 64110 (Outback AXS device)

## [0.3.1] - 2023-12-21

### Fixed

- Model discovery no longer fails if the device returns a
  `Illegal data address` for addresses it does not support.

## [0.3.0] - 2023-12-09

### Added

- Generate types for Enum16 and Enum32 points
- Generate types for Bitfield16, Bitfield32 and Bitfield64 points

### Changed

- Load models bigger than 125 registers in chunks
- Change representation of IPv6 addresses to `std::net::Ipv6addr`
- Change representation of IPv4 addresses to `std::net::Ipv4addr`
- Load optional points as None if they don't contain a value
- Use `heck` to generate better `type` and `field` names
- Make `models::model*` module public as they do now contain
  more than just the `Model` struct.

### Removed

- Model\* reexports in `models` module

## [0.2.0] - 2023-11-21

### Added

- Added 7xx models which were only part of the JSON files.

### Changed

- Generate models from JSON instead of SMDX.

### Removed

- Removed the obsolete `LENGTH` field from all models. This
  field will come back once repeating groups are supported.

## [0.1.0] - 2023-11-04

### Added

- First release

[unreleased]: https://github.com/bikeshedder/sunspec/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/bikeshedder/sunspec/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/bikeshedder/sunspec/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/bikeshedder/sunspec/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/bikeshedder/sunspec/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/bikeshedder/sunspec/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/bikeshedder/sunspec/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/bikeshedder/sunspec/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/bikeshedder/sunspec/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/bikeshedder/sunspec/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/bikeshedder/sunspec/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/bikeshedder/sunspec/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/bikeshedder/sunspec/releases/tag/v0.1.0

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/bikeshedder/sunspec/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/bikeshedder/sunspec/releases/tag/v0.4.0
[0.3.1]: https://github.com/bikeshedder/sunspec/releases/tag/v0.3.1
[0.3.0]: https://github.com/bikeshedder/sunspec/releases/tag/v0.3.0
[0.2.0]: https://github.com/bikeshedder/sunspec/releases/tag/v0.2.0
[0.1.0]: https://github.com/bikeshedder/sunspec/releases/tag/v0.1.0

<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add support for accessing models dynamically (#11):
  - `ModelInfo` with the id, name and label of a model. It is available via
    `Model::INFO`, `ModelInfo::by_id` and `str::parse` (accepting e.g.
    `"103"`, `"m103"` or `"inverter_three_phase"`).
  - `MODELS` listing all models enabled via Cargo features
  - `AnyModel` enum containing any model. With the `serde` feature enabled it
    is serialized with an internal `"model"` tag containing the model name.
  - `DynModel` trait providing access to model information via
    `AnyModel::as_dyn`
  - `AsyncDevice::models::<AnyModel>()` selecting all discovered models. They
    are read as `AnyModel` and can be converted into typed models via
    `ModelHandle::downcast`.
  - `Model::NAME`, `Model::LABEL` and `Model::INFO` constants
- Add support for devices containing the same model multiple times.
  `AsyncDevice::models::<M>()` selects all instances of a model in the order
  they appear in the Modbus map.
- Add `AsyncDevice::discovery` and `AsyncClient::device_from_discovery` for
  connecting to a device again without running the model discovery. With the
  `serde` feature enabled `DiscoveryResult` can be serialized.

### Changed

- **Breaking:** Seal the `Model`, `DynModel`, `Group`, `Value`, `FixedSize`
  and `EnumValue` traits. They were never meant to be implemented outside of
  this crate, which allows extending them without further breaking changes.
- Update sunspec models (2026-08-20)
  - Add `subscribed_resource` and `subscription_ena` points to model 64415
  - Fix size of `DeptRef` point in model 64410
- **Breaking:** Fix word splitting of units and acronyms in 392 generated
  identifiers (#26). For example:
  - Fields: `v_ar_rtg_q1` → `var_rtg_q1`, `tot_v_ah_exp` → `tot_vah_exp`,
    `aph_a` → `a_ph_a`, `ph_vph_a` → `ph_v_ph_a`, `dca` → `dc_a`,
    `vl1l2` → `v_l1_l2`
  - Constants: `V_AR_RTG_Q1` → `VAR_RTG_Q1`, `APH_A` → `A_PH_A`
  - Types: `VArAct` → `VarAct`, `PfwAbs` → `PfWAbs`
  - Enum variants and bitflags: `VArMax` → `VarMax`, `VoltVAr` → `VoltVar`
- **Breaking:** `Models` is now a list of `DiscoveredModel`s in the order
  they appear in the Modbus map. The fields for each model (e.g.
  `models.m103`) and `ModelAddr` were removed.
- **Breaking:** Models are accessed via a `ModelHandle` returned by
  `AsyncDevice::model::<M>()` and `AsyncDevice::models::<M>()`. The methods
  `read_model`, `read_point` and `write_point` moved from `AsyncDevice` to
  `ModelHandle` as `read`, `read_point` and `write_point`, e.g.
  `device.model::<Model103>()?.read().await?`. `AsyncDevice::model` returns a
  `LookupError` if the model was not discovered or discovered more than once.
- **Breaking:** The `client`, `slave_id`, `models` and `unknown_models`
  fields of `AsyncDevice` are private. Use the `client()`, `slave_id()` and
  `discovery()` methods instead.
- **Breaking:** Parsing incomplete model data returns
  `ParseError::TooShort` or `ParseError::InvalidGroupLength` instead of
  `DecodeError::OutOfBounds`. `ReadModelError::DecodeError` was replaced by
  `ReadModelError::Parse`.
- **Breaking:** Rename `DecodeError::OutOfBounds` to
  `DecodeError::InvalidLength`. It is returned if the data does not have the
  length required by the value type.

### Removed

- **Breaking:** `Models::supported_model_ids` in favor of
  `AsyncDevice::models::<AnyModel>()`
- **Breaking:** `InvalidPointData`, which was no longer used since point
  validation was removed. `ParseError` and `ReadModelError` no longer have a
  type parameter.

### Fixed

- Reading or writing a point of a model which was not discovered no longer
  accesses an arbitrary register at the start of the device's address space.
  Such a model can no longer be selected. Points outside of the discovered
  model length return a `PointOutOfBounds` error.
- Reading a model which is too short to contain all points returns a
  `ModelTooShort` error.
- All instances of a model contained multiple times are discovered. Previously
  only the last instance was accessible.
- Writing a string which is shorter than its point pads the remaining
  registers with zeros. Previously the characters of the previous value
  remained in place.
- **Breaking:** A transport returning a different number of registers than
  requested no longer causes a panic or shifts the values of the following
  points. The new `ModbusError::InvalidResponseLength` error is returned
  instead.
- A `max_read_length` of 0 no longer causes a panic and is treated as 1.
- The model discovery no longer panics if the SunS identifier is located at
  the end of the address space.

## [0.9.1] - 2026-08-25

### Added

- Model and group structs now derive `Clone` and `PartialEq`. Models that
  contain no floating point values additionally derive `Eq`.
- `ModelAddr` now implements `Clone`, `Copy`, `PartialEq`, `Eq` and `Hash`
  regardless of the enabled features. The `Models` struct derives `Clone`,
  `PartialEq` and `Eq`.

## [0.9.0] - 2026-04-01

### Added

- Add support for nested and repeating groups
- Add `examples/model712` showing how to read model 712 from a device
- Add Cargo feature flags for each generated model plus an `all-models` feature

### Changed

- Update `strum` to version `0.28`
- Parsing is now more permissive for enum points: unknown or non-compliant values are preserved as `Invalid(raw)` instead of causing decode failure.

### Fixed

- `tokio-modbus` now implies the `tokio` feature
- timeout support is now feature gated by the `tokio` feature

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

[unreleased]: https://github.com/bikeshedder/sunspec/compare/v0.9.1...HEAD
[0.9.1]: https://github.com/bikeshedder/sunspec/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/bikeshedder/sunspec/compare/v0.8.0...v0.9.0
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

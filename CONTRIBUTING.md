# Contributing

This document is for people working on the `sunspec` crate itself. For using
the crate, see the [README](README.md).

## Getting started

The SunSpec model definitions are pulled in as a Git submodule, so clone with
`--recursive` or initialize it afterwards:

```sh
git submodule update --init
```

Build and test everything:

```sh
cargo build --workspace --all-features
cargo test --all-features
```

## Repository layout

| Path            | Contents                                                                    |
| --------------- | --------------------------------------------------------------------------- |
| `src/`          | The `sunspec` library                                                       |
| `src/models/`   | **Generated** model code, one file per model plus `mod.rs`                  |
| `models/`       | Submodule of [sunspec/models](https://github.com/sunspec/models)            |
| `sunspec-gen/`  | Code generator turning `models/json/*.json` into `src/models/`              |
| `examples/`     | Example binaries, each a separate workspace member                          |
| `tests/`        | Integration tests                                                           |

## Generated code

Everything in `src/models/` and the block between the
`# BEGIN generated model features` and `# END generated model features`
markers in `Cargo.toml` is generated. Don't edit it by hand. Change the
generator in `sunspec-gen/` instead and regenerate.

The generator is run via the `cargo models` alias defined in
`.cargo/config.toml`:

| Command                | Description                                                         |
| ---------------------- | ------------------------------------------------------------------- |
| `cargo models gen`     | Regenerate `src/models/` and the model features in `Cargo.toml`     |
| `cargo models check`   | Fail if the generated files are out of date (also run in CI)        |
| `cargo models update`  | Pull the latest upstream models into `models/` and regenerate       |

After changing the generator, run `cargo models gen` and commit the generator
change together with the regenerated code.

### Updating the upstream models

1. Run `cargo models update`.
2. Review the diff of `models/`, `src/models/` and `Cargo.toml`.
3. Add an entry to `CHANGELOG.md`, e.g. `Update sunspec models (YYYY-MM-DD)`,
   listing the changes users will notice (added models, added or renamed
   points, …).

## Minimum supported Rust version

The library has an MSRV of 1.76 (`rust-version` in `Cargo.toml`), checked in
CI with minimal dependency versions. `sunspec-gen` and the examples are not
subject to the MSRV.

## Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Entries describe changes from a user's point of view. Implementation details
and changes to `sunspec-gen`, CI or tooling don't belong there. Mark breaking
changes with **Breaking:**.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    deprecated_in_future,
    //missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results
)]

pub use discovery::{DiscoveryError, DiscoveryResult, ModelAddr, UnknownModel, SUNS_IDENTIFIER};
pub use model::{Model, ReadModelError};
pub use models::Models;
pub use point::{PointDef, ReadPointError, WritePointError};
pub use value::{DecodeError, FixedSize, Value};

mod discovery;
mod model;
/// This module contains all the genererated SunSpec models.
pub mod models;
mod point;
#[cfg(feature = "tokio-modbus")]
pub mod tokio_modbus;
mod utils;
mod value;

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

pub use constants::{DEFAULT_DISCOVERY_ADDRESSES, SUNS_IDENTIFIER};
pub use model::{Model, ModelAddr};
pub use models::Models;
pub use point::Point;
pub use value::{DecodeError, FixedSize, Value};

/// This module contains all client specific code.
pub mod client;
mod constants;
mod model;
/// This module contains all the genererated SunSpec models.
pub mod models;
mod point;
mod value;

//! Beautiful diagnostic reporting for compilation/decoding/deserialzation apps.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod diagnostic;
pub use diagnostic::*;

#[cfg(feature = "global")]
pub mod cache;

pub mod renderer;
pub mod sources;

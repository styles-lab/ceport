//! Beautiful diagnostic reporting for compilation/decoding/deserialzation apps.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod diagnostic;
pub use diagnostic::*;

pub mod renderer;

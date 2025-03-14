//! Beautiful diagnostic reporting for app errors/warns.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod diagnostic;
pub use diagnostic::*;

mod render;
pub use render::*;

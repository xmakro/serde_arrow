//! Support for the `arrow2` crate (*requires one the `arrow2-*` features*)
//!
//! Functions to convert Rust objects into Arrow arrays and back.
//!
#![deny(missing_docs)]
pub(crate) mod api;
pub(crate) mod deserialization;
pub(crate) mod schema;
pub(crate) mod serialization;
mod type_support;

#[cfg(test)]
mod test_deprecated_api;

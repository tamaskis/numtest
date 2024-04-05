#![doc = include_str!("../README.md")]

// Linter setup.
#![warn(missing_docs)]

// Linking project modules.
pub(crate) mod assert_array;
pub(crate) mod assert_float;
pub(crate) mod compare;
pub(crate) mod precision;

// Re-exports.
pub use crate::compare::Compare;
pub use crate::precision::Precision;

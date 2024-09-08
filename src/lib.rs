//! [![github]](https://github.com/tamaskis/numtest)&ensp;[![crates-io]](https://crates.io/crates/numtest)&ensp;[![docs-rs]](https://docs.rs/numtest)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Unit testing for numerical applications.
//!
//! # Summary
//!
//! This crate provides four main sets of utilities:
//!
//! 1. Macros for asserting equality between floating-point ([`f32`] and [`f64`]) numbers:
//!
//!     * [`assert_equal`]
//!     * [`assert_equal_to_decimal`]
//!     * [`assert_equal_to_atol`]
//!     * [`assert_equal_to_rtol`]
//!
//! 1. Macros for asserting equality between array-like structs of floats (the structs just need to
//!    implement the [`Iterator`] trait):
//!
//!     * [`assert_arrays_equal`]
//!     * [`assert_arrays_equal_to_decimal`]
//!     * [`assert_arrays_equal_to_atol`]
//!     * [`assert_arrays_equal_to_rtol`]
//!
//! 1. The [`Compare`] trait[^compare_note] (implemented for [`f32`] and [`f64`] types) for
//!    performing comparisons between floating-point numbers.
//! 1. The [`Precision`] trait[^precision_note] (implemented for [`f32`] and [`f64`] types)
//!    providing methods for accessing information regarding the precision of an _instance_ of a
//!    floating-point type.
//!
//! [^compare_note]: The methods implemented on this trait are used by the assertion macros for
//! performing float comparisons.
//!
//! [^precision_note]: Some of the methods implemented on this trait are used by the methods on the
//! [`Compare`] trait.
//!
//! # Equality assertions for floats
//!
//! ```
//! use numtest::*;
//!
//! assert_equal!(2.0, 2.0);
//! assert_equal_to_decimal!(2.0, 2.012, 1);
//! assert_equal_to_atol!(2.0, 2.00001, 1e-3);
//! assert_equal_to_rtol!(2.0, 2.01, 0.01);
//! ```
//!
//! # Equality assertions for arrays
//!
//! ```
//! use numtest::*;
//!
//! let arr1 = [1.1, 2.2, 3.3];
//! let arr2 = [1.1, 2.2, 3.3];
//!
//! assert_arrays_equal!(&arr1, &arr2);
//! ```
//!
//! ```
//! use numtest::*;
//!
//! let arr1 = [1.1, 2.2, 3.3];
//! let arr2 = [1.1, 2.22, 3.33];
//!
//! assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
//! ```
//!
//! ```
//! use nalgebra::Vector3;
//! use ndarray::Array1;
//! use numtest::*;
//!
//! let std_arr = [1.1, 2.2, 3.3];
//! let std_vec = vec![1.1, 2.22, 3.33];
//! let ndarray_arr = Array1::from_vec(vec![1.12, 2.23, 3.34]);
//! let nalgebra_vec = Vector3::new(1.13, 2.24, 3.35);
//!
//! assert_arrays_equal_to_decimal!(&std_arr, &std_vec, 1);
//! assert_arrays_equal_to_atol!(&std_arr, &ndarray_arr, 0.06);
//! assert_arrays_equal_to_rtol!(&std_arr, &nalgebra_vec, 0.03);
//! ```
//!
//! ```
//! use nalgebra::Matrix3;
//! use numtest::*;
//!
//! let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
//! let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
//!
//! assert_arrays_equal_to_decimal!(&mat1, &mat2, 1);
//! ```

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

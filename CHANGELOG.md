# Changelog

## 0.7.0

1. Combined the `Precision` trait into the `Compare` trait, after which the `Precision` trait was removed.
1. Removed `num_traits` dev dependency.
1. Added additional methods to `Compare` trait that would lead to more robust method resolution in downstream crates.

## 0.6.1

1. Small bugfix.

## 0.6.0

1. `num_traits::Float` is no longer required for implementing `numtest::Compare`.
1. Converted `num_traits` from a required dependency to a dev dependency.
1. Updated `nalgebra` dev dependency from `0.34.1` to `0.35.0`.

## 0.5.0

1. Enabled stricter lints.

## 0.4.1

1. Updated `ndarray` dev dependency from `0.17.1` to `0.17.2`.

## 0.4.0

1. Updated `nalgebra` dev dependency from `0.34.0` to `0.34.1`.
1. Updated `ndarray` dev dependency from `0.16.1` to `0.17.1`.

## 0.3.1

1. Updated `nalgebra` dev dependency from `0.33.2` to `0.34.0`.

## 0.3.0

1. Updated `rust` version to 2024.

## 0.2.2

1. Updated `nalgebra` dev dependency from `0.33.0` to `0.33.2`.
1. Updated `ndarray` dev dependency from `0.16.0` to `0.16.1`.

## 0.2.1

1. Array components are now printed in scientific notation.

## 0.2.0

1. Updated `nalgebra` dev dependency from `0.32.5` to `0.33.0`.
1. Updated `ndarray` dev dependency from `0.15.6` to `0.16.0`.

## 0.1.0 - 0.1.6

1. Initial releases.
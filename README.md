# numtest

[<img alt="github" src="https://img.shields.io/badge/github-tamaskis/numtest-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/tamaskis/numtest)
[<img alt="crates.io" src="https://img.shields.io/crates/v/numtest.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/numtest)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-numtest-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/numtest)

Unit testing for numerical applications in Rust.

## Documentation

Please see https://docs.rs/numtest.

## Examples

### Float equality

```rust
use numtest::*;

assert_equal!(2.0, 2.0);
assert_equal_to_decimal!(2.0, 2.012, 1);
assert_equal_to_atol!(2.0, 2.00001, 1e-3);
assert_equal_to_rtol!(2.0, 2.01, 0.01);
```

### Array equality

```rust
use numtest::*;

let arr1 = [1.1, 2.2, 3.3];
let arr2 = [1.1, 2.2, 3.3];

assert_arrays_equal!(&arr1, &arr2);
```

```rust
use numtest::*;

let arr1 = [1.1, 2.2, 3.3];
let arr2 = [1.1, 2.22, 3.33];

assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
```

```rust
use nalgebra::Vector3;
use ndarray::Array1;
use numtest::*;

let std_arr = [1.1, 2.2, 3.3];
let std_vec = vec![1.1, 2.22, 3.33];
let ndarray_arr = Array1::from_vec(vec![1.12, 2.23, 3.34]);
let nalgebra_vec = Vector3::new(1.13, 2.24, 3.35);

assert_arrays_equal_to_decimal!(&std_arr, &std_vec, 1);
assert_arrays_equal_to_atol!(&std_arr, &ndarray_arr, 0.06);
assert_arrays_equal_to_rtol!(&std_arr, &nalgebra_vec, 0.03);
```

```rust
use nalgebra::Matrix3;
use numtest::*;

let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);

assert_arrays_equal_to_decimal!(&mat1, &mat2, 1);
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or 
<a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
</sub>
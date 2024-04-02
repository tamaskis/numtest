#[allow(unused_imports)]
use crate::compare::Compare;

/// Asserts element-wise equality of two array-like structs to within a specified decimal precision.
///
/// This macro iterates over the elements of the two structs and checks if each pair of elements is
/// equal up to the specified decimal precision using [`Compare::is_equal_to_decimal`].
/// Additionally, this macro also checks whether the two structs have the same number of elements.
///
/// # Arguments
///
/// * `arr1` - First array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `arr2` - Second array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `decimal` - Decimal precision to use for comparison.
///
/// # Panics
///
/// * If the array-like structs do not have the same number of elements.
/// * If any of the element-wise comparisons fail.
///
/// # Note
///
/// See [`Compare::is_equal_to_decimal`] for details on how equality to within a specified decimal
/// precision is defined.
///
/// # Warning
///
/// We **_cannot_** directly perform comparisons between 2D `ndarray` arrays and `nalgebra`
/// matrices. This is because `ndarray` uses a row-major layout, while `nalgebra` uses a
/// column-major layout. This is demonstrated in the last example.
///
/// # Warning
///
/// Since this macro simply iterates over all elements, you theoreticaly _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let arr_1d = Array1::from_vec(vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]);
/// let arr_2d = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
/// assert_arrays_equal_to_decimal!(&arr_1d, &arr_2d, 1);
/// ```
///
/// However, in general, this practice should be avoided. Comparisons between structs of different
/// shapes will not be detected by the `numtest` crate since we do not specify dependencies to
/// external numerical computing crates such as `ndarray` and `nalgebra`.
///
/// # Examples
///
/// [`std::array`]
///
/// ```
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.11, 2.22, 3.33];
/// assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let vec1 = vec![1.1, 2.2, 3.3];
/// let vec2 = vec![1.11, 2.22, 3.33];
/// assert_arrays_equal_to_decimal!(&vec1, &vec2, 1);
/// ```
///
/// [`ndarray::Array1`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array1.html)
///
/// ```
/// use ndarray::Array1;
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
/// let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
/// assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
/// ```
///
/// [`nalgebra::Vector3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Vector3.html)
///
/// ```
/// use nalgebra::Vector3;
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let arr1 = Vector3::new(1.1, 2.2, 3.3);
/// let arr2 = Vector3::new(1.1, 2.22, 3.33);
/// assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
/// ```
///
/// Mix of 1D array-like structs
///
/// ```
/// use nalgebra::Vector3;
/// use ndarray::Array1;
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let std_arr = [1.1, 2.2, 3.3];
/// let std_vec = vec![1.1, 2.22, 3.33];
/// let ndarray_arr = Array1::from_vec(vec![1.12, 2.23, 3.34]);
/// let nalgebra_vec = Vector3::new(1.13, 2.24, 3.35);
///
/// assert_arrays_equal_to_decimal!(&std_arr, &std_vec, 1);
/// assert_arrays_equal_to_decimal!(&std_arr, &ndarray_arr, 1);
/// assert_arrays_equal_to_decimal!(&std_arr, &nalgebra_vec, 1);
/// ```
///
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```
/// use ndarray::Array2;
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let arr1 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9],
/// ).unwrap();
/// let arr2 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99],
/// ).unwrap();
/// assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html)
///
/// ```
/// use nalgebra::Matrix3;
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let mat2 = Matrix3::new(1.1, 2.22, 3.3, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
/// assert_arrays_equal_to_decimal!(&mat1, &mat2, 1);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html) and
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```should_panic
/// use nalgebra::Matrix3;
/// use ndarray::Array2;
/// use numtest::{assert_arrays_equal_to_decimal, Compare};
///
/// let mat = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let arr = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99],
/// ).unwrap();
/// assert_arrays_equal_to_decimal!(&mat, &arr, 1);
/// ```
#[macro_export]
macro_rules! assert_arrays_equal_to_decimal {
    ($arr1:expr, $arr2:expr, $decimal:expr) => {
        // "Tracker" variables.
        let mut min_precision = i32::MAX;
        let mut all_equal: bool = true;
        let mut not_equal_count = 0;

        // Assert that the two arrays have the same number of elements.
        let count1 = $arr1.iter().count();
        let count2 = $arr2.iter().count();
        if count1 != count2 {
            panic!("The two arrays must have the same number of elements.")
        }

        // Count the number of elements that aren't equal, and track the smallest precision.
        for (a, b) in $arr1.iter().zip($arr2.iter()) {
            let (equal, precision) = a.is_equal_to_decimal(*b, $decimal);
            if !equal {
                not_equal_count += 1;
            }
            min_precision = min_precision.min(precision);
            all_equal &= equal;
        }

        // Panic if equality not satisfied.
        if !all_equal {
            panic!(
                "\nThe two array-like structs are not equal to {} decimal places.\n --> Mismatched \
                Elements: {}/{}\n --> Maximum Decimal Places of Precision: {}\n",
                $decimal, not_equal_count, count1, min_precision
            )
        }
    };
}

/// Asserts element-wise equality of two array-like structs to within a specified absolute
/// tolerance.
///
/// This macro iterates over the elements of the two structs and checks if each pair of elements is
/// equal to within the specified absolute tolerance using [`Compare::is_equal_to_atol`].
/// Additionally, this macro also checks whether the two structs have the same number of elements.
///
/// # Arguments
///
/// * `arr1` - First array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `arr2` - Second array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `atol` - Absolute tolerance.
///
/// # Panics
///
/// * If the array-like structs do not have the same number of elements.
/// * If any of the element-wise comparisons fail.
///
/// # Note
///
/// See [`Compare::is_equal_to_atol`] for details on how equality to within a specified absolute
/// tolerance is defined.
///
/// # Warning
///
/// We **_cannot_** directly perform comparisons between 2D `ndarray` arrays and `nalgebra`
/// matrices. This is because `ndarray` uses a row-major layout, while `nalgebra` uses a
/// column-major layout. This is demonstrated in the last example.
///
/// # Warning
///
/// Since this macro simply iterates over all elements, you theoreticaly _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let arr_1d = Array1::from_vec(vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]);
/// let arr_2d = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
/// assert_arrays_equal_to_atol!(&arr_1d, &arr_2d, 0.07);
/// ```
///
/// However, in general, this practice should be avoided. Comparisons between structs of different
/// shapes will not be detected by the `numtest` crate since we do not specify dependencies to
/// external numerical computing crates such as `ndarray` and `nalgebra`.
///
/// # Examples
///
/// [`std::array`]
///
/// ```
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.11, 2.22, 3.33];
/// assert_arrays_equal_to_atol!(&arr1, &arr2, 0.05);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let vec1 = vec![1.1, 2.2, 3.3];
/// let vec2 = vec![1.11, 2.22, 3.33];
/// assert_arrays_equal_to_atol!(&vec1, &vec2, 0.05);
/// ```
///
/// [`ndarray::Array1`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array1.html)
///
/// ```
/// use ndarray::Array1;
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
/// let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
/// assert_arrays_equal_to_atol!(&arr1, &arr2, 0.05);
/// ```
///
/// [`nalgebra::Vector3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Vector3.html)
///
/// ```
/// use nalgebra::Vector3;
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let arr1 = Vector3::new(1.1, 2.2, 3.3);
/// let arr2 = Vector3::new(1.1, 2.22, 3.33);
/// assert_arrays_equal_to_atol!(&arr1, &arr2, 0.05);
/// ```
///
/// Mix of 1D array-like structs
///
/// ```
/// use nalgebra::Vector3;
/// use ndarray::Array1;
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let std_arr = [1.1, 2.2, 3.3];
/// let std_vec = vec![1.1, 2.22, 3.33];
/// let ndarray_arr = Array1::from_vec(vec![1.12, 2.23, 3.34]);
/// let nalgebra_vec = Vector3::new(1.13, 2.24, 3.35);
///
/// assert_arrays_equal_to_atol!(&std_arr, &std_vec, 0.051);
/// assert_arrays_equal_to_atol!(&std_arr, &ndarray_arr, 0.051);
/// assert_arrays_equal_to_atol!(&std_arr, &nalgebra_vec, 0.051);
/// ```
///
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```
/// use ndarray::Array2;
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let arr1 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9],
/// ).unwrap();
/// let arr2 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99],
/// ).unwrap();
/// assert_arrays_equal_to_atol!(&arr1, &arr2, 0.1);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html)
///
/// ```
/// use nalgebra::Matrix3;
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let mat2 = Matrix3::new(1.1, 2.22, 3.3, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
/// assert_arrays_equal_to_atol!(&mat1, &mat2, 0.1);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html) and
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```should_panic
/// use nalgebra::Matrix3;
/// use ndarray::Array2;
/// use numtest::{assert_arrays_equal_to_atol, Compare};
///
/// let mat = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let arr = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99],
/// ).unwrap();
/// assert_arrays_equal_to_atol!(&mat, &arr, 0.1);
/// ```
#[macro_export]
macro_rules! assert_arrays_equal_to_atol {
    ($arr1:expr, $arr2:expr, $atol:expr) => {
        // "Tracker" variables.
        let mut max_abs_diff: f64 = 0.0;
        let mut all_equal: bool = true;
        let mut not_equal_count = 0;

        // Assert that the two arrays have the same number of elements.
        let count1 = $arr1.iter().count();
        let count2 = $arr2.iter().count();
        if count1 != count2 {
            panic!("The two arrays must have the same number of elements.")
        }

        // Count the number of elements that aren't equal, and track the larget absolute difference.
        for (a, b) in $arr1.iter().zip($arr2.iter()) {
            let (equal, abs_diff) = a.is_equal_to_atol(*b, $atol);
            if !equal {
                not_equal_count += 1;
            }
            max_abs_diff = max_abs_diff.max(abs_diff.into());
            all_equal &= equal;
        }

        // Panic if equality not satisfied.
        if !all_equal {
            panic!(
                "\nThe two array-like structs are not equal to an absolute tolerance of {}.\n --> \
                Mismatched Elements: {}/{}\n --> Largest Absolute Difference: {}\n",
                $atol, not_equal_count, count1, max_abs_diff
            )
        }
    };
}

/// Asserts element-wise equality of two array-like structs to within a specified relative
/// tolerance.
///
/// This macro iterates over the elements of the two structs and checks if each pair of elements is
/// equal to within the specified relative tolerance using the [`Compare::is_equal_to_rtol`].
/// Additionally, this macro also checks whether the two structs have the same number of elements.
///
/// # Arguments
///
/// * `arr1` - First array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `arr2` - Second array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `rtol` - Relative tolerance.
///
/// # Panics
///
/// * If the array-like structs do not have the same number of elements.
/// * If any of the element-wise comparisons fail.
///
/// # Note
///
/// See [`Compare::is_equal_to_rtol`] for details on how equality to within a specified relative
/// tolerance is defined.
///
/// # Warning
///
/// We **_cannot_** directly perform comparisons between 2D `ndarray` arrays and `nalgebra`
/// matrices. This is because `ndarray` uses a row-major layout, while `nalgebra` uses a
/// column-major layout. This is demonstrated in the last example.
///
/// # Warning
///
/// Since this macro simply iterates over all elements, you theoreticaly _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let arr_1d = Array1::from_vec(vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]);
/// let arr_2d = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
/// assert_arrays_equal_to_rtol!(&arr_1d, &arr_2d, 0.01);
/// ```
///
/// However, in general, this practice should be avoided. Comparisons between structs of different
/// shapes will not be detected by the `numtest` crate since we do not specify dependencies to
/// external numerical computing crates such as `ndarray` and `nalgebra`.
///
/// # Examples
///
/// [`std::array`]
///
/// ```
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.11, 2.22, 3.33];
/// assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.01);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let vec1 = vec![1.1, 2.2, 3.3];
/// let vec2 = vec![1.11, 2.22, 3.33];
/// assert_arrays_equal_to_rtol!(&vec1, &vec2, 0.01);
/// ```
///
/// [`ndarray::Array1`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array1.html)
///
/// ```
/// use ndarray::Array1;
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
/// let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
/// assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.01);
/// ```
///
/// [`nalgebra::Vector3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Vector3.html)
///
/// ```
/// use nalgebra::Vector3;
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let arr1 = Vector3::new(1.1, 2.2, 3.3);
/// let arr2 = Vector3::new(1.1, 2.22, 3.33);
/// assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.01);
/// ```
///
/// Mix of 1D array-like structs
///
/// ```
/// use nalgebra::Vector3;
/// use ndarray::Array1;
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let std_arr = [1.1, 2.2, 3.3];
/// let std_vec = vec![1.1, 2.22, 3.33];
/// let ndarray_arr = Array1::from_vec(vec![1.12, 2.23, 3.34]);
/// let nalgebra_vec = Vector3::new(1.13, 2.24, 3.35);
///
/// assert_arrays_equal_to_rtol!(&std_arr, &std_vec, 0.01);
/// assert_arrays_equal_to_rtol!(&std_arr, &ndarray_arr, 0.02);
/// assert_arrays_equal_to_rtol!(&std_arr, &nalgebra_vec, 0.03);
/// ```
///
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```
/// use ndarray::Array2;
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let arr1 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9],
/// ).unwrap();
/// let arr2 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99],
/// ).unwrap();
/// assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.01);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html)
///
/// ```
/// use nalgebra::Matrix3;
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let mat2 = Matrix3::new(1.1, 2.22, 3.3, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
/// assert_arrays_equal_to_rtol!(&mat1, &mat2, 0.01);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html) and
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```should_panic
/// use nalgebra::Matrix3;
/// use ndarray::Array2;
/// use numtest::{assert_arrays_equal_to_rtol, Compare};
///
/// let mat = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let arr = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99],
/// ).unwrap();
/// assert_arrays_equal_to_rtol!(&mat, &arr, 0.01);
/// ```
#[macro_export]
macro_rules! assert_arrays_equal_to_rtol {
    ($arr1:expr, $arr2:expr, $rtol:expr) => {
        // "Tracker" variables.
        let mut max_rel_diff: f64 = 0.0;
        let mut all_equal: bool = true;
        let mut not_equal_count = 0;

        // Assert that the two arrays have the same number of elements.
        let count1 = $arr1.iter().count();
        let count2 = $arr2.iter().count();
        if count1 != count2 {
            panic!("The two arrays must have the same number of elements.")
        }

        // Count the number of elements that aren't equal, and track the larget relative difference.
        for (a, b) in $arr1.iter().zip($arr2.iter()) {
            let (equal, abs_diff) = a.is_equal_to_rtol(*b, $rtol);
            if !equal {
                not_equal_count += 1;
            }
            max_rel_diff = max_rel_diff.max(abs_diff.into());
            all_equal &= equal;
        }

        // Panic if equality not satisfied.
        if !all_equal {
            panic!(
                "\nThe two array-like structs are not equal to a relative tolerance of {}.\n --> \
                Mismatched Elements: {}/{}\n --> Largest Relative Difference: {}\n",
                $rtol, not_equal_count, count1, max_rel_diff
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Matrix3, Vector3};
    use ndarray::{Array1, Array2};

    #[test]
    fn test_std_array_decimal_pass() {
        let arr1: [f64; 3] = [1.1, 2.2, 3.3];
        let arr2: [f64; 3] = [1.11, 2.22, 3.33];
        assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
    }

    #[test]
    #[should_panic]
    fn test_std_array_decimal_fail() {
        let arr1: [f64; 3] = [1.1, 2.2, 3.3];
        let arr2: [f64; 3] = [1.1, 2.22, 3.33];
        assert_arrays_equal_to_decimal!(&arr1, &arr2, 2);
    }

    #[test]
    fn test_std_vec_decimal_pass() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.22, 3.33]);
        assert_arrays_equal_to_decimal!(&vec1, &vec2, 1);
    }

    #[test]
    #[should_panic]
    fn test_std_vec_decimal_fail() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.22, 3.33]);
        assert_arrays_equal_to_decimal!(&vec1, &vec2, 2);
    }

    #[test]
    fn test_ndarray_array1_decimal_pass() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
        assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array1_decimal_fail() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
        assert_arrays_equal_to_decimal!(&arr1, &arr2, 2);
    }

    #[test]
    fn test_ndarray_array2_decimal_pass() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
        assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array2_decimal_fail() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
        assert_arrays_equal_to_decimal!(&arr1, &arr2, 2);
    }

    #[test]
    fn test_nalgebra_vector3_decimal_pass() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.22, 3.33);
        assert_arrays_equal_to_decimal!(&vec1, &vec2, 1);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_vector3_decimal_fail() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.22, 3.33);
        assert_arrays_equal_to_decimal!(&vec1, &vec2, 2);
    }

    #[test]
    fn test_nalgebra_matrix3_decimal_pass() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
        assert_arrays_equal_to_decimal!(&mat1, &mat2, 1);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_matrix3_decimal_fail() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
        assert_arrays_equal_to_decimal!(&mat1, &mat2, 2);
    }

    #[test]
    fn test_std_array_atol_pass() {
        let arr1: [f64; 3] = [1.1, 2.2, 3.3];
        let arr2: [f64; 3] = [1.11, 2.22, 3.33];
        assert_arrays_equal_to_atol!(&arr1, &arr2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_std_array_atol_fail() {
        let arr1: [f32; 3] = [1.1, 2.2, 3.3];
        let arr2: [f32; 3] = [1.1, 2.22, 3.33];
        assert_arrays_equal_to_atol!(&arr1, &arr2, 0.01);
    }

    #[test]
    fn test_std_vec_atol_pass() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.22, 3.33]);
        assert_arrays_equal_to_atol!(&vec1, &vec2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_std_vec_atol_fail() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.22, 3.33]);
        assert_arrays_equal_to_atol!(&vec1, &vec2, 0.01);
    }

    #[test]
    fn test_ndarray_array1_atol_pass() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
        assert_arrays_equal_to_atol!(&arr1, &arr2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array1_atol_fail() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
        assert_arrays_equal_to_atol!(&arr1, &arr2, 0.01);
    }

    #[test]
    fn test_ndarray_array2_atol_pass() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
        assert_arrays_equal_to_atol!(&arr1, &arr2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array2_atol_fail() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
        assert_arrays_equal_to_atol!(&arr1, &arr2, 0.01);
    }

    #[test]
    fn test_nalgebra_vector3_atol_pass() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.22, 3.33);
        assert_arrays_equal_to_atol!(&vec1, &vec2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_vector3_atol_fail() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.22, 3.33);
        assert_arrays_equal_to_atol!(&vec1, &vec2, 0.01);
    }

    #[test]
    fn test_nalgebra_matrix3_atol_pass() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
        assert_arrays_equal_to_atol!(&mat1, &mat2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_matrix3_atol_fail() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
        assert_arrays_equal_to_atol!(&mat1, &mat2, 0.01);
    }

    #[test]
    fn test_std_array_rtol_pass() {
        let arr1: [f64; 3] = [1.1, 2.2, 3.3];
        let arr2: [f64; 3] = [1.11, 2.22, 3.33];
        assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_std_array_rtol_fail() {
        let arr1: [f32; 3] = [1.1, 2.2, 3.3];
        let arr2: [f32; 3] = [1.1, 2.22, 3.33];
        assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.001);
    }

    #[test]
    fn test_std_vec_rtol_pass() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.22, 3.33]);
        assert_arrays_equal_to_rtol!(&vec1, &vec2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_std_vec_rtol_fail() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.22, 3.33]);
        assert_arrays_equal_to_rtol!(&vec1, &vec2, 0.001);
    }

    #[test]
    fn test_ndarray_array1_rtol_pass() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
        assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array1_rtol_fail() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.22, 3.33]);
        assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.001);
    }

    #[test]
    fn test_ndarray_array2_rtol_pass() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
        assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array2_rtol_fail() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.22, 3.33, 4.4, 5.55, 6.66]).unwrap();
        assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.001);
    }

    #[test]
    fn test_nalgebra_vector3_rtol_pass() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.22, 3.33);
        assert_arrays_equal_to_rtol!(&vec1, &vec2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_vector3_rtol_fail() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.22, 3.33);
        assert_arrays_equal_to_rtol!(&vec1, &vec2, 0.001);
    }

    #[test]
    fn test_nalgebra_matrix3_rtol_pass() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
        assert_arrays_equal_to_rtol!(&mat1, &mat2, 0.1);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_matrix3_rtol_fail() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.22, 3.33, 4.4, 5.55, 6.66, 7.7, 8.88, 9.99);
        assert_arrays_equal_to_rtol!(&mat1, &mat2, 0.001);
    }
}

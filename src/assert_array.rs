#[allow(unused_imports)]
use crate::compare::Compare;

/// Counts the number of elements in two array-like structs.
///
/// # Arguments
///
/// * `arr1` - First array-like struct to count elements of.
/// * `arr2` - Second array-like struct to count elements of.
///
/// # Returns
///
/// Tuple:
///
/// 1. Number of elements in `arr1`.
/// 2. Number of elements in `arr2`.
#[macro_export]
macro_rules! count_elements {
    ($arr1:expr, $arr2:expr) => {
        ($arr1.iter().count(), $arr2.iter().count())
    };
}

/// Validate that two array-like structs have the same number of elements.
///
/// # Arguments
///
/// * `count1` - Number of elements in the first array-like struct.
/// * `count2` - Number of elements in the second array-like struct.
///
/// # Panics
///
/// If the two array-like structs do not have the same number of elements. In this case, the number
/// of elements in each array-like struct is also printed.
#[macro_export]
macro_rules! validate_counts {
    ($count1:expr, $count2:expr) => {
        if $count1 != $count2 {
            panic!(
                "\nThe two arrays must have the same number of elements.\n --> arr1 num elements\
                : {}\n --> arr 2 num elements: {}\n",
                $count1, $count2
            );
        }
    };
}

/// Get a string that can be used to print the mismatched elements between two array-like structs.
///
/// # Arguments
///
/// * `arr1` - The first array-like struct.
/// * `arr2` - The second array-like struct.
/// * `idx_mismatched` - Vector of indices where mismatches occur.
/// * `output` - A mutable string to which the formatted output will be appended.
#[macro_export]
macro_rules! get_mismatched_elements_str {
    ($arr1:expr, $arr2:expr, $idx_mismatched:expr, $output:expr) => {
        // Add the header.
        $output.push_str(&format!("{:>15} {:>15}\n", "arr1", "arr2"));
        $output.push_str(&format!("{:>15} {:>15}\n", "----", "----"));

        // Iterate over all elements.
        for (idx, (a, b)) in $arr1.iter().zip($arr2.iter()).enumerate() {
            // Determine if the current index is in the mismatched indices list.
            let is_mismatched = $idx_mismatched.contains(&idx);

            // Define ANSI escape codes for red and bold text.
            let red_bold_start = if is_mismatched { "\x1b[31;1m" } else { "" };
            let reset = if is_mismatched { "\x1b[0m" } else { "" };

            // Append formatted output to the string.
            $output.push_str(&format!(
                "{}{:>15}{} {}{:>15}{}\n",
                red_bold_start, a, reset, red_bold_start, b, reset
            ));
        }
    };
}

/// Asserts element-wise exact equality of two array-like structs.
///
/// This macro iterates over the elements of the two structs and checks if each pair of elements is
/// exactly equal using [`Compare::is_equal`]. Additionally, this macro also checks whether the two
/// structs have the same number of elements.
///
/// # Arguments
///
/// * `arr1` - First array-like struct to compare. Must implement the [`Iterator`] trait.
/// * `arr2` - Second array-like struct to compare. Must implement the [`Iterator`] trait.
///
/// # Panics
///
/// * If the two array-like structs do not have the same number of elements. In this case, the
///   number of elements in each array-like struct is also printed.
/// * If any of the element-wise comparisons fail. In this case, the two array-like structs will be
///   printed, with the mismatched elements shown in bolded red.
///
/// # Note
///
/// See [`Compare::is_equal`] for details on how exact equality is defined.
///
/// # Warning
///
/// We **_cannot_** directly perform comparisons between 2D `ndarray` arrays and `nalgebra`
/// matrices. This is because `ndarray` uses a row-major layout, while `nalgebra` uses a
/// column-major layout. This is demonstrated in the last example.
///
/// # Warning
///
/// Since this macro simply iterates over all elements, you theoretically _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::*;
///
/// let arr_1d = Array1::from_vec(vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]);
/// let arr_2d = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
/// assert_arrays_equal!(&arr_1d, &arr_2d);
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
/// use numtest::*;
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.1, 2.2, 3.3];
/// assert_arrays_equal!(&arr1, &arr2);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::*;
///
/// let vec1 = vec![1.1, 2.2, 3.3];
/// let vec2 = vec![1.1, 2.2, 3.3];
/// assert_arrays_equal!(&vec1, &vec2);
/// ```
///
/// [`ndarray::Array1`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array1.html)
///
/// ```
/// use ndarray::Array1;
/// use numtest::*;
///
/// let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
/// let arr2 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
/// assert_arrays_equal!(&arr1, &arr2);
/// ```
///
/// [`nalgebra::Vector3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Vector3.html)
///
/// ```
/// use nalgebra::Vector3;
/// use numtest::*;
///
/// let arr1 = Vector3::new(1.1, 2.2, 3.3);
/// let arr2 = Vector3::new(1.1, 2.2, 3.3);
/// assert_arrays_equal!(&arr1, &arr2);
/// ```
///
/// Mix of 1D array-like structs
///
/// ```
/// use nalgebra::Vector3;
/// use ndarray::Array1;
/// use numtest::*;
///
/// let std_arr = [1.1, 2.2, 3.3];
/// let std_vec = vec![1.1, 2.2, 3.3];
/// let ndarray_arr = Array1::from_vec(vec![1.1, 2.2, 3.3]);
/// let nalgebra_vec = Vector3::new(1.1, 2.2, 3.3);
///
/// assert_arrays_equal!(&std_arr, &std_vec);
/// assert_arrays_equal!(&std_arr, &ndarray_arr);
/// assert_arrays_equal!(&std_arr, &nalgebra_vec);
/// ```
///
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```
/// use ndarray::Array2;
/// use numtest::*;
///
/// let arr1 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9],
/// ).unwrap();
/// let arr2 = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9],
/// ).unwrap();
/// assert_arrays_equal!(&arr1, &arr2);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html)
///
/// ```
/// use nalgebra::Matrix3;
/// use numtest::*;
///
/// let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let mat2 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// assert_arrays_equal!(&mat1, &mat2);
/// ```
///
/// [`nalgebra::Matrix3`](https://docs.rs/nalgebra/0.25.0/nalgebra/base/type.Matrix3.html) and
/// [`ndarray::Array2`](https://docs.rs/ndarray/0.15.6/ndarray/type.Array2.html)
///
/// ```should_panic
/// use nalgebra::Matrix3;
/// use ndarray::Array2;
/// use numtest::*;
///
/// let mat = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
/// let arr = Array2::from_shape_vec(
///     (3, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9],
/// ).unwrap();
/// assert_arrays_equal!(&mat, &arr);
/// ```
#[macro_export]
macro_rules! assert_arrays_equal {
    ($arr1:expr, $arr2:expr) => {
        // Assert that the two arrays have the same number of elements.
        let (count1, count2) = count_elements!($arr1, $arr2);
        validate_counts!(count1, count2);

        // Track the indices of mismatched elements.
        let mut idx_mismatched: Vec<usize> = Vec::new();
        for (idx, (a, b)) in $arr1.iter().zip($arr2.iter()).enumerate() {
            let equal = a.is_equal(*b);
            if !equal {
                idx_mismatched.push(idx);
            }
        }

        // Panic if equality not satisfied.
        if idx_mismatched.len() > 0 {
            let mut mismatched_str = String::new();
            get_mismatched_elements_str!($arr1, $arr2, &idx_mismatched, &mut mismatched_str);
            panic!(
                "\nThe two array-like structs are not exactly equal.\n --> Mismatched \
                Elements: {}/{}\n\n{}",
                idx_mismatched.len(),
                count1,
                mismatched_str
            )
        }
    };
}

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
/// * If the two array-like structs do not have the same number of elements. In this case, the
///   number of elements in each array-like struct is also printed.
/// * If any of the element-wise comparisons fail. In this case, the two array-like structs will be
///   printed, with the mismatched elements shown in bolded red.
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
/// Since this macro simply iterates over all elements, you theoretically _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::*;
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
/// use numtest::*;
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.11, 2.22, 3.33];
/// assert_arrays_equal_to_decimal!(&arr1, &arr2, 1);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
        // Variable to track the minimum satisfied precision.
        let mut min_precision = i32::MAX;

        // Assert that the two arrays have the same number of elements.
        let (count1, count2) = count_elements!($arr1, $arr2);
        validate_counts!(count1, count2);

        // Track the indices of mismatched elements and the smallest precision that is satisfied.
        let mut idx_mismatched: Vec<usize> = Vec::new();
        for (idx, (a, b)) in $arr1.iter().zip($arr2.iter()).enumerate() {
            let (equal, precision) = a.is_equal_to_decimal(*b, $decimal);
            if !equal {
                idx_mismatched.push(idx);
            }
            min_precision = min_precision.min(precision);
        }

        // Panic if equality not satisfied.
        if idx_mismatched.len() > 0 {
            let mut mismatched_str = String::new();
            get_mismatched_elements_str!($arr1, $arr2, &idx_mismatched, &mut mismatched_str);
            panic!(
                "\nThe two array-like structs are not equal to {} decimal places.\n --> Mismatched \
                Elements: {}/{}\n --> Maximum Decimal Places of Precision: {}\n\n{}",
                $decimal, idx_mismatched.len(), count1, min_precision, mismatched_str
            );
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
/// * If the two array-like structs do not have the same number of elements. In this case, the
///   number of elements in each array-like struct is also printed.
/// * If any of the element-wise comparisons fail. In this case, the two array-like structs will be
///   printed, with the mismatched elements shown in bolded red.
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
/// Since this macro simply iterates over all elements, you theoretically _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::*;
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
/// use numtest::*;
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.11, 2.22, 3.33];
/// assert_arrays_equal_to_atol!(&arr1, &arr2, 0.05);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
        // Variable to track the maximum absolute difference.
        let mut max_abs_diff: f64 = 0.0;

        // Assert that the two arrays have the same number of elements.
        let (count1, count2) = count_elements!($arr1, $arr2);
        validate_counts!(count1, count2);

        // Track the indices of mismatched elements and the larget absolute difference.
        let mut idx_mismatched: Vec<usize> = Vec::new();
        for (idx, (a, b)) in $arr1.iter().zip($arr2.iter()).enumerate() {
            let (equal, abs_diff) = a.is_equal_to_atol(*b, $atol);
            if !equal {
                idx_mismatched.push(idx);
            }
            max_abs_diff = max_abs_diff.max(abs_diff.into());
        }

        // Panic if equality not satisfied.
        if idx_mismatched.len() > 0 {
            let mut mismatched_str = String::new();
            get_mismatched_elements_str!($arr1, $arr2, &idx_mismatched, &mut mismatched_str);
            panic!(
                "\nThe two array-like structs are not equal to an absolute tolerance of {}.\n --> \
                Mismatched Elements: {}/{}\n --> Largest Absolute Difference: {}\n\n{}",
                $atol,
                idx_mismatched.len(),
                count1,
                max_abs_diff,
                mismatched_str
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
/// * If the two array-like structs do not have the same number of elements. In this case, the
///   number of elements in each array-like struct is also printed.
/// * If any of the element-wise comparisons fail. In this case, the two array-like structs will be
///   printed, with the mismatched elements shown in bolded red.
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
/// Since this macro simply iterates over all elements, you theoretically _are_ able to compare
/// 1D arrays with 2D arrays. For example,
///
/// ```
/// use ndarray::{Array1, Array2};
/// use numtest::*;
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
/// use numtest::*;
///
/// let arr1 = [1.1, 2.2, 3.3];
/// let arr2 = [1.11, 2.22, 3.33];
/// assert_arrays_equal_to_rtol!(&arr1, &arr2, 0.01);
/// ```
///
/// [`Vec`]
///
/// ```
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
/// use numtest::*;
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
        // Variable to track the maximum relative difference.
        let mut max_rel_diff: f64 = 0.0;

        // Assert that the two arrays have the same number of elements.
        let (count1, count2) = count_elements!($arr1, $arr2);
        validate_counts!(count1, count2);

        // Track the indices of mismatched elements and the larget relative difference.
        let mut idx_mismatched: Vec<usize> = Vec::new();
        for (idx, (a, b)) in $arr1.iter().zip($arr2.iter()).enumerate() {
            let (equal, rel_diff) = a.is_equal_to_rtol(*b, $rtol);
            if !equal {
                idx_mismatched.push(idx);
            }
            max_rel_diff = max_rel_diff.max(rel_diff.into());
        }

        // Panic if equality not satisfied.
        if idx_mismatched.len() > 0 {
            let mut mismatched_str = String::new();
            get_mismatched_elements_str!($arr1, $arr2, &idx_mismatched, &mut mismatched_str);
            panic!(
                "\nThe two array-like structs are not equal to a relative tolerance of {}.\n --> \
                Mismatched Elements: {}/{}\n --> Largest Relative Difference: {}\n\n{}",
                $rtol,
                idx_mismatched.len(),
                count1,
                max_rel_diff,
                mismatched_str
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
    fn test_std_array_exact_pass() {
        let arr1: [f64; 3] = [1.1, 2.2, 3.3];
        let arr2: [f64; 3] = [1.1, 2.2, 3.3];
        assert_arrays_equal!(&arr1, &arr2);
    }

    #[test]
    #[should_panic]
    fn test_std_array_exact_fail() {
        let arr1: [f64; 3] = [1.1, 2.2, 3.3];
        let arr2: [f64; 3] = [1.1, 2.2, 3.33];
        assert_arrays_equal!(&arr1, &arr2);
    }

    #[test]
    fn test_std_vec_exact_pass() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.2, 3.3]);
        assert_arrays_equal!(&vec1, &vec2);
    }

    #[test]
    #[should_panic]
    fn test_std_vec_exact_fail() {
        let vec1 = Vec::from([1.1, 2.2, 3.3]);
        let vec2 = Vec::from([1.1, 2.2, 3.33]);
        assert_arrays_equal!(&vec1, &vec2);
    }

    #[test]
    fn test_ndarray_array1_exact_pass() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        assert_arrays_equal!(&arr1, &arr2);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array1_exact_fail() {
        let arr1 = Array1::from_vec(vec![1.1, 2.2, 3.3]);
        let arr2 = Array1::from_vec(vec![1.1, 2.2, 3.33]);
        assert_arrays_equal!(&arr1, &arr2);
    }

    #[test]
    fn test_ndarray_array2_exact_pass() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        assert_arrays_equal!(&arr1, &arr2);
    }

    #[test]
    #[should_panic]
    fn test_ndarray_array2_exact_fail() {
        let arr1 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]).unwrap();
        let arr2 = Array2::from_shape_vec((2, 3), vec![1.1, 2.2, 3.33, 4.4, 5.5, 6.66]).unwrap();
        assert_arrays_equal!(&arr1, &arr2);
    }

    #[test]
    fn test_nalgebra_vector3_exact_pass() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.2, 3.3);
        assert_arrays_equal!(&vec1, &vec2);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_vector3_exact_fail() {
        let vec1 = Vector3::new(1.1, 2.2, 3.3);
        let vec2 = Vector3::new(1.1, 2.2, 3.33);
        assert_arrays_equal!(&vec1, &vec2);
    }

    #[test]
    fn test_nalgebra_matrix3_exact_pass() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        assert_arrays_equal!(&mat1, &mat2);
    }

    #[test]
    #[should_panic]
    fn test_nalgebra_matrix3_exact_fail() {
        let mat1 = Matrix3::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9);
        let mat2 = Matrix3::new(1.1, 2.2, 3.33, 4.4, 5.5, 6.66, 7.7, 8.8, 9.99);
        assert_arrays_equal!(&mat1, &mat2);
    }

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

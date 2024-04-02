#[allow(unused_imports)]
use crate::compare::Compare;

/// Asserts equality of two floating-point numbers to within a specified decimal precision.
///
/// # Arguments
///
/// * `a` - The first floating-point number to compare.
/// * `b` - The second floating-point number to compare against.
/// * `decimal` - Decimal precision to use for comparison.
///
/// # Panics
///
/// If the two floating-point numbers are not equal to within the specified decimal precision.
///
/// # Note
///
/// See [`Compare::is_equal_to_decimal`] for details on how equality to within a specified decimal
/// precision is defined.
///
/// # Examples
///
/// ```
/// use numtest::{assert_equal_to_decimal, Compare};
///
/// assert_equal_to_decimal!(2.0, 2.012, 2)
/// ```
///
/// ```should_panic
/// use numtest::{assert_equal_to_decimal, Compare};
///
/// assert_equal_to_decimal!(2.0, 2.012, 4)
/// ```
#[macro_export]
macro_rules! assert_equal_to_decimal {
    ($a:expr, $b:expr, $decimal:expr) => {{
        let (are_equal, actual_decimal) = $a.is_equal_to_decimal($b, $decimal);
        if !are_equal {
            panic!(
                "Values are not equal to {} decimal places. They ARE equal to {} decimal places.",
                $decimal, actual_decimal
            );
        }
    }};
}

/// Asserts equality of two floating-point numbers to within a specified absolute tolerance.
///
/// # Arguments
///
/// * `a` - The first floating-point number to compare.
/// * `b` - The second floating-point number to compare against.
/// * `atol` - Absolute tolerance.
///
/// # Panics
///
/// If the two floating-point numbers are not equal to within the specified absolute tolerance.
///
/// # Note
///
/// See [`Compare::is_equal_to_atol`] for details on how equality to within a specified absolute
/// tolerance is defined.
///
/// # Examples
///
/// ```
/// use numtest::{assert_equal_to_atol, Compare};
///
/// assert_equal_to_atol!(2.0, 2.00001, 1e-3);
/// ```
///
/// ```should_panic
/// use numtest::{assert_equal_to_atol, Compare};
///
/// assert_equal_to_atol!(2.0, 2.00001, 1e-6);
/// ```
#[macro_export]
macro_rules! assert_equal_to_atol {
    ($a:expr, $b:expr, $atol:expr) => {{
        let (are_equal, abs_diff) = $a.is_equal_to_atol($b, $atol);
        if !are_equal {
            panic!(
                "Values are not equal to within an absolute tolerance of {}. They ARE equal to \
                within an absolute tolerance of {}.",
                $atol, abs_diff
            );
        }
    }};
}

/// Asserts equality of two floating-point numbers to within a specified relative tolerance.
///
/// # Arguments
///
/// * `a` - The first floating-point number to compare.
/// * `b` - The second floating-point number to compare against.
/// * `rtol` - Relative tolerance.
///
/// # Panics
///
/// If the two floating-point numbers are not equal to within the specified relative tolerance.
///
/// # Note
///
/// See [`Compare::is_equal_to_rtol`] for details on how equality to within a specified relative
/// tolerance is defined.
///
/// # Examples
///
/// ```
/// use numtest::{assert_equal_to_rtol, Compare};
///
/// assert_equal_to_rtol!(2.0, 2.01, 0.05);
/// ```
///
/// ```should_panic
/// use numtest::{assert_equal_to_rtol, Compare};
///
/// assert_equal_to_rtol!(2.0, 2.01, 1e-5);
/// ```
#[macro_export]
macro_rules! assert_equal_to_rtol {
    ($a:expr, $b:expr, $rtol:expr) => {{
        let (are_equal, rel_diff) = $a.is_equal_to_rtol($b, $rtol);
        if !are_equal {
            panic!(
                "Values are not equal to within a relative tolerance of {}. They ARE equal to \
                within a relative tolerance of {}.",
                $rtol, rel_diff
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_equal_to_decimal_should_pass() {
        assert_equal_to_decimal!(1.0_f32, 0.9999999, 6);
        assert_equal_to_decimal!(1.0_f64, 1.00000001, 8);
        assert_equal_to_decimal!(1234.2222_f64, 1234.2223_f64, 3);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_decimal_should_fail_1() {
        assert_equal_to_decimal!(1.0_f32, 0.9999999, 8);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_decimal_should_fail_2() {
        assert_equal_to_decimal!(1.0_f64, 1.00000001, 10);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_decimal_should_fail_3() {
        assert_equal_to_decimal!(1234.2222_f64, 1234.2223_f64, 10);
    }

    #[test]
    fn assert_equal_to_atol_should_pass() {
        assert_equal_to_atol!(1.0_f32, 0.9999999, 1e-5);
        assert_equal_to_atol!(1.0_f64, 1.00000001, 1e-5);
        assert_equal_to_atol!(1234.2222_f64, 1234.2223_f64, 0.1);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_atol_should_fail_1() {
        assert_equal_to_atol!(1.0_f32, 0.9999999, 1e-8);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_atol_should_fail_2() {
        assert_equal_to_atol!(1.0_f64, 1.00000001, 1e-10);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_atol_should_fail_3() {
        assert_equal_to_atol!(1234.2222_f64, 1234.2223_f64, 1e-5);
    }

    #[test]
    fn assert_equal_to_rtol_should_pass() {
        assert_equal_to_rtol!(1.0_f32, 0.9999999, 1e-5);
        assert_equal_to_rtol!(1.0_f64, 1.00000001, 1e-5);
        assert_equal_to_rtol!(1234.2222_f64, 1234.2223_f64, 1e-5);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_rtol_should_fail_1() {
        assert_equal_to_rtol!(1.0_f32, 0.9999999, 1e-15);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_rtol_should_fail_2() {
        assert_equal_to_rtol!(1.0_f64, 1.00000001, 1e-15);
    }

    #[test]
    #[should_panic]
    fn assert_equal_to_rtol_should_fail_3() {
        assert_equal_to_rtol!(1234.2222_f64, 1234.2223_f64, 1e-15);
    }
}

use crate::precision::Precision;
use num_traits::Float;

/// Trait for comparing floating-point numbers.
pub trait Compare {
    /// Determines if a floating-point number is exactly equal to another.
    ///
    /// # Arguments
    ///
    /// * `self` - The first floating-point number to compare.
    /// * `other` - The second floating-point number to compare against.
    ///
    /// # Returns
    ///
    /// `true` if the two floats are exactly equal to one another, `false` otherwise.
    ///
    /// # Special Cases
    ///
    /// Like [NumPy](https://numpy.org/doc/stable/reference/generated/numpy.testing.assert_equal.html),
    /// this method handles `NaN` comparisons as if `NaN` were a "normal" number (in contrast with
    /// the [IEEE 754 Standards](https://en.wikipedia.org/wiki/NaN), which say that `NaN`
    /// cannot be equal to itself, since `NaN` compared to anything returns `false`). Additionally,
    /// we assume that `NaN == -NaN`.
    ///
    /// # Example
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// assert!(123.45678.is_equal(123.45678));
    /// ```
    fn is_equal(&self, other: Self) -> bool
    where
        Self: Float;

    /// Determines if a floating-point number is equal to another within the specified decimal
    /// precision.
    ///
    /// # Arguments
    ///
    /// * `self` - The first floating-point number to compare.
    /// * `other` - The second floating-point number to compare against.
    /// * `decimal` - Decimal precision to use for comparison.
    ///
    /// # Returns
    ///
    /// A tuple where the first element indicates whether the two floats are equal (`true`) or not
    /// (`false`) and the second element is the actual decimal precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// // Positive decimal precision.
    /// let (result, actual_decimal) = 123.45678.is_equal_to_decimal(123.45891, 2);
    /// assert!(result);
    /// assert_eq!(actual_decimal, 2);
    ///
    /// // Negative decimal precision.
    /// let (result, actual_decimal) = 1200.0.is_equal_to_decimal(1300.0, 2);
    /// assert!(!result);
    /// assert_eq!(actual_decimal, -2);
    /// ```
    ///
    /// # Definition
    ///
    /// We say that two float, $a$ and $b$, are equal to the specified decimal precision, $d$, if
    ///
    /// $$\|a-b\|\leq(1.5)\left(10^{-d}\right)$$
    ///
    /// This is the definition used by [NumPy](https://numpy.org/doc/stable/reference/generated/numpy.testing.assert_almost_equal.html)[^discussion].
    ///
    /// [^discussion]: In a perfect world, we'd want to do the comparison $\|a-b\|<10^{-d}$. For
    /// example, by visual inspection, the numbers 9.876543 and 9.876542 are equal to 5 decimal
    /// places. The _exact_ absolute difference between the two is $10^{-6}$, indicating that they
    /// are equal to 5 decimal places (if we tried comparing them to 6 decimal places, we'd be
    /// checking if $10^{-6}$ is less than $10^{-6}$, which is not true since $10^{-6}=10^{-6}$).
    /// However, evaluating this absolute difference _numerically_ yields
    /// `9.876543 - 9.876542 = 9.999999992515995e-7`, which _is_ less than $10^{-6}$. This indicates
    /// that it would be impossible to reliably compute the decimal precision between any two
    /// numbers numerically; even if you came up with some heuristic that worked in 99% cases, there
    /// would still be a few cases where you wouldn't get the exact decimal precision. <br><br> As a
    /// result, we just use the 1.5 multiplier on $10^{-d}$ when checking equality to $d$ decimal
    /// places. This multiplier of 1.5 is a heuristic that serves to put looser bounds on how many
    /// decimal places two numbers are considered equal to. Since we cannot reliably obtain the
    /// exact decimal precision, we choose to adopt this heuristic that `NumPy` already uses. <br>
    ///
    /// # Note
    ///
    /// Unlike [NumPy's `assert_almost_equal` function](https://numpy.org/doc/stable/reference/generated/numpy.testing.assert_almost_equal.html),
    /// this method allows negative decimal precisions. For example, a decimal precision of
    /// `decimal = -1` corresponds to equality to the 10's place.
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// let (result, decimal) = 12345_f64.is_equal_to_decimal(12340_f64, -1);
    /// assert!(result);
    /// assert_eq!(decimal, -1);
    /// ```
    ///
    /// # Note
    ///
    /// * If the two floats are exactly equal, then the actual decimal precision will be returned as
    ///   either [`f32::MIN_10_EXP`] or [`f64::MIN_10_EXP`] (depending on the type of floats being
    ///   compared).
    /// * If the two floats are completely unequal (for example, comparing `NaN` with `0.0` or
    ///   `Inf`), the actual decimal precision will be returned as either [`f32::MAX_10_EXP`] or
    ///   [`f64::MAX_10_EXP`] (depending on the type of floats being compared).
    ///
    /// # Warning
    ///
    /// Unlike the [IEEE 754 Standards](https://en.wikipedia.org/wiki/NaN), which say that `NaN`
    /// cannot be equal to itself (since `NaN` compared to anything returns `false`), we
    /// assume[^note] here that `NaN` can in fact be equal to itself. For example,
    ///
    /// [^note]: Note that [NumPy](https://numpy.org/doc/stable/reference/generated/numpy.testing.assert_equal.html)
    /// makes the same assumptions.
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// let (result, decimal) = f64::NAN.is_equal_to_decimal(f64::NAN, 15);
    /// assert!(result);
    /// assert_eq!(decimal, 307);
    /// ```
    ///
    /// Additionally, `-NaN` is also considered equal to `NaN`. For example,
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// let (result, decimal) = f64::NAN.is_equal_to_decimal(-f64::NAN, 15);
    /// assert!(result);
    /// assert_eq!(decimal, 307);
    /// ```
    ///
    /// Note that [NumPy](https://numpy.org/doc/stable/reference/generated/numpy.testing.assert_almost_equal.html)
    /// makes the same assumptions.
    fn is_equal_to_decimal(&self, other: Self, decimal: i32) -> (bool, i32)
    where
        Self: Float;

    /// Determines if a floating-point number is equal to another within the specified absolute
    /// tolerance.
    ///
    /// # Arguments
    ///
    /// * `self` - The first floating-point number to compare.
    /// * `other` - The second floating-point number to compare against.
    /// * `atol` - Absolute tolerance.
    ///
    /// # Returns
    ///
    /// A tuple where the first element indicates whether the two floats are equal (`true`) or not
    /// (`false`) to within the specified absolute tolerance, and the second element is the actual
    /// absolute difference between the two floats.
    ///
    /// # Definition
    ///
    /// We define the absolute difference between two floats, $a$ and $b$, as
    ///
    /// $$\text{absolute difference} = \|a-b\|$$
    ///
    /// This method performs the comparison
    ///
    /// $$\|a-b\|\leq\text{absolute tolerance}$$
    ///
    /// It also returns the absolute difference itself.
    ///
    /// # Special Cases
    ///
    /// | Float 1 | Float 2 | Absolute Difference |
    /// | ------- | ------- | ------------------- |
    /// | `NaN` | `NaN` | `0.0` |
    /// | `NaN` | `-NaN` | `0.0` |
    /// | `-NaN` | `-NaN` | `0.0` |
    /// | `Inf` | `NaN` | `NaN` |
    /// | `Inf` | `Inf` | `0.0` |
    /// | `-Inf` | `-Inf` | `0.0` |
    ///
    /// # Example
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// let (result, abs_diff) = 123.45678.is_equal_to_atol(123.45891, 0.1);
    /// assert!(result);
    /// assert_eq!(abs_diff, 0.002130000000008181);
    /// ```
    fn is_equal_to_atol(&self, other: Self, atol: Self) -> (bool, Self)
    where
        Self: Float;

    /// Determines if a floating-point number is equal to another within the specified relative
    /// tolerance.
    ///
    /// # Arguments
    ///
    /// * `self` - The first floating-point number to compare.
    /// * `other` - The second floating-point number to compare against.
    /// * `rtol` - Relative tolerance.
    ///
    /// # Returns
    ///
    /// A tuple where the first element indicates whether the two floats are equal (`true`) or not
    /// (`false`) to within the specified absolute tolerance, and the second element is the actual
    /// relative difference between the two floats.
    ///
    /// # Definition
    ///
    /// We define the relative difference between two floats, $a$ and $b$, as
    ///
    /// $$\text{relative difference} = \frac{\|a-b\|}{\mathrm{max}(\|a\|,\|b\|)}$$
    ///
    /// The use of a maximum in the denominator is used to (a) ensure that `a.is_equal_to_rtol(b)`
    /// and `b.is_equal_to_rtol(a)` return identical results, and (b) to ensure conservatism.
    ///
    /// This method performs the comparison
    ///
    /// $$\|a-b\|\leq(\text{relative tolerance})\mathrm{max}(\|a\|,\|b\|)$$
    ///
    /// # Special Cases
    ///
    /// Unlike `NumPy`, we restrict the relative difference to be in the range $(0,1)$
    /// (corresponding to a maximum percent difference of 100%).
    ///
    /// | Float 1 | Float 2 | Relative Difference |
    /// | ------- | ------- | ------------------- |
    /// | `0.0` | `0.0` | `0.0` |
    /// | `NaN` | `NaN` | `0.0` |
    /// | `NaN` | `-NaN` | `0.0` |
    /// | `-NaN` | `-NaN` | `0.0` |
    /// | `Inf` | `NaN` | `1.0` |
    /// | `Inf` | `Inf` | `0.0` |
    /// | `-Inf` | `-Inf` | `0.0` |
    /// | `NaN` | [any other float] | `1.0` |
    /// | `Inf` | [any other float] | `1.0` |
    ///
    /// # Example
    ///
    /// ```
    /// use numtest::Compare;
    ///
    /// let (result, rel_diff) = 123.45678.is_equal_to_rtol(123.45891, 1e-3);
    /// assert!(result);
    /// assert_eq!(rel_diff, 1.7252703753890107e-5);
    /// ```
    fn is_equal_to_rtol(&self, other: Self, rtol: Self) -> (bool, Self)
    where
        Self: Float;
}

// Implementing Compare trait for f32's and f64's.
macro_rules! impl_compare {
    ($t:ty) => {
        impl Compare for $t {
            // Implements the is_equal method.
            fn is_equal(&self, other: Self) -> bool {
                // Edge case: NaNs.
                if self.is_nan() || other.is_nan() {
                    return self.is_nan() && other.is_nan();
                }

                // Standard case.
                *self == other
            }

            // Implements the is_equal_to_decimal method.
            fn is_equal_to_decimal(&self, other: Self, decimal: i32) -> (bool, i32) {
                // Edge case: NaNs.
                if self.is_nan() || other.is_nan() {
                    if (self.is_nan() && other.is_nan()) {
                        return (true, self.min_10_exp().abs());
                    }
                    return (decimal == -self.max_10_exp(), -self.max_10_exp());
                }

                // Edge case: Infs.
                if self.is_infinite() || other.is_infinite() {
                    if (self.is_infinite() && other.is_infinite()) && (*self == other) {
                        return (true, self.min_10_exp().abs());
                    }
                    return (decimal == -self.max_10_exp(), -self.max_10_exp());
                }

                // Determines if the two numbers are equal to the specified decimal precision.
                let result = (self - other).abs() <= 1.5 * 10.0.powi(-decimal);

                // Determines the actual decimal precision between the two numbers.
                let mut actual_decimal = decimal;
                let mut new_result = result;
                if result {
                    while new_result && actual_decimal < self.min_10_exp().abs() {
                        actual_decimal += 1;
                        new_result = (self - other).abs() <= 1.5 * 10.0.powi(-actual_decimal);
                    }
                    if actual_decimal < self.min_10_exp().abs() {
                        actual_decimal -= 1;
                    }
                } else {
                    while !new_result && actual_decimal > -self.max_10_exp() {
                        actual_decimal -= 1;
                        new_result = (self - other).abs() <= 1.5 * 10.0.powi(-actual_decimal);
                    }
                }
                (result, actual_decimal)
            }

            // Implements the is_equal_to_atol method.
            fn is_equal_to_atol(&self, other: Self, atol: Self) -> (bool, Self) {
                // Edge case: both are NaNs.
                if self.is_nan() && other.is_nan() {
                    (true, 0.0)
                }
                // Edge case: only one is NaN.
                else if self.is_nan() || other.is_nan() {
                    (atol.is_nan(), Self::NAN)
                }
                // Edge case: Infs of same sign.
                else if self.is_infinite()
                    && other.is_infinite()
                    && self.signum() == other.signum()
                {
                    (true, 0.0)
                }
                // Standard case.
                else {
                    let abs_diff = (self - other).abs();
                    let result = abs_diff <= atol;
                    (result, abs_diff)
                }
            }

            // Implements the is_equal_to_rtol method.
            fn is_equal_to_rtol(&self, other: Self, rtol: Self) -> (bool, Self) {
                // Edge case: both are 0.
                if (*self == 0.0) && (other == 0.0) {
                    (true, 0.0)
                }
                // Edge case: both are NaNs.
                else if self.is_nan() && other.is_nan() {
                    (true, 0.0)
                }
                // Edge case: both are Infs.
                else if self.is_infinite() && other.is_infinite() {
                    if self.signum() == other.signum() {
                        (true, 0.0)
                    } else {
                        (rtol == 1.0, 1.0)
                    }
                }
                // Edge case: only one is NaN.
                else if self.is_nan() || other.is_nan() {
                    (rtol == 1.0, 1.0)
                }
                // Edge case: only one is Inf.
                else if self.is_infinite() || other.is_infinite() {
                    (rtol == 1.0, 1.0)
                }
                // Standard case.
                else {
                    let abs_diff = (self - other).abs();
                    let max = self.abs().max(other.abs());
                    let result = abs_diff <= rtol * max;
                    (result, abs_diff / max)
                }
            }
        }
    };
}
impl_compare!(f32);
impl_compare!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    /// Function used for testing the `is_equal_to_decimal` method.
    ///
    /// # Arguments
    ///
    /// * `a` - The first floating-point number to compare.
    /// * `b` - The second floating-point number to compare against.
    /// * `decimal` - Decimal precision to use for comparison.
    /// * `exp_result` - The expected boolean result of the comparison.
    /// * `exp_actual_decimal` - The expected actual decimal precision.
    fn test_decimal<T>(a: T, b: T, decimal: i32, exp_result: bool, exp_actual_decimal: i32)
    where
        T: Compare + Float,
    {
        // Run is_equal_to_decimal() method.
        let (result, actual_decimal) = a.is_equal_to_decimal(b, decimal);

        // Check that the expected result was obtained (true if equal to expected number of decimal
        // places, false otherwise).
        if exp_result {
            assert!(result);
        } else {
            assert!(!result);
        }

        // Check that the decimal precision matches the expected value.
        assert_eq!(actual_decimal, exp_actual_decimal);
    }

    /// Function used for testing the `is_equal_to_atol` method.
    ///
    /// # Arguments
    ///
    /// * `a` - The first floating-point number to compare.
    /// * `b` - The second floating-point number to compare against.
    /// * `atol` - Absolute tolerance.
    /// * `exp_result` - The expected boolean result of the comparison.
    /// * `exp_abs_diff` - The expected absolute difference.
    fn test_atol<T>(a: T, b: T, atol: T, exp_result: bool, exp_abs_diff: T)
    where
        T: Compare + Float + std::fmt::Debug,
    {
        // Run is_equal_to_atol() method.
        let (result, abs_diff) = a.is_equal_to_atol(b, atol);

        // Check that the expected result was obtained.
        if exp_result {
            assert!(result);
        } else {
            assert!(!result);
        }

        // Check that the absolute difference matches the expected value.
        if exp_abs_diff.is_nan() {
            assert!(abs_diff.is_nan());
        } else {
            assert_eq!(abs_diff, exp_abs_diff);
        }
    }

    /// Function used for testing the `is_equal_to_rtol` method.
    ///
    /// # Arguments
    ///
    /// * `a` - The first floating-point number to compare.
    /// * `b` - The second floating-point number to compare against.
    /// * `rtol` - Relative tolerance.
    /// * `exp_result` - The expected boolean result of the comparison.
    /// * `exp_rel_diff` - The expected relative difference.
    fn test_rtol<T>(a: T, b: T, rtol: T, exp_result: bool, exp_rel_diff: T)
    where
        T: Compare + Float + std::fmt::Debug,
    {
        // Run is_equal_to_rtol() method.
        let (result, rel_diff) = a.is_equal_to_rtol(b, rtol);

        // Check that the expected result was obtained.
        if exp_result {
            assert!(result);
        } else {
            assert!(!result);
        }

        // Check that the relative difference matches the expected value.
        if exp_rel_diff.is_nan() {
            assert!(rel_diff.is_nan());
        } else {
            assert_eq!(rel_diff, exp_rel_diff);
        }
    }

    #[test]
    fn is_equal() {
        // f32 equal.
        assert!(0.0_f32.is_equal(0.0_f32));
        assert!(1.0_f32.is_equal(1.0_f32));
        assert!(1.1234_f32.is_equal(1.1234_f32));
        assert!((-1.0_f32).is_equal(-1.0_f32));
        assert!(f32::NAN.is_equal(f32::NAN));
        assert!((-f32::NAN).is_equal(f32::NAN));
        assert!((-f32::NAN).is_equal(-f32::NAN));
        assert!(f32::INFINITY.is_equal(f32::INFINITY));
        assert!(f32::NEG_INFINITY.is_equal(f32::NEG_INFINITY));
        assert!((-f32::INFINITY).is_equal(f32::NEG_INFINITY));

        // f32 unequal.
        assert!(!0.0_f32.is_equal(1.0_f32));
        assert!(!1.234_567_f32.is_equal(1.234_568_f32));
        assert!(!f32::NAN.is_equal(0.0_f32));
        assert!(!f32::NAN.is_equal(f32::INFINITY));

        // f64 equal.
        assert!(0.0_f64.is_equal(0.0_f64));
        assert!(1.0_f64.is_equal(1.0_f64));
        assert!(1.1234_f64.is_equal(1.1234_f64));
        assert!((-1.0_f64).is_equal(-1.0_f64));
        assert!(f64::NAN.is_equal(f64::NAN));
        assert!((-f64::NAN).is_equal(f64::NAN));
        assert!((-f64::NAN).is_equal(-f64::NAN));
        assert!(f64::INFINITY.is_equal(f64::INFINITY));
        assert!(f64::NEG_INFINITY.is_equal(f64::NEG_INFINITY));
        assert!((-f64::INFINITY).is_equal(f64::NEG_INFINITY));

        // f64 unequal.
        assert!(!0.0_f64.is_equal(1.0_f64));
        assert!(!1.234_567_f64.is_equal(1.234_568_f64));
        assert!(!f64::NAN.is_equal(0.0_f64));
        assert!(!f64::NAN.is_equal(f64::INFINITY));
    }

    #[test]
    fn exact_equality() {
        // f32
        test_decimal(1.0_f32, 1.0_f32, 5, true, 37);
        test_atol(1.0_f32, 1.0_f32, 0.1, true, 0.0);
        test_rtol(1.0_f32, 1.0_f32, 0.01, true, 0.0);

        // f64
        test_decimal(1.0_f64, 1.0_f64, 5, true, 307);
        test_atol(1.0_f64, 1.0_f64, 0.1, true, 0.0);
        test_rtol(1.0_f64, 1.0_f64, 0.01, true, 0.0);
    }

    #[test]
    fn exact_equality_within_precision() {
        // f32
        test_decimal(1.0_f32, 0.999_999_99, 7, true, 37);
        test_atol(1.0_f32, 0.999_999_99, 1e-8, true, 0.0);
        test_rtol(1.0_f32, 0.999_999_99, 1e-8, true, 0.0);
        test_decimal(1.0_f32, 1.00000001, 8, true, 37);
        test_atol(1.0_f32, 1.00000001, 1e-8, true, 0.0);
        test_rtol(1.0_f32, 1.00000001, 1e-8, true, 0.0);

        // f64
        test_decimal(1.0_f64, 0.99999999999999999, 15, true, 307);
        test_atol(1.0_f64, 0.99999999999999999, 1e-15, true, 0.0);
        test_rtol(1.0_f64, 0.99999999999999999, 1e-15, true, 0.0);
        test_decimal(1.0_f64, 1.00000000000000001, 15, true, 307);
        test_atol(1.0_f64, 1.00000000000000001, 1e-15, true, 0.0);
        test_rtol(1.0_f64, 1.00000000000000001, 1e-15, true, 0.0);
    }

    #[test]
    fn near_equality_due_to_float_precision() {
        // f32
        test_decimal(1.0_f32, 0.9999999, 8, false, 7);
        test_atol(1.0_f32, 0.9999999, 1e-8, false, 1.1920929e-7);
        test_rtol(1.0_f32, 0.9999999, 1e-8, false, 1.1920929e-7);
        test_decimal(1.0_f32, 1.0000001, 8, false, 7);
        test_atol(1.0_f32, 1.0000001, 1e-8, false, 1.1920929e-7);
        test_rtol(1.0_f32, 1.0000001, 1e-8, false, 1.19209275e-7);

        // f64
        test_decimal(1.0_f64, 0.9999999999999999, 16, true, 16);
        test_atol(
            1.0_f64,
            0.9999999999999999,
            1e-17,
            false,
            1.1102230246251565e-16,
        );
        test_rtol(
            1.0_f64,
            0.9999999999999999,
            1e-17,
            false,
            1.1102230246251565e-16,
        );
        test_decimal(1.0_f64, 1.000000000000001, 15, true, 15);
        test_atol(
            1.0_f64,
            1.000000000000001,
            1e-17,
            false,
            1.1102230246251565e-15,
        );
        test_rtol(
            1.0_f64,
            1.000000000000001,
            1e-17,
            false,
            1.1102230246251554e-15,
        );
    }

    #[test]
    fn one_or_both_numbers_0() {
        // f32 #1
        test_decimal(0.0_f32, 0.0_f32, 7, true, 37);
        test_atol(0.0_f32, 0.0_f32, 1e-7, true, 0.0);
        test_rtol(0.0_f32, 0.0_f32, 1e-1, true, 0.0);

        // f32 #2
        test_decimal(0.0_f32, -0.01_f32, 3, false, 2);
        test_atol(0.0_f32, -0.01_f32, 0.0001, false, 0.01);
        test_rtol(0.0_f32, -0.01_f32, 0.5, false, 1.0);

        // f32 #3
        test_decimal(0.00001_f32, 0.0_f32, 3, true, 5);
        test_atol(0.00001_f32, 0.0_f32, 0.00001, true, 0.00001);
        test_rtol(0.00001_f32, 0.0_f32, 1.0, true, 1.0);

        // f64 #1
        test_decimal(0.0_f64, 0.0_f64, 7, true, 307);
        test_atol(0.0_f64, 0.0_f64, 1e-15, true, 0.0);
        test_rtol(0.0_f64, 0.0_f64, 0.0, true, 0.0);

        // f64 #2
        test_decimal(0.0_f64, -0.01_f64, 3, false, 2);
        test_atol(0.0_f64, -0.01_f64, 1e-3, false, 0.01);
        test_rtol(0.0_f64, -0.01_f64, 0.01, false, 1.0);

        // f64 #3
        test_decimal(0.00001_f64, 0.0_f64, 3, true, 5);
        test_atol(0.00001_f64, 0.0_f64, 1e-7, false, 0.00001);
        test_rtol(0.00001_f64, 0.0_f64, 0.01, false, 1.0);
    }

    #[test]
    fn basic_test_small_magnitude() {
        // f32
        test_decimal(1.0_f32, 1.01_f32, 3, false, 2);
        test_atol(1.0_f32, 1.01_f32, 1e-3, false, 0.00999999);
        test_rtol(1.0_f32, 1.01_f32, 1e-4, false, 0.009900981);

        // f64
        test_decimal(1.0_f64, 1.01_f64, 2, true, 2);
        test_atol(1.0_f64, 1.01_f64, 0.01, false, 0.010000000000000009);
        test_rtol(1.0_f64, 1.01_f64, 1e-4, false, 0.00990099009900991);
    }

    #[test]
    fn basic_test_larger_magnitude() {
        // f32
        test_decimal(1000.0_f32, 999.99_f32, 3, false, 2);
        test_atol(1000.0_f32, 999.99_f32, 1e-3, false, 0.010009766);
        test_rtol(1000.0_f32, 999.99_f32, 1e-4, true, 1.0009766e-5);

        // f64
        test_decimal(1000.0_f64, 999.99_f64, 2, true, 2);
        test_atol(1000.0_f64, 999.99_f64, 1e-3, false, 0.009999999999990905);
        test_rtol(1000.0_f64, 999.99_f64, 1e-4, true, 9.999999999990905e-6);
    }

    #[test]
    fn negative_decimal_places() {
        // f32
        test_decimal(12345_f32, 12300_f32, -2, true, -2);
        test_atol(12345_f32, 12300_f32, 50.0, true, 45.0);
        test_rtol(12345_f32, 12300_f32, 0.1, true, 0.0036452005);

        // f64
        test_decimal(12345_f64, 12300_f64, -2, true, -2);
        test_atol(12345_f64, 12300_f64, 50.0, true, 45.0);
        test_rtol(12345_f64, 12300_f64, 0.1, true, 0.0036452004860267314);
    }

    #[test]
    fn tiny_numbers() {
        // f32 #1
        test_decimal(1.0e-30_f32, 1.01e-30_f32, 31, true, 32);
        test_atol(1.0e-30_f32, 1.01e-30_f32, 1e-31, true, 9.9999775e-33);
        test_rtol(1.0e-30_f32, 1.01e-30_f32, 0.1, true, 0.009900968);

        // f32 #2
        test_decimal(1.0e-30_f32, 1.01e-30_f32, 32, true, 32);
        test_atol(1.0e-30_f32, 1.01e-30_f32, 1e-32, true, 9.9999775e-33);
        test_rtol(1.0e-30_f32, 1.01e-30_f32, 0.01, true, 0.009900968);

        // f32 #3
        test_decimal(1.0e-30_f32, 1.01e-30_f32, 33, false, 32);
        test_atol(1.0e-30_f32, 1.01e-30_f32, 1e-33, false, 9.9999775e-33);
        test_rtol(1.0e-30_f32, 1.01e-30_f32, 0.001, false, 0.009900968);

        // f32 #4
        test_decimal(1.0e-30_f32, 1.1e-30_f32, 32, false, 31);
        test_atol(1.0e-30_f32, 1.1e-30_f32, 1e-32, false, 9.999996e-32);
        test_rtol(1.0e-30_f32, 1.1e-30_f32, 0.01, false, 0.09090906);

        // f64 #1
        test_decimal(1.0e-100_f64, 1.01e-100_f64, 101, true, 102);
        test_atol(
            1.0e-100_f64,
            1.01e-100_f64,
            1e-101,
            true,
            1.0000000000000005e-102,
        );
        test_rtol(1.0e-100_f64, 1.01e-100_f64, 0.1, true, 0.009900990099009906);

        // f64 #2
        test_decimal(1.0e-100_f64, 1.01e-100_f64, 102, true, 102);
        test_atol(
            1.0e-100_f64,
            1.01e-100_f64,
            1e-102,
            false,
            1.0000000000000005e-102,
        );
        test_rtol(
            1.0e-100_f64,
            1.01e-100_f64,
            0.01,
            true,
            0.009900990099009906,
        );

        // f64 #3
        test_decimal(1.0e-100_f64, 1.01e-100_f64, 103, false, 102);
        test_atol(
            1.0e-100_f64,
            1.01e-100_f64,
            1e-103,
            false,
            1.0000000000000005e-102,
        );
        test_rtol(
            1.0e-100_f64,
            1.01e-100_f64,
            0.001,
            false,
            0.009900990099009906,
        );

        // f64 #4
        test_decimal(1.0e-100_f64, 1.1e-100_f64, 102, false, 101);
        test_atol(
            1.0e-100_f64,
            1.1e-100_f64,
            1e-102,
            false,
            9.999999999999993e-102,
        );
        test_rtol(1.0e-100_f64, 1.1e-100_f64, 0.01, false, 0.09090909090909084);
    }

    #[test]
    fn infinity() {
        // f32 #1
        test_decimal(f32::INFINITY, f32::INFINITY, 7, true, 37);
        test_atol(f32::INFINITY, f32::INFINITY, 1e-7, true, 0.0);
        test_rtol(f32::INFINITY, f32::INFINITY, 0.001, true, 0.0);

        // f32 #2
        test_decimal(f32::INFINITY, f32::NEG_INFINITY, 7, false, -38);
        test_atol(f32::INFINITY, f32::NEG_INFINITY, 1e-7, false, f32::INFINITY);
        test_rtol(f32::INFINITY, f32::NEG_INFINITY, 0.001, false, 1.0);

        // f32 #3
        test_decimal(f32::NEG_INFINITY, f32::NEG_INFINITY, 7, true, 37);
        test_atol(f32::NEG_INFINITY, f32::NEG_INFINITY, 1e-7, true, 0.0);
        test_rtol(f32::NEG_INFINITY, f32::NEG_INFINITY, 0.001, true, 0.0);

        // f32 #4
        test_decimal(f32::INFINITY, 0.0, 7, false, -38);
        test_atol(f32::NEG_INFINITY, 0.0, 1e-7, false, f32::INFINITY);
        test_rtol(f32::NEG_INFINITY, 0.0, 0.001, false, 1.0);

        // f32 #5
        test_decimal(0.0, f32::INFINITY, 7, false, -38);
        test_atol(0.0, f32::INFINITY, 1e-7, false, f32::INFINITY);
        test_rtol(0.0, f32::INFINITY, 0.001, false, 1.0);

        // f64 #1
        test_decimal(f64::INFINITY, f64::INFINITY, 7, true, 307);
        test_atol(f64::INFINITY, f64::INFINITY, 1e-7, true, 0.0);
        test_rtol(f64::INFINITY, f64::INFINITY, 0.001, true, 0.0);

        // f64 #2
        test_decimal(f64::INFINITY, f64::NEG_INFINITY, 7, false, -308);
        test_atol(f64::INFINITY, f64::NEG_INFINITY, 1e-7, false, f64::INFINITY);
        test_rtol(f64::INFINITY, f64::NEG_INFINITY, 0.001, false, 1.0);

        // f64 #3
        test_decimal(f64::NEG_INFINITY, f64::NEG_INFINITY, 7, true, 307);
        test_atol(f64::NEG_INFINITY, f64::NEG_INFINITY, 1e-7, true, 0.0);
        test_rtol(f64::NEG_INFINITY, f64::NEG_INFINITY, 0.001, true, 0.0);

        // f64 #4
        test_decimal(f64::INFINITY, 0.0, 7, false, -308);
        test_atol(f64::INFINITY, 0.0, 1e-7, false, f64::INFINITY);
        test_rtol(f64::INFINITY, 0.0, 0.001, false, 1.0);

        // f64 #5
        test_decimal(0.0, f64::INFINITY, 7, false, -308);
        test_atol(0.0, f64::INFINITY, 1e-7, false, f64::INFINITY);
        test_rtol(0.0, f64::INFINITY, 0.001, false, 1.0);
    }

    #[test]
    fn nan() {
        // f32 #1
        test_decimal(f32::NAN, f32::NAN, 7, true, 37);
        test_atol(f32::NAN, f32::NAN, 0.0, true, 0.0);
        test_rtol(f32::NAN, f32::NAN, 0.0, true, 0.0);

        // f32 #2
        test_decimal(f32::NAN, -f32::NAN, 7, true, 37);
        test_atol(f32::NAN, -f32::NAN, 0.0, true, 0.0);
        test_rtol(f32::NAN, -f32::NAN, 0.0, true, 0.0);

        // f32 #3
        test_decimal(f32::NAN, 0.0, 7, false, -38);
        test_atol(f32::NAN, 0.0, 0.0, false, f32::NAN);
        test_rtol(f32::NAN, 0.0, 0.0, false, 1.0);

        // f32 #4
        test_decimal(0.0, f32::NAN, -38, true, -38);
        test_atol(0.0, f32::NAN, f32::NAN, true, f32::NAN);
        test_rtol(0.0, f32::NAN, 1.0, true, 1.0);

        // f64 #1
        test_decimal(f64::NAN, f64::NAN, 7, true, 307);
        test_atol(f64::NAN, f64::NAN, 0.0, true, 0.0);
        test_rtol(f64::NAN, f64::NAN, 0.0, true, 0.0);

        // f64 #2
        test_decimal(f64::NAN, -f64::NAN, 7, true, 307);
        test_atol(f64::NAN, -f64::NAN, 0.0, true, 0.0);
        test_rtol(f64::NAN, -f64::NAN, 0.0, true, 0.0);

        // f64 #3
        test_decimal(f64::NAN, 0.0, 7, false, -308);
        test_atol(f64::NAN, 0.0, 0.0, false, f64::NAN);
        test_rtol(f64::NAN, 0.0, 0.0, false, 1.0);

        // f64 #4
        test_decimal(0.0, f64::NAN, -308, true, -308);
        test_atol(0.0, f64::NAN, f64::NAN, true, f64::NAN);
        test_rtol(0.0, f64::NAN, 1.0, true, 1.0);
    }

    #[test]
    fn infinity_vs_nan() {
        // f32 #1
        test_decimal(f32::INFINITY, f32::NAN, 7, false, -38);
        test_atol(f32::INFINITY, f32::NAN, f32::NAN, true, f32::NAN);
        test_rtol(f32::INFINITY, f32::NAN, 0.9999, false, 1.0);

        // f32 #1
        test_decimal(f32::NAN, f32::INFINITY, 7, false, -38);
        test_atol(f32::NAN, f32::INFINITY, 0.1, false, f32::NAN);
        test_rtol(f32::NAN, f32::INFINITY, 1.0, true, 1.0);

        // f64 #1
        test_decimal(f64::INFINITY, f64::NAN, 7, false, -308);
        test_atol(f64::INFINITY, f64::NAN, f64::NAN, true, f64::NAN);
        test_rtol(f64::INFINITY, f64::NAN, 0.9999, false, 1.0);

        // f64 #2
        test_decimal(f64::NAN, f64::INFINITY, 7, false, -308);
        test_atol(f64::NAN, f64::INFINITY, 0.1, false, f64::NAN);
        test_rtol(f64::NAN, f64::INFINITY, 1.0, true, 1.0);
    }

    // https://github.com/numpy/numpy/blob/main/numpy/testing/tests/test_utils.py
    #[test]
    fn numpy() {
        // from TestArrayAlmostEqual::test_closenesss
        test_decimal(1.499999_f64, 0.0_f64, 0, true, 0);
        test_decimal(1.4999999_f64, 1.49999991_f64, 7, true, 8);
        test_decimal(0.00003_f64, 0.0_f64, 7, false, 4);
        test_atol(0.00003_f64, 0.0_f64, 3e-5, true, 3e-5);

        // from TestArrayAlmostEqual::test_simple
        test_decimal(1234.2222_f64, 1234.2223_f64, 3, true, 4);
        test_decimal(1234.2222_f64, 1234.2223_f64, 4, true, 4);
        test_decimal(1234.2222_f64, 1234.2223_f64, 5, false, 4);
        test_rtol(
            1234.2222_f64,
            1234.2223_f64,
            1e-7,
            true,
            8.10226812259817e-8,
        );

        // from TestArrayAlmostEqual::test_array_vs_scalar
        test_decimal(5498.42354_f64, 5498.42354_f64, 9, true, 307);

        // from TestArrayAlmostEqual::test_nan
        test_decimal(f64::NAN, 1.0_f64, 0, false, -308);
    }

    #[test]
    fn misc_targeted_tests() {
        // f32
        test_decimal(0.000001_f32, 0.0000011_f32, 6, true, 7);
        test_decimal(-12345.679_f32, -12345.679_f32, 7, true, 37);
        test_decimal(-12345.679_f32, -12345.678_f32, 4, false, 3);
        test_decimal(1.234567e10_f32, 1.2345678e10_f32, 0, false, -4);
        test_decimal(1.2345678e10_f32, 1.2345678e10_f32, 7, true, 37);
        test_decimal(1.0_f32, 1.1_f32, 0, true, 1);
        test_decimal(0.123456_f32, 0.12345601_f32, 7, true, 8);
        test_decimal(1.0e5_f32, 1.0e-2_f32, 7, false, -5);
        test_decimal(1.0e5_f32, 1.0e-5_f32, 0, false, -5);
        test_decimal(123.456_f32, -123.456_f32, 0, false, -3);
        test_decimal(-123.456_f32, -123.456_f32, 7, true, 37);
        test_decimal(0.0_f32, 0.0000001_f32, 7, true, 7);
        test_decimal(0.0_f32, 0.0000001_f32, 8, false, 7);
        test_decimal(3.0_f32, 3.000001_f32, 5, true, 6);
        test_decimal(1.23_f32, 1.234_f32, 2, true, 2);
        test_decimal(4.56_f32, 4.567_f32, 2, true, 2);
        test_decimal(7.89_f32, 7.891_f32, 2, true, 3);
        test_decimal(1.1_f32, 1.11_f32, 1, true, 2);
        test_decimal(2.2_f32, 2.22_f32, 1, true, 1);
        test_decimal(3.3_f32, 3.33_f32, 1, true, 1);
        test_decimal(4.4_f32, 4.44_f32, 1, true, 1);
        test_decimal(5.5_f32, 5.55_f32, 1, true, 1);
        test_decimal(6.6_f32, 6.66_f32, 1, true, 1);
        test_decimal(7.7_f32, 7.77_f32, 1, true, 1);
        test_decimal(8.8_f32, 8.88_f32, 1, true, 1);
        test_decimal(9.9_f32, 9.99_f32, 1, true, 1);

        // f64
        test_decimal(0.000001_f64, 0.0000011_f64, 6, true, 7);
        test_decimal(-12345.679_f64, -12345.679_f64, 7, true, 307);
        test_decimal(-12345.679_f64, -12345.678_f64, 4, false, 3);
        test_decimal(1.234567e10_f64, 1.2345678e10_f64, 0, false, -4);
        test_decimal(1.2345678e10_f64, 1.2345678e10_f64, 7, true, 307);
        test_decimal(1.0_f64, 1.1_f64, 0, true, 1);
        test_decimal(0.123456_f64, 0.12345601_f64, 7, true, 8);
        test_decimal(1.0e5_f64, 1.0e-2_f64, 7, false, -5);
        test_decimal(1.0e5_f64, 1.0e-5_f64, 0, false, -5);
        test_decimal(123.456_f64, -123.456_f64, 0, false, -3);
        test_decimal(-123.456_f64, -123.456_f64, 7, true, 307);
        test_decimal(0.0_f64, 0.0000001_f64, 7, true, 7);
        test_decimal(0.0_f64, 0.0000001_f64, 8, false, 7);
        test_decimal(3.0_f64, 3.000001_f64, 5, true, 6);
        test_decimal(1.23_f64, 1.234_f64, 2, true, 2);
        test_decimal(4.56_f64, 4.567_f64, 2, true, 2);
        test_decimal(7.89_f64, 7.891_f64, 2, true, 3);
        test_decimal(1.1_f64, 1.11_f64, 1, true, 2);
        test_decimal(2.2_f64, 2.22_f64, 1, true, 1);
        test_decimal(3.3_f64, 3.33_f64, 1, true, 1);
        test_decimal(4.4_f64, 4.44_f64, 1, true, 1);
        test_decimal(5.5_f64, 5.55_f64, 1, true, 1);
        test_decimal(6.6_f64, 6.66_f64, 1, true, 1);
        test_decimal(7.7_f64, 7.77_f64, 1, true, 1);
        test_decimal(8.8_f64, 8.88_f64, 1, true, 1);
        test_decimal(9.9_f64, 9.99_f64, 1, true, 1);
    }

    #[test]
    fn random_tests() {
        // f32
        test_decimal(76.20494_f32, 76.20501_f32, 4, true, 4);
        test_decimal(-10.471176_f32, -10.4711685_f32, 6, false, 5);
        test_decimal(-76.6927_f32, -76.69034_f32, 1, true, 2);
        test_decimal(-28.888557_f32, -28.879892_f32, 2, true, 2);
        test_decimal(81.864136_f32, 81.87704_f32, 0, true, 2);
        test_decimal(8.148766_f32, 8.156124_f32, 0, true, 2);
        test_decimal(8.148766_f32, 8.156124_f32, 2, true, 2);
        test_decimal(8.148766_f32, 8.156124_f32, 3, false, 2);
        test_decimal(6.9613953_f32, 6.9613953_f32, 7, true, 37);

        // f64
        test_decimal(76.2049420494204_f64, 76.20494250128455_f64, 4, true, 6);
        test_decimal(-10.471176234456469_f64, -10.471176234456333, 6, true, 13);
        test_decimal(-76.6927_f64, -76.69034_f64, 1, true, 2);
        test_decimal(-28.888557333333333_f64, -28.879892333333333_f64, 2, true, 2);
        test_decimal(81.86413612345678_f64, 81.8771111111111_f64, 0, true, 2);
        test_decimal(8.148766_f64, 8.156124_f64, 0, true, 2);
        test_decimal(8.148766_f64, 8.156124_f64, 2, true, 2);
        test_decimal(8.148766_f64, 8.156124_f64, 3, false, 2);
        test_decimal(6.9613953_f64, 6.9613953_f64, 7, true, 307);
    }
}

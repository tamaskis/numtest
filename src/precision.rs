/// Trait for accessing information regarding the numerical precision of a floating-point type given
/// an instance of that type.
///
/// # References
///
/// * [Wikipedia: Single-precision floating-point format](https://en.wikipedia.org/wiki/Single-precision_floating-point_format)
/// * [Wikipedia: Double-precision floating-point format](https://en.wikipedia.org/wiki/Double-precision_floating-point_format)
pub trait Precision {
    /// Maximum number of guarenteed correct decimal places for a floating-point number.
    const MAX_DECIMAL: u32;

    /// Method that can be called on an instance of a floating-point type to return the maximum
    /// number of correct decimal places for that floating-point type.
    ///
    /// # Returns
    ///
    /// Maximum number of guarenteed correct decimal places for the type of this floating-point
    /// variable.
    fn max_decimal(&self) -> u32;

    /// Method that can be called on an instance of a floating-point type to return the maximum
    /// power of 10 exponent for that floating-point type.
    ///
    /// # Returns
    ///
    /// Maximum power of 10 exponent for the type of this floating-point variable.
    fn max_10_exp(&self) -> i32;

    /// Method that can be called on an instance of a floating-point type to return the minimum
    /// power of 10 exponent for that floating-point type.
    ///
    /// # Returns
    ///
    /// Minimum exponent for the type of this floating-point variable.
    fn min_10_exp(&self) -> i32;

    /// Method that can be called on an instance of a floating-point type to return the machine
    /// epsilon for that floating-point type.
    ///
    /// # Returns
    ///
    /// Machine epsilon for the type of this floating-point variable.
    fn epsilon(&self) -> Self;
}

// Implementing Precision trait for f32's.
// https://en.wikipedia.org/wiki/Single-precision_floating-point_format
impl Precision for f32 {
    const MAX_DECIMAL: u32 = 7;
    fn max_decimal(&self) -> u32 {
        f32::MAX_DECIMAL
    }
    fn max_10_exp(&self) -> i32 {
        f32::MAX_10_EXP
    }
    fn min_10_exp(&self) -> i32 {
        f32::MIN_10_EXP
    }
    fn epsilon(&self) -> Self {
        f32::EPSILON
    }
}

// Implementing Precision trait for f64's.
// https://en.wikipedia.org/wiki/Double-precision_floating-point_format
impl Precision for f64 {
    const MAX_DECIMAL: u32 = 15;
    fn max_decimal(&self) -> u32 {
        f64::MAX_DECIMAL
    }
    fn max_10_exp(&self) -> i32 {
        f64::MAX_10_EXP
    }
    fn min_10_exp(&self) -> i32 {
        f64::MIN_10_EXP
    }
    fn epsilon(&self) -> Self {
        f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precision_f32() {
        assert_eq!(f32::MAX_DECIMAL, 7_u32);
        assert_eq!(0.0_f32.max_decimal(), f32::MAX_DECIMAL);
        assert_eq!(0.0_f32.max_10_exp(), f32::MAX_10_EXP);
        assert_eq!(0.0_f32.max_10_exp(), 38_i32);
        assert_eq!(0.0_f32.min_10_exp(), f32::MIN_10_EXP);
        assert_eq!(0.0_f32.min_10_exp(), -37_i32);
        assert_eq!(0.0_f32.epsilon(), f32::EPSILON);
    }

    #[test]
    fn precision_f64() {
        assert_eq!(f64::MAX_DECIMAL, 15_u32);
        assert_eq!(0.0_f64.max_decimal(), f64::MAX_DECIMAL);
        assert_eq!(0.0_f64.max_10_exp(), f64::MAX_10_EXP);
        assert_eq!(0.0_f64.max_10_exp(), 308_i32);
        assert_eq!(0.0_f64.min_10_exp(), f64::MIN_10_EXP);
        assert_eq!(0.0_f64.min_10_exp(), -307_i32);
        assert_eq!(0.0_f64.epsilon(), f64::EPSILON);
    }
}

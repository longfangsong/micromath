use crate::f32ext::F32Ext;

/// `f64` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
pub trait F64Ext: Sized {
    /// Compute absolute value with a constant-time, data-independent
    /// implementation.
    fn abs(self) -> f64;

    /// Approximate `asin(x)` in radians in the range `[-pi/2, pi/2]`.
    fn asin(self) -> f64;

    /// Approximate `acos(x)` in radians in the range `[0, pi]`
    fn acos(self) -> f64;

    /// Approximate `atan(x)` in radians with a maximum error of `0.002`.
    fn atan(self) -> f64;

    /// Approximate `atan(x)` normalized to the `[−1,1]` range with a maximum
    /// error of `0.1620` degrees.
    fn atan_norm(self) -> f64;

    /// Approximate the four quadrant arctangent `atan2(x)` in radians, with
    /// a maximum error of `0.002`.
    fn atan2(self, other: f64) -> f64;

    /// Approximate the four quadrant arctangent.
    /// Normalized to the `[0,4)` range with a maximum error of `0.1620` degrees.
    fn atan2_norm(self, other: f64) -> f64;

    /// Approximate floating point ceiling.
    fn ceil(self) -> f64;

    /// Approximate cosine in radians with a maximum error of `0.002`.
    fn cos(self) -> f64;

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    fn div_euclid(self, other: f64) -> f64;

    /// Approximate floating point floor.
    fn floor(self) -> f64;

    /// Approximate the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    fn hypot(self, other: f64) -> f64;

    /// Approximate `1/x` with an average deviation of ~8%.
    fn inv(self) -> f64;

    /// Approximate inverse square root with an average deviation of ~5%.
    fn invsqrt(self) -> f64;

    /// Calculates the least nonnegative remainder of `self (mod other)`.
    fn rem_euclid(self, other: f64) -> f64;

    /// Approximate sine in radians with a maximum error of `0.002`.
    fn sin(self) -> f64;

    /// Approximate square root with an average deviation of ~5%.
    fn sqrt(self) -> f64;

    /// Approximate `tan(x)` in radians with a maximum error of `0.6`.
    fn tan(self) -> f64;

    /// Retrieve whole number part of floating point with sign.
    fn trunc(self) -> f64;

    /// Round the number part of floating point with sign.
    fn round(self) -> f64;

    /// Retrieve the fractional part of floating point with sign.
    fn fract(self) -> f64;

    /// Copies the sign from one number to another and returns it.
    fn copysign(self, sign: f64) -> f64;

    /// Approximate `ln(x)`.
    fn ln(self) -> f64;

    /// Approximate `e^x`.
    fn exp(self) -> f64;

    /// Approximate `log` with an arbitrary base.
    fn log(self, base: f64) -> f64;

    /// Approximate `log2`.
    fn log2(self) -> f64;

    /// Approximate `log10`.
    fn log10(self) -> f64;

    /// Approximate `self^n`.
    fn powf(self, n: f64) -> f64;

    /// Approximate `self^n` where n is an `i32`
    fn powi(self, n: i32) -> f64;
}

impl F64Ext for f64 {
    fn abs(self) -> f64 {
        (self as f32).abs() as _
    }

    fn asin(self) -> f64 {
        (self as f32).asin() as _
    }

    fn acos(self) -> f64 {
        (self as f32).acos() as _
    }

    fn atan(self) -> f64 {
        (self as f32).atan() as _
    }

    fn atan_norm(self) -> f64 {
        (self as f32).atan_norm() as _
    }

    fn atan2(self, other: f64) -> f64 {
        (self as f32).atan2(other as f32) as _
    }

    fn atan2_norm(self, other: f64) -> f64 {
        (self as f32).atan2_norm(other as f32) as _
    }

    fn ceil(self) -> f64 {
        (self as f32).ceil() as _
    }

    fn cos(self) -> f64 {
        (self as f32).cos() as _
    }

    fn div_euclid(self, other: f64) -> f64 {
        (self as f32).div_euclid(other as f32) as _
    }

    fn floor(self) -> f64 {
        (self as f32).floor() as _
    }

    fn hypot(self, other: f64) -> f64 {
        (self as f32).hypot(other as f32) as _
    }

    fn inv(self) -> f64 {
        (self as f32).inv() as _
    }

    fn invsqrt(self) -> f64 {
        (self as f32).invsqrt() as _
    }

    fn rem_euclid(self, other: f64) -> f64 {
        (self as f32).rem_euclid(other as f32) as _
    }

    fn sin(self) -> f64 {
        (self as f32).sin() as _
    }

    fn sqrt(self) -> f64 {
        (self as f32).sqrt() as _
    }

    fn tan(self) -> f64 {
        (self as f32).tan() as _
    }

    fn trunc(self) -> f64 {
        (self as f32).trunc() as _
    }

    fn round(self) -> f64 {
        (self as f32).round() as _
    }

    fn fract(self) -> f64 {
        (self as f32).fract() as _
    }

    fn copysign(self, sign: f64) -> f64 {
        (self as f32).copysign(sign as f32) as _
    }

    fn ln(self) -> f64 {
        (self as f32).ln() as _
    }

    fn exp(self) -> f64 {
        (self as f32).exp() as _
    }

    fn log(self, base: f64) -> f64 {
        (self as f32).log(base as f32) as _
    }

    fn log2(self) -> f64 {
        (self as f32).log2() as _
    }

    fn log10(self) -> f64 {
        (self as f32).log10() as _
    }

    fn powf(self, n: f64) -> f64 {
        (self as f32).powf(n as f32) as _
    }

    fn powi(self, n: i32) -> f64 {
        (self as f32).powi(n) as _
    }
}

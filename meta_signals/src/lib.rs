use std::{fmt::Display, ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

/// A meta-signal representing a value between 0.0 and 1.0 inclusive.
/// Supports arithmetic operations and comparisons.
/// Construction and all operations ensure the value remains within bounds.
#[derive(Debug, Clone, Copy)]
pub struct MetaSignal {
    value: f64,
}

impl From<f64> for MetaSignal {
    fn from(value: f64) -> Self {
        MetaSignal::new(value)
    }
}

impl Deref for MetaSignal {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MetaSignal {
    /// Creates a new MetaSignal, clamping the value between 0.0 and 1.0.
    pub fn new(value: f64) -> Self {
        Self { value: value.clamp(0.0, 1.0) }
    }

    pub const LOW: MetaSignal = MetaSignal { value: 0.0 };
    pub const HIGH: MetaSignal = MetaSignal { value: 1.0 };
}

impl Default for MetaSignal {
    fn default() -> Self {
        MetaSignal::new(0.0)
    }
}

impl Add for MetaSignal {
    type Output = MetaSignal;

    /// Adds two meta_signals, clamping the result to a maximum of 1.0.
    fn add(self, other: Self) -> Self::Output {
        let value = self.value + other.value;
        MetaSignal {
            value: f64::min(value, 1.0),
        }
    }
}

impl Add<MetaSignal> for f64 {
    type Output = MetaSignal;

    /// Adds a MetaSignal to a f64, clamping the result between 0.0 and 1.0.
    fn add(self, other: MetaSignal) -> Self::Output {
        MetaSignal::new(self + other.value)
    }
}

impl Add<f64> for MetaSignal {
    type Output = MetaSignal;

    /// Adds a f64 to a MetaSignal, clamping the result between 0.0 and 1.0.
    fn add(self, other: f64) -> Self::Output {
        MetaSignal::new(self.value + other)
    }
}

impl AddAssign for MetaSignal {
    /// Adds another MetaSignal to this one, clamping the result to a maximum of 1.0.
    fn add_assign(&mut self, other: Self) {
        self.value = f64::min(self.value + other.value, 1.0);
    }
}

impl AddAssign<MetaSignal> for f64 {
    /// Adds a MetaSignal to this f64, clamping the result between 0.0 and 1.0.
    fn add_assign(&mut self, other: MetaSignal) {
        let value = *self + other.value;
        *self = value.clamp(0.0, 1.0);
    }
}

impl AddAssign<f64> for MetaSignal {
    /// Adds a f64 to this MetaSignal, clamping the result between 0.0 and 1.0.
    fn add_assign(&mut self, other: f64) {
        let value = self.value + other;
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Sub for MetaSignal {
    type Output = MetaSignal;

    /// Subtracts one MetaSignal from another, clamping the result to a minimum of 0.0.
    fn sub(self, other: Self) -> Self::Output {
        let value = self.value - other.value;
        MetaSignal {
            value: f64::max(value, 0.0),
        }
    }
}

impl Sub<MetaSignal> for f64 {
    type Output = MetaSignal;

    /// Subtracts a MetaSignal from a f64, clamping the result between 0.0 and 1.0.
    fn sub(self, other: MetaSignal) -> Self::Output {
        MetaSignal::new(self - other.value)
    }
}

impl Sub<f64> for MetaSignal {
    type Output = MetaSignal;

    /// Subtracts a f64 from a MetaSignal, clamping the result between 0.0 and 1.0.
    fn sub(self, other: f64) -> Self::Output {
        MetaSignal::new(self.value - other)
    }
}

impl SubAssign for MetaSignal {
    /// Subtracts another MetaSignal from this one, clamping the result to a minimum of 0.0.
    fn sub_assign(&mut self, other: Self) {
        self.value = f64::max(self.value - other.value, 0.0);
    }
}

impl SubAssign<MetaSignal> for f64 {
    /// Subtracts a MetaSignal from this f64, clamping the result between 0.0 and 1.0.
    fn sub_assign(&mut self, other: MetaSignal) {
        let value = *self - other.value;
        *self = value.clamp(0.0, 1.0);
    }
}

impl SubAssign<f64> for MetaSignal {
    /// Subtracts a f64 from this MetaSignal, clamping the result between 0.0 and 1.0.
    fn sub_assign(&mut self, other: f64) {
        let value = self.value - other;
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Mul for MetaSignal {
    type Output = MetaSignal;
    /// Multiplies two meta_signals.
    fn mul(self, rhs: MetaSignal) -> Self::Output {
        MetaSignal {
            value: self.value * rhs.value,
        }
    }
}

impl Mul<MetaSignal> for f64 {
    type Output = MetaSignal;

    /// Multiplies a f64 by a MetaSignal, clamping the result between 0.0 and 1.0.
    fn mul(self, rhs: MetaSignal) -> Self::Output {
        MetaSignal::new(self * rhs.value)
    }
}

impl Mul<f64> for MetaSignal {
    type Output = MetaSignal;

    /// Multiplies a MetaSignal by a f64, clamping the result between 0.0 and 1.0.
    fn mul(self, rhs: f64) -> Self::Output {
        MetaSignal::new(self.value * rhs)
    }
}

impl MulAssign for MetaSignal {
    /// Multiplies this MetaSignal by another. Bounds checking is not necessary
    /// since both values are guaranteed to be between 0.0 and 1.0.
    fn mul_assign(&mut self, rhs: MetaSignal) {
        self.value *= rhs.value;
    }
}

impl MulAssign<MetaSignal> for f64 {
    /// Multiplies this f64 by a MetaSignal, clamping the result between 0.0 and 1.0.
    fn mul_assign(&mut self, rhs: MetaSignal) {
        let value = *self * rhs.value;
        *self = value.clamp(0.0, 1.0);
    }
}

impl MulAssign<f64> for MetaSignal {
    /// Multiplies this MetaSignal by a f64, clamping the result between 0.0 and 1.0.
    fn mul_assign(&mut self, rhs: f64) {
        let value = self.value * rhs;
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Div for MetaSignal {
    type Output = MetaSignal;

    /// Divides one MetaSignal by another, clamping the result to a maximum of 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div(self, rhs: MetaSignal) -> Self::Output {
        if rhs == 0.0 {
            return MetaSignal { value: 1.0 };
        }
        let value = self.value / *rhs;
        MetaSignal {
            value: f64::min(value, 1.0)
        }
    }
}

impl Div<MetaSignal> for f64 {
    type Output = MetaSignal;

    /// Divides a f64 by a MetaSignal, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div(self, rhs: MetaSignal) -> Self::Output {
        if rhs == 0.0 {
            return MetaSignal { value: 1.0 };
        }
        MetaSignal::new(self / *rhs)
    }
}

impl Div<f64> for MetaSignal {
    type Output = MetaSignal;

    /// Divides a MetaSignal by a f64, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return MetaSignal { value: 1.0 };
        }
        MetaSignal::new(self.value / rhs)
    }
}

impl DivAssign for MetaSignal {
    /// Divides this MetaSignal by another, clamping the result to a maximum of 1.0.
    fn div_assign(&mut self, rhs: MetaSignal) {
        if rhs == 0.0 {
            self.value = 1.0;
        } else {
            let value = self.value / *rhs;
            self.value = f64::min(value, 1.0);
        }
    }
}

impl DivAssign<MetaSignal> for f64 {
    /// Divides this f64 by a MetaSignal, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a value of 1.0.
    fn div_assign(&mut self, rhs: MetaSignal) {
        if rhs == 0.0 {
            *self = 1.0;
        } else {
            let value = *self / *rhs;
            *self = value.clamp(0.0, 1.0);
        }
    }
}

impl DivAssign<f64> for MetaSignal {
    /// Divides this MetaSignal by a f64, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            self.value = 1.0;
        } else {
            let value = self.value / rhs;
            self.value = value.clamp(0.0, 1.0);
        }
    }
}

impl PartialEq for MetaSignal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<f64> for MetaSignal {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl PartialEq<MetaSignal> for f64 {
    fn eq(&self, other: &MetaSignal) -> bool {
        *self == other.value
    }
}

impl Eq for MetaSignal {}

impl PartialOrd for MetaSignal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<f64> for MetaSignal {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<MetaSignal> for f64 {
    fn partial_cmp(&self, other: &MetaSignal) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl Ord for MetaSignal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl Display for MetaSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.>1}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::MetaSignal;
    #[test]
    fn test_meta_signal() {
        let mut a = MetaSignal::new(0.5);
        let b = MetaSignal::new(0.3);
        let c = MetaSignal::new(0.8);
        let d = MetaSignal::new(1.2);
        let e = MetaSignal::new(-0.5);

        assert_eq!(*a, 0.5);
        assert_eq!(*b, 0.3);
        assert_eq!(*c, 0.8);
        assert_eq!(*d, 1.0);
        assert_eq!(*e, 0.0);

        assert_eq!(a + b, f64::clamp(*a + *b, 0.0, 1.0));
        assert_eq!(a + 0.6, f64::clamp(*a + 0.6, 0.0, 1.0));
        assert_eq!(0.6 + a, f64::clamp(0.6 + *a, 0.0, 1.0));
        assert_eq!(a - b, f64::clamp(*a - *b, 0.0, 1.0));
        assert_eq!(a - 0.6, f64::clamp(*a - 0.6, 0.0, 1.0));
        assert_eq!(0.4 - a, f64::clamp(0.4 - *a, 0.0, 1.0));
        assert_eq!(a * b, f64::clamp(*a * *b, 0.0, 1.0));
        assert_eq!(a * 10.0, f64::clamp(*a * 10.0, 0.0, 1.0));
        assert_eq!(10.0 * a, f64::clamp(10.0 * *a, 0.0, 1.0));
        assert_eq!(a / b, f64::clamp(*a / *b, 0.0, 1.0));
        assert_eq!(a / 10.0, f64::clamp(*a / 10.0, 0.0, 1.0));
        assert_eq!(10.0 / a, f64::clamp(10.0 / *a, 0.0, 1.0));
        assert_eq!(a / 0.0, 1.0);
        a += b;
        assert_eq!(*a, f64::clamp(0.5 + 0.3, 0.0, 1.0));
        a -= b;
        assert_eq!(*a, f64::clamp(0.8 - 0.3, 0.0, 1.0));
        a -= 0.6;
        assert_eq!(*a, f64::clamp(0.5 - 0.6, 0.0, 1.0));
        a -= 0.6;
        assert_eq!(*a, 0.0);
        a += 0.5;
        assert_eq!(*a, 0.5);
        a *= b;
        assert_eq!(*a, 0.5 * 0.3);
        a *= 10.0;
        assert_eq!(*a, 1.0);
        a *= 0.1;
        assert_eq!(*a, 0.1);
        a /= b;
        assert_eq!(*a, f64::clamp(0.1 / 0.3, 0.0, 1.0));
        a /= 0.0;
        assert_eq!(*a, 1.0);
        a /= 10.0;
        assert_eq!(*a, f64::clamp(1.0 / 10.0, 0.0, 1.0));
        a /= 0.0;
        assert_eq!(*a, 1.0);
        assert!(a > b);
        assert!(b < a);
        assert!(a >= b);
        assert!(b <= a);
        assert_ne!(a, b);
        assert_eq!(a, a);
        assert!(a >= 0.5);
        assert!(a <= 1.0);
        assert!(a > 0.4);
        assert!(a < 1.1);
        assert_ne!(a, 0.4);
        assert_eq!(a, 1.0);
        assert!(0.4 < a);
        assert!(1.1 > a);
        assert!(0.5 <= a);
        assert!(1.0 >= a);
        assert_ne!(0.4, a);
        assert_eq!(1.0,a);
    }
}
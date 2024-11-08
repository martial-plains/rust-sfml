use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};
use sfml_sys::{
    sfTime, sfTime_Zero, sfTime_asMicroseconds, sfTime_asMilliseconds, sfTime_asSeconds,
};

/// Represents a time value.
///
/// The `Time` struct encapsulates a time value that can be expressed in
/// seconds, milliseconds, or microseconds. This flexible design allows
/// the API to handle time values with any resolution, while letting users
/// choose the unit that best suits their needs.
///
/// `Time` values support the usual mathematical operations: adding, subtracting,
/// multiplying, dividing, and comparing. Additionally, since `Time` represents
/// a duration, it can be negative.
///
/// # Example
///
/// ```rust
/// use rust_sfml::system::time::Time;
///
/// // Create a time value representing 0.1 seconds
/// let t1 = Time::seconds(0.1);
/// let milli = t1.as_milliseconds(); // 100 milliseconds
///
/// // Create a time value representing 30 milliseconds
/// let t2 = Time::milliseconds(30);
/// let micro = t2.as_microseconds(); // 30000 microseconds
///
/// // Create a time value representing -800000 microseconds
/// let t3 = Time::microseconds(-800000);
/// let sec = t3.as_seconds(); // -0.8 seconds
/// ```
///
/// ```rust
/// use rust_sfml::system::time::Time;
///
/// // A common use case: updating position based on elapsed time
/// fn update(position: &mut f32, speed: f32, elapsed: Time) {
///     *position += speed * elapsed.as_seconds();
/// }
///
/// // Call update with a 100ms time delta
/// let mut position = 0.0;
/// let speed = 10.0;
/// update(&mut position, speed, Time::milliseconds(100));
/// ```
///
/// # See also
/// - [`sf::Clock`](crate::system::Clock)
#[derive(Debug, Clone, Copy, AsRef, AsMut, Deref, DerefMut)]
pub struct Time {
    pub __inner: sfTime,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            __inner: unsafe { sfTime_Zero },
        }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.as_microseconds() == other.as_microseconds()
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_microseconds().partial_cmp(&other.as_microseconds())
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::microseconds(-rhs.as_microseconds())
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        let lhs = *self;
        *self = lhs - rhs
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::microseconds(self.as_microseconds() + rhs.as_microseconds())
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        let lhs = *self;
        *self = lhs + rhs
    }
}

impl Mul<f32> for Time {
    type Output = Time;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::seconds(self.as_seconds() * rhs)
    }
}

impl Mul<i64> for Time {
    type Output = Time;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::microseconds(self.as_microseconds() * rhs)
    }
}

impl Mul<Time> for i64 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Time {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl MulAssign<i64> for Time {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs
    }
}

impl Div<f32> for Time {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::seconds(self.as_seconds() / rhs)
    }
}

impl Div<i64> for Time {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self::microseconds(self.as_microseconds() / rhs)
    }
}

impl DivAssign<f32> for Time {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl DivAssign<i64> for Time {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl Rem for Time {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::microseconds(self.as_microseconds() % rhs.as_microseconds())
    }
}

impl RemAssign for Time {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl Time {
    /// Creates a new `Time` instance from a number of microseconds.
    ///
    /// # Parameters
    ///
    /// - `microseconds`: The number of microseconds representing the time.
    ///
    /// # Returns
    ///
    /// Returns a `Time` instance representing the specified time.
    pub fn new(microseconds: i64) -> Self {
        Self {
            __inner: sfTime { microseconds },
        }
    }

    /// Returns the time value as a number of seconds.
    ///
    /// # Returns
    ///
    /// Returns the time value as a floating-point number representing seconds.
    ///
    /// # See also
    /// - [`as_milliseconds`]
    /// - [`as_milliseconds`]
    pub fn as_seconds(&self) -> f32 {
        unsafe { sfTime_asSeconds(self.__inner) }
    }

    /// Returns the time value as a number of milliseconds.
    ///
    /// # Returns
    ///
    /// Returns the time value as an integer representing milliseconds.
    ///
    /// # See also
    /// - [`as_seconds`]
    /// - [`as_microseconds`]
    pub fn as_milliseconds(&self) -> i32 {
        unsafe { sfTime_asMilliseconds(self.__inner) }
    }

    /// Returns the time value as a number of microseconds.
    ///
    /// # Returns
    ///
    /// Returns the time value as an integer representing microseconds.
    ///
    /// # See also
    /// - [`as_seconds`]
    /// - [`as_milliseconds`]
    pub fn as_microseconds(&self) -> i64 {
        unsafe { sfTime_asMicroseconds(self.__inner) }
    }

    /// Constructs a `Time` value from the given number of seconds.
    ///
    /// # Parameters
    ///
    /// - `amount`: The number of seconds to convert.
    ///
    /// # Returns
    ///
    /// Returns a `Time` instance representing the specified number of seconds.
    ///
    /// # See also
    /// - [`milliseconds`]
    /// - [`microseconds`]
    pub fn seconds(amount: f32) -> Self {
        Self::new((amount * 1000000.0) as i64)
    }

    /// Constructs a `Time` value from the given number of milliseconds.
    ///
    /// # Parameters
    ///
    /// - `amount`: The number of milliseconds to convert.
    ///
    /// # Returns
    ///
    /// Returns a `Time` instance representing the specified number of milliseconds.
    ///
    /// # See also
    /// - [`seconds`]
    /// - [`microseconds`]
    pub fn milliseconds(amount: i32) -> Self {
        Self::new(amount as i64 * 1000)
    }

    /// Constructs a `Time` value from the given number of microseconds.
    ///
    /// # Parameters
    ///
    /// - `amount`: The number of microseconds to convert.
    ///
    /// # Returns
    ///
    /// Returns a `Time` instance representing the specified number of microseconds.
    ///
    /// # See also
    /// - [`seconds`]
    /// - [`milliseconds`]
    pub fn microseconds(amount: i64) -> Self {
        Self::new(amount)
    }
}

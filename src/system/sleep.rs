use sfml_sys::sfSleep;

use super::time::Time;

/// Makes the current thread sleep for a given duration.
///
/// `sleep` is the best way to block a program or one of its threads
/// for a specified duration without consuming any CPU resources.
///
/// # Parameters
///
/// - `duration`: The time to sleep, represented as a `Time` object. This value determines
///   how long the current thread will be paused.
///
/// # Example
///
/// ```rust
/// use rust_sfml::system::time::Time;
/// use rust_sfml::system::sleep;
///
/// // Sleep for 500 milliseconds
/// sleep(Time::milliseconds(500));
/// ```
/// # See also
/// - [`Time`](crate::system::Time) for more on time representations.
/// - [`Clock`](crate::system::Clock) for time measurement and intervals.
pub fn sleep(duration: Time) {
    unsafe { sfSleep(*duration) };
}

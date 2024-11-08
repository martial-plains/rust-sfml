use sfml_sys::{sfClock_create, sfClock_destroy, sfClock_getElapsedTime, sfClock_restart};

use super::time::Time;

/// A utility struct for measuring elapsed time.
///
/// The `Clock` struct allows you to measure the amount of time that has
/// passed since the clock was created or last restarted. It provides the
/// most precise time the underlying operating system can achieve, typically
/// in microseconds or nanoseconds, and ensures that the time is monotonic,
/// meaning the time value can never go backward even if the system clock changes.
///
/// # Example
///
/// ```rust
/// use rust_sfml::system::clock::Clock;
///
/// // Create a new clock
/// let mut clock = Clock::new();
///
/// // Get the elapsed time since the clock was created
/// let time1 = clock.elapsed_time();
/// println!("Elapsed time: {:?}", time1);
///
/// // Restart the clock and get the elapsed time again
/// let time2 = clock.restart();
/// println!("Elapsed time after restart: {:?}", time2);
/// ```
#[derive(Debug, Clone)]
pub struct Clock {
    __ptr: *mut sfml_sys::sfClock,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            __ptr: unsafe { sfClock_create() },
        }
    }
}

impl Drop for Clock {
    fn drop(&mut self) {
        unsafe { sfClock_destroy(self.__ptr) };
    }
}

impl Clock {
    /// Creates a new `Clock` instance.
    ///
    /// The clock starts automatically when constructed, and measures
    /// the time elapsed from the moment of creation.
    ///
    /// # Returns
    ///
    /// Returns a new `Clock` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the time elapsed since the last call to `restart` or the
    /// creation of the clock.
    ///
    /// # Returns
    ///
    /// Returns a `Time` object representing the elapsed time.
    pub fn elapsed_time(&self) -> Time {
        unsafe {
            Time {
                __inner: sfClock_getElapsedTime(self.__ptr),
            }
        }
    }

    /// Restarts the clock and returns the time elapsed since the last restart.
    ///
    /// This function resets the clock and returns the amount of time
    /// that has passed since the clock was last started or reset.
    ///
    /// # Returns
    ///
    /// Returns a `Time` value representing the time elapsed before the restart.
    pub fn restart(&mut self) -> Time {
        Time {
            __inner: unsafe { sfClock_restart(self.__ptr) },
        }
    }
}

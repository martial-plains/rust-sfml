//! A module providing synchronization primitives, specifically a `Mutex`
//! for mutual exclusion in a multithreaded environment. This module is
//! a Rust implementation inspired by SFML's `sf::Mutex` class and its
//! related helper `sf::Lock` class.
//!
//! ## Notes on Deadlock and Best Practices
//!
//! Be cautious with how you use `Mutex` and `Lock`. A common pitfall is
//! **deadlock**, where two or more threads are waiting on each other to
//! release a mutex, causing the program to get stuck. Avoid situations
//! where a thread locks multiple mutexes in a nested manner unless
//! absolutely necessary. Always try to lock mutexes in the same order
//! to minimize the risk of deadlock.
//!
//! In general, it's best practice to keep the scope of locked mutexes
//! as small as possible to reduce contention between threads and to
//! avoid performance bottlenecks.

use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};
use sfml_sys::{sfMutex, sfMutex_create, sfMutex_destroy, sfMutex_lock, sfMutex_unlock};

#[derive(Debug, Clone, Deref, DerefMut, AsRef, AsMut)]
pub struct Mutex {
    __ptr: *mut sfMutex, // Pointer to the internal mutex implementation
}

impl Default for Mutex {
    fn default() -> Self {
        Self {
            __ptr: unsafe { sfMutex_create() },
        }
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe { sfMutex_destroy(self.__ptr) };
    }
}

impl Mutex {
    // Creates a new Mutex
    pub fn new() -> Self {
        Self::default()
    }

    // Locks the mutex, blocking if the mutex is already locked
    pub fn lock(&self) {
        unsafe { sfMutex_lock(self.__ptr) };
    }

    // Unlocks the mutex
    pub fn unlock(&self) {
        unsafe { sfMutex_unlock(self.__ptr) };
    }
}

// RAII (Resource Acquisition Is Initialization) wrapper for Mutex to automatically unlock
// the mutex when it goes out of scope
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Lock<'a> {
    mutex: &'a Mutex, // Reference to the Mutex being locked
}

impl<'a> Lock<'a> {
    // Locks the mutex when the Lock object is created
    pub fn new(mutex: &'a Mutex) -> Self {
        mutex.lock();

        Self { mutex }
    }
}

impl Drop for Lock<'_> {
    // Automatically unlocks the mutex when the Lock object goes out of scope
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

//! This module provides the `InputStream` trait and implementations for different input stream types:
//! - `MemoryInputStream`: A stream backed by an in-memory buffer.
//! - `FileInputStream`: A stream that reads from a file on disk.
//!
//! The `InputStream` trait is abstract and serves as a common interface for reading data in various
//! formats. It allows custom streams (such as reading from a network, compressed file, etc.) to be
//! used in the same way as conventional file or memory streams, as long as they implement the necessary
//! methods for reading and seeking within the data.
//!
//! The `MemoryInputStream` and `FileInputStream` types are concrete implementations that provide
//! specific functionality for reading data from memory or disk, respectively.
//!
//! # Examples
//!
//! ## Using `MemoryInputStream`
//!
//! ```rust
//! use std::ptr;
//! use rust_sfml::system::{MemoryInputStream, InputStream};
//!
//! let data: Vec<u8> = vec![1, 2, 3, 4, 5];
//! let mut stream = MemoryInputStream::default();
//! stream.open(data.as_ptr() as *const _, data.len());
//!
//! let mut buffer = vec![0u8; 5];
//! stream.read(buffer.as_mut_ptr() as *mut _, 5);
//!
//! assert_eq!(buffer, vec![1, 2, 3, 4, 5]);
//! ```
//!
//! ## Using `FileInputStream`
//!
//! ```rust
//! use rust_sfml::system::{FileInputStream, InputStream};
//!
//! let mut stream = FileInputStream::default();
//! if stream.open("some_file.txt") {
//!     let mut buffer = vec![0u8; 1024];
//!     let bytes_read = stream.read(buffer.as_mut_ptr() as *mut _, 1024);
//!     println!("Read {} bytes from the file", bytes_read);
//! }
//! ```

use std::{
    ffi::c_char,
    fs::File,
    io::{Read, Seek, SeekFrom},
    os::raw::c_void,
    ptr, slice,
};

/// A trait that defines a common interface for reading data from various sources.
///
/// Concrete implementations of this trait must provide methods to read data, seek, tell the position,
/// and determine the size of the stream.
pub trait InputStream {
    /// Reads data from the stream.
    ///
    /// After reading, the stream's reading position must be advanced by the amount of bytes read.
    ///
    /// # Parameters
    /// - `data`: A mutable pointer to a buffer where data will be written.
    /// - `size`: The number of bytes to read.
    ///
    /// # Return
    /// The number of bytes actually read, or -1 on error.
    fn read(&mut self, data: *mut c_void, size: i64) -> i64;

    /// Changes the current reading position in the stream.
    ///
    /// # Parameters
    /// - `position`: The position to seek to, from the beginning of the stream.
    ///
    /// # Return
    /// The position actually sought to, or -1 on error.
    fn seek(&mut self, position: i64) -> i64;

    /// Gets the current reading position in the stream.
    ///
    /// # Return
    /// The current position, or -1 on error.
    fn tell(&mut self) -> i64;

    /// Returns the size of the stream.
    ///
    /// # Return
    /// The total number of bytes available in the stream, or -1 on error.
    fn size(&mut self) -> i64;
}

/// `MemoryInputStream` is a concrete implementation of the `InputStream` trait for reading data
/// from a memory chunk. It allows you to treat a block of memory as a stream and read data from it.
pub struct MemoryInputStream {
    data: *const c_char,
    size: i64,
    offset: i64,
}

impl Default for MemoryInputStream {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            size: 0,
            offset: 0,
        }
    }
}

impl MemoryInputStream {
    /// Opens the stream from a memory buffer.
    ///
    /// # Parameters
    /// - `data`: Pointer to the data in memory.
    /// - `size_in_bytes`: The size of the data in bytes.
    pub fn open(&mut self, data: *const c_void, size_in_bytes: usize) {
        self.data = data.cast::<c_char>();
        self.size = size_in_bytes as i64;
        self.offset = 0;
    }
}

impl InputStream for MemoryInputStream {
    fn read(&mut self, data: *mut c_void, size: i64) -> i64 {
        if self.data.is_null() {
            return -1;
        }

        let end_position = self.offset + size;
        let count = if end_position <= self.size {
            size
        } else {
            self.size - self.offset
        };

        if count > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.data.add(self.offset as usize),
                    data.cast::<c_char>(),
                    count as usize,
                );
            }

            self.offset += count;
        }

        count
    }

    fn seek(&mut self, position: i64) -> i64 {
        if self.data.is_null() {
            return -1;
        }

        self.offset = if position < self.size {
            position
        } else {
            self.size
        };

        self.offset
    }

    fn tell(&mut self) -> i64 {
        if self.data.is_null() {
            return 1;
        }

        self.offset
    }

    fn size(&mut self) -> i64 {
        if self.data.is_null() {
            return -1;
        }

        self.size
    }
}

/// `FileInputStream` is a concrete implementation of the `InputStream` trait for reading data from a file.
///
/// It wraps a standard file input stream and provides an interface for reading,
/// seeking, and querying the file's size.
#[derive(Debug, Default)]
pub struct FileInputStream {
    file: Option<File>,
}

impl Drop for FileInputStream {
    fn drop(&mut self) {
        if self.file.is_some() {
            self.file = None
        }
    }
}

impl FileInputStream {
    /// Opens the stream from a file path.
    ///
    /// # Parameters
    /// - `filename`: The name of the file to open.
    ///
    /// # Return
    /// `true` on success, `false` on error.
    pub fn open(&mut self, filename: &str) -> bool {
        if self.file.is_some() {
            self.file = None
        }

        self.file = File::open(filename).ok();

        self.file.is_some()
    }
}

impl InputStream for FileInputStream {
    fn read(&mut self, data: *mut c_void, size: i64) -> i64 {
        match &mut self.file {
            Some(f) => {
                let buffer = unsafe { slice::from_raw_parts_mut(data as *mut u8, size as usize) };
                let bytes_read = f.read(buffer).unwrap_or(0);
                bytes_read as i64
            }
            None => -1,
        }
    }

    fn seek(&mut self, position: i64) -> i64 {
        if let Some(ref mut file) = self.file {
            if file.seek(SeekFrom::Start(position as u64)).is_err() {
                return -1;
            }
            self.tell() // Return the current position after seeking
        } else {
            -1
        }
    }

    fn tell(&mut self) -> i64 {
        if let Some(ref mut file) = self.file {
            match file.stream_position() {
                Ok(pos) => pos as i64,
                Err(_) => -1,
            }
        } else {
            -1
        }
    }

    fn size(&mut self) -> i64 {
        let current_pos = self.tell();

        if let Some(file) = &mut self.file {
            if file.seek(SeekFrom::End(0)).is_err() {
                return -1;
            }
        }

        let size = self.tell();

        self.seek(current_pos);

        size
    }
}

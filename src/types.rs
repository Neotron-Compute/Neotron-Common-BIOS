//! # Types
//!
//! Contains types used in the Neotron API.
//!
//! Note that all types in this file *must* be `#[repr(C)]` and ABI stable.
//!
//! ## License
//!
//! > Copyright (C) The Neotron Developers, 2019-2022
//! >
//! > This program is free software: you can redistribute it and/or modify
//! > it under the terms of the GNU General Public License as published by
//! > the Free Software Foundation, either version 3 of the License, or
//! > at your option) any later version.
//! >
//! > This program is distributed in the hope that it will be useful,
//! > but WITHOUT ANY WARRANTY; without even the implied warranty of
//! > MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! > GNU General Public License for more details.
//! >
//! > You should have received a copy of the GNU General Public License
//! > along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ============================================================================
// Imports
// ============================================================================

// None

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

/// The type of the function which starts up the Operating System. The BIOS
/// finds and calls this function.
pub type OsStartFn = extern "C" fn(&crate::Api) -> !;

/// Any API function which can return an error, uses this error type.
#[derive(Debug)]
#[repr(C)]
pub enum Error {
	/// An invalid device number was given to the function.
	InvalidDevice,
	/// That function doesn't work at this time.
	Unimplemented,
	/// The underlying hardware reported some error. The numeric code is BIOS
	/// implementation specific but may give some clues.
	DeviceError(u16),
	/// The underlying hardware could not accept the given configuration. The
	/// numeric code is BIOS implementation specific but may give some clues.
	UnsupportedConfiguration(u16),
}

/// All API functions which can fail return this type. We don't use the
/// `Result` type from the standard library because that isn't FFI safe and
/// may change layout between compiler versions.
#[repr(C)]
pub enum Result<T> {
	/// The operation succeeded (the same as `core::result::Result::Ok`).
	Ok(T),
	/// The operation failed (the same as `core::result::Result::Err`).
	Err(Error),
}

/// All API functions which take/return optional values return this type. We
/// don't use the `Option` type from the standard library because that isn't
/// FFI safe and may change layout between compiler versions.
#[repr(C)]
pub enum Option<T> {
	/// There is some data (the same as `core::option::Option::Some`)
	Some(T),
	/// There is no data (the same as `core::option::Option::None`)
	None,
}

/// Describes a period of time, after which the BIOS should give up.
#[repr(C)]
pub struct Timeout(u32);

/// A Rust UTF-8 string, but compatible with FFI. Assume the lifetime is only
/// valid until the callee returns to the caller. Is not null-terminated.
#[repr(C)]
#[derive(Clone)]
pub struct ApiString<'a>(ApiByteSlice<'a>);

/// A Rust u8 slice, but compatible with FFI. Assume the lifetime is only valid
/// until the callee returns to the caller.
#[repr(C)]
#[derive(Clone)]
pub struct ApiByteSlice<'a> {
	/// A pointer to the data
	pub data: *const u8,
	/// The number of bytes we are pointing at
	pub data_len: usize,
	/// A phantom object to hold the lifetime
	_phantom: core::marker::PhantomData<&'a [u8]>,
}

/// A Rust u8 mutable slice, but compatible with FFI. Assume the lifetime is
/// only valid until the callee returns to the caller.
#[repr(C)]
#[derive(Clone)]
pub struct ApiBuffer<'a> {
	/// A pointer to where the data can be put
	pub data: *mut u8,
	/// The maximum number of bytes we can store in this buffer
	pub data_len: usize,
	/// A phantom object to hold the lifetime
	_phantom: core::marker::PhantomData<&'a [u8]>,
}

/// Represents an instant in time between 2000-01-01T00:00:00Z and
/// 2136-02-07T06:28:16Z.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Time {
	/// Seconds since the epoch
	pub secs: u32,
	/// Nanoseconds since the last second rolled over
	pub nsecs: u32
}

// ============================================================================
// Impls
// ============================================================================

// OsStartFn

// Error

impl<T> Result<T> {
	/// Obtain the inner value, or panic - just like `core::Result::unwrap`.
	pub fn unwrap(self) -> T {
		match self {
			crate::Result::Ok(val) => val,
			crate::Result::Err(e) => {
				panic!("Unwrap called, got err {:?}", e);
			}
		}
	}
}

impl<T> Option<T> {
	/// Obtain the inner value, or panic - just like `core::Option::unwrap`.
	pub fn unwrap(self) -> T {
		match self {
			crate::Option::Some(val) => val,
			crate::Option::None => {
				panic!("Unwrap called on empty option");
			}
		}
	}
}

// Timeout

// ApiString

impl<'a> ApiString<'a> {
	/// Create a new string slice we can send over the FFI.
	pub fn new(s: &'a str) -> ApiString<'a> {
		ApiString(ApiByteSlice::new(s.as_bytes()))
	}

	/// Turn this FFI string into a Rust string slice.
	pub fn as_str(&'a self) -> &'a str {
		unsafe { core::str::from_utf8_unchecked(self.0.as_slice()) }
	}
}

impl<'a> From<&'a str> for ApiString<'a> {
	/// Create a new FFI string from a string slice.
	fn from(input: &'a str) -> ApiString<'a> {
		ApiString::new(input)
	}
}

impl core::fmt::Debug for ApiString<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
		let s = unsafe { core::str::from_utf8_unchecked(buffer) };
		write!(f, "{:?}", s)
	}
}

impl core::fmt::Display for ApiString<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
		let s = unsafe { core::str::from_utf8_unchecked(buffer) };
		write!(f, "{}", s)
	}
}

// ApiByteSlice

impl<'a> ApiByteSlice<'a> {
	/// Create a new byte slice we can send over the FFI. NB: By doing this Rust
	/// can't track lifetimes any more.
	pub fn new(s: &'a [u8]) -> ApiByteSlice<'a> {
		ApiByteSlice {
			data: s.as_ptr(),
			data_len: s.len(),
			_phantom: core::marker::PhantomData,
		}
	}

	/// Turn this byte slice into a Rust byte slice.
	pub fn as_slice(&self) -> &[u8] {
		unsafe { core::slice::from_raw_parts(self.data, self.data_len) }
	}
}

impl core::fmt::Debug for ApiByteSlice<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let slice = self.as_slice();
		write!(f, "[ ")?;
		if let Some((last, rest)) = slice.split_last() {
			for i in rest.iter() {
				write!(f, "0x{:02x}, ", i)?;
			}
			write!(f, "0x{:02x} ", last)?;
		}
		write!(f, "]")
	}
}

impl<'a> From<&'a [u8]> for ApiByteSlice<'a> {
	/// Convert from a Rust byte slice into an FFI compatible byte slice
	fn from(input: &'a [u8]) -> ApiByteSlice<'a> {
		ApiByteSlice::new(input)
	}
}

// ApiBuffer

impl<'a> ApiBuffer<'a> {
	/// Create a new buffer we can send over the FFI.
	///
	/// This buffer is a mutable borrow of some storage space allocated
	/// elsewhere. If you are given this type in an API, assume it is only
	/// valid for as long as the function call you were given in it.
	pub fn new(s: &'a mut [u8]) -> ApiBuffer<'a> {
		ApiBuffer {
			data: s.as_mut_ptr(),
			data_len: s.len(),
			_phantom: core::marker::PhantomData,
		}
	}

	/// Turn this buffer into a Rust byte slice.
	pub fn as_slice(&self) -> &[u8] {
		unsafe { core::slice::from_raw_parts(self.data, self.data_len) }
	}

	/// Turn this buffer into a Rust mutable byte slice.
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		unsafe { core::slice::from_raw_parts_mut(self.data, self.data_len) }
	}
}

impl core::fmt::Debug for ApiBuffer<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let slice = self.as_slice();
		write!(f, "[ ")?;
		if let Some((last, rest)) = slice.split_last() {
			for i in rest.iter() {
				write!(f, "0x{:02x}, ", i)?;
			}
			write!(f, "0x{:02x} ", last)?;
		}
		write!(f, "]")
	}
}

impl<'a> From<&'a mut [u8]> for ApiBuffer<'a> {
	/// Convert from a Rust byte slice into an FFI compatible byte slice
	fn from(input: &'a mut [u8]) -> ApiBuffer<'a> {
		ApiBuffer::new(input)
	}
}

// Time

impl core::fmt::Display for Time {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		let timestamp: chrono::DateTime<chrono::Utc> = self.into();
		write!(f, "{}", timestamp)
	}
}

impl From<&Time> for chrono::DateTime<chrono::Utc> {
	fn from(time: &Time) -> Self {
		use chrono::prelude::*;
		let our_epoch = Utc.ymd(2001, 1, 1).and_hms(0, 0, 0).timestamp();
		chrono::Utc.timestamp(
			i64::from(time.secs) + our_epoch,
			time.nsecs
		)
	}
}

// ============================================================================
// End of File
// ============================================================================

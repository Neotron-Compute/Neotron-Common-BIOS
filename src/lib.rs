//! # Neotron Common BIOS
//!
//! Contains the common API for all Neotron BIOS implementations.
//!
//! ## License
//!
//!     Copyright (C) 2019 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
//!
//!     This program is free software: you can redistribute it and/or modify
//!     it under the terms of the GNU General Public License as published by
//!     the Free Software Foundation, either version 3 of the License, or
//!     (at your option) any later version.
//!
//!     This program is distributed in the hope that it will be useful,
//!     but WITHOUT ANY WARRANTY; without even the implied warranty of
//!     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!     GNU General Public License for more details.
//!
//!     You should have received a copy of the GNU General Public License
//!     along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![deny(missing_docs)]

/// BIOS API semantic version for the API defined in this crate, as <0x00>
/// <major> <minor> <patch>.
pub const API_VERSION: u32 = 0x0000_0100;

/// The type of the function which starts up the Operating System. The BIOS
/// finds and calls this function.
pub type OsStartFn = extern "C" fn(&Api) -> !;

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

/// Represents an instant in time between 2000-01-01T00:00:00Z and
/// 2136-02-07T06:28:16Z.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Time {
	/// Seconds since 2000-01-01T00:00:00Z
	pub seconds_since_epoch: u32,
	/// Number of 60 Hz frames that have elapsed since the current second
	/// began [0..59].
	pub frames_since_second: u8,
}

/// The BIOS API.
///
/// All Neotron BIOSes should provide this structure to the OS initialisation
/// function.
#[repr(C)]
pub struct Api {
	/// Gets the version number of the BIOS API. You need this value to
	/// determine which of the following API calls are valid in this
	/// particular version.
	pub api_version_get: extern "C" fn() -> u32,
	/// Returns a pointer to a static string slice. This string contains the
	/// version number and build string of the BIOS. For C compatibility this
	/// string is null-terminated and guaranteed to only contain ASCII
	/// characters (bytes with a value 127 or lower). We also pass the length
	/// (excluding the null) to make it easy to construct a Rust string. It is
	/// unspecified as to whether the string is located in Flash ROM or RAM
	/// (but it's likely to be Flash ROM).
	pub bios_version_get: extern "C" fn() -> ApiString<'static>,
	/// Get information about the Serial ports in the system. Serial ports are
	/// ordered octet-oriented pipes. You can push octets into them using a
	/// 'write' call, and pull bytes out of them using a 'read' call. They
	/// have options which allow them to be configured at different speeds, or
	/// with different transmission settings (parity bits, stop bits, etc) -
	/// you set these with a call to `SerialConfigure`. They may physically be
	/// a MIDI interface, an RS-232 port or a USB-Serial port. There is no
	/// sense of 'open' or 'close' - that is an Operating System level design
	/// feature. These APIs just reflect the raw hardware, in a similar manner
	/// to the registers exposed by a memory-mapped UART peripheral.
	pub serial_get_info: extern "C" fn(device: u8) -> crate::Option<serial::DeviceInfo>,
	/// Set the options for a given serial device. An error is returned if the
	/// options are invalid for that serial device.
	pub serial_configure: extern "C" fn(device: u8, config: serial::Config) -> crate::Result<()>,
	/// Write bytes to a serial port. There is no sense of 'opening' or
	/// 'closing' the device - serial devices are always open. If the return
	/// value is `Ok(n)`, the value `n` may be less than the size of the given
	/// buffer. If so, that means not all of the data could be transmitted -
	/// only the first `n` bytes were.
	pub serial_write: extern "C" fn(
		device: u8,
		data: ApiByteSlice,
		timeout: crate::Option<Timeout>,
	) -> crate::Result<usize>,
	/// Get the current wall time. The Neotron BIOS does not understand time
	/// zones, leap-seconds or the Gregorian calendar. It simply stores time
	/// as an incrementing number of seconds since some epoch, and the number
	/// of video frames (at 60 Hz) since that second began. A day is assumed
	/// to be exactly 86,400 seconds long. This is a lot like POSIX time,
	/// except we have a different epoch. The Neotron epoch is
	/// 2000-01-01T00:00:00Z. It is highly recommend that you store UTC in the
	/// BIOS and use the OS to handle time-zones.
	///
	/// If the BIOS does not have a battery-backed clock, or if that battery
	/// has failed to keep time, the system starts up assuming it is the
	/// epoch.
	pub time_get: extern "C" fn() -> Time,
	/// Set the current wall time. See `time_get` for a description of now the
	/// Neotron BIOS should handle time.
	///
	/// You only need to call this whenever you get a new sense of the current
	/// time (e.g. the user has updated the current time, or if you get a GPS
	/// fix). The BIOS should push the time out to the battery-backed Real
	/// Time Clock, if it has one.
	pub time_set: extern "C" fn(time: Time),
	/// Get the memory address for the video buffer.
	///
	/// Currently this only supports text mode. Each character on screen uses
	/// two consecutive bytes - one for the glyph and one for the attribute.
	pub video_memory_info_get:
		extern "C" fn(address: &mut *mut u8, width_cols: &mut u8, height_rows: &mut u8),
}

/// Serial Port / UART related types.
pub mod serial {
	/// Identities which sort of serial port each device represents.
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum DeviceType {
		/// A MIDI interface
		Midi,
		/// A Commodore Serial interface
		Cbm,
		/// An RS-485 bus
		Rs485,
		/// An RS-232 interface
		Rs232,
		/// An RS-232 interface, but at TTL voltages. Typically used with an
		/// FTDI FT232 cable.
		TtlUart,
		/// A USB Device implementing Communications Class Device (also known
		/// as a USB Serial port). The USB Device implementation may be
		/// on-chip, or off-chip.
		UsbCdc,
	}

	/// Whether each word contains a parity bit, and if so, how it is
	/// calculated.
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum Parity {
		/// An extra parity bit is added to each word. There will be an odd
		/// number of `1` bits in the new word (old word + parity bit). This
		/// parity bit can be used to detect a single bitflip in each word, but
		/// it cannot correct that bitflip.
		Odd,
		/// An extra parity bit is added to each word. There will be an even
		/// number of `1` bits in the new word (old word + parity bit). This
		/// parity bit can be used to detect a single bitflip in each word, but
		/// it cannot correct that bitflip.
		Even,
		/// No extra parity bit is added.
		None,
	}

	/// Whether to use hardware handshaking lines.
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum Handshaking {
		/// No hardware handshaking - bytes will be dropped if there is an
		/// overflow
		None,
		/// The Data Terminal Equipment (DTE) asserts Request-To-Send (RTS) when
		/// it is ready to receive data, and the Data Communications Equipment
		/// (DCE) asserts Clear-To-Send (CTS) when it is ready to receive data.
		RtsCts,
		/// Each device will send a Transmit-Off (XOFF) byte (0x13) when its
		/// receiving serial buffer is full, and a Transmit-On (XON) byte (0x11)
		/// when there is buffer space and the transmission can be resumed.
		///
		/// Note that the driver will not replace or delete any XON or XOFF
		/// bytes sent to the stream, so both sides must avoid sending them as
		/// part of the normal data flow.
		XonXoff,
	}

	/// The number of stop bits after each word.
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum StopBits {
		/// One stop bit is added to each word
		One,
		/// Two stop bits are added to each word
		Two,
	}

	/// The number of data bits in each word sent or received by the UART.
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum DataBits {
		/// Each word comprises 7 data bits (plus start bit, stop bits and any
		/// parity bits)
		Seven,
		/// Each word comprises 8 data bits (plus start bit, stop bits and any
		/// parity bits)
		Eight,
	}

	/// A particular configuration for a serial port.
	#[repr(C)]
	#[derive(Clone)]
	pub struct Config {
		/// The desired transmission speed, in bits per second (also known as
		/// the 'baud rate'). Some hardware implementations allow a free choice
		/// of data rate, while other limit you to a small subset (e.g. 9600,
		/// 57600 and 115200).
		pub data_rate_bps: u32,
		/// The desired number of data bits
		pub data_bits: DataBits,
		/// The desired number of stop bits
		pub stop_bits: StopBits,
		/// The desired parity configuration
		pub parity: Parity,
		/// The desired handshaking configuration
		pub handshaking: Handshaking,
	}

	/// Information about a particular serial device.
	#[repr(C)]
	#[derive(Clone)]
	pub struct DeviceInfo {
		/// Some human-readable name for this serial device (e.g. `RS232` or
		/// `USB0`)
		pub name: crate::ApiString<'static>,
		/// The type of this serial device
		pub device_type: DeviceType,
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

impl<'a> From<&'a [u8]> for ApiByteSlice<'a> {
	/// Convert from a Rust byte slice into an FFI compatible byte slice
	fn from(input: &'a [u8]) -> ApiByteSlice<'a> {
		ApiByteSlice::new(input)
	}
}

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
		let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
		write!(f, "{:?}", s)
	}
}

impl core::fmt::Display for ApiString<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
		let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
		write!(f, "{}", s)
	}
}

impl core::fmt::Display for Time {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		use chrono::prelude::*;
		let our_epoch = Utc.ymd(2001, 1, 1).and_hms(0, 0, 0).timestamp();
		let time = chrono::Utc.timestamp(
			i64::from(self.seconds_since_epoch) + our_epoch,
			((u32::from(self.frames_since_second) * 1_000_000) / 60) * 1_000,
		);
		write!(f, "{}", time)
	}
}

// End of file

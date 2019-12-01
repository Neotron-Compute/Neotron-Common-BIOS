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
	InvalidDevice,
}

/// All API functions which can fail return this type. We don't use the
/// `Result` type from the standard library because that isn't FFI safe and
/// may change layout between compiler versions.
#[repr(C)]
pub enum Result<T> {
	Ok(T),
	Err(Error),
}

/// All API functions which take/return optional values return this type. We
/// don't use the `Option` type from the standard library because that isn't
/// FFI safe and may change layout between compiler versions.
#[repr(C)]
pub enum Option<T> {
	Some(T),
	None,
}

/// Describes a period of time, after which the BIOS should give up.
#[repr(C)]
pub struct Timeout(u32);

/// A Rust UTF-8 string, but compatible with FFI. Assume the lifetime is only
/// valid until the callee returns to the caller. Is not null-terminated.
#[repr(C)]
#[derive(Clone)]
pub struct ApiString(ApiByteSlice);

/// A Rust u8 slice, but compatible with FFI. Assume the lifetime is only valid
/// until the callee returns to the caller.
#[repr(C)]
#[derive(Clone)]
pub struct ApiByteSlice {
	pub data: *const u8,
	pub data_len: usize,
}

/// Represents an instant in time between 2000-01-01T00:00:00Z and 2136-02-07T06:28:16Z.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Time {
	/// Seconds since 2000-01-01T00:00:00Z
	seconds_since_epoch: u32,
	/// Number of 60 Hz frames that have elapsed since the current second
	/// began [0..59].
	frames_since_second: u8,
}

/// The BIOS API.
///
/// All Neotron BIOSes should provide this structure to the OS initialisation function.
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
	pub bios_version_get: extern "C" fn() -> ApiString,
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
}

/// Serial Port / UART related types.
pub mod serial {
	/// Identities which sort of serial port each device represents.
	#[repr(C)]
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
	pub enum Parity {
		Odd,
		Even,
		None,
	}

	/// Whether to use hardware handshaking lines.
	#[repr(C)]
	pub enum Handshaking {
		None,
		RtsCts,
	}

	/// The number of stop bits after each word.
	#[repr(C)]
	pub enum StopBits {
		One,
		Two,
	}

	/// The number of data bits in each word sent or received by the UART.
	#[repr(C)]
	pub enum DataBits {
		Seven,
		Eight,
	}

	/// A particular configuration for a serial port.
	#[repr(C)]
	pub struct Config {
		pub data_rate_bps: u32,
		pub data_bits: DataBits,
		pub stop_bits: StopBits,
		pub parity: Parity,
		pub handshaking: Handshaking,
	}

	/// Information about a particular serial device.
	#[repr(C)]
	pub struct DeviceInfo {
		pub name: crate::ApiString,
		pub device_type: DeviceType,
	}
}

impl core::fmt::Debug for ApiByteSlice {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "[ ")?;
		for i in 0..self.data_len {
			write!(f, "0x{:02x}, ", unsafe { *self.data.add(i) })?;
		}
		write!(f, "]")
	}
}

impl core::fmt::Debug for ApiString {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
		let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
		write!(f, "{:?}", s)
	}
}

impl core::fmt::Display for ApiString {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
		let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
		write!(f, "{}", s)
	}
}

impl ApiByteSlice {
	pub const fn new(s: &[u8]) -> ApiByteSlice {
		ApiByteSlice {
			data: s.as_ptr(),
			data_len: s.len(),
		}
	}
}

impl ApiString {
	pub const fn new(s: &str) -> ApiString {
		ApiString(ApiByteSlice::new(s.as_bytes()))
	}
}

impl From<&str> for ApiString {
	fn from(input: &str) -> ApiString {
		ApiString::new(input)
	}
}

// End of file

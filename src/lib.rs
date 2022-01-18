//! # Neotron Common BIOS
//!
//! Contains the common API for all Neotron BIOS implementations.
//!
//! ## License
//!
//!     Copyright (C) The Neotron Developers, 2019-2022
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

pub mod serial;
pub mod types;
pub mod version;
pub mod video;

pub use types::*;
pub use version::Version;

/// BIOS API semantic version for the API defined in this crate.
pub const API_VERSION: Version = Version::new(0, 3, 0);

/// The BIOS API.
///
/// All Neotron BIOSes should provide this structure to the OS initialisation
/// function.
#[repr(C)]
pub struct Api {
	/// Gets the version number of the BIOS API.
	///
	/// You need this value to determine which of the following API calls are
	/// valid in this particular version.
	pub api_version_get: extern "C" fn() -> Version,
	/// Returns a pointer to a static string slice.
	///
	/// This string contains the version number and build string of the BIOS.
	/// For C compatibility this string is null-terminated and guaranteed to
	/// only contain ASCII characters (bytes with a value 127 or lower). We
	/// also pass the length (excluding the null) to make it easy to construct
	/// a Rust string. It is unspecified as to whether the string is located
	/// in Flash ROM or RAM (but it's likely to be Flash ROM).
	pub bios_version_get: extern "C" fn() -> ApiString<'static>,
	/// Get information about the Serial ports in the system.
	///
	/// Serial ports are ordered octet-oriented pipes. You can push octets
	/// into them using a 'write' call, and pull bytes out of them using a
	/// 'read' call. They have options which allow them to be configured at
	/// different speeds, or with different transmission settings (parity
	/// bits, stop bits, etc) - you set these with a call to
	/// `SerialConfigure`. They may physically be a MIDI interface, an RS-232
	/// port or a USB-Serial port. There is no sense of 'open' or 'close' -
	/// that is an Operating System level design feature. These APIs just
	/// reflect the raw hardware, in a similar manner to the registers exposed
	/// by a memory-mapped UART peripheral.
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
	/// Get the current wall time.
	///
	/// The Neotron BIOS does not understand time zones, leap-seconds or the
	/// Gregorian calendar. It simply stores time as an incrementing number of
	/// seconds since some epoch, and the number of video frames (at 60 Hz)
	/// since that second began. A day is assumed to be exactly 86,400 seconds
	/// long. This is a lot like POSIX time, except we have a different epoch
	/// - the Neotron epoch is 2000-01-01T00:00:00Z. It is highly recommend
	/// that you store UTC in the BIOS and use the OS to handle time-zones.
	///
	/// If the BIOS does not have a battery-backed clock, or if that battery
	/// has failed to keep time, the system starts up assuming it is the
	/// epoch.
	pub time_get: extern "C" fn() -> Time,
	/// Set the current wall time.
	///
	/// See `time_get` for a description of now the Neotron BIOS should handle
	/// time.
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
	/// Get the configuration data block.
	///
	/// Configuration data is, to the BIOS, just a block of bytes of a given
	/// length. How it stores them is up to the BIOS - it could be EEPROM, or
	/// battery-backed SRAM.
	pub configuration_get: extern "C" fn(buffer: ApiBuffer) -> crate::Result<usize>,
	/// Set the configuration data block.
	///
	/// See `configuration_get`.
	pub configuration_set: extern "C" fn(buffer: ApiByteSlice) -> crate::Result<()>,
}

// End of file

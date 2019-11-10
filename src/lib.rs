//! # Neotron Common BIOS
//!
//! Contains the common API for all Neotron BIOS implementations.

#![no_std]

/// BIOS API semantic version for the API defined in this crate, as <0x00>
/// <major> <minor> <patch>.
pub const API_VERSION: u32 = 0x0000_0100;

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

/// Describes a period of time, after which the BIOS should give up.
#[repr(C)]
pub struct Timeout(u32);

/// The BIOS API.
///
/// All Neotron BIOSes should provide this structure to the OS initialisation function.
#[repr(C)]
pub struct Api {
	/// Gets the version number of the BIOS API. You need this value to
	/// determine which of the following API calls are valid in this
	/// particular version.
	api_version_get: extern "C" fn() -> u32,
	/// Returns a pointer to a static string slice. This string contains the
	/// version number and build string of the BIOS. For C compatibility this
	/// string is null-terminated and guaranteed to only contain ASCII
	/// characters (bytes with a value 127 or lower). We also pass the length
	/// (excluding the null) to make it easy to construct a Rust string. It is
	/// unspecified as to whether the string is located in Flash ROM or RAM
	/// (but it's likely to be Flash ROM).
	bios_version_get: extern "C" fn() -> (*const u8, usize),
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
	serial_get_info: extern "C" fn(device: u8) -> Option<serial::DeviceInfo>,
	/// Set the options for a given serial device. An error is returned if the
	/// options are invalid for that serial device.
	serial_configure: extern "C" fn(device: u8, config: serial::Config) -> crate::Result<()>,
	/// Write bytes to a serial port. There is no sense of 'opening' or
	/// 'closing' the device - serial devices are always open. If the return
	/// value is `Ok(n)`, the value `n` may be less than the size of the given
	/// buffer. If so, that means not all of the data could be transmitted -
	/// only the first `n` bytes were.
	serial_write:
		extern "C" fn(device: u8, data: &[u8], timeout: Option<Timeout>) -> crate::Result<usize>,
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
		/// A USB Device implementing Communications Class Device (also known as
		/// a USB Serial port). The USB Device implementation may be on-chip, or off-chip.
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
		pub name: &'static str,
		pub device_type: DeviceType,
	}
}

// End of file

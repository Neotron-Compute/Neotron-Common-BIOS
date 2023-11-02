//! # Serial
//!
//! Serial Port / UART related types.
//!
//! Note that all types in this file that are exported in the `Api` structure
//! *must* be `#[repr(C)]` and ABI stable.

// Copyright (C) The Neotron Developers, 2019-2022
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ============================================================================
// Imports
// ============================================================================

use crate::make_ffi_enum;

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

make_ffi_enum!("Identifies which sort of serial port each device represents.",
DeviceType, FfiDeviceType, {
	#[doc = "An RS-232 interface"]
	Rs232,
	#[doc = "An RS-232 interface, but at TTL voltages. Typically used with an "]
	#[doc = "FTDI FT232 cable."]
	TtlUart,
	#[doc = "A USB Device implementing Communications Class Device (also known"]
	#[doc = "as a USB Serial port). The USB Device implementation may be"]
	#[doc = "on-chip, or off-chip."]
	UsbCdc,
	#[doc = "A MIDI interface"]
	Midi
});

make_ffi_enum!("Whether each word contains a parity bit, and if so, how it is calculated",
	Parity, FfiParity, {
	#[doc = "An extra parity bit is added to each word. There will be an odd"]
	#[doc = "number of `1` bits in the new word (old word + parity bit). This"]
	#[doc = "parity bit can be used to detect a single bitflip in each word, but"]
	#[doc = "it cannot correct that bitflip."]
	Odd,
	#[doc = "An extra parity bit is added to each word. There will be an even"]
	#[doc = "number of `1` bits in the new word (old word + parity bit). This"]
	#[doc = "parity bit can be used to detect a single bitflip in each word, but"]
	#[doc = "it cannot correct that bitflip."]
	Even,
	#[doc = "No extra parity bit is added."]
	None
});

make_ffi_enum!("Whether to use hardware handshaking lines.",
Handshaking, FfiHandshaking, {
	#[doc = "No hardware handshaking - bytes will be dropped if there is an overflow"]
	None,
	#[doc ="The Data Terminal Equipment (DTE) asserts Request-To-Send (RTS) when"]
	#[doc ="it is ready to receive data, and the Data Communications Equipment "]
	#[doc ="(DCE) asserts Clear-To-Send (CTS) when it is ready to receive data."]
	RtsCts,
	#[doc ="Each device will send a Transmit-Off (XOFF) byte (0x13) when its "]
	#[doc ="receiving serial buffer is full, and a Transmit-On (XON) byte (0x11) "]
	#[doc ="when there is buffer space and the transmission can be resumed. "]
	#[doc =""]
	#[doc ="Note that the driver will not replace or delete any XON or XOFF "]
	#[doc ="bytes sent to the stream, so both sides must avoid sending them as "]
	#[doc ="part of the normal data flow."]
	XonXoff
});

make_ffi_enum!("The number of stop bits after each word.",
	StopBits, FfiStopBits, {
	#[doc = "One stop bit is added to each word"]
	One,
	#[doc = "Two stop bits are added to each word"]
	Two
});

make_ffi_enum!("The number of data bits in each word sent or received by the UART.",
	DataBits, FfiDataBits, {
	#[doc = "Each word comprises 7 data bits (plus start bit, stop bits and any "]
	#[doc = "parity bits"]
	Seven,
	#[doc = "Each word comprises 8 data bits (plus start bit, stop bits and any "]
	#[doc = "parity bits"]
	Eight
});

/// A particular configuration for a serial port.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
	/// The desired transmission speed, in bits per second (also known as
	/// the 'baud rate'). Some hardware implementations allow a free choice
	/// of data rate, while other limit you to a small subset (e.g. 9600,
	/// 57600 and 115200).
	pub data_rate_bps: u32,
	/// The desired number of data bits
	pub data_bits: FfiDataBits,
	/// The desired number of stop bits
	pub stop_bits: FfiStopBits,
	/// The desired parity configuration
	pub parity: FfiParity,
	/// The desired handshaking configuration
	pub handshaking: FfiHandshaking,
}

/// Information about a particular serial device.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
	/// Some human-readable name for this serial device (e.g. `RS232` or
	/// `USB0`)
	pub name: crate::FfiString<'static>,
	/// The type of this serial device
	pub device_type: FfiDeviceType,
}

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

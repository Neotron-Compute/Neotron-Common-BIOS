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

// None

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

/// Identifies which sort of serial port each device represents.
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceType {
	/// An RS-232 interface
	Rs232,
	/// An RS-232 interface, but at TTL voltages. Typically used with an
	/// FTDI FT232 cable.
	TtlUart,
	/// A USB Device implementing Communications Class Device (also known
	/// as a USB Serial port). The USB Device implementation may be
	/// on-chip, or off-chip.
	UsbCdc,
	/// A MIDI interface
	Midi,
}

/// Whether each word contains a parity bit, and if so, how it is
/// calculated.
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StopBits {
	/// One stop bit is added to each word
	One,
	/// Two stop bits are added to each word
	Two,
}

/// The number of data bits in each word sent or received by the UART.
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
	/// Some human-readable name for this serial device (e.g. `RS232` or
	/// `USB0`)
	pub name: crate::FfiString<'static>,
	/// The type of this serial device
	pub device_type: DeviceType,
}

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

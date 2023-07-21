//! # Types
//!
//! Contains types used in the Neotron API.
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

/// The type of the function which starts up the Operating System. The BIOS
/// finds and calls this function.
pub type OsStartFn = extern "C" fn(&crate::Api) -> !;

/// Any API function which can return an error, uses this error type.
#[derive(Clone, Debug, PartialEq, Eq)]
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
	/// You used a Block Device API but there was no media in the drive
	NoMediaFound,
	/// You used a Block Device API asked for a block the device doesn't have
	BlockOutOfBounds,
}

/// Describes a period of time, after which the BIOS should give up.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timeout(u32);

/// Represents an instant in time between 2000-01-01T00:00:00Z and
/// 2136-02-07T06:28:16Z.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Time {
	/// Seconds since the epoch
	pub secs: u32,
	/// Nanoseconds since the last second rolled over
	pub nsecs: u32,
}

/// Represents a tick of some internal monotonic clock.
///
/// Usually runs at 1 kHz.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ticks(pub u64);

/// The kinds of memory we know about
#[repr(C)]
#[derive(Debug, Clone)]
pub enum MemoryKind {
	/// Read-write memory
	Ram,
	/// Read-only memory
	Rom,
}

/// Represents a region in memory.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MemoryRegion {
	/// The address the region starts at
	pub start: *mut u8,
	/// The length of the region
	pub length: usize,
	/// The kind of memory found at this region
	pub kind: MemoryKind,
}

/// The kinds of power control we can do.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PowerMode {
	/// Turn the system power off
	Off,
	/// Reboot the main processor
	Reset,
	/// Reboot the main processor, but tell it to enter a bootloader mode for
	/// programming. Precisely what this will do will depend upon the BIOS. Some
	/// BIOSes will not have a bootloader mode and this will do a regular
	/// reboot.
	Bootloader,
}

// ============================================================================
// Impls
// ============================================================================

// OsStartFn

// Timeout

impl Timeout {
	/// Create a new timeout, in milliseconds.
	pub fn new_ms(milliseconds: u32) -> Timeout {
		Timeout(milliseconds)
	}

	/// Create a new timeout, in seconds.
	pub fn new_secs(seconds: u16) -> Timeout {
		let milliseconds = u32::from(seconds) * 1000;
		Self::new_ms(milliseconds)
	}

	/// Get the timeout, in milliseconds
	pub fn get_ms(self) -> u32 {
		self.0
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
		let our_epoch = Utc
			.with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
			.unwrap()
			.timestamp();
		chrono::Utc
			.timestamp_opt(i64::from(time.secs) + our_epoch, time.nsecs)
			.unwrap()
	}
}

// MemoryKind

impl core::fmt::Display for MemoryKind {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				MemoryKind::Rom => "ROM",
				MemoryKind::Ram => "RAM",
			}
		)
	}
}

// MemoryRegion

impl core::fmt::Display for MemoryRegion {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(
			f,
			"{} KiB {} @ {:p}",
			self.length / 1024,
			self.kind,
			self.start
		)
	}
}

// ============================================================================
// End of File
// ============================================================================

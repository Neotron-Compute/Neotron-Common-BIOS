//! # Version
//!
//! Contains the version API.
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

/// Describes a semantic version.
///
/// The version is internally stored as a 32-bit value, but comprises an 8-bit
/// major version, and 8-bit minor version and an 8-bit patch version.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u32);

// ============================================================================
// Impls
// ============================================================================

impl Version {
	/// Create a new Version.
	pub const fn new(major: u8, minor: u8, patch: u8) -> Version {
		Version(u32::from_be_bytes([0x00, major, minor, patch]))
	}

	/// Get the major version portion.
	pub const fn major(&self) -> u8 {
		(self.0 >> 16) as u8
	}

	/// Get the minor version portion.
	pub const fn minor(&self) -> u8 {
		(self.0 >> 8) as u8
	}

	/// Get the patch version portion.
	pub const fn patch(&self) -> u8 {
		self.0 as u8
	}
}

// ============================================================================
// End of File
// ============================================================================

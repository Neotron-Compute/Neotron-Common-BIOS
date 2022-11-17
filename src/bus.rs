//! # Neotron Bus
//!
//! Neotron Bus related types.
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

/// The kinds of Peripheral you can put on a Neotron Bus
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PeripheralKind {
	/// A Neotron Bus Slot. The OS will need to read the EEPROM at address
	/// `0x50 + slot_id` to find out what is fitted (if anything).
	Slot,
	/// A hard-wired SD/MMC Card slot wired for SPI Mode. The interrupt pin is
	/// wired to "Card Detect" with a pull-up, so the line goes low when a
	/// card is inserted and goes high when the card is removed.
	SdCard,
	/// This Peripheral ID is reserved for the BIOS to use.
	Reserved,
}

/// Describes a Neotron Bus Peripheral
#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PeripheralInfo {
	/// A name, such as `slot0`
	pub name: crate::ApiString<'static>,
	/// The kind of peripheral
	pub kind: PeripheralKind,
}

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

//! # Block Devices
//!
//! Block Device related types.
//!
//! Note that all types in this file *must* be `#[repr(C)]` and ABI stable.

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

/// The types of block device we support.
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceType {
	/// An *SD* Card
	SecureDigitalCard,
	/// A Hard Drive
	HardDiskDrive,
	/// A floppy disk in a floppy disk drive
	FloppyDiskDrive,
	/// A compact flash card
	CompactFlashCard,
}

/// Information about a block device.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeviceInfo {
	/// Some human-readable name for this block device (e.g. `SdCard0` or
	/// `CF1`)
	pub name: crate::FfiString<'static>,
	/// The kind of block device this is.
	pub device_type: DeviceType,
	/// The size of an addressable block, in bytes.
	pub block_size: u32,
	/// The total number of addressable blocks.
	pub num_blocks: u64,
	/// Can this device be ejected?
	pub ejectable: bool,
	/// Can this device be removed?
	pub removable: bool,
	/// Does this have media in it right now?
	pub media_present: bool,
	/// Is this media read-only?
	pub read_only: bool,
}

/// Uniquely represents a block on a block device.
#[repr(C)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct BlockIdx(pub u64);

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

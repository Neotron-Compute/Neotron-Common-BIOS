//! # Audio
//!
//! Audio related types.
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

/// Defines the format of each sample (mono, stereo, 8-bit, 16-bit, etc).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SampleFormat {
	/// 8-bit, signed, mono samples.
	EightBitMono,
	/// 8-bit, signed, mono samples. Left, then Right.
	EightBitStereo,
	/// 16-bit, signed, mono samples. Little-endian.
	SixteenBitMono,
	/// 16-bit, signed, stereo samples. Little-endian. Left, then Right.
	SixteenBitStereo,
}

/// Configuration for an Audio Output or Input
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
	/// What format are the samples
	pub sample_format: SampleFormat,
	/// How many samples are there per second (e.g. 48,000)?
	///
	/// Supported values are likely to include some of the following:
	///
	/// * 8,000 Hz (Telephone/Voice)
	/// * 11,025 Hz (CD Audio / 4)
	/// * 16,000 Hz (DVD Audio / 3)
	/// * 22,050 Hz (CD Audio / 2)
	/// * 24,000 Hz (DVD Audio / 2)
	/// * 44,100 Hz (CD Audio)
	/// * 48,000 Hz (DVD Audio)
	pub sample_rate_hz: u32,
}

/// Describes the direction audio is flowing, for a given Audio Mixer Channel.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	/// Audio In, e.g. Line-In
	Input,
	/// Audio Out, e.g. Headphone Out
	Output,
	/// Internal audio loop-back from an Input to an Output, e.g. Side-tone
	Loopback,
}

/// Describes an Audio Mixer Channel.
///
/// For example "Line In", or "PCM Output"
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixerChannelInfo {
	/// The name of this Audio Mixer Channel (e.g. `Line In`)
	pub name: crate::ApiString<'static>,
	/// Is this an Input or an Output?
	pub direction: Direction,
	/// What value of `current_level` gives the loudest audio? All values
	/// equal to, or above, this value will be equally and maximally loud.
	pub max_level: u8,
	/// What is the current volume level for this Audio Mixer Channel, on a
	/// scale of `0` to `max_level`. A value of `0` mutes the channel.
	pub current_level: u8,
}

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

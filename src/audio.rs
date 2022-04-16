//! # Audio
//!
//! Audio related types.
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

/// Defines the format of each sample (mono, stereo, 8-bit, 16-bit, etc).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Config {
	/// What format are the samples
	pub sample_format: SampleFormat,
	/// How many samples per second (e.g. 48,000)?
	pub sample_rate_hz: u32,
}

/// Describes the direction audio is flowing, for a given Audio Mixer Channel.
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
pub struct MixerChannelInfo {
	/// The name of this Audio Mixer Channel
	pub name: crate::ApiString<'static>,
	/// Is this an Input or an Output?
	pub direction: Direction,
	/// What value of `current_level` gives the loudest audio? All values above this will be equally loud.
	pub max_level: u8,
	/// What is the current volume level for this Audio Mixer Channel, on a
	/// scale of `0` to `max_level`.
	pub current_level: u8,
}

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

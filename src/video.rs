//! # Video
//!
//! Video/Display related types.
//!
//! Note that all types in this file *must* be `#[repr(C)]` and ABI stable.
//!
//! ## License
//!
//! > Copyright (C) The Neotron Developers, 2019-2022
//! >
//! > This program is free software: you can redistribute it and/or modify
//! > it under the terms of the GNU General Public License as published by
//! > the Free Software Foundation, either version 3 of the License, or
//! > at your option) any later version.
//! >
//! > This program is distributed in the hope that it will be useful,
//! > but WITHOUT ANY WARRANTY; without even the implied warranty of
//! > MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! > GNU General Public License for more details.
//! >
//! > You should have received a copy of the GNU General Public License
//! > along with this program.  If not, see <https://www.gnu.org/licenses/>.

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

/// Describes a video mode.
///
/// A Neotron BIOS may support multiple video modes. Each is described using
/// an instance of this type.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mode(u8);

/// Describes the format of the video memory.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
	/// Text mode with an 8x16 font.
	///
	/// Memory is arranged into `(u8, u8)` units. The first `u8` is the
	/// character, the second `u8` unit is the foreground/background colour.
	///
	/// The font consists of 8px by 16px glyphs.
	Text8x16 = 0,
	/// Text mode with an 8x8 font.
	///
	/// Memory is arranged into `(u8, u8)` units. The first `u8` is the
	/// character, the second `u8` unit is the foreground/background colour.
	///
	/// The font consists of 8px by 8px glyphs.
	Text8x8 = 1,
	/// True-colour graphics mode, with 24-bit pixels in 32-bit units.
	///
	/// Memory is arranged into `u32` units. Each unit is of the format
	/// `0x00RRGGBB`.
	Chunky32 = 2,
	/// High-colour graphics mode, with 16-bit pixels.
	///
	/// Memory is arranged into `u16` units. Each unit is of the format
	/// `0bRRRRR_GGGGGG_BBBBB`.
	Chunky16 = 3,
	/// Colour graphics mode, with 8-bit indexed pixels.
	///
	/// Memory is arranged into `u8` units. Each unit is a lookup into the
	/// pallette.
	Chunky8 = 4,
	/// Colour graphics mode, with 4-bit indexed pixels.
	///
	/// Memory is arranged into `u8` units. Each unit is two 4-bit pixels,
	/// each a lookup into the pallette, or `0bAAAA_BBBB`.
	Chunky4 = 5,
	/// Colour graphics mode, with 2-bit indexed pixels.
	///
	/// Memory is arranged into `u8` units. Each unit is four 2-bit pixels,
	/// each a lookup into the pallette, or `0bAA_BB_CC_DD`
	Chunky2 = 6,
	/// Mono graphics mode, with 1-bit per pixel.
	///
	/// Memory is arranged into `u8` units. Each unit is eight 1-bit pixels,
	/// each a lookup into the pallette, or `0bA_B_C_D_E_F_G_H`
	Chunky1 = 7,
}

/// Describes the timing of the video signal.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Timing {
	/// VGA Standard 640x480 @ 60Hz.
	///
	/// Has a nominal 25.175 MHz pixel clock and a 31.5 kHz horizontal scan
	/// rate - but a specific implementation may differ.
	T640x480 = 0,
	/// VGA Standard 640x400 @ 70Hz.
	///
	/// Has a 25.175 MHz pixel clock and a 31.5 kHz horizontal scan rate - but
	/// a specific implementation may differ.
	T640x400 = 1,
	/// VESA Standard 800x600 @ 60Hz.
	///
	/// Has a 40.000 MHz pixel clock and a 37.9 kHz horizontal scan rate - but
	/// a specific implementation may differ.
	T800x600 = 2,
}

// ============================================================================
// Impls
// ============================================================================

impl Mode {
	const VERT_2X_SHIFT: usize = 7;
	const TIMING_SHIFT: usize = 4;
	const HORIZ_2X_SHIFT: usize = 3;
	const FORMAT_SHIFT: usize = 0;

	/// Create a new video mode
	pub const fn new(timing: Timing, format: Format) -> Mode {
		let t = timing as u8;
		let f = format as u8;
		let mode = (t << Self::TIMING_SHIFT) | (f << Self::FORMAT_SHIFT);
		Mode(mode)
	}

	/// Create a new double-height video mode.
	///
	/// This will set the 'Vert 2x' bit.
	pub const fn new_double_height(timing: Timing, format: Format) -> Mode {
		Mode(Self::new(timing, format).0 | 1 << Self::VERT_2X_SHIFT)
	}

	/// Create a new double-width video mode.
	///
	/// This will set the 'Horiz 2x' bit.
	pub const fn new_double_width(timing: Timing, format: Format) -> Mode {
		Mode(Self::new(timing, format).0 | 1 << Self::HORIZ_2X_SHIFT)
	}

	/// Create a new double-width, double-height video mode.
	///
	/// This will set the 'Horiz 2x' and the 'Vert 2x' bits.
	pub const fn new_double_height_width(timing: Timing, format: Format) -> Mode {
		Mode(Self::new(timing, format).0 | 1 << Self::VERT_2X_SHIFT | 1 << Self::HORIZ_2X_SHIFT)
	}

	/// If true, this mode is 2x taller than nominal.
	///
	/// e.g. a 640x480 mode is dropped to 640x240.
	pub const fn is_vert_2x(self) -> bool {
		(self.0 & (1 << Self::VERT_2X_SHIFT)) != 0
	}

	/// If true, this mode is 2x wider than nominal.
	///
	/// e.g. a 640x480 mode is dropped to 320x480.
	pub const fn is_horiz_2x(self) -> bool {
		(self.0 & (1 << Self::HORIZ_2X_SHIFT)) != 0
	}

	/// Gets how big a line is in bytes.
	///
	/// This could be a line of pixels or a line of characters, depending on
	/// the mode.
	pub const fn line_size_bytes(self) -> usize {
		let horizontal_pixels = self.horizontal_pixels() as usize;

		match self.format() {
			Format::Text8x8 | Format::Text8x16 => (horizontal_pixels / 8) * 2,
			Format::Chunky32 => horizontal_pixels * 4,
			Format::Chunky16 => horizontal_pixels * 2,
			Format::Chunky8 => horizontal_pixels,
			Format::Chunky4 => horizontal_pixels / 2,
			Format::Chunky2 => horizontal_pixels / 4,
			Format::Chunky1 => horizontal_pixels / 8,
		}
	}

	/// Gets how big a line is in glyph-attribute pairs.
	pub const fn text_width(self) -> Option<u16> {
		let horizontal_pixels = self.horizontal_pixels();

		match self.format() {
			Format::Text8x8 | Format::Text8x16 => Some(horizontal_pixels / 8),
			_ => None,
		}
	}

	/// Gets how many rows of text are on screen.
	pub const fn text_height(self) -> Option<u16> {
		match self.format() {
			Format::Text8x8 => Some(self.vertical_lines() / 8),
			Format::Text8x16 => Some(self.vertical_lines() / 16),
			_ => None,
		}
	}

	/// Gets how big the frame is, in bytes.
	pub const fn frame_size_bytes(self) -> usize {
		let line_size = self.line_size_bytes();
		let num_lines = self.vertical_lines() as usize
			/ match self.format() {
				Format::Text8x8 => 8,
				Format::Text8x16 => 16,
				_ => 1,
			};
		line_size * num_lines
	}

	/// Get the pixel format for this mode.
	pub const fn format(self) -> Format {
		match (self.0 >> Self::FORMAT_SHIFT) & 0b111 {
			0 => Format::Text8x16,
			1 => Format::Text8x8,
			2 => Format::Chunky32,
			3 => Format::Chunky16,
			4 => Format::Chunky8,
			5 => Format::Chunky4,
			6 => Format::Chunky2,
			7 => Format::Chunky1,
			_ => unreachable!(),
		}
	}

	/// Get the timing for this mode.
	pub const fn timing(self) -> Timing {
		match (self.0 >> Self::TIMING_SHIFT) & 0b111 {
			0 => Timing::T640x480,
			1 => Timing::T640x400,
			2 => Timing::T800x600,
			_ => unreachable!(),
		}
	}

	/// Get how many horizontal pixels are in the visible image.
	///
	/// The size of the sync pulse and the blanking period is for the BIOS to
	/// handle internally. The OS only cares about visible pixels.
	pub const fn horizontal_pixels(self) -> u16 {
		match (self.timing(), self.is_horiz_2x()) {
			(Timing::T640x480, false) => 640,
			(Timing::T640x400, false) => 640,
			(Timing::T800x600, false) => 800,
			(Timing::T640x480, true) => 320,
			(Timing::T640x400, true) => 320,
			(Timing::T800x600, true) => 400,
		}
	}

	/// Get how many vertical lines are in the visible image.
	///
	/// The size of the sync pulse and the blanking period is for the BIOS to
	/// handle internally. The OS only cares about visible lines.
	pub const fn vertical_lines(self) -> u16 {
		match (self.timing(), self.is_vert_2x()) {
			(Timing::T640x480, false) => 480,
			(Timing::T640x400, false) => 400,
			(Timing::T800x600, false) => 600,
			(Timing::T640x480, true) => 240,
			(Timing::T640x400, true) => 200,
			(Timing::T800x600, true) => 300,
		}
	}

	/// Get the nominal pixel clock.
	///
	/// Note this is only the nominal value. VESA allows +/- 0.5% tolerance.
	pub const fn pixel_clock_hz(self) -> u32 {
		match (self.0 >> Self::TIMING_SHIFT) & 0b111 {
			0 => 25175000,
			1 => 25175000,
			2 => 40000000,
			_ => 0,
		}
	}

	/// Get the nominal frame rate.
	///
	/// Note this is only the nominal value. VESA allows +/- 0.5% tolerance.
	pub const fn frame_rate_hz(self) -> u32 {
		match self.timing() {
			Timing::T640x480 => 60,
			Timing::T640x400 => 70,
			Timing::T800x600 => 60,
		}
	}

	/// Get the mode as an integer.
	pub const fn as_u8(self) -> u8 {
		self.0
	}
}

// ============================================================================
// Impls
// ============================================================================

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn mode_vga() {
		let mode = Mode::new(Timing::T640x480, Format::Text8x16);
		assert_eq!(0x00, mode.as_u8());
	}

	#[test]
	fn mode_sizes() {
		// These frame size numbers are taken from the Neotron Book.
		// https://neotron-compute.github.io/Neotron-Book/hardware_soc_video.html

		assert_eq!(
			Mode::new(Timing::T640x480, Format::Text8x16).frame_size_bytes(),
			4800
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Text8x8).frame_size_bytes(),
			9600
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Chunky32).frame_size_bytes(),
			1228800
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Chunky16).frame_size_bytes(),
			614400
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Chunky8).frame_size_bytes(),
			307200
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Chunky4).frame_size_bytes(),
			153600
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Chunky2).frame_size_bytes(),
			76800
		);
		assert_eq!(
			Mode::new(Timing::T640x480, Format::Chunky1).frame_size_bytes(),
			38400
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Text8x16).frame_size_bytes(),
			4000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Text8x8).frame_size_bytes(),
			8000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Chunky32).frame_size_bytes(),
			1024000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Chunky16).frame_size_bytes(),
			512000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Chunky8).frame_size_bytes(),
			256000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Chunky4).frame_size_bytes(),
			128000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Chunky2).frame_size_bytes(),
			64000
		);
		assert_eq!(
			Mode::new(Timing::T640x400, Format::Chunky1).frame_size_bytes(),
			32000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Text8x16).frame_size_bytes(),
			7400
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Text8x8).frame_size_bytes(),
			15000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Chunky32).frame_size_bytes(),
			1920000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Chunky16).frame_size_bytes(),
			960000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Chunky8).frame_size_bytes(),
			480000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Chunky4).frame_size_bytes(),
			240000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Chunky2).frame_size_bytes(),
			120000
		);
		assert_eq!(
			Mode::new(Timing::T800x600, Format::Chunky1).frame_size_bytes(),
			60000
		);

		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Text8x16).frame_size_bytes(),
			2400,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Text8x8).frame_size_bytes(),
			4800,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Chunky32).frame_size_bytes(),
			614400,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Chunky16).frame_size_bytes(),
			307200,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Chunky8).frame_size_bytes(),
			153600,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Chunky4).frame_size_bytes(),
			76800,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Chunky2).frame_size_bytes(),
			38400,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x480, Format::Chunky1).frame_size_bytes(),
			19200,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Text8x16).frame_size_bytes(),
			2000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Text8x8).frame_size_bytes(),
			4000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Chunky32).frame_size_bytes(),
			512000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Chunky16).frame_size_bytes(),
			256000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Chunky8).frame_size_bytes(),
			128000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Chunky4).frame_size_bytes(),
			64000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Chunky2).frame_size_bytes(),
			32000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T640x400, Format::Chunky1).frame_size_bytes(),
			16000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Text8x16).frame_size_bytes(),
			3700,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Text8x8).frame_size_bytes(),
			7500,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Chunky32).frame_size_bytes(),
			960000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Chunky16).frame_size_bytes(),
			480000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Chunky8).frame_size_bytes(),
			240000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Chunky4).frame_size_bytes(),
			120000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Chunky2).frame_size_bytes(),
			60000,
		);
		assert_eq!(
			Mode::new_double_width(Timing::T800x600, Format::Chunky1).frame_size_bytes(),
			30000,
		);

		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Text8x16).frame_size_bytes(),
			2400
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Text8x8).frame_size_bytes(),
			4800
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Chunky32).frame_size_bytes(),
			614400
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Chunky16).frame_size_bytes(),
			307200
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Chunky8).frame_size_bytes(),
			153600
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Chunky4).frame_size_bytes(),
			76800
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Chunky2).frame_size_bytes(),
			38400
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x480, Format::Chunky1).frame_size_bytes(),
			19200
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Text8x16).frame_size_bytes(),
			1920
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Text8x8).frame_size_bytes(),
			4000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Chunky32).frame_size_bytes(),
			512000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Chunky16).frame_size_bytes(),
			256000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Chunky8).frame_size_bytes(),
			128000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Chunky4).frame_size_bytes(),
			64000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Chunky2).frame_size_bytes(),
			32000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T640x400, Format::Chunky1).frame_size_bytes(),
			16000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Text8x16).frame_size_bytes(),
			3600
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Text8x8).frame_size_bytes(),
			7400
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Chunky32).frame_size_bytes(),
			960000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Chunky16).frame_size_bytes(),
			480000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Chunky8).frame_size_bytes(),
			240000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Chunky4).frame_size_bytes(),
			120000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Chunky2).frame_size_bytes(),
			60000
		);
		assert_eq!(
			Mode::new_double_height(Timing::T800x600, Format::Chunky1).frame_size_bytes(),
			30000
		);

		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Text8x16).frame_size_bytes(),
			1200
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Text8x8).frame_size_bytes(),
			2400
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Chunky32).frame_size_bytes(),
			307200
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Chunky16).frame_size_bytes(),
			153600
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Chunky8).frame_size_bytes(),
			76800
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Chunky4).frame_size_bytes(),
			38400
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Chunky2).frame_size_bytes(),
			19200
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x480, Format::Chunky1).frame_size_bytes(),
			9600
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Text8x16).frame_size_bytes(),
			960
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Text8x8).frame_size_bytes(),
			2000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Chunky32).frame_size_bytes(),
			256000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Chunky16).frame_size_bytes(),
			128000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Chunky8).frame_size_bytes(),
			64000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Chunky4).frame_size_bytes(),
			32000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Chunky2).frame_size_bytes(),
			16000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T640x400, Format::Chunky1).frame_size_bytes(),
			8000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Text8x16).frame_size_bytes(),
			1800
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Text8x8).frame_size_bytes(),
			3700
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Chunky32).frame_size_bytes(),
			480000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Chunky16).frame_size_bytes(),
			240000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Chunky8).frame_size_bytes(),
			120000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Chunky4).frame_size_bytes(),
			60000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Chunky2).frame_size_bytes(),
			30000
		);
		assert_eq!(
			Mode::new_double_height_width(Timing::T800x600, Format::Chunky1).frame_size_bytes(),
			15000
		);
	}
}

// ============================================================================
// End of File
// ============================================================================

//! # Video
//!
//! Video/Display related types.
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

/// The set of errors you can get from this module.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
	/// You supplied a parameter that was out of range, or otherwise unsupported.
	BadParam,
}

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

/// Describes an RGB colour-triple.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RGBColour(u32);

/// Represents a glyph in the current font.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Glyph(pub u8);

/// Represents a pallette index that we can use as a text foreground colour.
///
/// Only supports the range `0..=15`
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TextForegroundColour(u8);

/// Represents a pallette index that we can use as a text background colour.
///
/// Only supports the range `0..=7`
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TextBackgroundColour(u8);

/// Represents VGA format foreground/background attributes.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Attr(pub u8);

/// Represents a glyph/attribute pair.
/// 
/// This is what out text console is made out of. They work in exactly the same
/// way as IBM PC VGA.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct GlyphAttr(pub u16);

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

	/// Make a mode from an integer.
	///
	/// # Safety
	///
	/// The integer `mode_value` must represent a valid mode, as returned from
	/// `Mode::as_u8`. This function does not validate the given value.
	pub unsafe fn from_u8(mode_value: u8) -> Mode {
		Mode(mode_value)
	}
}

impl RGBColour {
	/// The colour Red
	pub const RED: RGBColour = RGBColour::from_rgb(0xFF, 0, 0);
	/// The colour Green
	pub const GREEN: RGBColour = RGBColour::from_rgb(0, 0xFF, 0);
	/// The colour Blue
	pub const BLUE: RGBColour = RGBColour::from_rgb(0, 0, 0xFF);
	/// The colour Yellow
	pub const YELLOW: RGBColour = RGBColour::from_rgb(0xFF, 0xFF, 0);
	/// The colour White
	pub const WHITE: RGBColour = RGBColour::from_rgb(0xFF, 0xFF, 0xFF);
	/// The colour Black
	pub const BLACK: RGBColour = RGBColour::from_rgb(0, 0, 0);
	/// The colour Cyan
	pub const CYAN: RGBColour = RGBColour::from_rgb(0, 0xFF, 0xFF);
	/// The colour Magenta
	pub const MAGENTA: RGBColour = RGBColour::from_rgb(0xFF, 0, 0xFF);

	/// Create a new RGB colour from a packed 32-bit value
	pub const fn from_packed(packed: u32) -> RGBColour {
		RGBColour(packed)
	}

	/// Get a packed 32-bit value from this RGB Colour
	pub const fn as_packed(self) -> u32 {
		self.0
	}

	/// Create a new RGB colour
	pub const fn from_rgb(red: u8, green: u8, blue: u8) -> RGBColour {
		let mut colour = (red as u32) << 16;
		colour |= (green as u32) << 8;
		colour |= blue as u32;
		RGBColour(colour)
	}

	/// Get the red-channel value
	pub const fn red(self) -> u8 {
		((self.0 >> 16) & 0xFF) as u8
	}

	/// Get the green-channel value
	pub const fn green(self) -> u8 {
		((self.0 >> 8) & 0xFF) as u8
	}

	/// Get the blue-channel value
	pub const fn blue(self) -> u8 {
		(self.0 & 0xFF) as u8
	}
}

impl TextForegroundColour {
	/// The highest value a VGA text-mode foreground colour can have.
	pub const MAX: u8 = 15;

	/// The colour *Black*, assuming the default palette is loaded.
	pub const BLACK: Self = Self(0);
	/// The colour *Dark Red*, assuming the default palette is loaded.
	pub const DARK_RED: Self = Self(1);
	/// The colour *Dark Green*, assuming the default palette is loaded.
	pub const DARK_GREEN: Self = Self(2);
	/// The colour *Orange*, assuming the default palette is loaded.
	pub const ORANGE: Self = Self(3);
	/// The colour *Blue*, assuming the default palette is loaded.
	pub const BLUE: Self = Self(4);
	/// The colour *Dark Magenta*, assuming the default palette is loaded.
	pub const DARK_MAGENTA: Self = Self(5);
	/// The colour *Dark Cyan*, assuming the default palette is loaded.
	pub const DARK_CYAN: Self = Self(6);
	/// The colour *Yellow*, assuming the default palette is loaded.
	pub const YELLOW: Self = Self(7);
	/// The colour *Grey*, assuming the default palette is loaded.
	pub const GREY: Self = Self(8);
	/// The colour *Bright Red*, assuming the default palette is loaded.
	pub const BRIGHT_RED: Self = Self(9);
	/// The colour *Bright Green*, assuming the default palette is loaded.
	pub const BRIGHT_GREEN: Self = Self(10);
	/// The colour *Bright Yellow*, assuming the default palette is loaded.
	pub const BRIGHT_YELLOW: Self = Self(11);
	/// The colour *Bright Blue*, assuming the default palette is loaded.
	pub const BRIGHT_BLUE: Self = Self(12);
	/// The colour *Bright Magenta*, assuming the default palette is loaded.
	pub const BRIGHT_MAGENTA: Self = Self(13);
	/// The colour *Bright Cyan*, assuming the default palette is loaded.
	pub const BRIGHT_CYAN: Self = Self(14);
	/// The colour *White*, assuming the default palette is loaded.
	pub const WHITE: Self = Self(15);

	/// Make a new `TextForegroundColour` from an integer.
	///
	/// The value must be `<= Self::MAX`, or you will get an error.
	pub const fn new(value: u8) -> Result<Self, Error> {
		if value <= Self::MAX {
			Ok(Self(value))
		} else {
			Err(Error::BadParam)
		}
	}

	/// Make a new `TextForegroundColour` from an integer without bounds checking.
	///
	/// # Safety
	///
	/// The value must be `<= Self::MAX`, or you will get undefined behaviour.
	pub const unsafe fn new_unchecked(value: u8) -> Self {
		Self(value)
	}

	/// Convert to a raw integer
	pub const fn as_u8(self) -> u8 {
		self.0
	}
}

impl TextBackgroundColour {
	/// The highest value a VGA text-mode background colour can have.
	pub const MAX: u8 = 7;

	/// The colour *Black*, assuming the default palette is loaded.
	pub const BLACK: TextBackgroundColour = TextBackgroundColour(0);
	/// The colour *Dark Red*, assuming the default palette is loaded.
	pub const DARK_RED: TextBackgroundColour = TextBackgroundColour(1);
	/// The colour *Dark Green*, assuming the default palette is loaded.
	pub const DARK_GREEN: TextBackgroundColour = TextBackgroundColour(2);
	/// The colour *Orange*, assuming the default palette is loaded.
	pub const ORANGE: TextBackgroundColour = TextBackgroundColour(3);
	/// The colour *Blue*, assuming the default palette is loaded.
	pub const BLUE: TextBackgroundColour = TextBackgroundColour(4);
	/// The colour *Dark Magenta*, assuming the default palette is loaded.
	pub const DARK_MAGENTA: TextBackgroundColour = TextBackgroundColour(5);
	/// The colour *Dark Cyan*, assuming the default palette is loaded.
	pub const DARK_CYAN: TextBackgroundColour = TextBackgroundColour(6);
	/// The colour *Yellow*, assuming the default palette is loaded.
	pub const YELLOW: TextBackgroundColour = TextBackgroundColour(7);

	/// Make a new TextForegroundColour from an integer.
	///
	/// The value must be `<= Self::MAX`, or you will get an error.
	pub const fn new(value: u8) -> Result<Self, Error> {
		if value <= Self::MAX {
			Ok(Self(value))
		} else {
			Err(Error::BadParam)
		}
	}

	/// Make a new TextForegroundColour from an integer without bounds checking.
	///
	/// # Safety
	///
	/// The value must be `<= Self::MAX`, or you will get undefined behaviour.
	pub const unsafe fn new_unchecked(value: u8) -> Self {
		Self(value)
	}

	/// Convert to a raw integer
	pub const fn as_u8(self) -> u8 {
		self.0
	}
}

impl Attr {
	/// Make a new Attribute Value.
	///
	/// This is packed according to the format for the IBM *Video Graphics Array* (VGA) standard,
	/// with a four-bit (`0..=15`) foreground colour, a three-bit (`0..=7`) background colour
	/// and a single bit for *blink* which makes the text blink on and off roughly once a second.
	///
	/// ```text
	/// +-------+-----+-----+-----+-----+-----+-----+-----+
	/// + BLINK | BG2 | BG1 | BG0 | FG3 | FG2 | FG1 | FG0 |
	/// +-------+-----+-----+-----+-----+-----+-----+-----+
	/// ```
	pub const fn new(fg: TextForegroundColour, bg: TextBackgroundColour, blink: bool) -> Attr {
		let fg = fg.0 & 0b1111;
		let bg = (bg.0 & 0b111) << 4;
		let blink = if blink { 1 << 7 } else { 0 };
		let value = blink | bg | fg;
		Attr(value)
	}

	/// Get the foreground colour
	pub const fn fg(&self) -> TextForegroundColour {
		TextForegroundColour(self.0 & 0x0F)
	}

	/// Get the background colour
	pub const fn bg(&self) -> TextBackgroundColour {
		TextBackgroundColour((self.0 >> 4) & 0x07)
	}

	/// Is the text blinking?
	pub const fn blink(&self) -> bool {
		(self.0 & 0x80) != 0
	}

	/// Make a new attribute with the new foreground colour
	pub fn set_fg(&mut self, fg: TextForegroundColour) {
		*self = Self::new(fg, self.bg(), self.blink());
	}

	/// Make a new Selfibute with the new background colour
	pub fn set_bg(&mut self, bg: TextBackgroundColour) {
		*self = Self::new(self.fg(), bg, self.blink());
	}

	/// Make a new attribute with the new blink state
	pub fn set_blink(&mut self, blink: bool) {
		*self = Self::new(self.fg(), self.bg(), blink);
	}

	/// Convert this attribute into a raw 8-bit value
	pub const fn as_u8(self) -> u8 {
		self.0
	}
}

impl GlyphAttr {
	/// Make a new glyph/attribute pair.
	pub const fn new(glyph: Glyph, attr: Attr) -> GlyphAttr {
		let value: u16 = (glyph.0 as u16) + ((attr.0 as u16) << 8);
		GlyphAttr(value)
	}

	/// Get the glyph component of this pair.
	pub const fn glyph(self) -> Glyph {
		Glyph(self.0 as u8)
	}

	/// Get the attribute component of this pair.
	pub const fn attr(self) -> Attr {
		Attr((self.0 >> 8) as u8)
	}
}

// ============================================================================
// Tests
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

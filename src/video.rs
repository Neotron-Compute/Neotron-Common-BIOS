//! # Video
//!
//! Video/Display related types.
//!
//! Note that all types in this file *must* be `#[repr(C)]` and ABI stable.
//!
//! ## License
//!
//!     Copyright (C) The Neotron Developers, 2019-2022
//!
//!     This program is free software: you can redistribute it and/or modify
//!     it under the terms of the GNU General Public License as published by
//!     the Free Software Foundation, either version 3 of the License, or
//!     (at your option) any later version.
//!
//!     This program is distributed in the hope that it will be useful,
//!     but WITHOUT ANY WARRANTY; without even the implied warranty of
//!     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!     GNU General Public License for more details.
//!
//!     You should have received a copy of the GNU General Public License
//!     along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ============================================================================
// Imports
// ============================================================================

// None

// ============================================================================
// Constants
// ============================================================================

/// VGA 80x25 text mode.
///
/// This is the classic IBM PC text mode.
pub const MODE_80X25_TEXT: Mode = Mode::new(Timing::T640x400, Format::Text8x16);

/// VGA 80x30 text mode.
///
/// This is the 640x480 graphical mode used for text.
pub const MODE_80X30_TEXT: Mode = Mode::new(Timing::T640x480, Format::Text8x16);

/// VGA 80x50 text mode.
pub const MODE_80X50_TEXT: Mode = Mode::new(Timing::T640x400, Format::Text8x8);

/// VGA 80x60 text mode.
pub const MODE_80X60_TEXT: Mode = Mode::new(Timing::T640x480, Format::Text8x8);

/// Classic QVGA 256 colour graphics
pub const MODE_320X240_GRAPHICS: Mode =
	Mode::new_double_height_width(Timing::T640x480, Format::Chunky8);

/// Classic CGA-resolution 256 colour graphics, as used by Doom
pub const MODE_320X200_GRAPHICS: Mode =
	Mode::new_double_height_width(Timing::T640x400, Format::Chunky8);

// ============================================================================
// Types
// ============================================================================

/// Describes a video mode.
///
/// A Neotron BIOS may support multiple video modes. Each is described using
/// and instance of this type.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Mode(u8);

/// Describes the format of the video memory.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Format {
	/// Text mode with an 8x8 font.
	///
	/// Memory is arranged into `(u8, u8)` units. The first `u8` is the
	/// character, the second `u8` unit is the foreground/background colour.
	///
	/// The font consists of 8px by 8px glyphs.
	Text8x8 = 0,
	/// Text mode with an 8x16 font.
	///
	/// Memory is arranged into `(u8, u8)` units. The first `u8` is the
	/// character, the second `u8` unit is the foreground/background colour.
	///
	/// The font consists of 8px by 16px glyphs.
	Text8x16 = 1,
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
#[derive(Debug, Copy, Clone)]
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
	pub fn is_vert_2x(self) -> bool {
		(self.0 & (1 << Self::VERT_2X_SHIFT)) != 0
	}

	/// If true, this mode is 2x wider than nominal.
	///
	/// e.g. a 640x480 mode is dropped to 320x480.
	pub fn is_horiz_2x(self) -> bool {
		(self.0 & (1 << Self::HORIZ_2X_SHIFT)) != 0
	}

	/// Gets how big a line is in bytes.
	///
	/// This could be a line of pixels or a line of characters, depending on
	/// the mode.
	pub fn line_size_bytes(self) -> usize {
		0
	}

	/// Gets how big the frame is, in bytes.
	pub fn frame_size_bytes(self) -> usize {
		0
	}

	/// Get the pixel format for this mode.
	pub fn format(self) -> Format {
		match (self.0 >> Self::FORMAT_SHIFT) & 0b111 {
			0 => Format::Text8x8,
			1 => Format::Text8x16,
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
	pub fn timing(self) -> Timing {
		match (self.0 >> Self::FORMAT_SHIFT) & 0b111 {
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
	pub fn horizontal_pixels(self) -> u16 {
		match (self.0 >> Self::TIMING_SHIFT) & 0b111 {
			0 => 640,
			1 => 640,
			2 => 800,
			_ => 0,
		}
	}

	/// Get how many vertical lines are in the visible image.
	///
	/// The size of the sync pulse and the blanking period is for the BIOS to
	/// handle internally. The OS only cares about visible lines.
	pub fn vertical_lines(self) -> u16 {
		match (self.0 >> Self::TIMING_SHIFT) & 0b111 {
			0 => 480,
			1 => 400,
			2 => 600,
			_ => 0,
		}
	}

	/// Get the nominal pixel clock.
	///
	/// Note this is only the nominal value. VESA allows +/- 0.5% tolerance.
	pub fn pixel_clock_hz(self) -> u32 {
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
	pub fn frame_rate_hz(self) -> u32 {
		match (self.0 >> Self::TIMING_SHIFT) & 0b111 {
			0 => 60,
			1 => 70,
			2 => 60,
			_ => 0,
		}
	}
}

/// Does this Neotron BIOS support this video mode?
pub fn is_valid_mode(_mode: Mode) -> bool {
	false
}

/// Switch to a new video mode.
///
/// The contents of the screen are undefined after a call to this function.
pub fn set_mode(_mode: Mode) -> crate::Result<()> {
	crate::Result::Err(crate::Error::Unimplemented)
}

/// Return a pointer to the framebuffer.
///
/// In text mode, you do:
///
/// ```rust,ignore
///
/// let fb = get_framebuffer();
/// let row = 2;
/// let col = 9;
/// let line_size =
/// fb.add((row * line_size + col) * 2).write(b'A');
/// fb.add(((row * line_size + col) * 2) + 1).write(0x0F);
pub fn get_framebuffer() -> *mut u8 {
	core::ptr::null_mut()
}

// ============================================================================
// Impls
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================

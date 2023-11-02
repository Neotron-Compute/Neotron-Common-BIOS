//! # HID
//!
//! Human Interface Device (keyboard/mouse) related types.
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

pub use pc_keyboard::KeyCode;

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

/// Represents a event from a Human Input Device (such as a mouse or keyboard).
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HidEvent {
	/// A key was pressed.
	KeyPress(KeyCode),
	/// A key was pressed.
	KeyRelease(KeyCode),
	/// A mouse was moved or mouse button was clicked.
	///
	/// Or these may be generated periodically even if there was no movement or clicking.
	MouseInput(MouseData),
}

/// Represents the movement of a mouse over the previous period of time, and
/// the current state of the mouse buttons.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MouseData {
	/// How far the mouse moved left(-ve)/right(+ve) since the last request
	pub x: i16,
	/// How far the mouse moved up(-ve)/down(+ve) since the last request
	pub y: i16,
	/// The current state of the mouse buttons.
	pub buttons: MouseButtons,
}

/// Represents the buttons on a mouse.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MouseButtons(u8);

/// Represents the LEDs on a keyboard.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct KeyboardLeds(u8);

// ============================================================================
// Impls
// ============================================================================

impl MouseButtons {
	const LEFT_BIT: u8 = 1 << 0;
	const MIDDLE_BIT: u8 = 1 << 1;
	const RIGHT_BIT: u8 = 1 << 2;

	/// Create a new `MouseButtons` value.
	///
	/// All buttons default to *not pressed*
	pub const fn new() -> Self {
		Self(0)
	}

	/// Note that the left mouse button is currently being pressed.
	pub const fn set_left_pressed(self) -> Self {
		let value = self.0 | Self::LEFT_BIT;
		Self(value)
	}

	/// Note that the middle mouse button is currently being pressed.
	pub const fn set_middle_pressed(self) -> Self {
		let value = self.0 | Self::MIDDLE_BIT;
		Self(value)
	}

	/// Note that the right mouse button is currently being pressed.
	pub const fn set_right_pressed(self) -> Self {
		let value = self.0 | Self::RIGHT_BIT;
		Self(value)
	}

	/// Returns `true` if the left mouse button is currently being pressed.
	pub const fn is_left_pressed(self) -> bool {
		self.0 & Self::LEFT_BIT != 0
	}

	/// Returns `true` if the middle mouse button is currently being pressed.
	pub const fn is_middle_pressed(self) -> bool {
		self.0 & Self::MIDDLE_BIT != 0
	}

	/// Returns `true` if the right mouse button is currently being pressed.
	pub const fn is_right_pressed(self) -> bool {
		self.0 & Self::RIGHT_BIT != 0
	}
}

impl Default for MouseButtons {
	fn default() -> Self {
		Self::new()
	}
}

impl KeyboardLeds {
	const CAPS_LOCK_BIT: u8 = 1 << 0;
	const SCROLL_LOCK_BIT: u8 = 1 << 1;
	const NUM_LOCK_BIT: u8 = 1 << 2;

	/// Create a new `KeyboardLeds` value.
	///
	/// All buttons default to *not pressed*
	pub const fn new() -> Self {
		Self(0)
	}

	/// Note that the Caps Lock light should be on.
	pub const fn set_caps_lock_on(self) -> Self {
		let value = self.0 | Self::CAPS_LOCK_BIT;
		Self(value)
	}

	/// Note that the Scroll Lock light should be on.
	pub const fn set_scroll_lock_on(self) -> Self {
		let value = self.0 | Self::SCROLL_LOCK_BIT;
		Self(value)
	}

	/// Note that the Num Lock light should be on.
	pub const fn set_num_lock_on(self) -> Self {
		let value = self.0 | Self::NUM_LOCK_BIT;
		Self(value)
	}

	/// Returns `true` if the Caps Lock light should be on.
	pub const fn is_caps_lock_on(self) -> bool {
		self.0 & Self::CAPS_LOCK_BIT != 0
	}

	/// Returns `true` if the Scroll Lock light should be on.
	pub const fn is_scroll_lock_on(self) -> bool {
		self.0 & Self::SCROLL_LOCK_BIT != 0
	}

	/// Returns `true` if the Num Lock light should be on.
	pub const fn is_num_lock_on(self) -> bool {
		self.0 & Self::NUM_LOCK_BIT != 0
	}
}

impl Default for KeyboardLeds {
	fn default() -> Self {
		Self::new()
	}
}

// ============================================================================
// End of File
// ============================================================================

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

// None

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

/// Represents a event from a Human Input Device (such as a mouse or keyboard).
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug)]
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

/// Represents a single key on a standard keyboard.
///
/// These keys are labelled according to their function on a standard United
/// States layout 104-key keyboard. If you have a different layout keyboard,
/// the symbol printed in the key you pressed may not match the `KeyCode`
/// value it generates. For example, if you press the `Z` key on a German
/// keyboard, you will get `KeyCode::Y` because that same key is the `Y` key
/// on a US keyboard.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum KeyCode {
	/// The left-hand `Alt` key
	AltLeft,
	/// The right-hand `Alt` (or `AltGr`) key
	AltRight,
	/// The Down cursor key
	ArrowDown,
	/// The Left cursor key
	ArrowLeft,
	/// The Right cursor key
	ArrowRight,
	/// The Up cursor key
	ArrowUp,
	/// The `\\|` key
	BackSlash,
	/// The `Backspace` (`Bksp` or `<--`) key
	Backspace,
	/// The `\`~` key.
	///
	/// Left of the `1` key. On a UK keyboard this is `\`¬|`.
	BackTick,
	/// The `[{` key
	BracketSquareLeft,
	/// The `]}` key
	BracketSquareRight,
	/// The `Caps Lock` key
	CapsLock,
	/// The `,<` key
	Comma,
	/// The left-most `Control` (`Ctrl`) key
	ControlLeft,
	/// The right-most `Control` (`Ctrl`) key
	ControlRight,
	/// The `Delete` (`Del`) key
	Delete,
	/// The `End` key
	End,
	/// The `Enter` (`Return`) key, on the right-hand side of the letters.
	Enter,
	/// The `Escape` (`Esc`) key
	Escape,
	/// The `=+` key
	Equals,
	/// The `F1` key
	F1,
	/// The `F2` key
	F2,
	/// The `F3` key
	F3,
	/// The `F4` key
	F4,
	/// The `F5` key
	F5,
	/// The `F6` key
	F6,
	/// The `F7` key
	F7,
	/// The `F8` key
	F8,
	/// The `F9` key
	F9,
	/// The `F10` key
	F10,
	/// The `F11` key
	F11,
	/// The `F12` key
	F12,
	/// The `.>` key
	Fullstop,
	/// The `Home` key
	Home,
	/// The `Insert` key
	Insert,
	/// The `1!` key
	Key1,
	/// The `2@` key
	Key2,
	/// The `3#` key
	Key3,
	/// The `4$` key
	Key4,
	/// The `5%` key
	Key5,
	/// The `6^` key
	Key6,
	/// The `7&` key
	Key7,
	/// The `8*` key
	Key8,
	/// The `9(` key
	Key9,
	/// The `0)` key
	Key0,
	/// The `Right-click Menu` key
	Menus,
	/// The `-_` key
	Minus,
	/// The `0` key on the Numeric Keypad
	Numpad0,
	/// The `1` key on the Numeric Keypad
	Numpad1,
	/// The `2` key on the Numeric Keypad
	Numpad2,
	/// The `3` key on the Numeric Keypad
	Numpad3,
	/// The `4` key on the Numeric Keypad
	Numpad4,
	/// The `5` key on the Numeric Keypad
	Numpad5,
	/// The `6` key on the Numeric Keypad
	Numpad6,
	/// The `7` key on the Numeric Keypad
	Numpad7,
	/// The `8` key on the Numeric Keypad
	Numpad8,
	/// The `9` key on the Numeric Keypad
	Numpad9,
	/// The `Enter` key on the Numeric Keypad
	NumpadEnter,
	/// The `Num Lock` key on the Numeric Keypad
	NumpadLock,
	/// The `/` key on the Numeric Keypad
	NumpadSlash,
	/// The `*` key on the Numeric Keypad
	NumpadStar,
	/// The `-` key on the Numeric Keypad
	NumpadMinus,
	/// The `.` key on the Numeric Keypad
	NumpadPeriod,
	/// The `+` key on the Numeric Keypad
	NumpadPlus,
	/// The `Page Down` key
	PageDown,
	/// The `Page Up` key
	PageUp,
	/// The `Pause/Break` key
	PauseBreak,
	/// The `Print Screen` (`PrtScr`) key
	PrintScreen,
	/// The `Scroll Lock` key
	ScrollLock,
	/// The `;:` key
	SemiColon,
	/// The left-most shift key
	ShiftLeft,
	/// The right-most shift key
	ShiftRight,
	/// The `/?` key
	Slash,
	/// The `Space` key
	Spacebar,
	/// The `Tab` key
	Tab,
	/// The `'` key
	Quote,
	/// The left-most Windows (or GUI) key
	WindowsLeft,
	/// The right-most Windows (or GUI) key
	WindowsRight,
	/// The `Aa` key
	A,
	/// The `Bb` key
	B,
	/// The `Cc` key
	C,
	/// The `Dd` key
	D,
	/// The `Ee` key
	E,
	/// The `Ff` key
	F,
	/// The `Gg` key
	G,
	/// The `Hh` key
	H,
	/// The `Ii` key
	I,
	/// The `Jj` key
	J,
	/// The `Kk` key
	K,
	/// The `Ll` key
	L,
	/// The `Mm` key
	M,
	/// The `Nn` key
	N,
	/// The `Oo` key
	O,
	/// The `Pp` key
	P,
	/// The `Qq` key
	Q,
	/// The `Rr` key
	R,
	/// The `Ss` key
	S,
	/// The `Tt` key
	T,
	/// The `Uu` key
	U,
	/// The `Vv` key
	V,
	/// The `Ww` key
	W,
	/// The `Xx` key
	X,
	/// The `Yy` key
	Y,
	/// The `Zz` key
	Z,
	/// Found on non-US (e.g. UK) keyboards next to the main *Return* key.
	HashTilde,
	/// Media transport: previous track
	PrevTrack,
	/// Media transport: next track
	NextTrack,
	/// Media transport: mute audio
	Mute,
	/// Application key: open calculator
	Calculator,
	/// Media transport: play audio
	Play,
	/// Media transport: stop audio
	Stop,
	/// Media transport: turn volume down
	VolumeDown,
	/// Media transport: turn volume up
	VolumeUp,
	/// Media transport: open browser to homepage
	WWWHome,
	/// Keyboard Power On Test passed.
	///
	/// Expect this once on start-up.
	PowerOnTestOk,
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

//! # Neotron Common BIOS
//!
//! Contains the common API for all Neotron BIOS implementations.
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

#![no_std]
#![deny(missing_docs)]

// ============================================================================
// Imports
// ============================================================================

pub mod audio;
pub mod block_dev;
pub mod bus;
pub mod hid;
pub mod i2c;
pub mod serial;
pub mod types;
pub mod version;
pub mod video;

pub use types::*;
pub use version::Version;

pub use neotron_ffi::{FfiBuffer, FfiByteSlice, FfiOption, FfiResult, FfiString};

// ============================================================================
// Constants
// ============================================================================

/// BIOS API semantic version for the API defined in this crate.
pub const API_VERSION: Version = Version::new(0, 6, 1);

// ============================================================================
// Types
// ============================================================================

/// Describes the result of an API call.
///
/// It's an FFI-safe Result [`FfiResult`], but the error type is fixed to be
/// [`Error`].
pub type ApiResult<T> = neotron_ffi::FfiResult<T, Error>;

/// The BIOS API, expressed as a structure of function pointers.
///
/// All Neotron BIOSes should provide this structure to the OS initialisation
/// function.
#[repr(C)]
pub struct Api {
	// ========================================================================
	// Version and Metadata
	// ========================================================================
	/// Gets the version number of the BIOS API.
	///
	/// You need this value to determine which of the following API calls are
	/// valid in this particular version.
	pub api_version_get: extern "C" fn() -> Version,
	/// Returns a pointer to a static string slice.
	///
	/// This string contains the version number and build string of the BIOS.
	/// For C compatibility this string is null-terminated and guaranteed to
	/// only contain ASCII characters (bytes with a value 127 or lower). We
	/// also pass the length (excluding the null) to make it easy to construct
	/// a Rust string. It is unspecified as to whether the string is located
	/// in Flash ROM or RAM (but it's likely to be Flash ROM).
	pub bios_version_get: extern "C" fn() -> FfiString<'static>,

	// ========================================================================
	// Serial Port Support
	// ========================================================================
	/// Get information about the Serial ports in the system.
	///
	/// Serial ports are ordered octet-oriented pipes. You can push octets
	/// into them using a 'write' call, and pull bytes out of them using a
	/// 'read' call. They have options which allow them to be configured at
	/// different speeds, or with different transmission settings (parity
	/// bits, stop bits, etc) - you set these with a call to
	/// `SerialConfigure`. They may physically be a MIDI interface, an RS-232
	/// port or a USB-Serial port. There is no sense of 'open' or 'close' -
	/// that is an Operating System level design feature. These APIs just
	/// reflect the raw hardware, in a similar manner to the registers exposed
	/// by a memory-mapped UART peripheral.
	pub serial_get_info: extern "C" fn(device_id: u8) -> crate::FfiOption<serial::DeviceInfo>,
	/// Set the options for a given serial device. An error is returned if the
	/// options are invalid for that serial device.
	pub serial_configure:
		extern "C" fn(device_id: u8, config: serial::Config) -> crate::ApiResult<()>,
	/// Write bytes to a serial port. There is no sense of 'opening' or
	/// 'closing' the device - serial devices are always open. If the return
	/// value is `Ok(n)`, the value `n` may be less than the size of the given
	/// buffer. If so, that means not all of the data could be transmitted -
	/// only the first `n` bytes were.
	pub serial_write: extern "C" fn(
		device_id: u8,
		data: FfiByteSlice,
		timeout: crate::FfiOption<Timeout>,
	) -> crate::ApiResult<usize>,
	/// Read bytes from a serial port. There is no sense of 'opening'
	/// or 'closing' the device - serial devices are always open. If the
	/// return value is `Ok(n)`, the value `n` may be less than the size of
	/// the given buffer. If so, that means not all of the requested data
	/// could be received - only the first `n` bytes were (and hence only the
	/// first `n` bytes of the given buffer now contain data).
	pub serial_read: extern "C" fn(
		device_id: u8,
		data: FfiBuffer,
		timeout: crate::FfiOption<Timeout>,
	) -> crate::ApiResult<usize>,

	// ========================================================================
	// Time Support
	// ========================================================================
	/// Get the current wall time.
	///
	/// The Neotron BIOS does not understand time zones, leap-seconds or the
	/// Gregorian calendar. It simply stores time as an incrementing number of
	/// seconds since some epoch, and the number of video frames (at 60 Hz)
	/// since that second began. A day is assumed to be exactly 86,400 seconds
	/// long. This is a lot like POSIX time, except we have a different epoch
	/// - the Neotron epoch is 2000-01-01T00:00:00Z. It is highly recommend
	/// that you store UTC in the BIOS and use the OS to handle time-zones.
	///
	/// If the BIOS does not have a battery-backed clock, or if that battery
	/// has failed to keep time, the system starts up assuming it is the
	/// epoch.
	pub time_clock_get: extern "C" fn() -> Time,
	/// Set the current wall time.
	///
	/// See `time_get` for a description of now the Neotron BIOS should handle
	/// time.
	///
	/// You only need to call this whenever you get a new sense of the current
	/// time (e.g. the user has updated the current time, or if you get a GPS
	/// fix). The BIOS should push the time out to the battery-backed Real
	/// Time Clock, if it has one.
	pub time_clock_set: extern "C" fn(time: Time),
	/// Get the current monotonic system time.
	///
	/// This value will never go backwards and it should never wrap.
	pub time_ticks_get: extern "C" fn() -> Ticks,
	/// Report the system tick rate, in ticks-per-second.
	pub time_ticks_per_second: extern "C" fn() -> Ticks,

	// ========================================================================
	// Persistent Configuration Support
	// ========================================================================
	/// Get the configuration data block.
	///
	/// Configuration data is, to the BIOS, just a block of bytes of a given
	/// length. How it stores them is up to the BIOS - it could be EEPROM, or
	/// battery-backed SRAM.
	pub configuration_get: extern "C" fn(buffer: FfiBuffer) -> crate::ApiResult<usize>,
	/// Set the configuration data block.
	///
	/// See `configuration_get`.
	pub configuration_set: extern "C" fn(buffer: FfiByteSlice) -> crate::ApiResult<()>,

	// ========================================================================
	// Video Output Support
	// ========================================================================
	/// Does this Neotron BIOS support this video mode?
	pub video_is_valid_mode: extern "C" fn(mode: video::Mode) -> bool,
	/// Does this Neotron BIOS require extra VRAM (passed with
	/// `video_set_framebuffer`) before this mode will work?
	pub video_mode_needs_vram: extern "C" fn(mode: video::Mode) -> bool,
	/// Switch to a new video mode.
	///
	/// The contents of the screen are undefined after a call to this function.
	///
	/// If the BIOS does not have enough reserved RAM (or dedicated VRAM) to
	/// support this mode, the change will succeed but a subsequent call to
	/// `video_get_framebuffer` will return `null`. You must then supply a
	/// pointer to a block of size `Mode::frame_size_bytes()` to
	/// `video_set_framebuffer` before any video will appear.
	pub video_set_mode: extern "C" fn(mode: video::Mode) -> crate::ApiResult<()>,
	/// Returns the video mode the BIOS is currently in.
	///
	/// The OS should call this function immediately after start-up and note
	/// the value - this is the `default` video mode which can always be
	/// serviced without supplying extra RAM.
	pub video_get_mode: extern "C" fn() -> video::Mode,
	/// Get the framebuffer address.
	///
	/// We can write through this address to the video framebuffer. The
	/// meaning of the data we write, and the size of the region we are
	/// allowed to write to, is a function of the current video mode (see
	/// `video_get_mode`).
	///
	/// This function will return `null` if the BIOS isn't able to support the
	/// current video mode from its memory reserves. If that happens, you will
	/// need to use some OS RAM or Application RAM and provide that as a
	/// framebuffer to `video_set_framebuffer`. The BIOS will always be able
	/// to provide the 'basic' text buffer experience from reserves, so this
	/// function will never return `null` on start-up.
	pub video_get_framebuffer: extern "C" fn() -> *mut u8,
	/// Set the framebuffer address.
	///
	/// Tell the BIOS where it should start fetching pixel or textual data
	/// from(depending on the current video mode). This pointer is retained
	/// and the memory is continually acccessed after this function call ends.
	///
	/// This value is forgotten after a video mode change and must be
	/// re-supplied.
	///
	/// Once the BIOS has handed over to the OS, it will never write to this
	/// video memory, only read from it.
	///
	/// # Safety
	///
	/// The region pointed to by `start_address` must be large enough to
	/// contain however much video memory is required by both the current
	/// video mode.
	pub video_set_framebuffer:
		unsafe extern "C" fn(start_address: *const u8) -> crate::ApiResult<()>,
	/// Wait for the next occurence of the specified video scan-line.
	///
	/// In general we must assume that the video memory is read top-to-bottom
	/// as the picture is being drawn on the monitor (e.g. via a VGA video
	/// signal). If you modify video memory during this *drawing period*
	/// there is a risk that the image on the monitor (however briefly) may
	/// contain some parts from before the modification and some parts from
	/// after. This can given rise to the *tearing effect* where it looks
	/// like the screen has been torn (or ripped) across because there is a
	/// discontinuity part-way through the image.
	///
	/// This function busy-waits until the video drawing has reached a
	/// specified scan-line on the video frame.
	///
	/// There is no error code here. If the line you ask for is beyond the
	/// number of visible scan-lines in the current video mode, it waits util
	/// the last visible scan-line is complete.
	///
	/// If you wait for the last visible line until drawing, you stand the
	/// best chance of your pixels operations on the video RAM being
	/// completed before scan-lines start being sent to the monitor for the
	/// next frame.
	///
	/// You can also use this for a crude `16.7 ms` delay but note that
	/// some video modes run at `70 Hz` and so this would then give you a
	/// `14.3ms` second delay.
	pub video_wait_for_line: extern "C" fn(line: u16),
	/// Get an entry from the colour palette.
	///
	/// Almost all video modes (except `Chunky16` and `Chunky32`) use a video
	/// palette. This function returns the RGB colour for a given palette
	/// index.
	///
	/// If you ask for an entry that is beyond the capabilities of the current
	/// video mode, you get `None`.
	pub video_get_palette: extern "C" fn(palette_idx: u8) -> crate::FfiOption<video::RGBColour>,
	/// Set an entry in the colour palette.
	///
	/// Almost all video modes (except `Chunky16` and `Chunky32`) use a video
	/// palette. This function changes the RGB colour for a given palette
	/// index.
	///
	/// If you set an entry beyond what the current mode supports, the value
	/// is ignored.
	pub video_set_palette: extern "C" fn(palette_idx: u8, video::RGBColour),
	/// Sets all the entries in the colour palette at once.
	///
	/// Almost all video modes (except `Chunky16` and `Chunky32`) use a video
	/// palette. This function changes all the RGB colours in the current
	/// palette.
	///
	/// If you pass a `len` beyond what the current mode supports, the extra
	/// values are ignored. The given buffer is copied so it doesn't need to
	/// live beyond this function call.
	///
	/// # Safety
	///
	/// The value `start` must point to an array of `RGBColour` of length
	/// `length`.
	///
	pub video_set_whole_palette:
		unsafe extern "C" fn(start: *const video::RGBColour, length: usize),

	// ========================================================================
	// Memory Region Support
	// ========================================================================
	/// Find out about regions of memory in the system.
	///
	/// The first region (index `0`) must be the 'application region' which is
	/// defined to always start at address `0x2000_0400` (that is, 1 KiB into
	/// main SRAM) on a standard Cortex-M system. This application region stops
	/// just before the BIOS reserved memory, typically at the top of the
	/// internal SRAM.
	///
	/// Other regions may be located at other addresses (e.g. external DRAM or
	/// PSRAM).
	///
	/// The OS will always load non-relocatable applications into the bottom of
	/// Region 0. It can allocate OS specific structures from any other Region
	/// (if any), or from the top of Region 0 (although this reduces the maximum
	/// application space available). The OS will prefer lower numbered regions
	/// (other than Region 0), so faster memory should be listed first.
	pub memory_get_region: extern "C" fn(region_index: u8) -> crate::FfiOption<MemoryRegion>,

	// ========================================================================
	// Human Interface Device Support
	// ========================================================================
	/// Get the next available HID event, if any.
	///
	/// This function doesn't block. It will return `Ok(None)` if there is no event ready.
	pub hid_get_event: extern "C" fn() -> crate::ApiResult<crate::FfiOption<hid::HidEvent>>,
	/// Control the keyboard LEDs.
	pub hid_set_leds: extern "C" fn(leds: hid::KeyboardLeds) -> crate::ApiResult<()>,

	// ========================================================================
	// I²C Bus Support
	// ========================================================================
	/// Get information about the I²C Buses in the system.
	///
	/// I²C Bus 0 should be the one connected to the Neotron Bus.
	/// I²C Bus 1 is typically the VGA DDC bus.
	pub i2c_bus_get_info: extern "C" fn(bus_id: u8) -> crate::FfiOption<i2c::BusInfo>,
	/// Transact with a I²C Device on an I²C Bus
	///
	/// * `i2c_bus` - Which I²C Bus to use
	/// * `i2c_device_address` - The 7-bit I²C Device Address
	/// * `tx` - the first list of bytes to send (use `FfiByteSlice::empty()` if not required)
	/// * `tx2` - the second (and optional) list of bytes to send (use `FfiByteSlice::empty()` if not required)
	/// * `rx` - the buffer to fill with read data (use `FfiBuffer::empty()` if not required)
	///
	/// ```no_run
	/// # let api = neotron_common_bios::Api::make_dummy_api().unwrap();
	/// # use neotron_common_bios::{FfiByteSlice, FfiBuffer};
	/// // Read 16 bytes from the start of an EEPROM with device address 0x65 on Bus 0
	/// let mut buf = [0u8; 16];
	/// let _ = (api.i2c_write_read)(0, 0x65, FfiByteSlice::new(&[0x00, 0x00]), FfiByteSlice::empty(), FfiBuffer::new(&mut buf));
	/// // Write those bytes to somewhere else in an EEPROM with device address 0x65 on Bus 0
	/// // You can see now why it's useful to have *two* TX buffers available
	/// let _ = (api.i2c_write_read)(0, 0x65, FfiByteSlice::new(&[0x00, 0x10]), FfiByteSlice::new(&buf), FfiBuffer::empty());
	/// # Ok::<(), neotron_common_bios::Error>(())
	/// ```
	pub i2c_write_read: extern "C" fn(
		bus_id: u8,
		i2c_device_address: u8,
		tx: FfiByteSlice,
		tx2: FfiByteSlice,
		rx: FfiBuffer,
	) -> crate::ApiResult<()>,

	// ========================================================================
	// Audio Support
	// ========================================================================
	/// Get information about the Audio Mixer channels
	pub audio_mixer_channel_get_info:
		extern "C" fn(audio_mixer_id: u8) -> crate::FfiOption<audio::MixerChannelInfo>,
	/// Set an Audio Mixer level
	pub audio_mixer_channel_set_level:
		extern "C" fn(audio_mixer_id: u8, level: u8) -> crate::ApiResult<()>,
	/// Configure the audio output.
	///
	/// If accepted, the audio output FIFO is flushed and the changes apply
	/// immediately. If not accepted, an error is returned.
	///
	/// It is not currently possible to enumerate all the possible sample
	/// rates - you just have to try a variety of well know configurations to
	/// see which ones work.
	///
	/// Note that if your desired sample rate cannot be exactly accepted, but
	/// is within some tolerance, this function will still succeed. Therefore
	/// you should call `audio_output_get_config` to get the precise sample
	/// rate that the system is actually using if that matters to your
	/// application. For example, you might ask for 48,000 Hz but due to the
	/// system clock frequency and other factors, a sample rate of 48,018 Hz
	/// might actually be achieved. Regardless, to avoid buffer underflows
	/// you should supply as many samples as `audio_output_get_space` says
	/// you need, not what you think you need based on the sample rate you
	/// think you have.
	pub audio_output_set_config: extern "C" fn(config: audio::Config) -> crate::ApiResult<()>,
	/// Get the audio output's current configuration.
	pub audio_output_get_config: extern "C" fn() -> crate::ApiResult<audio::Config>,
	/// Send audio samples to the output FIFO.
	///
	/// The format of the samples (little-endian, 16-bit, etc), depends on the
	/// current output configuration. Note that the slice is in *bytes* and
	/// there will be between *one* and *four* bytes per sample depending on
	/// the format.
	///
	/// This function won't block, but it will return how much data was
	/// accepted. The given samples will be copied and so the buffer is free
	/// to re-use once the function returns. To avoid buffer underflows you
	/// should supply as many samples as `audio_output_get_space` says you
	/// need, not what you think you need based on the sample rate you think
	/// you have (as there will always be some error margin on that).
	///
	/// If the buffer underflows, silence is played out.
	///
	/// There is only one hardware output stream so any mixing has to be
	/// performed in software by the OS.
	pub audio_output_data: unsafe extern "C" fn(samples: FfiByteSlice) -> crate::ApiResult<usize>,
	/// Get audio buffer space.
	///
	/// How many samples in the current format can be sent to
	/// `audio_output_data` without blocking?
	pub audio_output_get_space: extern "C" fn() -> crate::ApiResult<usize>,
	/// Configure the audio input.
	///
	/// If accepted, the audio input FIFO is flushed and the changes apply
	/// immediately. If not accepted, an error is returned.
	///
	/// It is not currently possible to enumerate all the possible sample
	/// rates - you just have to try a variety of well know configurations to
	/// see which ones work.
	///
	/// Note that if your desired sample rate cannot be exactly accepted, but
	/// is within some tolerance, this function will still succeed. Therefore
	/// you should call `audio_output_get_config` to get the precise sample
	/// rate that the system is actually using if that matters to your
	/// application. For example, you might ask for 48,000 Hz but due to the
	/// system clock frequency and other factors, a sample rate of 48,018 Hz
	/// might actually be achieved.
	pub audio_input_set_config: extern "C" fn(config: audio::Config) -> crate::ApiResult<()>,
	/// Get the audio input's current configuration.
	pub audio_input_get_config: extern "C" fn() -> crate::ApiResult<audio::Config>,
	/// Get 16-bit stereo audio from the input FIFO.
	///
	/// The format of the samples (little-endian, 16-bit, etc), depends on the
	/// current output configuration. Note that the slice is in *bytes* and
	/// there will be between *one* and *four* bytes per sample depending on
	/// the format.
	///
	/// This function won't block, but it will return how much data was
	/// actually written to the buffer.
	///
	/// If you don't call it often enough, there will be a buffer overflow and
	/// audio will be dropped.
	pub audio_input_data: unsafe extern "C" fn(samples: FfiBuffer) -> crate::ApiResult<usize>,
	/// Get audio buffer space.
	///
	/// How many samples in the current format can be read right now using
	/// `audio_input_data`?
	pub audio_input_get_count: extern "C" fn() -> crate::ApiResult<usize>,

	// ========================================================================
	// Neotron (SPI) Bus Support
	// ========================================================================
	/// Select a Neotron Bus Peripheral. This drives the SPI chip-select line
	/// low for that peripheral. Selecting a peripheral de-selects any other
	/// peripherals. Select peripheral 'None' to select no peripherals. If
	/// you lock the bus then interrupt routines that need the bus are
	/// blocked and must be deferred. Therefore you should try and release
	/// the bus whilst waiting for things to happen (if your peripheral can
	/// tolerate the CS line being de-activated at that time).
	pub bus_select: extern "C" fn(peripheral_id: crate::FfiOption<u8>),
	/// Find out some details about each particular Neotron Bus Peripheral.
	pub bus_get_info: extern "C" fn(peripheral_id: u8) -> crate::FfiOption<bus::PeripheralInfo>,
	/// Transact with the currently selected Neotron Bus Peripheral.
	///
	/// You should select a peripheral with `bus_select` first,
	/// however you can send unselected traffic (e.g. to configure an SD Card
	/// into SPI mode).
	///
	/// * `tx` - the first list of bytes to send (use `&[]` if not required)
	/// * `tx2` - the second (and optional) list of bytes to send (use `&[]` if not required)
	/// * `rx` - the buffer to fill with read data (use `&mut []` if not required)
	///
	/// Because SPI is full-duplex, we discard incoming bytes during the TX
	/// portion. We must also clock out *something* during the RX portion,
	/// and we chose `0xFF` bytes. If that doesn't work, use `bus_exchange`.
	///
	/// ```no_run
	/// # let api = neotron_common_bios::Api::make_dummy_api().unwrap();
	/// # use neotron_common_bios::{FfiByteSlice, FfiBuffer, FfiOption};
	/// // Grab Peripheral 1 on the bus
	/// let _ = (api.bus_select)(FfiOption::Some(1));
	/// // Read 16 bytes from Register 0 of the selected peripheral
	/// let mut buf = [0u8; 16];
	/// let _ = (api.bus_write_read)(FfiByteSlice::new(&[0, 16]), FfiByteSlice::empty(), FfiBuffer::new(&mut buf));
	/// // Write those bytes to Register 2. You can see now why it's useful to
	/// // have *two* TX buffers in the API
	/// let _ = (api.bus_write_read)(FfiByteSlice::new(&[2, 16]), FfiByteSlice::new(&buf), FfiBuffer::empty());
	/// // Release the bus
	/// let _ = (api.bus_select)(FfiOption::None);
	/// # Ok::<(), neotron_common_bios::Error>(())
	/// ```
	pub bus_write_read:
		extern "C" fn(tx: FfiByteSlice, tx2: FfiByteSlice, rx: FfiBuffer) -> crate::ApiResult<()>,
	/// Exchange bytes with the currently selected Neotron Bus Peripheral.
	///
	/// You should select a peripheral with `bus_select` first,
	/// however you can send unselected traffic (e.g. to configure an SD Card
	/// into SPI mode).
	///
	/// SPI is full-duplex, and this routine clocks out the bytes in `buffer`
	/// one at a time, and replaces them with the bytes received from the
	/// peripheral.
	///
	/// ```no_run
	/// # let api = neotron_common_bios::Api::make_dummy_api().unwrap();
	/// # use neotron_common_bios::{FfiByteSlice, FfiBuffer, FfiOption};
	/// // Grab Peripheral 1 on the bus
	/// let _ = (api.bus_select)(FfiOption::Some(1));
	/// // Exchange four bytes with the peripheral
	/// let mut buf = [0, 1, 2, 3];
	/// let _ = (api.bus_exchange)(FfiBuffer::new(&mut buf));
	/// // buf now contains whatever the peripheral sent us.
	/// // Release the bus
	/// let _ = (api.bus_select)(FfiOption::None);
	/// # Ok::<(), neotron_common_bios::Error>(())
	/// ```
	pub bus_exchange: extern "C" fn(buffer: FfiBuffer) -> crate::ApiResult<()>,
	/// Get bus interrupt status.
	///
	/// Up to 32 interrupts can be returned as a single 32-bit value. A bit is
	/// set when the interrupt is pending. There is no masking - ignore the bits
	/// you don't care about.
	pub bus_interrupt_status: extern "C" fn() -> u32,

	// ========================================================================
	// Block Device Support
	// ========================================================================
	/// Get information about the Block Devices in the system.
	///
	/// Block Devices are also known as *disk drives*. They can be read from
	/// (and often written to) but only in units called *blocks* or *sectors*.
	///
	/// The BIOS should enumerate removable devices first, followed by fixed
	/// devices.
	///
	/// The set of devices is not expected to change at run-time - removal of
	/// media is indicated with a boolean field in the
	/// `block_dev::DeviceInfo` structure.
	pub block_dev_get_info: extern "C" fn(device_id: u8) -> crate::FfiOption<block_dev::DeviceInfo>,
	/// Eject a disk from the drive.
	///
	/// Will return an error if this device is not removable. Does not return an
	/// error if the drive is already empty.
	pub block_dev_eject: extern "C" fn(device_id: u8) -> crate::ApiResult<()>,
	/// Write one or more sectors to a block device.
	///
	/// The function will block until all data is written. The array pointed
	/// to by `data` must be `num_blocks * block_size` in length, where
	/// `block_size` is given by `block_dev_get_info`.
	///
	/// There are no requirements on the alignment of `data` but if it is
	/// aligned, the BIOS may be able to use a higher-performance code path.
	pub block_write: extern "C" fn(
		device_id: u8,
		start_block: block_dev::BlockIdx,
		num_blocks: u8,
		data: FfiByteSlice,
	) -> crate::ApiResult<()>,
	/// Read one or more sectors to a block device.
	///
	/// The function will block until all data is read. The array pointed
	/// to by `data` must be `num_blocks * block_size` in length, where
	/// `block_size` is given by `block_dev_get_info`.
	///
	/// There are no requirements on the alignment of `data` but if it is
	/// aligned, the BIOS may be able to use a higher-performance code path.
	pub block_read: extern "C" fn(
		device_id: u8,
		start_block: block_dev::BlockIdx,
		num_blocks: u8,
		data: FfiBuffer,
	) -> crate::ApiResult<()>,
	/// Verify one or more sectors on a block device (that is read them and
	/// check they match the given data).
	///
	/// The function will block until all data is verified. The array pointed
	/// to by `data` must be `num_blocks * block_size` in length, where
	/// `block_size` is given by `block_dev_get_info`.
	///
	/// There are no requirements on the alignment of `data` but if it is
	/// aligned, the BIOS may be able to use a higher-performance code path.
	pub block_verify: extern "C" fn(
		device_id: u8,
		start_block: block_dev::BlockIdx,
		num_blocks: u8,
		data: FfiByteSlice,
	) -> crate::ApiResult<()>,

	// ========================================================================
	// Power management functions
	// ========================================================================
	/// The OS will call this function when it's idle.
	///
	/// On a microcontroller, this will wait for interrupts. Running in an
	/// emulator, this will sleep the thread for a while.
	pub power_idle: extern "C" fn(),
	/// The OS will call this function to turn the power off.
	///
	/// This function will not return, because the system will be switched off
	/// before it can return. In the event on an error, this function will hang
	/// instead.
	pub power_off: extern "C" fn() -> !,
	/// The OS will call this function to reboot the system.
	///
	/// This function will not return, because the system will be rebooted
	/// before it can return. In the event on an error, this function will hang
	/// instead.
	pub power_reboot: extern "C" fn() -> !,

	// ========================================================================
	// Mutex functions
	// ========================================================================
	/// Performs a compare-and-swap on `value`.
	///
	/// * If `value == old_value`, sets `value = new_value` and returns `true`
	/// * If `value != old_value`, returns `false`
	pub compare_and_swap_bool: extern "C" fn(
		value: &core::sync::atomic::AtomicBool,
		old_value: bool,
		new_value: bool,
	) -> bool,
}

// ============================================================================
// Impls
// ============================================================================

impl Api {
	/// This function only exists to make the doctests compile.
	///
	/// It always returns `None`.
	#[doc(hidden)]
	pub fn make_dummy_api() -> core::option::Option<Api> {
		None
	}
}

// ============================================================================
// End of File
// ============================================================================

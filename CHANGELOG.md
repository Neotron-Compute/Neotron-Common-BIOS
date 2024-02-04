# Changelog

## Unreleased Changes

* None

## v0.12.0 - 2023-10-21 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.12.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.12.0) | [Crate](https://crates.io/crates/neotron-common-bios/0.12.0))

* The `video_set_mode` API now takes a framebuffer pointer.
* The `video_set_framebuffer` API was removed.
* Made all types FFI safe (use `struct Foo(u8)` not `enum Foo`)
* Add `MemoryKind::StackFree` and `MemoryKind::StackUsed`
* `TextForegroundColour` and `TextBackgroundColour` are now enums, not structs with const values.

## v0.11.1 - 2023-10-01 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.11.1) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.11.1) | [Crate](https://crates.io/crates/neotron-common-bios/v0.11.1))

* New `video::Mode::new_with_scaling` method
* New `video::Scaling` type
* Marked methods as inline to help avoid thunks when code is in RAM

## v0.11.0 - 2023-07-21 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.11.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.11.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.11.0))

* Add compare_and_swap_bool function
* Add power_control function

## v0.10.0 - 2023-07-15 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.10.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.10.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.10.0))

* Change palette constants to match the VGA standard.

## v0.9.0 - 2023-07-15 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.9.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.9.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.9.0))

* Use types from [`neotron-ffi`](https://crates.io/crates/neotron-ffi) crate.

## v0.8.0 - 2023-02-12 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.8.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.8.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.8.0))

* `audio_mixer_channel_get_info` now returns an `Option` not `Result`
* Add `impl From<core::option::Option for Option`
* Add `impl From<Option for core::option::Option`
* Add `impl From<core::result::Result for Result`
* Add `impl From<Result for core::result::Result`
* Clarify that some parameters are actually expected to be numeric IDs
* Use [`pc-keyboard::KeyCode`](https://crates.io/crates/pc-keyboard) to define Key Codes.

## v0.7.0 - 2022-11-17 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.7.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.7.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.7.0))

* Change `time_get` to `time_clock_get`
* Change `time_set` to `time_clock_set`
* Add `time_ticks_get` and `time_ticks_per_second`
* Add `bus_interrupt_status`
* Remove `delay`
* Add back in the `block_XXX` API for reading/writing Block Devices.
* Add idle function.
* `memory_get_region` returns `Option`, not `Result`
* Fix epoch used in conversion to chrono timestamp.

## v0.6.1 - 2022-04-18 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.6.1) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.6.1) | [Crate](https://crates.io/crates/neotron-common-bios/v0.6.1))

* No changes - v0.6.0 release was incorrect so re-releasing

## v0.6.0 - 2022-04-18 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.6.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.6.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.6.0))

* Removed 'block_X' APIs
* Added 'bus_X' APIs
* Added 'audio_X' APIs
* Added 'i2c_X' APIs
* Added 'video_get/set_palette' APIs
* Added 'delay' API

## v0.5.0 - 2022-04-15 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.5.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.5.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.5.0))

* Added `serial_read` API
* Added `video_mode_needs_vram` API
* Added `hid_get_event` API
* Added `hid_set_leds` API
* Added `video_wait_for_line` API
* Added `block_dev_get_info` API
* Added `block_write` API
* Added `block_read` API
* Added `block_verify` API

## v0.4.0 - 2022-04-06 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.4.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.4.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.4.0))

* Changed `memory_get_region` to return a `MemoryRegion`
* Changed `video_set_framebuffer` to take a `*const u8` not `*mut u8` - as the
  BIOS doesn't change video RAM.

## v0.1.0 - 2022-03-18 ([Source](https://github.com/neotron-compute/neotron-common-bios/tree/v0.1.0) | [Release](https://github.com/neotron-compute/neotron-common-bios/releases/tag/v0.1.0) | [Crate](https://crates.io/crates/neotron-common-bios/v0.1.0))

* First published version. `Cargo.toml` reports at `0.1.0`.

# Neotron Common BIOS

This crate contains the BIOS API and common routines for all Neotron systems.

![Build Status](https://github.com/neotron-compute/Neotron-Common-BIOS/workflows/Build/badge.svg "Github Action Build Status")

![Format Status](https://github.com/neotron-compute/Neotron-Common-BIOS/workflows/Format/badge.svg "Github Action Format Check Status")

## Hardware

Neotron runs on a variety of ARM Cortex-M based systems.

## Status

This BIOS API crate is a work in progress.

## License

    Copyright (C) The Neotron Developers, 2019-2022

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Changelog

### Unreleased Changes

* None

### v0.5.0

* Added `serial_read` API
* Added `video_mode_needs_vram` API
* Added `hid_get_event` API
* Added `hid_set_leds` API
* Added `video_wait_for_line` API
* Added `block_dev_get_info` API
* Added `block_write` API
* Added `block_read` API
* Added `block_verify` API

### v0.4.0

* Changed `memory_get_region` to return a `MemoryRegion`
* Changed `video_set_framebuffer` to take a `*const u8` not `*mut u8` - as the
  BIOS doesn't change video RAM.

### v0.3.0

* First published version. `Cargo.toml` reports at `0.1.0`.

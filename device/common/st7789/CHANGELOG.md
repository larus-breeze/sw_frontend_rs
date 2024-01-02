# CHANGELOG

## v0.7.0

* introduced control over the backlight pin and made reset pin optional
* fix `clear()` on targets with `usize` smaller than `u32` 
* fix negative pixel coordinate overflow bug in the `batch` module

## v0.6

* `v0.6.1` - remove unused `buffer` feature
* `v0.6.1` - optimize `fill_solid` and add tearing effect support
* `v0.6.0` - update `embedded-graphics` support to `v0.7`

## v0.5

* `v0.5.0` - add hardware scrolling support

## v0.4

* `v0.4.3` - fix endianness issue if `buffer` feature is turned off
* `v0.4.2` - fix [CASET/RASET 16bit bug](https://github.com/almindor/st7789/issues/9)
* `v0.4.1` - add `release` deconstructor
* `v0.4.0` - use `v3` of `display-interface` with 16bit support

## v0.3

* `v0.3.1` - deprecate `new_display_driver`, fix docs
* `v0.3.0` - use `display-interface`

## v0.2

* `v0.2.3` - add optimized clear()
* `v0.2.2` - add buffering for SPI transfers
* `v0.2.1` - use static dispatch for `set_pixels`
* `v0.2.0` - batch support

## v0.1

* `v0.1.0` - initial release


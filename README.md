# arm7tdmi_aeabi

Implements runtime support functions according to ARM's [AEABI][aeabi]. All
functions are specialized to the ARM7TDMI CPU. They should work with any later
ARM CPU as well, but because of instruction pipeline differences they might have
less than optimal performance.

[aeabi]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst

These functions are intended to support Rust development on the GBA, but they're
written in assembly so they should work with any language and any ARMv4T or
later device.

## Current Support

Currently the code uses slightly alternate names from the "real" names so that
it can be compiled into tests without clashing with the system version of each
symbol.

The following functions are currently provided:

```rust
extern "C" {
  pub fn libc_memcpy(d: *mut u8, s: *const u8, bytes: usize) -> *mut u8;
  pub fn aeabi_memcpy(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memcpy4(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memcpy8(d: *mut u8, s: *const u8, bytes: usize);
  pub fn gba_memcpy_sram(d: *mut u8, s: *const u8, bytes: usize);

  pub fn libc_memmove(d: *mut u8, s: *const u8, bytes: usize) -> *mut u8;
  pub fn aeabi_memmove(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memmove4(d: *mut u8, s: *const u8, bytes: usize);
  pub fn aeabi_memmove8(d: *mut u8, s: *const u8, bytes: usize);

  pub fn libc_memset(d: *mut u8, val: i32, bytes: usize) -> *mut u8;
  pub fn aeabi_memset(d: *mut u8, bytes: usize, val: i32);
  pub fn aeabi_memset4(d: *mut u8, bytes: usize, val: i32);
  pub fn aeabi_memset8(d: *mut u8, bytes: usize, val: i32);

  pub fn aeabi_memclr(d: *mut u8, bytes: usize);
  pub fn aeabi_memclr4(d: *mut u8, bytes: usize);
  pub fn aeabi_memclr8(d: *mut u8, bytes: usize);

  pub fn aeabi_uread4(address: *const u32) -> u32;
  pub fn aeabi_uread8(address: *const u64) -> u64;
  pub fn aeabi_uwrite4(value: u32, address: *mut u32) -> u32;
  pub fn aeabi_uwrite8(value: u64, address: *mut u64) -> u64;

  pub fn aeabi_idiv(n: i32, d: i32) -> i32;
  pub fn aeabi_uidiv(n: u32, d: u32) -> u32;
  pub fn aeabi_idivmod(n: i32, d: i32) -> u64;
  pub fn aeabi_uidivmod(n: u32, d: u32) -> u64;
}
```

* "memory copy" functions assume exclusive regions. The `gba_memcpy_sram`
  function is not part of the AEABI, but lets you do a copy that is *guaranteed*
  to access only one byte at a time. This makes it safe to use with the GBA's
  SRAM.
* "memory move" functions allow for overlapping regions.
* "memory set" functions will set `val as u8` too all bytes in the region. It
  might seem silly to pass an `i32` that's only used as a `u8`, but `memset`
  *predates* function prototypes in C, so that's how it works. Note that the
  libc and aeabi versions have swapped argument order. This isn't a huge deal in
  Rust, the compiler will catch when you mix it up.
* "memory clear" functions work like "memory set", but the value to set is
  implied to be 0, which allows a minor optimization for the aligned versions.
* "unaligned read" functions return the value read.
* "unaligned write" functions return the value written.

Functions with a 4 or 8 on the end require that input pointers be aligned to
that much. The `bytes` value does *not* need to be an even multiple of the
alignment requirement.

All `libc_` functions give the original destination pointer they were passed as
their return value. All `aeabi_` functions return nothing at all (and this is
more efficient, so use them when possible).

## Use

Put this somewhere in your project:

```rust
arm7tdmi_aeabi::generate_fns!(section_prefix = ".iwram");
```

You can adjust the section_prefix as you like, `.iwram` is used on the GBA.

## Testing

Testing of this crate is generally easiest using [cross][cross-rs].

[cross-rs]: https://github.com/cross-rs/cross

```
cross test --target arm-unknown-linux-gnueabi
```

Or, if you're running on an ARM device (eg: rpi with the 32-bit OS) then you can
probably test natively I guess.

## License

All the code here is released under `CC0-1.0`.

OR (if you really want to use the standard Rust project licenses) `Apache-2.0` OR
`MIT` can also be used.

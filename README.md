# green-threads

Stackful coroutines for stable rust.

## Status: pre-alpha, do not use

This library is essentially a hand-written pile of various assembly
dialects written from first principles and the help of the
internet. It has not yet been tested at all and it's likely broken if
you can manage to find a use for it at all.

Are you an ABI expert? Please review my code. God knows it could use more eyes.

## Usage

<!-- ```rust -->
<!-- use green_threads::*; -->

<!-- fn main() { -->
<!--   // First detect a profile of the hardware we need to support. -->
<!--   let profile = Profile::detect(); -->
<!--   // Then turn that into a vtable -->
<!--   let vtable = profile.vtable(); -->
  
  
<!-- } -->
<!-- ``` -->

## Support

Supported architectures:

* aarch64
* arm
* x86
* x86_64

Architectures we'd like to support:

* riscv

Note that this completes the list of supported architectures for the
`asm` macro, so anything else would likely be painful.


## Copyright and License

Copyright (c) 2022 James Laver, green-threads contributors.

[Licensed](LICENSE) under Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0),
with LLVM Exceptions (https://spdx.org/licenses/LLVM-exception.html).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.



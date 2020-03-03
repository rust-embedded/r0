# r0

[![Build status](https://api.travis-ci.org/rust-embedded/r0.svg?branch=master)](https://travis-ci.org/rust-embedded/r0)
[![crates.io](https://img.shields.io/crates/d/r0.svg)](https://crates.io/crates/r0)
[![crates.io](https://img.shields.io/crates/v/r0.svg)](https://crates.io/crates/r0)

Memory initialization code written in Rust.

This crate is for bare metal systems where there is no ELF loader or OS to take care of
initializing RAM for the program.

r0 is not meant to be used by user applications directly. Instead, it is most often used by
embedded runtime crates, like:

* [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt)
* [riscv-rt](https://github.com/rust-embedded/riscv-rt)
* [msp430-rt](https://github.com/rust-embedded/msp430-rt)

The r0 crate provides similar functionality to [crt0](https://en.wikipedia.org/wiki/Crt0) in the C
runtime.

This project is developed and maintained by the [Cortex-A, Cortex-M, Cortex-R, MSP430, and RISCV
teams][teams].

## [Documentation](https://docs.rs/r0)

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainers of this crate, the [Cortex-A, Cortex-M,
Cortex-R, MSP430, and RISCV teams][teams], promise to intervene to uphold that
code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[teams]: https://github.com/rust-embedded/wg#organization

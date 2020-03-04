# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v1.0.0] - 2020-03-03

### Breaking Changes

- Remove the init array APIs as they are not pulling their weight.
- Bound `zero_bss` and `init_data` by a new `Word` trait.

## [v0.2.2] - 2017-07-21

### Changed

- Optimized `zero_bss` and `init_data` for binary size. This change also results
  in faster routines on non Cortex-M architectures.

## [v0.2.1] - 2017-04-08

### Added

- `.init_array` / `.pre_init_array` support

## [v0.2.0] - 2017-01-22

### Changed

- [breaking-change] The signature of the `zero_bss` and `init_data` functions;
  the end `.bss`/`.data` pointer now must be `*mut T`. This makes it impossible
  for people to bind a `static`, i.e. not `static mut`, variable to the end
  of these sections, which could make the compiler, or other programmer, think
  that such variable is allocated in the `.rodata` or the `.text` section.

## v0.1.0 - 2016-10-03

### Added

- `init_data` and `zero_bss` functions.

[Unreleased]: https://github.com/rust-embedded/r0/compare/v1.0.0...HEAD
[v1.0.0]: https://github.com/rust-embedded/r0/compare/v0.2.2...v1.0.0
[v0.2.2]: https://github.com/rust-embedded/r0/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/r0/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/r0/compare/v0.1.0...v0.2.0

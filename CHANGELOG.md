# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2016-01-22

### Changed

- [breaking-change] The signature of the `zero_bss` and `init_data` functions;
  the end `.bss`/`.data` pointer now must be `*mut T`. This makes it impossible
  for people to bind a `static`, i.e. not `static mut`, variable to the end
  of these sections, which could make the compiler, or other programmer, think
  that such variable is allocated in the `.rodata` or the `.text` section.

## v0.1.0 - 2016-10-03

### Added

- `init_data` and `zero_bss` functions.

[Unreleased]: https://github.com/japaric/xargo/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/xargo/compare/v0.1.0...v0.2.0

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-10

### 🚀 Features

- **Async Support**: Added asynchronous implementation of the HS3003 driver using `embedded-hal-async`.
- **Improved Testing**: Added comprehensive unit tests for both sync and async I2C operations using `embedded-hal-mock`.

### 🐛 Bug Fixes

- **RP2350 (Pico 2) Compatibility**: Added required `IMAGE_DEF` (boot block) to async example to fix boot issues on RP2350.
- **I2C Reliability**: Reverted to single-byte I2C writes for measurement triggers to maintain compatibility with RP2350 hardware limitations.

### 📚 Documentation

- Fixed and improved doctests in `README.md` and library source.

## [0.1.1] - 2026-01-13

### 📚 Documentation

- Consolidated CI/CD documentation and improved README structure

### ⚙️ CI

- Fixed incorrect GitHub Action names and modernized workflows
- Set up `docs.rs` metadata for better build reliability
- Configured conditional release triggering using `[release]` tag

## [0.1.0] - Initial release

### Features

- Platform-agnostic driver for HS3003 temperature and humidity sensor
- `no_std` compatible using `embedded-hal` 1.0 traits
- Support for custom I2C addresses
- Temperature range: -40°C to +125°C
- Humidity range: 0% to 100% RH

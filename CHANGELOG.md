# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/melastmohican/hs3003-rs/compare/v0.1.0...v0.1.1) - 2026-01-14

### Ci

- make release-plz conditional on [release] and consolidate CI docs
- add docs.rs metadata and GitHub Pages workflow

### 📚 Documentation

- remove redundant CI section from README

## [0.1.0] - Initial release

### Features

- Platform-agnostic driver for HS3003 temperature and humidity sensor
- `no_std` compatible using `embedded-hal` 1.0 traits
- Support for custom I2C addresses
- Temperature range: -40°C to +125°C
- Humidity range: 0% to 100% RH



# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.3] - 2026-05-18

### Added
- Homebrew installation instructions in README
- Rust build caching via Swatinem/rust-cache in CI/CD pipeline
- Docker-based cross-compilation for ARM targets (rust-musl-cross container)

### Changed
- Cargo.toml edition updated from 2021 to 2024
- Improved package description in Cargo.toml for better clarity and discoverability
- ARM build process now uses Docker containers for better reproducibility and reliability
- Extended .gitignore with development-related entries (.idea/, tmp/, settings.local.json, .DS_Store)

### Fixed
- Package name for crates.io publication: now uses `luciferous-case-converter` instead of `case-converter`

## [1.1.2] - 2026-05-11

### Added
- Homebrew Formula automatic publication workflow

### Changed
- Improved product description in documentation

## [1.1.1] - 2026-05-09

### Added
- Third-party license generation in build process

### Changed
- Updated cargo-about installation command with `--locked` and `--features cli` flags
- Refactored CI/CD workflow for improved reliability

## [1.1.0] - 2026-05-08

### Added
- Comprehensive case conversion test suite

### Fixed
- Upper Snake Case to PascalCase conversion bug

## [1.0.0] - 2025-05-07

### Added
- Initial release: Case Converter CLI tool
- Support for multiple case styles:
  - snake_case
  - UPPER_SNAKE_CASE
  - camelCase
  - PascalCase
  - kebab-case
- Command-line argument parsing using clap
- Standard input support for pipe usage
- Cross-platform compatibility (Windows, macOS, Linux)
- Homebrew installation support
- crates.io publication

[Unreleased]: https://github.com/sinofseven/case-converter/compare/v1.1.3...HEAD
[1.1.3]: https://github.com/sinofseven/case-converter/compare/v1.1.2...v1.1.3
[1.1.2]: https://github.com/sinofseven/case-converter/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/sinofseven/case-converter/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/sinofseven/case-converter/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/sinofseven/case-converter/releases/tag/v1.0.0

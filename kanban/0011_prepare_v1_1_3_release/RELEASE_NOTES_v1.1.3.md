# Release Notes v1.1.3 - Infrastructure & CI/CD Improvements

## Overview

v1.1.3 focuses on infrastructure improvements and CI/CD optimization. Key enhancements include Homebrew installation support, build caching for faster CI/CD pipelines, and more reliable ARM cross-compilation.

## What's New

### ✨ New Features
- **Homebrew Installation**: Users can now install case-converter via Homebrew: `brew install sinofseven/luciferous-tap/case-converter`
- **Faster CI/CD Builds**: Rust build caching (via Swatinem/rust-cache) significantly reduces build times
- **Improved ARM Support**: Docker-based cross-compilation for ARM targets ensures consistent, reproducible builds

### 🔄 Changes
- **Edition Update**: Cargo.toml upgraded to Rust 2024 edition
- **Better Documentation**: Package description refined for improved clarity and discoverability
- **Build Reliability**: ARM build process migrated from direct cross-compilation to Docker containers for reproducibility
- **Development Environment**: Extended .gitignore with editor/IDE and temporary files

### 🐛 Fixes
- **crates.io Publication**: Package correctly published as `luciferous-case-converter` (resolves naming conflict)

## Installation

### Homebrew (New!)
```bash
brew install sinofseven/luciferous-tap/case-converter
```

### Cargo
```bash
cargo install luciferous-case-converter
```

### From Source
```bash
git clone https://github.com/sinofseven/case-converter
cd case-converter
cargo build --release
./target/release/case-converter --help
```

## Complete Changelog

See [CHANGELOG.md](https://github.com/sinofseven/case-converter/blob/v1.1.3/CHANGELOG.md) for a detailed list of all changes.

## Downloads

- [macOS (Intel)](https://github.com/sinofseven/case-converter/releases/download/v1.1.3/case-converter-macos-x86_64)
- [macOS (Apple Silicon)](https://github.com/sinofseven/case-converter/releases/download/v1.1.3/case-converter-macos-aarch64)
- [Linux x86_64](https://github.com/sinofseven/case-converter/releases/download/v1.1.3/case-converter-linux-x86_64)
- [Linux ARM64](https://github.com/sinofseven/case-converter/releases/download/v1.1.3/case-converter-linux-aarch64)
- [Linux ARM](https://github.com/sinofseven/case-converter/releases/download/v1.1.3/case-converter-linux-arm)
- [Windows x86_64](https://github.com/sinofseven/case-converter/releases/download/v1.1.3/case-converter-windows-x86_64.exe)

---

**Thank you for using case-converter!** For questions or issues, please visit our [GitHub repository](https://github.com/sinofseven/case-converter).

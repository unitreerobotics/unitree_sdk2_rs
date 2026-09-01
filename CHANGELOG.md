# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- English-only code comments; bilingual (English + 简体中文) documentation
  (`README.md` / `README_cn.md`, `docs/*.md` / `docs/*_cn.md`)
- MIT license file (`LICENSE`)
- Community files: `CONTRIBUTING.md`, `CHANGELOG.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`
- CI workflow (`.github/workflows/ci.yml`): fmt, clippy, build, test, package check
- Cargo metadata: `homepage`, `documentation`, `rust-version`, docs.rs config

### Changed

- Fixed `repository` metadata to point to this crate's repository
- `license` metadata set to `MIT` to match the license file

## [0.1.4] - 2026-08-24

- Fixed docs.rs build: skip C++ compilation when `DOCS_RS=1` (docs.rs sandbox has no network/SDK)

## [0.1.3] - 2026-08-24

- Maintenance release (documentation)

## [0.1.2] - 2026-08-24

- Maintenance release (documentation)

## [0.1.1] - 2026-08-24

- Maintenance release (documentation)

## [0.1.0] - 2026-07-31

- Initial release: msg-driven DDS type generation, CDR serialization,
  publish/subscribe and RPC bindings for the Unitree SDK2

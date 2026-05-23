# Changelog

All notable changes to obscura.deck will be documented in this file.

The format is based on Keep a Changelog, and this project intends to use semantic versioning once release artifacts begin.

## [Unreleased]

### Added

- M0 workspace foundation with pnpm and Cargo workspaces.
- Tauri 2 + React + TypeScript desktop shell scaffold.
- Cross-platform CI skeleton for Rust, frontend, and Tauri smoke checks.
- Documentation foundation for install, development, build, test, CI, and security model.
- Minimal React and Rust smoke tests for the bootstrapped shell.
- M0 milestone closure record.

### Security

- Documented the simulation-only boundary.
- Documented that Discord user-token automation, scraping, and self-bot behavior are out of scope.
- Documented that any future Discord integration must use bot tokens only and remain disabled by default.

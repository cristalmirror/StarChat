# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
-

### Changed
-

### Fixed
-
## [0.0.2] - 2026-07-05

### Added
- WebSocket endpoint (`/ws`) using axum's `WebSocketUpgrade` extractor
- `models.rs` module with `ClientEvent` and `ServerEvent` enums (serde-tagged JSON) to define the client-server protocol
- `client_connection_manager.rs` module to isolate all WebSocket connection handling, separate from `main.rs`
- Structured message handling: incoming client text is parsed as `ClientEvent` JSON and answered with a `ServerEvent` (`Ack` or `Error`)
- `LICENSE-server-AGPL-3.0.txt` and `LICENSE-client-MIT.txt` for dual licensing (server vs. client components)

### Changed
- Refactored `main.rs` to only handle routing and server startup, delegating WebSocket logic to `client_connection_manager.rs`
- Replaced plain-text WebSocket echo with JSON-based event handling

### Fixed
- Removed `target/` from git tracking (was committed before `.gitignore` existed) and corrected `.gitignore` path to match the actual project location (`server/` subfolder)
- Resolved duplicate `axum` versions in the dependency tree (`0.6.20` pulled in transitively by `tonic`, conflicting with the direct `0.8.9` dependency) by removing `tonic` and `prost` until the server-to-server communication phase
## [0.0.1] - 2026-07-05

### Added
- Initial project setup (`cargo new`)
- Base dependencies: tokio, axum, sqlx, redis, tonic, serde, uuid, chrono, tracing
- Minimal axum server with `/health` endpoint
- `.gitignore` for Rust and Emacs

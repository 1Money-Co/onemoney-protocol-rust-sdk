# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the OneMoney Rust SDK - a Rust library/SDK project that is currently in its initial setup phase.

## Build and Development Commands

### Common Rust/Cargo Commands
- **Build**: `cargo build` (debug mode) or `cargo build --release` (optimized)
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Single test**: `cargo test test_name` or `cargo test -- --exact test_name`
- **Check compilation**: `cargo check` (faster than build, doesn't produce binaries)
- **Format code**: `cargo fmt`
- **Lint**: `cargo clippy`
- **Documentation**: `cargo doc --open`
- **Clean build artifacts**: `cargo clean`

### Development Workflow
1. Before committing changes, always run:
   - `cargo fmt` to ensure consistent formatting
   - `cargo clippy` to catch common mistakes and improve code quality
   - `cargo test` to ensure all tests pass

## Project Structure

This is a Rust project using Cargo as the build system. The codebase follows standard Rust project conventions:

- `Cargo.toml` - Project manifest with dependencies and metadata
- `src/` - Source code directory
  - `main.rs` - Entry point (currently a simple "Hello, world!" program)
- `target/` - Build output directory (git-ignored)

## Architecture Notes

As this SDK develops, consider organizing the code into:
- `src/lib.rs` - Library root for the SDK's public API
- `src/client/` - API client implementation
- `src/models/` - Data structures and types
- `src/error.rs` - Error handling types
- `examples/` - Usage examples
- `tests/` - Integration tests

The project uses Rust edition 2024 and currently has no external dependencies.
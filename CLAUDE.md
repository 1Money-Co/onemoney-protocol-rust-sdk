# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the OneMoney Rust SDK - a Rust library/SDK project for interacting with the L1 blockchain REST API. It is currently in its initial setup phase.

## Related Projects

### L1 Blockchain Project
- **Location**: `/Users/frank/workspace/l1client`
- **Purpose**: Main L1 blockchain implementation with REST API endpoints
- **Language**: Rust
- **Usage**: Reference this codebase for understanding API endpoints, data structures, and blockchain functionality

### Go SDK (Reference Implementation)
- **Location**: `/Users/frank/workspace/1money-go-sdk`
- **Purpose**: Go language SDK for L1 blockchain REST API
- **Language**: Go
- **Usage**: Use as reference for SDK architecture, API client patterns, and implementation approach

## Build and Development Commands

### Common Rust/Cargo Commands
- **Build**: `cargo build` (debug mode) or `cargo build --release` (optimized)
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Single test**: `cargo test test_name` or `cargo test -- --exact test_name`
- **Check compilation**: `cargo check` (faster than build, doesn't produce binaries)
- **Format code**: `cargo fmt`
- **Lint**: `cargo clippy`
- **Comprehensive lint**: `cargo clippy --all-targets --workspace --lib --examples --all-features --tests --benches -- -D warnings`
- **Documentation**: `cargo doc --open`
- **Clean build artifacts**: `cargo clean`

### Development Workflow
1. Install pre-commit hooks: `pre-commit install`
2. Before committing changes, pre-commit hooks will automatically run:
   - Code formatting checks (`cargo fmt`)
   - Compilation checks (`cargo check`)
   - Linting (`cargo clippy`)
   - Tests (`cargo test`)
3. Manual commands (also run by pre-commit):
   - `cargo fmt` to ensure consistent formatting
   - `cargo clippy` to catch common mistakes and improve code quality
   - `cargo clippy --all-targets --workspace --lib --examples --all-features --tests --benches -- -D warnings` for comprehensive linting with strict warnings
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

## Development Guidelines

When implementing the Rust SDK:
1. **API Reference**: Check `/Users/frank/workspace/l1client` for the actual REST API implementation and data structures
2. **Pattern Reference**: Use `/Users/frank/workspace/1money-go-sdk` as a reference for SDK architecture and client patterns
3. **Consistency**: Maintain similar API surface and naming conventions as the Go SDK where possible
4. **Rust Idioms**: Implement using Rust best practices (Result types, ownership patterns, etc.)

### Code Quality Rules

1. **Avoid `Option` when possible** - Prefer explicit types and clear APIs over optional parameters
2. **Never use `unwrap()` or `expect()`** - Always handle errors gracefully with proper error propagation using `?` operator or explicit error handling
3. **Use `thiserror` for error definitions** - Always use `thiserror` to define structured error types, do not use `anyhow` for library code

4. **Import Organization and Path Cleanliness** - Always use explicit `use` statements and avoid inline paths for maximum code readability and maintainability:

   **4.1 Import Types and Constants Explicitly**
   - **Never use inline paths**: Avoid `crate::client::api_path(crate::client::endpoints::states::LATEST)`
   - **Import constants and functions**: Import constants, functions, and types at the file top with `use` statements
   - **Use short, clean references**: After importing, reference items directly by their name

   **4.2 Import Organization Rules**
   - **Import commonly used items**: Import types like `std::error::Error`, traits like `std::fmt::Display`, and constants
   - **Organize imports logically**: Group in order: standard library, external crates, local crates, then local modules
   - **Import function parameters**: For function signatures, import all types so signatures are clean and readable

   **4.3 Practical Examples**

   **Good Example - Clean Method Implementation:**
   ```rust
   use crate::client::api_path;
   use crate::client::endpoints::states::LATEST_EPOCH_CHECKPOINT;
   use crate::{LatestStateResponse, Result};

   pub async fn get_latest_epoch_checkpoint(&self) -> Result<LatestStateResponse> {
       self.get(&api_path(LATEST_EPOCH_CHECKPOINT)).await
   }
   ```

   **Bad Example - Inline Paths:**
   ```rust
   pub async fn get_latest_epoch_checkpoint(&self) -> Result<LatestStateResponse> {
       self.get(&crate::client::api_path(
           crate::client::endpoints::states::LATEST_EPOCH_CHECKPOINT,
       ))
       .await
   }
   ```

   **Good Example - Function Signatures:**
   ```rust
   use std::error::Error;
   use crate::{OneMoneyAddress, TokenAmount, Result};

   async fn create_payment(address: OneMoneyAddress, amount: TokenAmount) -> Result<(), Box<dyn Error>> {
       // implementation
   }
   ```

   **Bad Example - Inline Path Function Signatures:**
   ```rust
   async fn create_payment(
       address: crate::OneMoneyAddress,
       amount: crate::TokenAmount
   ) -> Result<(), Box<dyn std::error::Error>> {
       // implementation
   }
   ```

5. **No Emojis Anywhere** - Completely avoid using emojis and Unicode symbols throughout the entire project:
   - **Zero Tolerance**: No emojis are allowed anywhere in the codebase - production code, examples, documentation, comments, or any text
   - **Library Code**: Production library code must never contain emojis in log messages, error strings, or any user-facing text
   - **Examples**: Example code must also be professional and emoji-free for consistency and professionalism
   - **Error Messages**: Keep error messages professional and machine-readable without emojis or decorative symbols
   - **Logging**: Use plain text for all logging statements to ensure compatibility with log parsers and monitoring systems
   - **API Responses**: Never include emojis in API responses, error codes, or structured data
   - **Documentation**: Use clear, professional language without decorative Unicode symbols

   **Good Example:**
   ```rust
   error!("Failed to connect to database: {}", e);
   return Err(Error::connection_failed("Database connection timeout"));
   log::info!("Transaction completed successfully");
   ```

   **Bad Example:**
   ```rust
   error!("❌ Failed to connect to database: {}", e);
   return Err(Error::connection_failed("🚫 Database connection timeout"));
   log::info!("✅ Transaction completed successfully");
   ```

6. **No Tracing/Logging Dependencies** - Keep logging simple and minimal:
   - **Prohibited dependencies**: Never use `tracing`, `tracing-subscriber`, `log`, or similar logging crates
   - **Use println! only**: All logging must use standard `println!` macro for simplicity
   - **Minimal logging**: Keep log output concise and focused on essential information only
   - **No debug/trace logs**: Avoid verbose debug or trace-level logging in production code
   - **Clean examples**: Example files should have minimal, focused output without excessive explanatory text

   **Good Example:**
   ```rust
   println!("Transaction sent: {}", tx_hash);
   println!("✓ Payment completed successfully");
   ```

   **Bad Example:**
   ```rust
   // Don't use these:
   tracing::info!("Starting transaction processing...");
   log::debug!("Processing transaction with details: {:?}", tx);

   // Don't create verbose output:
   println!("========================================");
   println!("Transaction Processing Complete!");
   println!("========================================");
   println!("Key Takeaways:");
   println!("• Always validate inputs");
   println!("• Handle errors gracefully");
   println!("========================================");
   ```

7. **Mandatory Code Formatting** - AI must always format code after making changes:
   - **Always run `cargo fmt`**: After any code modification, AI must run `cargo fmt` to ensure consistent formatting
   - **Before completion**: Never consider a task complete without running `cargo fmt`
   - **Zero tolerance for unformatted code**: All code must be properly formatted according to project standards
   - **Automatic formatting**: Use `cargo fmt` to automatically format all Rust code files
   - **Consistency**: This ensures all code follows the same formatting rules across the entire project

   **Workflow Example:**
   ```bash
   # 1. Make code changes
   # 2. Always run formatting
   cargo fmt
   # 3. Verify changes are properly formatted
   # 4. Complete the task
   ```

   **Important Notes:**
   - Run `cargo fmt` even for small changes like single-line modifications
   - This rule applies to all Rust source files (.rs) including lib, examples, tests, and benches
   - Formatting should be run before any quality checks or compilation
   - Never skip formatting due to "minor changes" - consistency is paramount


### Mandatory Quality Checks

**IMPORTANT**: After making any code changes, AI must always run the following commands in sequence and ensure ALL pass before completing the task:

1. **Format code**: `cargo fmt`
   - Ensures consistent code formatting according to project standards
   - Must pass without any changes needed

2. **Comprehensive lint check**: `cargo clippy --all-targets --workspace --lib --examples --all-features --tests --benches -- -D warnings`
   - Catches common mistakes and enforces best practices
   - Must pass with zero warnings or errors
   - All clippy suggestions must be addressed

3. **Full test suite**: `cargo test`
   - Ensures all unit tests, integration tests, and doc tests pass
   - Must have 100% test success rate

4. **Compilation check**: `cargo check`
   - Verifies all code compiles successfully
   - Must pass without compilation errors

5. **Pre-commit file fixes**: Run the following pre-commit hooks to ensure file consistency:
   - `pre-commit run end-of-file-fixer --all-files` - Ensures files end with a newline
   - `pre-commit run mixed-line-ending --all-files` - Fixes mixed line endings (Unix/Windows)
   - `pre-commit run trailing-whitespace --all-files` - Removes trailing whitespace

**These checks replicate the pre-commit pipeline and ensure code quality. Do not consider a task complete until all five check categories pass successfully.**

## CI/CD Setup

### GitHub Actions
- **Lint and Test**: Runs on PR/push to main, includes formatting, linting, testing, and documentation build
- **Release**: Automatically creates releases with changelog generation when version tags are pushed
- **GitHub Actions Lint**: Validates workflow files when `.github/` changes

### Pre-commit Configuration
The project uses pre-commit hooks (`.pre-commit-config.yaml`) that run:
- Standard checks (trailing whitespace, YAML validation, etc.)
- Rust-specific checks (formatting, compilation, linting, testing)
- Install with: `pip install pre-commit && pre-commit install`

### Configuration Files
- `cliff.toml` - Changelog generation configuration
- `rustfmt.toml` - Code formatting rules
- `.editorconfig` - Editor configuration for consistent styling

//! Client core functionality and configuration.

pub mod builder;
pub mod config;
pub mod hooks;
pub mod http;

// Re-export public interfaces
pub use builder::ClientBuilder;
pub use config::{Network, api_path, endpoints};
pub use hooks::{ConsoleLogger, Hook, LogLevel, Logger, LoggingHook};
pub use http::Client;

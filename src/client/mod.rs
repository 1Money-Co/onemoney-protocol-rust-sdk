//! Client core functionality and configuration.

pub mod builder;
pub mod config;
pub mod hooks;
pub mod http;

// Re-export public interfaces
pub use builder::ClientBuilder;
pub use config::{api_path, endpoints, Network};
pub use hooks::{ConsoleLogger, Hook, LogLevel, Logger, LoggingHook};
pub use http::Client;

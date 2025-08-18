//! Hook and logging system for request/response middleware.

/// Hook trait for request/response middleware.
pub trait Hook: Send + Sync {
    /// Called before sending a request.
    fn before_request(&self, method: &str, url: &str, body: Option<&str>);

    /// Called after receiving a response.
    fn after_response(&self, method: &str, url: &str, status: u16, body: Option<&str>);
}

/// Logger trait for pluggable logging.
pub trait Logger: Send + Sync {
    /// Log a message.
    fn log(&self, level: LogLevel, message: &str);
}

/// Log levels.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Simple console logger implementation.
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        match level {
            LogLevel::Trace => {} // Skip trace messages
            LogLevel::Debug => {} // Skip debug messages
            LogLevel::Info => println!("[INFO] {}", message),
            LogLevel::Warn => println!("[WARN] {}", message),
            LogLevel::Error => println!("[ERROR] {}", message),
        }
    }
}

/// Simple request/response logging hook.
pub struct LoggingHook {
    logger: Box<dyn Logger>,
}

impl LoggingHook {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl Hook for LoggingHook {
    fn before_request(&self, method: &str, url: &str, body: Option<&str>) {
        if let Some(body) = body {
            self.logger.log(
                LogLevel::Debug,
                &format!("-> {} {} with body: {}", method, url, body),
            );
        } else {
            self.logger
                .log(LogLevel::Debug, &format!("-> {} {}", method, url));
        }
    }

    fn after_response(&self, method: &str, url: &str, status: u16, body: Option<&str>) {
        if let Some(body) = body {
            self.logger.log(
                LogLevel::Debug,
                &format!("<- {} {} [{}] body: {}", method, url, status, body),
            );
        } else {
            self.logger.log(
                LogLevel::Debug,
                &format!("<- {} {} [{}]", method, url, status),
            );
        }
    }
}

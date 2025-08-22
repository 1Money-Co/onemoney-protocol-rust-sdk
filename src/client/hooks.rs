//! Hook and logging system for request/response middleware.

use std::str;

/// Type alias for redaction callback function.
/// Takes the original body and returns a redacted version.
pub type RedactionCallback = Box<dyn Fn(&str) -> String + Send + Sync>;

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
    redaction_callback: Option<RedactionCallback>,
}

impl LoggingHook {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self {
            logger,
            redaction_callback: None,
        }
    }

    /// Create a new LoggingHook with a redaction callback.
    pub fn with_redaction(logger: Box<dyn Logger>, redaction_callback: RedactionCallback) -> Self {
        Self {
            logger,
            redaction_callback: Some(redaction_callback),
        }
    }

    /// Create a safe preview of the body content for logging.
    /// Applies redaction if configured, then truncates to 100 characters.
    fn create_safe_preview(&self, body: &str) -> String {
        if body.is_empty() {
            return String::new();
        }

        // Apply redaction callback if provided
        let processed_body = if let Some(ref redactor) = self.redaction_callback {
            redactor(body)
        } else {
            body.to_string()
        };

        // Create safe preview - first 100 characters with ellipsis if truncated
        // Use character-aware truncation to avoid panics on UTF-8 boundaries
        if processed_body.chars().count() <= 100 {
            processed_body
        } else {
            format!(
                "{}...",
                processed_body.chars().take(100).collect::<String>()
            )
        }
    }
}

impl Hook for LoggingHook {
    fn before_request(&self, method: &str, url: &str, body: Option<&str>) {
        if let Some(body) = body {
            let safe_preview = self.create_safe_preview(body);
            if safe_preview.is_empty() {
                self.logger.log(
                    LogLevel::Debug,
                    &format!("-> {} {} with body: <empty>", method, url),
                );
            } else {
                self.logger.log(
                    LogLevel::Debug,
                    &format!("-> {} {} with body: {}", method, url, safe_preview),
                );
            }
        } else {
            self.logger
                .log(LogLevel::Debug, &format!("-> {} {}", method, url));
        }
    }

    fn after_response(&self, method: &str, url: &str, status: u16, body: Option<&str>) {
        if let Some(body) = body {
            let safe_preview = self.create_safe_preview(body);
            if safe_preview.is_empty() {
                self.logger.log(
                    LogLevel::Debug,
                    &format!("<- {} {} [{}] body: <empty>", method, url, status),
                );
            } else {
                self.logger.log(
                    LogLevel::Debug,
                    &format!("<- {} {} [{}] body: {}", method, url, status, safe_preview),
                );
            }
        } else {
            self.logger.log(
                LogLevel::Debug,
                &format!("<- {} {} [{}]", method, url, status),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct TestLogger {
        messages: Arc<Mutex<Vec<(LogLevel, String)>>>,
    }

    impl TestLogger {
        fn new() -> Self {
            Self {
                messages: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_messages(&self) -> Vec<(LogLevel, String)> {
            self.messages
                .lock()
                .expect("Failed to lock messages mutex")
                .clone()
        }
    }

    impl Logger for TestLogger {
        fn log(&self, level: LogLevel, message: &str) {
            self.messages
                .lock()
                .expect("Failed to lock messages mutex")
                .push((level, message.to_string()));
        }
    }

    impl Logger for Arc<TestLogger> {
        fn log(&self, level: LogLevel, message: &str) {
            self.as_ref().log(level, message);
        }
    }

    #[test]
    fn test_safe_preview_short_body() {
        let logger = Box::new(TestLogger::new());
        let hook = LoggingHook::new(logger);

        let short_body = "short message";
        let preview = hook.create_safe_preview(short_body);
        assert_eq!(preview, "short message");
    }

    #[test]
    fn test_safe_preview_long_body() {
        let logger = Box::new(TestLogger::new());
        let hook = LoggingHook::new(logger);

        let long_body = "a".repeat(150);
        let preview = hook.create_safe_preview(&long_body);
        assert_eq!(preview.len(), 103); // 100 chars + "..."
        assert!(preview.ends_with("..."));
        assert_eq!(&preview[..100], &"a".repeat(100));
    }

    #[test]
    fn test_safe_preview_empty_body() {
        let logger = Box::new(TestLogger::new());
        let hook = LoggingHook::new(logger);

        let preview = hook.create_safe_preview("");
        assert_eq!(preview, "");
    }

    #[test]
    fn test_redaction_callback() {
        let logger = Box::new(TestLogger::new());
        let redactor = Box::new(|body: &str| {
            body.replace("secret123", "***REDACTED***")
                .replace("token123", "***REDACTED***")
                .replace("password", "***REDACTED***")
                .replace("Authorization", "***REDACTED***")
        });
        let hook = LoggingHook::with_redaction(logger, redactor);

        let sensitive_body =
            r#"{"username": "john", "password": "secret123", "Authorization": "Bearer token123"}"#;
        let preview = hook.create_safe_preview(sensitive_body);

        assert!(!preview.contains("secret123"));
        assert!(!preview.contains("token123"));
        assert!(preview.contains("***REDACTED***"));
    }

    #[test]
    fn test_before_request_with_empty_body() {
        let logger = Arc::new(TestLogger::new());
        let hook = LoggingHook::new(Box::new(logger.clone()));

        hook.before_request("GET", "https://api.example.com", Some(""));

        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert!(messages[0].1.contains("<empty>"));
    }

    #[test]
    fn test_before_request_with_long_body() {
        let logger = Arc::new(TestLogger::new());
        let hook = LoggingHook::new(Box::new(logger.clone()));

        let long_body = "x".repeat(150);
        hook.before_request("POST", "https://api.example.com", Some(&long_body));

        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert!(messages[0].1.contains("..."));
        assert!(messages[0].1.len() < long_body.len() + 50); // Much shorter than original
    }

    #[test]
    fn test_after_response_with_redaction() {
        let logger = Arc::new(TestLogger::new());
        let redactor = Box::new(|body: &str| {
            body.replace("0x123456789abcdef", "***REDACTED***")
                .replace("private_key", "***REDACTED***")
        });
        let hook = LoggingHook::with_redaction(Box::new(logger.clone()), redactor);

        let response_body = r#"{"success": true, "private_key": "0x123456789abcdef"}"#;
        hook.after_response("POST", "https://api.example.com", 200, Some(response_body));

        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert!(!messages[0].1.contains("0x123456789abcdef"));
        assert!(messages[0].1.contains("***REDACTED***"));
    }

    #[test]
    fn test_safe_preview_with_multibyte_characters() {
        let logger = Box::new(TestLogger::new());
        let hook = LoggingHook::new(logger);

        // Test with multi-byte UTF-8 characters
        let multibyte_body = "Hello world with accents: café résumé naïve".repeat(5); // Creates ~215 characters
        let preview = hook.create_safe_preview(&multibyte_body);

        // Verify no panic occurred and truncation is correct
        assert!(preview.ends_with("..."));
        assert!(preview.chars().count() <= 103); // 100 chars + "..."

        // Verify the preview contains valid UTF-8 and doesn't cut multi-byte chars
        assert!(preview.is_ascii() || str::from_utf8(preview.as_bytes()).is_ok());

        // Test exactly 100 characters (no truncation needed)
        let exactly_100_chars = "a".repeat(97) + "xyz"; // 97 + 3 = 100 chars exactly
        let preview_exact = hook.create_safe_preview(&exactly_100_chars);
        assert_eq!(preview_exact, exactly_100_chars);
        assert!(!preview_exact.contains("..."));

        // Test short multi-byte string (no truncation)
        let short_multibyte = "Hello world!";
        let preview_short = hook.create_safe_preview(short_multibyte);
        assert_eq!(preview_short, short_multibyte);
        assert!(!preview_short.contains("..."));
    }
}

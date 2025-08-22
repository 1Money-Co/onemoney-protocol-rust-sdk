//! Comprehensive tests for HTTP error classification in Client::classify_error.

use onemoney_protocol::error::ErrorResponse;
use onemoney_protocol::{ClientBuilder, Error, Network};
use std::time::Duration;

/// Test case structure for table-driven testing.
#[derive(Debug, Clone)]
struct ErrorClassificationTestCase {
    /// Test case name for clear identification.
    name: &'static str,
    /// HTTP status code to test.
    status_code: u16,
    /// Error code from the API response.
    error_code: &'static str,
    /// Message from the API response.
    message: &'static str,
    /// Expected error variant and specific assertions.
    expected_assertion: fn(&Error),
}

/// Helper function to create a test client.
fn create_test_client() -> onemoney_protocol::Client {
    ClientBuilder::new()
        .network(Network::Local)
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Test client creation should not fail")
}

/// Helper function to simulate an error response and test classification.
fn test_error_classification(test_case: &ErrorClassificationTestCase) {
    let client = create_test_client();

    // Create an ErrorResponse for structured error testing
    let error_response = ErrorResponse {
        error_code: test_case.error_code.to_string(),
        message: test_case.message.to_string(),
    };

    // Serialize to JSON to simulate API response
    let response_body =
        serde_json::to_string(&error_response).expect("Test ErrorResponse should serialize");

    // Test the classify_error method via handle_error_response
    let result = client.test_handle_error_response(test_case.status_code, &response_body);

    // Apply the expected assertion
    (test_case.expected_assertion)(&result);
}

/// Helper function to test fallback error classification with non-JSON body.
fn test_fallback_error_classification(
    status_code: u16,
    body: &str,
    expected_assertion: fn(&Error),
) {
    let client = create_test_client();
    let result = client.test_handle_error_response(status_code, body);
    expected_assertion(&result);
}

#[test]
fn test_validation_parameter_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "validation_username",
            status_code: 400,
            error_code: "validation_username",
            message: "Username must be at least 3 characters",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "username");
                    assert_eq!(message, "Username must be at least 3 characters");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_email",
            status_code: 400,
            error_code: "validation_email",
            message: "Invalid email format",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "email");
                    assert_eq!(message, "Invalid email format");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_amount",
            status_code: 400,
            error_code: "validation_amount",
            message: "Amount must be positive",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "amount");
                    assert_eq!(message, "Amount must be positive");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_address",
            status_code: 400,
            error_code: "validation_address",
            message: "Invalid address format",
            expected_assertion: |error| match error {
                Error::InvalidParameter { parameter, message } => {
                    assert_eq!(parameter, "address");
                    assert_eq!(message, "Invalid address format");
                }
                _ => panic!("Expected InvalidParameter error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_empty_suffix",
            status_code: 400,
            error_code: "validation_",
            message: "Parameter validation failed",
            expected_assertion: |error| {
                match error {
                    Error::InvalidParameter { parameter, message } => {
                        assert_eq!(parameter, ""); // strip_prefix returns empty string, not "unknown"
                        assert_eq!(message, "Parameter validation failed");
                    }
                    _ => panic!(
                        "Expected InvalidParameter error with empty param, got: {:?}",
                        error
                    ),
                }
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_authentication_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "unauthorized_invalid_token",
            status_code: 401,
            error_code: "invalid_token",
            message: "The provided token is invalid",
            expected_assertion: |error| match error {
                Error::Authentication(message) => {
                    assert_eq!(message, "The provided token is invalid");
                }
                _ => panic!("Expected Authentication error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "unauthorized_expired_token",
            status_code: 401,
            error_code: "expired_token",
            message: "Token has expired",
            expected_assertion: |error| match error {
                Error::Authentication(message) => {
                    assert_eq!(message, "Token has expired");
                }
                _ => panic!("Expected Authentication error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "unauthorized_missing_auth",
            status_code: 401,
            error_code: "missing_auth",
            message: "Authentication required",
            expected_assertion: |error| match error {
                Error::Authentication(message) => {
                    assert_eq!(message, "Authentication required");
                }
                _ => panic!("Expected Authentication error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_authorization_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "forbidden_insufficient_permissions",
            status_code: 403,
            error_code: "insufficient_permissions",
            message: "You do not have permission to access this resource",
            expected_assertion: |error| match error {
                Error::Authorization(message) => {
                    assert_eq!(
                        message,
                        "You do not have permission to access this resource"
                    );
                }
                _ => panic!("Expected Authorization error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "forbidden_access_denied",
            status_code: 403,
            error_code: "access_denied",
            message: "Access denied for this operation",
            expected_assertion: |error| match error {
                Error::Authorization(message) => {
                    assert_eq!(message, "Access denied for this operation");
                }
                _ => panic!("Expected Authorization error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "forbidden_account_disabled",
            status_code: 403,
            error_code: "account_disabled",
            message: "Your account has been disabled",
            expected_assertion: |error| match error {
                Error::Authorization(message) => {
                    assert_eq!(message, "Your account has been disabled");
                }
                _ => panic!("Expected Authorization error, got: {:?}", error),
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_resource_not_found_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "resource_transaction_not_found",
            status_code: 404,
            error_code: "resource_transaction",
            message: "Transaction with hash 0x123 not found",
            expected_assertion: |error| match error {
                Error::ResourceNotFound {
                    resource_type,
                    identifier,
                } => {
                    assert_eq!(resource_type, "transaction");
                    assert_eq!(identifier, "Transaction with hash 0x123 not found");
                }
                _ => panic!("Expected ResourceNotFound error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "resource_account_not_found",
            status_code: 404,
            error_code: "resource_account",
            message: "Account not found",
            expected_assertion: |error| match error {
                Error::ResourceNotFound {
                    resource_type,
                    identifier,
                } => {
                    assert_eq!(resource_type, "account");
                    assert_eq!(identifier, "Account not found");
                }
                _ => panic!("Expected ResourceNotFound error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "resource_empty_suffix",
            status_code: 404,
            error_code: "resource_",
            message: "Resource not found",
            expected_assertion: |error| {
                match error {
                    Error::ResourceNotFound {
                        resource_type,
                        identifier,
                    } => {
                        assert_eq!(resource_type, ""); // strip_prefix returns empty string, not "unknown"
                        assert_eq!(identifier, "Resource not found");
                    }
                    _ => panic!(
                        "Expected ResourceNotFound error with empty type, got: {:?}",
                        error
                    ),
                }
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_timeout_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "request_timeout_exact_match",
            status_code: 408,
            error_code: "request_timeout",
            message: "Request timed out after 30 seconds",
            expected_assertion: |error| match error {
                Error::RequestTimeout {
                    endpoint,
                    timeout_ms,
                } => {
                    assert_eq!(endpoint, "Request timed out after 30 seconds");
                    assert_eq!(*timeout_ms, 0);
                }
                _ => panic!("Expected RequestTimeout error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "timeout_gateway_timeout",
            status_code: 408,
            error_code: "gateway_timeout",
            message: "Gateway timeout occurred",
            expected_assertion: |error| {
                // This should NOT match the specific timeout pattern and fall through to API error
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 408);
                        assert_eq!(error_code, "gateway_timeout");
                        assert_eq!(message, "Gateway timeout occurred");
                    }
                    _ => panic!(
                        "Expected Api error for non-matching timeout, got: {:?}",
                        error
                    ),
                }
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_business_logic_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "business_transfer_failed",
            status_code: 422,
            error_code: "business_transfer",
            message: "Insufficient balance for transfer",
            expected_assertion: |error| match error {
                Error::BusinessLogic { operation, reason } => {
                    assert_eq!(operation, "transfer");
                    assert_eq!(reason, "Insufficient balance for transfer");
                }
                _ => panic!("Expected BusinessLogic error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "business_mint_failed",
            status_code: 422,
            error_code: "business_mint",
            message: "Cannot mint to blacklisted address",
            expected_assertion: |error| match error {
                Error::BusinessLogic { operation, reason } => {
                    assert_eq!(operation, "mint");
                    assert_eq!(reason, "Cannot mint to blacklisted address");
                }
                _ => panic!("Expected BusinessLogic error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "business_empty_suffix",
            status_code: 422,
            error_code: "business_",
            message: "Business rule violation",
            expected_assertion: |error| {
                match error {
                    Error::BusinessLogic { operation, reason } => {
                        assert_eq!(operation, ""); // strip_prefix returns empty string, not "unknown"
                        assert_eq!(reason, "Business rule violation");
                    }
                    _ => panic!(
                        "Expected BusinessLogic error with empty operation, got: {:?}",
                        error
                    ),
                }
            },
        },
        ErrorClassificationTestCase {
            name: "unprocessable_entity_non_business",
            status_code: 422,
            error_code: "validation_failed",
            message: "Validation failed",
            expected_assertion: |error| {
                // This should NOT match the business_ pattern and fall through to API error
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 422);
                        assert_eq!(error_code, "validation_failed");
                        assert_eq!(message, "Validation failed");
                    }
                    _ => panic!("Expected Api error for non-business 422, got: {:?}", error),
                }
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_rate_limit_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "rate_limit_exceeded_exact_match",
            status_code: 429,
            error_code: "rate_limit_exceeded",
            message: "Too many requests",
            expected_assertion: |error| match error {
                Error::RateLimitExceeded {
                    retry_after_seconds,
                } => {
                    assert_eq!(*retry_after_seconds, None);
                }
                _ => panic!("Expected RateLimitExceeded error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "rate_limit_different_code",
            status_code: 429,
            error_code: "too_many_requests",
            message: "Rate limit hit",
            expected_assertion: |error| {
                // This should NOT match the exact "rate_limit_exceeded" pattern
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 429);
                        assert_eq!(error_code, "too_many_requests");
                        assert_eq!(message, "Rate limit hit");
                    }
                    _ => panic!(
                        "Expected Api error for non-matching rate limit, got: {:?}",
                        error
                    ),
                }
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_server_errors() {
    let test_cases = vec![
        ErrorClassificationTestCase {
            name: "system_database_error",
            status_code: 500,
            error_code: "system_database",
            message: "Database connection failed",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code,
                } => {
                    assert_eq!(message, "Database connection failed");
                    assert_eq!(*status_code, Some(500));
                }
                _ => panic!("Expected HttpTransport error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "system_service_unavailable",
            status_code: 503,
            error_code: "system_service",
            message: "Service temporarily unavailable",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code,
                } => {
                    assert_eq!(message, "Service temporarily unavailable");
                    assert_eq!(*status_code, Some(503));
                }
                _ => panic!("Expected HttpTransport error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "system_empty_suffix",
            status_code: 502,
            error_code: "system_",
            message: "System error occurred",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code,
                } => {
                    assert_eq!(message, "System error occurred");
                    assert_eq!(*status_code, Some(502));
                }
                _ => panic!("Expected HttpTransport error, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "server_error_non_system",
            status_code: 501,
            error_code: "not_implemented",
            message: "Method not implemented",
            expected_assertion: |error| {
                // This should NOT match the system_ pattern and fall through to API error
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 501);
                        assert_eq!(error_code, "not_implemented");
                        assert_eq!(message, "Method not implemented");
                    }
                    _ => panic!(
                        "Expected Api error for non-system server error, got: {:?}",
                        error
                    ),
                }
            },
        },
    ];

    for test_case in test_cases {
        println!("Running test case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_fallback_and_edge_cases() {
    // Test 200 OK fallback
    let test_case = ErrorClassificationTestCase {
        name: "fallback_200",
        status_code: 200,
        error_code: "success",
        message: "OK",
        expected_assertion: |error| match error {
            Error::Api {
                status_code,
                error_code,
                message,
            } => {
                assert_eq!(*status_code, 200);
                assert_eq!(error_code, "success");
                assert_eq!(message, "OK");
            }
            _ => panic!("Expected Api error for unknown status, got: {:?}", error),
        },
    };
    test_error_classification(&test_case);

    // Test 418 teapot fallback
    let test_case = ErrorClassificationTestCase {
        name: "fallback_418",
        status_code: 418,
        error_code: "teapot",
        message: "I'm a teapot",
        expected_assertion: |error| match error {
            Error::Api {
                status_code,
                error_code,
                message,
            } => {
                assert_eq!(*status_code, 418);
                assert_eq!(error_code, "teapot");
                assert_eq!(message, "I'm a teapot");
            }
            _ => panic!("Expected Api error for unknown status, got: {:?}", error),
        },
    };
    test_error_classification(&test_case);

    // Test 404 without resource_ prefix fallback
    let test_case = ErrorClassificationTestCase {
        name: "fallback_404_non_resource",
        status_code: 404,
        error_code: "not_resource",
        message: "Not a resource error",
        expected_assertion: |error| match error {
            Error::Api {
                status_code,
                error_code,
                message,
            } => {
                assert_eq!(*status_code, 404);
                assert_eq!(error_code, "not_resource");
                assert_eq!(message, "Not a resource error");
            }
            _ => panic!("Expected Api error for non-resource 404, got: {:?}", error),
        },
    };
    test_error_classification(&test_case);

    // Test 400 without validation_ prefix fallback
    let test_case = ErrorClassificationTestCase {
        name: "fallback_400_non_validation",
        status_code: 400,
        error_code: "not_validation",
        message: "Not a validation error",
        expected_assertion: |error| match error {
            Error::Api {
                status_code,
                error_code,
                message,
            } => {
                assert_eq!(*status_code, 400);
                assert_eq!(error_code, "not_validation");
                assert_eq!(message, "Not a validation error");
            }
            _ => panic!(
                "Expected Api error for non-validation 400, got: {:?}",
                error
            ),
        },
    };
    test_error_classification(&test_case);
}

#[test]
fn test_non_json_fallback_responses() {
    // Test fallback behavior when response body is not valid JSON (can't parse as ErrorResponse)

    // 400 Bad Request fallback
    test_fallback_error_classification(400, "Invalid request", |error| match error {
        Error::InvalidParameter { parameter, message } => {
            assert_eq!(parameter, "request");
            assert_eq!(message, "Invalid request");
        }
        _ => panic!(
            "Expected InvalidParameter for 400 fallback, got: {:?}",
            error
        ),
    });

    // 401 Unauthorized fallback
    test_fallback_error_classification(401, "Unauthorized access", |error| match error {
        Error::Authentication(message) => {
            assert_eq!(message, "Unauthorized access");
        }
        _ => panic!("Expected Authentication for 401 fallback, got: {:?}", error),
    });

    // 403 Forbidden fallback
    test_fallback_error_classification(403, "Access forbidden", |error| match error {
        Error::Authorization(message) => {
            assert_eq!(message, "Access forbidden");
        }
        _ => panic!("Expected Authorization for 403 fallback, got: {:?}", error),
    });

    // 404 Not Found fallback
    test_fallback_error_classification(404, "Resource not found", |error| match error {
        Error::ResourceNotFound {
            resource_type,
            identifier,
        } => {
            assert_eq!(resource_type, "unknown");
            assert_eq!(identifier, "Resource not found");
        }
        _ => panic!(
            "Expected ResourceNotFound for 404 fallback, got: {:?}",
            error
        ),
    });

    // 408 Request Timeout fallback
    test_fallback_error_classification(408, "Request timeout", |error| match error {
        Error::RequestTimeout {
            endpoint,
            timeout_ms,
        } => {
            assert_eq!(endpoint, "unknown");
            assert_eq!(*timeout_ms, 0);
        }
        _ => panic!("Expected RequestTimeout for 408 fallback, got: {:?}", error),
    });

    // 422 Unprocessable Entity fallback
    test_fallback_error_classification(422, "Validation failed", |error| match error {
        Error::BusinessLogic { operation, reason } => {
            assert_eq!(operation, "validation");
            assert_eq!(reason, "Validation failed");
        }
        _ => panic!("Expected BusinessLogic for 422 fallback, got: {:?}", error),
    });

    // 429 Too Many Requests fallback
    test_fallback_error_classification(429, "Rate limited", |error| match error {
        Error::RateLimitExceeded {
            retry_after_seconds,
        } => {
            assert_eq!(*retry_after_seconds, None);
        }
        _ => panic!(
            "Expected RateLimitExceeded for 429 fallback, got: {:?}",
            error
        ),
    });

    // 500 Internal Server Error fallback
    test_fallback_error_classification(500, "Internal server error", |error| match error {
        Error::HttpTransport {
            message,
            status_code,
        } => {
            assert_eq!(message, "Internal server error");
            assert_eq!(*status_code, Some(500));
        }
        _ => panic!("Expected HttpTransport for 500 fallback, got: {:?}", error),
    });

    // Unknown status code fallback
    test_fallback_error_classification(418, "I'm a teapot", |error| match error {
        Error::Api {
            status_code,
            error_code,
            message,
        } => {
            assert_eq!(*status_code, 418);
            assert_eq!(error_code, "unknown");
            assert_eq!(message, "I'm a teapot");
        }
        _ => panic!(
            "Expected Api error for unknown status fallback, got: {:?}",
            error
        ),
    });
}

#[test]
fn test_edge_cases_and_boundary_conditions() {
    // Test empty strings and edge cases
    let edge_cases = vec![
        ErrorClassificationTestCase {
            name: "empty_error_code",
            status_code: 400,
            error_code: "",
            message: "Empty error code",
            expected_assertion: |error| match error {
                Error::Api {
                    status_code,
                    error_code,
                    message,
                } => {
                    assert_eq!(*status_code, 400);
                    assert_eq!(error_code, "");
                    assert_eq!(message, "Empty error code");
                }
                _ => panic!("Expected Api error for empty error code, got: {:?}", error),
            },
        },
        ErrorClassificationTestCase {
            name: "empty_message",
            status_code: 401,
            error_code: "auth_failed",
            message: "",
            expected_assertion: |error| match error {
                Error::Authentication(message) => {
                    assert_eq!(message, "");
                }
                _ => panic!(
                    "Expected Authentication error for empty message, got: {:?}",
                    error
                ),
            },
        },
        ErrorClassificationTestCase {
            name: "validation_only_prefix",
            status_code: 400,
            error_code: "validation",
            message: "Just validation prefix",
            expected_assertion: |error| {
                // "validation" doesn't start with "validation_" so should fall through
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 400);
                        assert_eq!(error_code, "validation");
                        assert_eq!(message, "Just validation prefix");
                    }
                    _ => panic!("Expected Api error for just 'validation', got: {:?}", error),
                }
            },
        },
        ErrorClassificationTestCase {
            name: "business_only_prefix",
            status_code: 422,
            error_code: "business",
            message: "Just business prefix",
            expected_assertion: |error| {
                // "business" doesn't start with "business_" so should fall through
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 422);
                        assert_eq!(error_code, "business");
                        assert_eq!(message, "Just business prefix");
                    }
                    _ => panic!("Expected Api error for just 'business', got: {:?}", error),
                }
            },
        },
        ErrorClassificationTestCase {
            name: "status_code_boundary_499",
            status_code: 499,
            error_code: "client_closed",
            message: "Client closed connection",
            expected_assertion: |error| {
                // 499 is not in 500..=599 range, should fall through to API error
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 499);
                        assert_eq!(error_code, "client_closed");
                        assert_eq!(message, "Client closed connection");
                    }
                    _ => panic!("Expected Api error for 499, got: {:?}", error),
                }
            },
        },
        ErrorClassificationTestCase {
            name: "status_code_boundary_600",
            status_code: 600,
            error_code: "custom_error",
            message: "Custom HTTP status",
            expected_assertion: |error| {
                // 600 is not in 500..=599 range, should fall through to API error
                match error {
                    Error::Api {
                        status_code,
                        error_code,
                        message,
                    } => {
                        assert_eq!(*status_code, 600);
                        assert_eq!(error_code, "custom_error");
                        assert_eq!(message, "Custom HTTP status");
                    }
                    _ => panic!("Expected Api error for 600, got: {:?}", error),
                }
            },
        },
    ];

    for test_case in edge_cases {
        println!("Running edge case: {}", test_case.name);
        test_error_classification(&test_case);
    }
}

#[test]
fn test_system_error_range_boundaries() {
    // Test all status codes in the 500..=599 range with system_ prefix
    let system_status_codes = [500, 501, 502, 503, 504, 505, 550, 599];

    for &status_code in &system_status_codes {
        let test_case = ErrorClassificationTestCase {
            name: "system_error_boundary",
            status_code,
            error_code: "system_test",
            message: "System error occurred",
            expected_assertion: |error| match error {
                Error::HttpTransport {
                    message,
                    status_code: status,
                } => {
                    assert_eq!(message, "System error occurred");
                    assert!(status.unwrap() >= 500 && status.unwrap() <= 599);
                }
                _ => panic!(
                    "Expected HttpTransport error for system_ error, got: {:?}",
                    error
                ),
            },
        };
        test_error_classification(&test_case);
    }
}

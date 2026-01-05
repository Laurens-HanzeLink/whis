//! Retry logic with exponential backoff for cloud transcription providers.
//!
//! This module provides retry functionality for transient errors like:
//! - 408 Request Timeout (SLOW_UPLOAD)
//! - 429 Rate Limited
//! - 5xx Server Errors
//! - Network/connection errors

use std::time::Duration;

use reqwest::StatusCode;

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Base delay in milliseconds (doubles with each attempt)
    pub base_delay_ms: u64,
    /// Maximum delay cap in milliseconds
    pub max_delay_ms: u64,
    /// Multiplier for rate limit errors (429)
    pub rate_limit_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,    // 1 second
            max_delay_ms: 16000,    // 16 seconds
            rate_limit_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Calculate the delay for a given attempt number
    pub fn delay_for_attempt(&self, attempt: u32, is_rate_limited: bool) -> Duration {
        let base_delay = self.base_delay_ms * 2u64.pow(attempt);
        let delay_ms = base_delay.min(self.max_delay_ms);

        if is_rate_limited {
            Duration::from_millis((delay_ms as f64 * self.rate_limit_multiplier) as u64)
        } else {
            Duration::from_millis(delay_ms)
        }
    }
}

/// Check if an HTTP status code is retryable
pub fn is_retryable_status(status: StatusCode) -> bool {
    matches!(
        status.as_u16(),
        408 | 429 | 500 | 502 | 503 | 504
    )
}

/// Check if a status code indicates rate limiting
pub fn is_rate_limited(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS
}

/// Check if a reqwest error is retryable
pub fn is_retryable_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_status_codes() {
        assert!(is_retryable_status(StatusCode::REQUEST_TIMEOUT)); // 408
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS)); // 429
        assert!(is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR)); // 500
        assert!(is_retryable_status(StatusCode::BAD_GATEWAY)); // 502
        assert!(is_retryable_status(StatusCode::SERVICE_UNAVAILABLE)); // 503
        assert!(is_retryable_status(StatusCode::GATEWAY_TIMEOUT)); // 504

        assert!(!is_retryable_status(StatusCode::BAD_REQUEST)); // 400
        assert!(!is_retryable_status(StatusCode::UNAUTHORIZED)); // 401
        assert!(!is_retryable_status(StatusCode::FORBIDDEN)); // 403
        assert!(!is_retryable_status(StatusCode::NOT_FOUND)); // 404
    }

    #[test]
    fn test_delay_calculation() {
        let config = RetryConfig::default();

        // Normal delays: 1s, 2s, 4s, 8s (capped at 16s)
        assert_eq!(config.delay_for_attempt(0, false), Duration::from_millis(1000));
        assert_eq!(config.delay_for_attempt(1, false), Duration::from_millis(2000));
        assert_eq!(config.delay_for_attempt(2, false), Duration::from_millis(4000));
        assert_eq!(config.delay_for_attempt(3, false), Duration::from_millis(8000));
        assert_eq!(config.delay_for_attempt(4, false), Duration::from_millis(16000)); // capped

        // Rate limited: 2x multiplier
        assert_eq!(config.delay_for_attempt(0, true), Duration::from_millis(2000));
        assert_eq!(config.delay_for_attempt(1, true), Duration::from_millis(4000));
    }
}

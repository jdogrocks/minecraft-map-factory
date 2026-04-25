use crate::config::RetryConfig;
use std::time::Duration;

/// Classification of an error for retry decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorClassification {
    /// Transient errors that may succeed on retry (API timeout, network issues).
    Transient,
    /// Permanent errors that won't be fixed by retry (invalid coords, missing data).
    Permanent,
    /// Resource errors (OOM, disk full) that may benefit from reduced parameters.
    Resource,
}

/// Decision from the retry strategy.
#[derive(Debug)]
pub enum RetryDecision {
    Retry { backoff: Duration },
    GiveUp,
}

/// Retry strategy with exponential backoff.
pub struct RetryStrategy {
    max_retries: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
    shrink_bbox: bool,
    shrink_factor: f64,
}

impl RetryStrategy {
    pub fn new(config: &RetryConfig) -> Self {
        Self {
            max_retries: config.max_retries,
            initial_backoff: Duration::from_secs(config.initial_backoff_secs),
            max_backoff: Duration::from_secs(config.max_backoff_secs),
            shrink_bbox: config.shrink_bbox_on_retry,
            shrink_factor: config.bbox_shrink_factor,
        }
    }

    /// Classify an error as transient, permanent, or resource-related.
    pub fn classify_error(&self, error: &dyn std::fmt::Display) -> ErrorClassification {
        let msg = error.to_string().to_lowercase();

        if msg.contains("timeout")
            || msg.contains("connection refused")
            || msg.contains("connection reset")
            || msg.contains("temporarily unavailable")
            || msg.contains("429")
            || msg.contains("503")
        {
            ErrorClassification::Transient
        } else if msg.contains("out of memory")
            || msg.contains("oom")
            || msg.contains("no space left")
            || msg.contains("memory allocation")
        {
            ErrorClassification::Resource
        } else if msg.contains("invalid") || msg.contains("not found") || msg.contains("no data") {
            ErrorClassification::Permanent
        } else {
            // Default to transient for unknown errors
            ErrorClassification::Transient
        }
    }

    /// Determine whether to retry, and with what backoff.
    pub fn should_retry(
        &self,
        attempt: u32,
        classification: &ErrorClassification,
    ) -> RetryDecision {
        if *classification == ErrorClassification::Permanent {
            return RetryDecision::GiveUp;
        }

        if attempt >= self.max_retries {
            return RetryDecision::GiveUp;
        }

        let backoff = self.calculate_backoff(attempt);
        RetryDecision::Retry { backoff }
    }

    pub fn should_shrink_bbox(&self) -> bool {
        self.shrink_bbox
    }

    pub fn shrink_factor(&self) -> f64 {
        self.shrink_factor
    }

    fn calculate_backoff(&self, attempt: u32) -> Duration {
        let multiplier = 2u64.saturating_pow(attempt.saturating_sub(1));
        let backoff = self.initial_backoff.saturating_mul(multiplier as u32);
        std::cmp::min(backoff, self.max_backoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_strategy() -> RetryStrategy {
        RetryStrategy::new(&RetryConfig::default())
    }

    #[test]
    fn test_classify_transient() {
        let s = default_strategy();
        let err: Box<dyn std::error::Error + Send + Sync> = "connection timeout".into();
        assert_eq!(s.classify_error(&*err), ErrorClassification::Transient);
    }

    #[test]
    fn test_classify_permanent() {
        let s = default_strategy();
        let err: Box<dyn std::error::Error + Send + Sync> = "invalid coordinates".into();
        assert_eq!(s.classify_error(&*err), ErrorClassification::Permanent);
    }

    #[test]
    fn test_classify_resource() {
        let s = default_strategy();
        let err: Box<dyn std::error::Error + Send + Sync> = "out of memory".into();
        assert_eq!(s.classify_error(&*err), ErrorClassification::Resource);
    }

    #[test]
    fn test_no_retry_on_permanent() {
        let s = default_strategy();
        let decision = s.should_retry(1, &ErrorClassification::Permanent);
        assert!(matches!(decision, RetryDecision::GiveUp));
    }

    #[test]
    fn test_retry_on_transient() {
        let s = default_strategy();
        let decision = s.should_retry(1, &ErrorClassification::Transient);
        assert!(matches!(decision, RetryDecision::Retry { .. }));
    }

    #[test]
    fn test_give_up_after_max_retries() {
        let s = default_strategy();
        let decision = s.should_retry(3, &ErrorClassification::Transient);
        assert!(matches!(decision, RetryDecision::GiveUp));
    }

    #[test]
    fn test_exponential_backoff() {
        let s = default_strategy();
        let b1 = s.calculate_backoff(1);
        let b2 = s.calculate_backoff(2);
        let b3 = s.calculate_backoff(3);
        assert_eq!(b1, Duration::from_secs(1));
        assert_eq!(b2, Duration::from_secs(2));
        assert_eq!(b3, Duration::from_secs(4));
    }
}

//! Risk limit thresholds for agent actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

/// Type of risk limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitType {
    /// Rate limit (actions per time window).
    Rate,
    /// Threshold limit (value must stay below).
    Threshold,
    /// Budget limit (cumulative spending cap).
    Budget,
    /// Count limit (total number of actions).
    Count,
}

/// A risk limit threshold for a resource or action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimit {
    /// Unique identifier.
    pub id: ContractId,
    /// Human-readable label.
    pub label: String,
    /// Type of limit.
    pub limit_type: LimitType,
    /// Current accumulated value.
    pub current_value: f64,
    /// Maximum allowed value.
    pub max_value: f64,
    /// Time window in seconds (for rate limits).
    #[serde(default)]
    pub window_secs: Option<u64>,
    /// When the current window started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start: Option<DateTime<Utc>>,
    /// When this limit was created.
    pub created_at: DateTime<Utc>,
    /// When this limit was last updated.
    pub updated_at: DateTime<Utc>,
}

impl RiskLimit {
    /// Create a new risk limit.
    pub fn new(label: impl Into<String>, limit_type: LimitType, max_value: f64) -> Self {
        let now = Utc::now();
        Self {
            id: ContractId::new(),
            label: label.into(),
            limit_type,
            current_value: 0.0,
            max_value,
            window_secs: None,
            window_start: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set a time window (for rate limits).
    pub fn with_window(mut self, secs: u64) -> Self {
        self.window_secs = Some(secs);
        self.window_start = Some(Utc::now());
        self
    }

    /// Check if the limit would be exceeded by adding `amount`.
    pub fn would_exceed(&self, amount: f64) -> bool {
        self.current_value + amount > self.max_value
    }

    /// Get remaining capacity.
    pub fn remaining(&self) -> f64 {
        (self.max_value - self.current_value).max(0.0)
    }

    /// Get usage as a percentage (0.0 to 1.0).
    pub fn usage_ratio(&self) -> f64 {
        if self.max_value == 0.0 {
            return 1.0;
        }
        (self.current_value / self.max_value).min(1.0)
    }

    /// Increment the current value.
    pub fn increment(&mut self, amount: f64) {
        self.current_value += amount;
        self.updated_at = Utc::now();
    }

    /// Reset the current value (e.g. for a new time window).
    pub fn reset(&mut self) {
        self.current_value = 0.0;
        self.window_start = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Check if the time window has expired and needs resetting.
    pub fn window_expired(&self) -> bool {
        if let (Some(window_secs), Some(window_start)) = (self.window_secs, self.window_start) {
            let elapsed = (Utc::now() - window_start).num_seconds() as u64;
            elapsed >= window_secs
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_limit_creation() {
        let limit = RiskLimit::new("API calls per minute", LimitType::Rate, 100.0).with_window(60);

        assert_eq!(limit.label, "API calls per minute");
        assert_eq!(limit.limit_type, LimitType::Rate);
        assert_eq!(limit.max_value, 100.0);
        assert_eq!(limit.current_value, 0.0);
        assert!(limit.window_secs.is_some());
    }

    #[test]
    fn test_would_exceed() {
        let mut limit = RiskLimit::new("Budget", LimitType::Budget, 1000.0);
        limit.increment(900.0);

        assert!(!limit.would_exceed(50.0));
        assert!(limit.would_exceed(200.0));
        assert_eq!(limit.remaining(), 100.0);
    }

    #[test]
    fn test_usage_ratio() {
        let mut limit = RiskLimit::new("Count", LimitType::Count, 10.0);
        limit.increment(5.0);
        assert!((limit.usage_ratio() - 0.5).abs() < f64::EPSILON);
    }
}

//! Conditional execution rules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

/// Type of condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    /// Value must be above/below a threshold.
    Threshold,
    /// Time-based condition (before/after/during).
    TimeBased,
    /// Depends on another entity's state.
    Dependency,
    /// Custom expression.
    Custom,
}

/// Evaluation status of a condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionStatus {
    /// Not yet evaluated.
    Unevaluated,
    /// Condition is met.
    Met,
    /// Condition is not met.
    NotMet,
    /// Could not evaluate (missing data).
    Unknown,
}

/// A conditional execution rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Unique identifier.
    pub id: ContractId,
    /// Human-readable label.
    pub label: String,
    /// Type of condition.
    pub condition_type: ConditionType,
    /// Expression to evaluate.
    pub expression: String,
    /// Current evaluation status.
    pub status: ConditionStatus,
    /// Last evaluation result message.
    #[serde(default)]
    pub last_result: Option<String>,
    /// When this condition was created.
    pub created_at: DateTime<Utc>,
    /// When this condition was last evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_at: Option<DateTime<Utc>>,
}

impl Condition {
    /// Create a new condition.
    pub fn new(
        label: impl Into<String>,
        condition_type: ConditionType,
        expression: impl Into<String>,
    ) -> Self {
        Self {
            id: ContractId::new(),
            label: label.into(),
            condition_type,
            expression: expression.into(),
            status: ConditionStatus::Unevaluated,
            last_result: None,
            created_at: Utc::now(),
            evaluated_at: None,
        }
    }

    /// Mark the condition as met.
    pub fn mark_met(&mut self, message: impl Into<String>) {
        self.status = ConditionStatus::Met;
        self.last_result = Some(message.into());
        self.evaluated_at = Some(Utc::now());
    }

    /// Mark the condition as not met.
    pub fn mark_not_met(&mut self, message: impl Into<String>) {
        self.status = ConditionStatus::NotMet;
        self.last_result = Some(message.into());
        self.evaluated_at = Some(Utc::now());
    }

    /// Check if this condition is currently met.
    pub fn is_met(&self) -> bool {
        self.status == ConditionStatus::Met
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_lifecycle() {
        let mut condition = Condition::new(
            "CPU below 80%",
            ConditionType::Threshold,
            "system.cpu_usage < 0.8",
        );

        assert_eq!(condition.status, ConditionStatus::Unevaluated);
        assert!(!condition.is_met());

        condition.mark_met("CPU at 65%");
        assert!(condition.is_met());
        assert!(condition.evaluated_at.is_some());
    }
}

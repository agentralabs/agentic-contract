//! Violation records for contract and policy breaches.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

/// Severity of a violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViolationSeverity {
    /// Informational — logged but no action needed.
    Info,
    /// Warning — should be investigated.
    Warning,
    /// Critical — requires immediate attention.
    Critical,
    /// Fatal — system should halt.
    Fatal,
}

impl std::fmt::Display for ViolationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationSeverity::Info => write!(f, "info"),
            ViolationSeverity::Warning => write!(f, "warning"),
            ViolationSeverity::Critical => write!(f, "critical"),
            ViolationSeverity::Fatal => write!(f, "fatal"),
        }
    }
}

/// A recorded violation of a contract or policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Unique identifier.
    pub id: ContractId,
    /// Which policy was violated (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<ContractId>,
    /// Description of the violation.
    pub description: String,
    /// Severity level.
    pub severity: ViolationSeverity,
    /// Who or what triggered the violation.
    pub actor: String,
    /// When the violation was detected.
    pub detected_at: DateTime<Utc>,
    /// Additional context data.
    #[serde(default)]
    pub context: serde_json::Value,
}

impl Violation {
    /// Create a new violation record.
    pub fn new(
        description: impl Into<String>,
        severity: ViolationSeverity,
        actor: impl Into<String>,
    ) -> Self {
        Self {
            id: ContractId::new(),
            policy_id: None,
            description: description.into(),
            severity,
            actor: actor.into(),
            detected_at: Utc::now(),
            context: serde_json::Value::Null,
        }
    }

    /// Link to a policy.
    pub fn for_policy(mut self, policy_id: ContractId) -> Self {
        self.policy_id = Some(policy_id);
        self
    }

    /// Add context data.
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violation_creation() {
        let violation =
            Violation::new("Rate limit exceeded", ViolationSeverity::Warning, "agent_1")
                .with_context(serde_json::json!({"requests": 150, "limit": 100}));

        assert_eq!(violation.severity, ViolationSeverity::Warning);
        assert!(violation.policy_id.is_none());
        assert!(!violation.context.is_null());
    }

    #[test]
    fn test_severity_ordering() {
        assert!(ViolationSeverity::Fatal > ViolationSeverity::Critical);
        assert!(ViolationSeverity::Critical > ViolationSeverity::Warning);
        assert!(ViolationSeverity::Warning > ViolationSeverity::Info);
    }
}

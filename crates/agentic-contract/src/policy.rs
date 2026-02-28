//! Policy rules governing agent behavior.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

/// What happens when a policy matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    /// Action is allowed.
    Allow,
    /// Action is denied.
    Deny,
    /// Action requires approval before proceeding.
    RequireApproval,
    /// Action is logged but not blocked.
    AuditOnly,
}

/// Scope at which a policy applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyScope {
    /// Applies to all sessions and agents.
    Global,
    /// Applies to the current session only.
    Session,
    /// Applies to a specific agent.
    Agent,
}

impl std::fmt::Display for PolicyScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyScope::Global => write!(f, "global"),
            PolicyScope::Session => write!(f, "session"),
            PolicyScope::Agent => write!(f, "agent"),
        }
    }
}

/// Whether a policy is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyStatus {
    /// Policy is active and enforced.
    Active,
    /// Policy is disabled.
    Disabled,
    /// Policy has expired.
    Expired,
}

/// A policy rule governing agent behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Unique identifier.
    pub id: ContractId,
    /// Human-readable label.
    pub label: String,
    /// Description of what this policy governs.
    pub description: String,
    /// Scope of the policy.
    pub scope: PolicyScope,
    /// What action to take when the policy matches.
    pub action: PolicyAction,
    /// Conditions that trigger this policy (expression strings).
    #[serde(default)]
    pub conditions: Vec<String>,
    /// Whether the policy is currently active.
    pub status: PolicyStatus,
    /// Tags for categorization.
    #[serde(default)]
    pub tags: Vec<String>,
    /// When the policy was created.
    pub created_at: DateTime<Utc>,
    /// When the policy was last updated.
    pub updated_at: DateTime<Utc>,
    /// Optional expiration time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl Policy {
    /// Create a new policy with the given label, scope, and action.
    pub fn new(label: impl Into<String>, scope: PolicyScope, action: PolicyAction) -> Self {
        let now = Utc::now();
        Self {
            id: ContractId::new(),
            label: label.into(),
            description: String::new(),
            scope,
            action,
            conditions: vec![],
            status: PolicyStatus::Active,
            tags: vec![],
            created_at: now,
            updated_at: now,
            expires_at: None,
        }
    }

    /// Set the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Add a condition expression.
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Add a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set expiration time.
    pub fn expires_at(mut self, time: DateTime<Utc>) -> Self {
        self.expires_at = Some(time);
        self
    }

    /// Check if this policy is currently active (not expired or disabled).
    pub fn is_active(&self) -> bool {
        if self.status != PolicyStatus::Active {
            return false;
        }
        if let Some(expires) = self.expires_at {
            if Utc::now() > expires {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_creation() {
        let policy = Policy::new(
            "No deploys on Friday",
            PolicyScope::Global,
            PolicyAction::Deny,
        )
        .with_description("Prevents deployments on Fridays")
        .with_tag("safety")
        .with_condition("day_of_week == Friday");

        assert_eq!(policy.label, "No deploys on Friday");
        assert_eq!(policy.scope, PolicyScope::Global);
        assert_eq!(policy.action, PolicyAction::Deny);
        assert!(policy.is_active());
        assert_eq!(policy.tags, vec!["safety"]);
        assert_eq!(policy.conditions.len(), 1);
    }

    #[test]
    fn test_policy_status() {
        let mut policy = Policy::new("Test policy", PolicyScope::Session, PolicyAction::Allow);
        assert!(policy.is_active());

        policy.status = PolicyStatus::Disabled;
        assert!(!policy.is_active());
    }
}

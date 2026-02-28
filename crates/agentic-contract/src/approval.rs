//! Approval workflow for controlled actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

/// Status of an approval request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    /// Awaiting decision.
    Pending,
    /// Approved.
    Approved,
    /// Denied.
    Denied,
    /// Expired without decision.
    Expired,
}

/// Type of approval decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionType {
    /// Approved the request.
    Approve,
    /// Denied the request.
    Deny,
}

/// A rule defining when approval is required.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRule {
    /// Unique identifier.
    pub id: ContractId,
    /// Human-readable label.
    pub label: String,
    /// What action types require approval.
    pub action_pattern: String,
    /// Who can approve (agent IDs or roles).
    #[serde(default)]
    pub approvers: Vec<String>,
    /// Timeout before the request expires (seconds).
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    /// When this rule was created.
    pub created_at: DateTime<Utc>,
}

impl ApprovalRule {
    /// Create a new approval rule.
    pub fn new(label: impl Into<String>, action_pattern: impl Into<String>) -> Self {
        Self {
            id: ContractId::new(),
            label: label.into(),
            action_pattern: action_pattern.into(),
            approvers: vec![],
            timeout_secs: None,
            created_at: Utc::now(),
        }
    }

    /// Add an approver.
    pub fn with_approver(mut self, approver: impl Into<String>) -> Self {
        self.approvers.push(approver.into());
        self
    }

    /// Set a timeout.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }
}

/// A pending approval request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// Unique identifier.
    pub id: ContractId,
    /// Which rule triggered this request.
    pub rule_id: ContractId,
    /// Description of the action needing approval.
    pub action_description: String,
    /// Who requested the action.
    pub requestor: String,
    /// Current status.
    pub status: ApprovalStatus,
    /// When the request was created.
    pub created_at: DateTime<Utc>,
    /// When the request expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl ApprovalRequest {
    /// Create a new approval request.
    pub fn new(
        rule_id: ContractId,
        action_description: impl Into<String>,
        requestor: impl Into<String>,
    ) -> Self {
        Self {
            id: ContractId::new(),
            rule_id,
            action_description: action_description.into(),
            requestor: requestor.into(),
            status: ApprovalStatus::Pending,
            created_at: Utc::now(),
            expires_at: None,
        }
    }

    /// Set an expiration time.
    pub fn expires_at(mut self, time: DateTime<Utc>) -> Self {
        self.expires_at = Some(time);
        self
    }

    /// Check if this request is still pending.
    pub fn is_pending(&self) -> bool {
        if self.status != ApprovalStatus::Pending {
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

/// A decision on an approval request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    /// Unique identifier.
    pub id: ContractId,
    /// Which request this decides.
    pub request_id: ContractId,
    /// The decision.
    pub decision: DecisionType,
    /// Who made the decision.
    pub decider: String,
    /// Reason for the decision.
    pub reason: String,
    /// When the decision was made.
    pub decided_at: DateTime<Utc>,
}

impl ApprovalDecision {
    /// Create a new approval decision.
    pub fn new(
        request_id: ContractId,
        decision: DecisionType,
        decider: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: ContractId::new(),
            request_id,
            decision,
            decider: decider.into(),
            reason: reason.into(),
            decided_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approval_workflow() {
        let rule = ApprovalRule::new("Deploy approval", "deploy:*")
            .with_approver("admin")
            .with_timeout(3600);

        let request = ApprovalRequest::new(rule.id, "Deploy to production", "agent_1");
        assert!(request.is_pending());

        let decision =
            ApprovalDecision::new(request.id, DecisionType::Approve, "admin", "Looks good");
        assert_eq!(decision.decision, DecisionType::Approve);
    }
}

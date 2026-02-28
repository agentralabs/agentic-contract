//! Obligations that agents must fulfill.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

/// Status of an obligation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationStatus {
    /// Not yet due.
    Pending,
    /// Fulfilled successfully.
    Fulfilled,
    /// Past deadline without fulfillment.
    Overdue,
    /// Explicitly waived.
    Waived,
}

/// An obligation that an agent must fulfill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    /// Unique identifier.
    pub id: ContractId,
    /// Human-readable label.
    pub label: String,
    /// Description of what must be done.
    pub description: String,
    /// Who is responsible.
    pub assignee: String,
    /// Deadline for fulfillment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    /// Current status.
    pub status: ObligationStatus,
    /// When the obligation was created.
    pub created_at: DateTime<Utc>,
    /// When the obligation was fulfilled/waived.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
}

impl Obligation {
    /// Create a new obligation.
    pub fn new(
        label: impl Into<String>,
        description: impl Into<String>,
        assignee: impl Into<String>,
    ) -> Self {
        Self {
            id: ContractId::new(),
            label: label.into(),
            description: description.into(),
            assignee: assignee.into(),
            deadline: None,
            status: ObligationStatus::Pending,
            created_at: Utc::now(),
            resolved_at: None,
        }
    }

    /// Set a deadline.
    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Mark as fulfilled.
    pub fn fulfill(&mut self) {
        self.status = ObligationStatus::Fulfilled;
        self.resolved_at = Some(Utc::now());
    }

    /// Mark as waived.
    pub fn waive(&mut self) {
        self.status = ObligationStatus::Waived;
        self.resolved_at = Some(Utc::now());
    }

    /// Check if this obligation is overdue.
    pub fn is_overdue(&self) -> bool {
        if self.status != ObligationStatus::Pending {
            return false;
        }
        if let Some(deadline) = self.deadline {
            Utc::now() > deadline
        } else {
            false
        }
    }

    /// Check if this obligation is resolved (fulfilled or waived).
    pub fn is_resolved(&self) -> bool {
        matches!(
            self.status,
            ObligationStatus::Fulfilled | ObligationStatus::Waived
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligation_lifecycle() {
        let mut obligation = Obligation::new(
            "Submit compliance report",
            "Monthly compliance report for Q1",
            "agent_compliance",
        );

        assert_eq!(obligation.status, ObligationStatus::Pending);
        assert!(!obligation.is_resolved());

        obligation.fulfill();
        assert_eq!(obligation.status, ObligationStatus::Fulfilled);
        assert!(obligation.is_resolved());
        assert!(obligation.resolved_at.is_some());
    }
}

//! # AgenticContract
//!
//! Policy engine for AI agents. Models policies, risk limits,
//! approvals, conditions, obligations, and violations
//! in a single `.acon` file.

pub mod approval;
pub mod condition;
pub mod contract_engine;
pub mod contracts;
pub mod error;
pub mod file_format;
pub mod inventions;
pub mod obligation;
pub mod policy;
pub mod risk_limit;
pub mod violation;

pub use approval::{ApprovalDecision, ApprovalRequest, ApprovalRule, ApprovalStatus, DecisionType};
pub use condition::{Condition, ConditionStatus, ConditionType};
pub use contract_engine::ContractEngine;
pub use error::{ContractError, ContractResult};
pub use file_format::{ContractFile, EntityType, FileHeader};
pub use obligation::{Obligation, ObligationStatus};
pub use policy::{Policy, PolicyAction, PolicyScope, PolicyStatus};
pub use risk_limit::{LimitType, RiskLimit};
pub use violation::{Violation, ViolationSeverity};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for contract entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContractId(pub Uuid);

impl ContractId {
    /// Generate a new random contract ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ContractId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ContractId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ContractId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

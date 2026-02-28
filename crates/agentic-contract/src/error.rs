//! Error types for AgenticContract.

use thiserror::Error;

/// Errors that can occur in contract operations.
#[derive(Error, Debug)]
pub enum ContractError {
    /// Entity not found by ID.
    #[error("Contract entity not found: {0}")]
    NotFound(String),

    /// Policy violation detected.
    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    /// Risk limit exceeded.
    #[error("Risk limit exceeded: {limit} (current: {current}, max: {max})")]
    RiskLimitExceeded {
        /// Which limit was exceeded.
        limit: String,
        /// Current value.
        current: f64,
        /// Maximum allowed.
        max: f64,
    },

    /// Approval is required for this action.
    #[error("Approval required: {0}")]
    ApprovalRequired(String),

    /// Approval was denied.
    #[error("Approval denied: {0}")]
    ApprovalDenied(String),

    /// Condition not met.
    #[error("Condition not met: {0}")]
    ConditionNotMet(String),

    /// Obligation unfulfilled.
    #[error("Obligation unfulfilled: {0}")]
    ObligationUnfulfilled(String),

    /// Contract has expired.
    #[error("Contract expired: {0}")]
    ContractExpired(String),

    /// Invalid contract definition.
    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    /// File format error.
    #[error("File format error: {0}")]
    FileFormat(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Result type alias for contract operations.
pub type ContractResult<T> = Result<T, ContractError>;

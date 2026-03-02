//! Sister integration bridge traits for AgenticContract.
//!
//! Each bridge defines the interface for integrating with another Agentra sister.
//! Default implementations are no-ops, allowing gradual adoption.
//! Trait-based design ensures Hydra compatibility — swap implementors without refactoring.

/// Bridge to agentic-memory for persisting contract events.
pub trait MemoryBridge: Send + Sync {
    /// Store a contract event as a memory node
    fn store_contract_event(&self, event_type: &str, details: &str) -> Result<u64, String> {
        let _ = (event_type, details);
        Err("Memory bridge not connected".to_string())
    }

    /// Recall policy history from memory
    fn recall_policy_history(&self, topic: &str, max_results: usize) -> Vec<String> {
        let _ = (topic, max_results);
        Vec::new()
    }

    /// Link a violation to a memory node for context
    fn link_violation_to_memory(&self, violation_id: &str, node_id: u64) -> Result<(), String> {
        let _ = (violation_id, node_id);
        Err("Memory bridge not connected".to_string())
    }
}

/// Bridge to agentic-identity for verified contract operations.
pub trait IdentityBridge: Send + Sync {
    /// Verify the signer of a contract
    fn verify_signer(&self, contract_id: &str, agent_id: &str) -> bool {
        let _ = (contract_id, agent_id);
        true // Default: trust all
    }

    /// Get trust level for an agent (for approval routing)
    fn get_trust_level(&self, agent_id: &str) -> Option<f64> {
        let _ = agent_id;
        None
    }

    /// Sign a contract action with identity proof
    fn sign_action(&self, action: &str, contract_id: &str) -> Result<String, String> {
        let _ = (action, contract_id);
        Err("Identity bridge not connected".to_string())
    }

    /// Anchor a contract receipt to identity chain
    fn anchor_receipt(&self, action: &str, details: &str) -> Result<String, String> {
        let _ = (action, details);
        Err("Identity bridge not connected".to_string())
    }
}

/// Bridge to agentic-time for temporal contract enforcement.
pub trait TimeBridge: Send + Sync {
    /// Create a deadline for a contract obligation
    fn create_deadline(&self, label: &str, due_at: u64) -> Result<String, String> {
        let _ = (label, due_at);
        Err("Time bridge not connected".to_string())
    }

    /// Check if an obligation deadline has passed
    fn is_deadline_past(&self, deadline_id: &str) -> Option<bool> {
        let _ = deadline_id;
        None
    }

    /// Schedule an approval timeout
    fn schedule_approval_timeout(
        &self,
        request_id: &str,
        timeout_at: u64,
    ) -> Result<String, String> {
        let _ = (request_id, timeout_at);
        Err("Time bridge not connected".to_string())
    }

    /// Get temporal decay context for trust gradient
    fn trust_decay_context(&self, agent_id: &str) -> Option<f64> {
        let _ = agent_id;
        None
    }
}

/// Bridge to agentic-codebase for code-aware policy enforcement.
pub trait CodebaseBridge: Send + Sync {
    /// Check if a code change is covered by policy
    fn check_code_policy(&self, symbol: &str, change_type: &str) -> Result<bool, String> {
        let _ = (symbol, change_type);
        Ok(true) // Default: allow all
    }

    /// Get code impact for risk assessment
    fn code_impact(&self, symbol: &str) -> Option<String> {
        let _ = symbol;
        None
    }
}

/// Bridge to agentic-vision for visual contract evidence.
pub trait VisionBridge: Send + Sync {
    /// Capture visual evidence of a contract violation
    fn capture_violation_evidence(&self, description: &str) -> Result<u64, String> {
        let _ = description;
        Err("Vision bridge not connected".to_string())
    }

    /// Link a contract entity to a visual capture
    fn link_to_capture(&self, entity_id: &str, capture_id: u64) -> Result<(), String> {
        let _ = (entity_id, capture_id);
        Err("Vision bridge not connected".to_string())
    }
}

/// Bridge to agentic-comm for contract-aware messaging.
pub trait CommBridge: Send + Sync {
    /// Broadcast a policy change notification
    fn broadcast_policy_change(&self, policy_id: &str, channel_id: u64) -> Result<(), String> {
        let _ = (policy_id, channel_id);
        Err("Comm bridge not connected".to_string())
    }

    /// Send approval request via comm channel
    fn send_approval_request(&self, request_id: &str, approver: &str) -> Result<(), String> {
        let _ = (request_id, approver);
        Err("Comm bridge not connected".to_string())
    }

    /// Notify of a contract violation
    fn notify_violation(&self, violation_id: &str, channel_id: u64) -> Result<(), String> {
        let _ = (violation_id, channel_id);
        Err("Comm bridge not connected".to_string())
    }
}

/// No-op implementation of all bridges for standalone use.
#[derive(Debug, Clone, Default)]
pub struct NoOpBridges;

impl MemoryBridge for NoOpBridges {}
impl IdentityBridge for NoOpBridges {}
impl TimeBridge for NoOpBridges {}
impl CodebaseBridge for NoOpBridges {}
impl VisionBridge for NoOpBridges {}
impl CommBridge for NoOpBridges {}

/// Configuration for which bridges are active.
#[derive(Debug, Clone, Default)]
pub struct BridgeConfig {
    pub memory_enabled: bool,
    pub identity_enabled: bool,
    pub time_enabled: bool,
    pub codebase_enabled: bool,
    pub vision_enabled: bool,
    pub comm_enabled: bool,
}

/// Hydra adapter trait — future orchestrator discovery interface.
pub trait HydraAdapter: Send + Sync {
    fn adapter_id(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    fn handle_request(&self, method: &str, params: &str) -> Result<String, String>;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_bridges_implements_all_traits() {
        let b = NoOpBridges;
        let _: &dyn MemoryBridge = &b;
        let _: &dyn IdentityBridge = &b;
        let _: &dyn TimeBridge = &b;
        let _: &dyn CodebaseBridge = &b;
        let _: &dyn VisionBridge = &b;
        let _: &dyn CommBridge = &b;
    }

    #[test]
    fn memory_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.store_contract_event("violation", "details").is_err());
        assert!(b.recall_policy_history("topic", 10).is_empty());
        assert!(b.link_violation_to_memory("viol-1", 1).is_err());
    }

    #[test]
    fn identity_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.verify_signer("contract-1", "agent-1"));
        assert!(b.get_trust_level("agent-1").is_none());
        assert!(b.sign_action("sign", "contract-1").is_err());
        assert!(b.anchor_receipt("sign", "details").is_err());
    }

    #[test]
    fn time_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.create_deadline("label", 1000).is_err());
        assert!(b.is_deadline_past("dl-1").is_none());
        assert!(b.schedule_approval_timeout("req-1", 1000).is_err());
        assert!(b.trust_decay_context("agent-1").is_none());
    }

    #[test]
    fn codebase_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.check_code_policy("my_func", "behavior").unwrap());
        assert!(b.code_impact("my_func").is_none());
    }

    #[test]
    fn vision_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.capture_violation_evidence("screenshot").is_err());
        assert!(b.link_to_capture("entity-1", 1).is_err());
    }

    #[test]
    fn comm_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.broadcast_policy_change("pol-1", 1).is_err());
        assert!(b.send_approval_request("req-1", "approver").is_err());
        assert!(b.notify_violation("viol-1", 1).is_err());
    }

    #[test]
    fn bridge_config_defaults_all_false() {
        let cfg = BridgeConfig::default();
        assert!(!cfg.memory_enabled);
        assert!(!cfg.identity_enabled);
        assert!(!cfg.time_enabled);
        assert!(!cfg.codebase_enabled);
        assert!(!cfg.vision_enabled);
        assert!(!cfg.comm_enabled);
    }

    #[test]
    fn noop_bridges_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NoOpBridges>();
    }

    #[test]
    fn noop_bridges_default_and_clone() {
        let b = NoOpBridges;
        let _b2 = b.clone();
    }
}

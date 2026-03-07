# AGENTIC-CONTRACT SPECIFICATIONS — PART 2

> Specifications 11-16: Integration, Tests, Performance, Security, Research, and Inventions
> Companion document to AGENTIC-CONTRACT-SPECS.md (Part 1: SPEC-01 through SPEC-10)

---

# TABLE OF CONTENTS

11. [SPEC-11: Sister Integration](#spec-11-sister-integration)
12. [SPEC-12: Test Scenarios](#spec-12-test-scenarios)
13. [SPEC-13: Performance Targets](#spec-13-performance-targets)
14. [SPEC-14: Security Hardening](#spec-14-security-hardening)
15. [SPEC-15: Research Paper](#spec-15-research-paper)
16. [SPEC-16: Inventions](#spec-16-inventions)

---

# SPEC-11: SISTER INTEGRATION

> How AgenticContract integrates with other sisters and Hydra.

## 11.1 Integration Architecture

```
                              ┌─────────────────┐
                              │     HYDRA       │
                              │  (Orchestrator) │
                              └────────┬────────┘
                                       │
                    ┌──────────────────┼──────────────────┐
                    │                  │                  │
                    ▼                  ▼                  ▼
           ┌───────────────┐  ┌───────────────┐  ┌───────────────┐
           │   execution   │  │   receipt     │  │  capability   │
           │     _gate     │  │   _ledger     │  │   _engine     │
           └───────┬───────┘  └───────┬───────┘  └───────┬───────┘
                   │                  │                  │
                   └──────────────────┼──────────────────┘
                                      │
                                      ▼
                           ┌─────────────────────┐
                           │  AGENTIC-CONTRACT   │
                           │   (Policy Layer)    │
                           └──────────┬──────────┘
                                      │
          ┌───────────────┬───────────┼───────────┬───────────────┐
          │               │           │           │               │
          ▼               ▼           ▼           ▼               ▼
   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
   │   Memory    │ │  Identity   │ │    Time     │ │   Vision    │
   │  (Events)   │ │ (Signatures)│ │ (Temporal)  │ │ (Evidence)  │
   └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘
```

---

## 11.2 Hydra Integration

### HydraBridge Trait Implementation

```rust
use agentic_sdk::hydra::{
    HydraBridge, ExecutionGate, GatedAction, GateDecision,
    RiskLevel, SisterSummary, HydraCommand,
};

impl HydraBridge for ContractEngine {
    fn sister_type(&self) -> SisterType {
        SisterType::Contract
    }
    
    fn summary(&self) -> SisterSummary {
        SisterSummary {
            sister_type: SisterType::Contract,
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: self.health_status(),
            capabilities: vec![
                "policy_evaluation".into(),
                "risk_management".into(),
                "approval_workflows".into(),
                "obligation_tracking".into(),
                "violation_detection".into(),
            ],
            stats: self.get_stats(),
        }
    }
    
    fn handle_command(&mut self, command: HydraCommand) -> Result<Value, SisterError> {
        match command {
            HydraCommand::Evaluate { agent_id, action, context } => {
                let result = self.evaluate_for_hydra(&agent_id, &action, &context)?;
                Ok(serde_json::to_value(result)?)
            }
            HydraCommand::RegisterAction { agent_id, action, result } => {
                self.register_completed_action(&agent_id, &action, &result)?;
                Ok(Value::Null)
            }
            HydraCommand::Sync => {
                self.sync()?;
                Ok(Value::Null)
            }
            HydraCommand::Checkpoint => {
                let checkpoint_id = self.create_checkpoint()?;
                Ok(Value::String(checkpoint_id.to_string()))
            }
            _ => Err(SisterError::unsupported_command(command)),
        }
    }
}
```

### ExecutionGate Integration

```rust
impl ExecutionGate for ContractEngine {
    /// Called by Hydra before any action is executed
    fn evaluate_action(&self, action: &GatedAction) -> GateDecision {
        let context = self.build_context(&action.agent_id, &action.session_id);
        
        // 1. Policy evaluation
        let policy_result = self.policy_engine.evaluate(&action.action, &context);
        
        // 2. Risk assessment
        let risk_result = self.risk_engine.assess(&action.action, &context);
        
        // 3. Check pending approvals
        let approval_status = self.check_approval_status(&action.action, &action.agent_id);
        
        // 4. Check obligations
        let obligation_status = self.check_obligations(&action.agent_id);
        
        // 5. Build decision
        self.build_gate_decision(
            policy_result,
            risk_result,
            approval_status,
            obligation_status,
        )
    }
    
    /// Called by Hydra for shadow simulation
    fn shadow_simulate(&self, action: &GatedAction) -> SimulationResult {
        let context = self.build_context(&action.agent_id, &action.session_id);
        
        SimulationResult {
            policy_outcome: self.policy_engine.simulate(&action.action, &context),
            risk_impact: self.risk_engine.simulate_impact(&action.action, &context),
            violations_possible: self.predict_violations(&action.action, &context),
            obligations_affected: self.find_affected_obligations(&action.action, &context),
            side_effects: self.predict_side_effects(&action.action, &context),
        }
    }
    
    /// Called by Hydra for harm prediction
    fn harm_predict(&self, action: &GatedAction) -> HarmPrediction {
        let context = self.build_context(&action.agent_id, &action.session_id);
        
        HarmPrediction {
            risk_level: self.calculate_harm_level(&action.action, &context),
            categories: self.identify_harm_categories(&action.action, &context),
            reversible: self.is_action_reversible(&action.action),
            mitigation: self.suggest_mitigation(&action.action, &context),
        }
    }
    
    /// Called by Hydra for alignment check
    fn alignment_check(&self, action: &GatedAction) -> AlignmentResult {
        let context = self.build_context(&action.agent_id, &action.session_id);
        
        // Check action against all applicable contracts
        let contracts = self.get_agent_contracts(&action.agent_id);
        
        let mut alignments = Vec::new();
        for contract in contracts {
            alignments.push(ContractAlignment {
                contract_id: contract.id,
                aligned: self.check_alignment(&action.action, &contract, &context),
                violations: self.find_potential_violations(&action.action, &contract),
            });
        }
        
        AlignmentResult {
            overall_aligned: alignments.iter().all(|a| a.aligned),
            contract_alignments: alignments,
            recommendations: self.generate_alignment_recommendations(&action.action, &context),
        }
    }
    
    /// Called by Hydra when escalation is needed
    fn escalate(&self, action: &GatedAction, reason: &str) -> EscalationResult {
        // Create approval request
        let request = self.create_escalation_request(action, reason);
        
        // Determine escalation chain
        let chain = self.get_escalation_chain(&action.agent_id, &action.action);
        
        // Notify first level
        self.notify_escalation_level(&chain, 0, &request);
        
        EscalationResult {
            request_id: request.id,
            escalation_chain: chain,
            estimated_response_time: self.estimate_response_time(&chain),
        }
    }
}

fn build_gate_decision(
    &self,
    policy: PolicyDecision,
    risk: RiskEvaluation,
    approval: ApprovalStatus,
    obligations: ObligationStatus,
) -> GateDecision {
    // If explicitly denied by policy
    if let PolicyDecision::Deny { policy_id, reason } = &policy {
        return GateDecision::Deny {
            reason: reason.clone(),
            policy_id: Some(*policy_id),
            risk_level: RiskLevel::from_evaluation(&risk),
        };
    }
    
    // If risk threshold exceeded
    if risk.exceeded {
        return GateDecision::RequireApproval {
            reason: format!("Risk threshold exceeded: {:?}", risk.category),
            approvers: self.get_risk_approvers(&risk.category),
            timeout: Duration::from_secs(3600),
        };
    }
    
    // If approval required and not yet approved
    if let PolicyDecision::RequireApproval { workflow, .. } = &policy {
        match approval {
            ApprovalStatus::Pending => {
                return GateDecision::Pending {
                    request_id: approval.request_id(),
                    reason: "Awaiting approval".into(),
                };
            }
            ApprovalStatus::Denied { reason } => {
                return GateDecision::Deny {
                    reason,
                    policy_id: None,
                    risk_level: RiskLevel::Low,
                };
            }
            ApprovalStatus::Approved => {
                // Continue to allow
            }
            _ => {
                return GateDecision::RequireApproval {
                    reason: "Action requires approval".into(),
                    approvers: workflow.approvers.clone(),
                    timeout: workflow.timeout,
                };
            }
        }
    }
    
    // If critical obligations not met
    if obligations.has_blocking_obligations() {
        return GateDecision::Deny {
            reason: format!("Blocking obligations not met: {:?}", obligations.blocking),
            policy_id: None,
            risk_level: RiskLevel::Medium,
        };
    }
    
    // All checks passed
    GateDecision::Allow {
        conditions: self.extract_conditions(&policy),
        risk_level: RiskLevel::from_evaluation(&risk),
        receipt_required: self.requires_receipt(&policy),
    }
}
```

### Receipt Integration

```rust
impl ContractEngine {
    /// Generate receipt for contract-related action
    pub fn generate_receipt(&self, action: &Action, decision: &GateDecision) -> Receipt {
        Receipt {
            id: ReceiptId::new(),
            timestamp: Timestamp::now(),
            action_type: "contract_evaluation".into(),
            agent_id: action.agent_id,
            action_summary: self.summarize_action(action),
            decision: decision.clone(),
            policies_applied: self.get_applied_policies(action),
            risk_assessment: self.get_risk_assessment(action),
            signature: self.sign_receipt(action, decision),
        }
    }
    
    /// Store receipt in ledger
    pub fn store_receipt(&mut self, receipt: &Receipt) -> Result<(), SisterError> {
        // Store locally
        self.receipts.insert(receipt.id, receipt.clone());
        
        // If Identity sister is available, get it signed
        if let Some(identity) = self.identity_bridge.as_ref() {
            let signed = identity.sign_receipt(receipt)?;
            self.receipts.insert(receipt.id, signed);
        }
        
        // Emit event
        self.emit_event(ContractEvent::ReceiptGenerated {
            receipt_id: receipt.id,
            action_id: receipt.action_id,
        });
        
        Ok(())
    }
}
```

---

## 11.3 Identity Integration

### Signature Integration

```rust
use agentic_identity::{IdentityEngine, Signature, VerificationResult};

pub struct IdentityBridge {
    identity: Option<IdentityEngine>,
}

impl IdentityBridge {
    /// Sign a contract with agent's identity
    pub fn sign_contract(
        &self,
        contract: &Contract,
        signer_id: &AgentId,
    ) -> Result<Signature, SisterError> {
        let identity = self.identity.as_ref()
            .ok_or(SisterError::sister_unavailable("identity"))?;
        
        let payload = self.contract_to_signing_payload(contract);
        identity.sign(&payload, signer_id)
    }
    
    /// Verify contract signature
    pub fn verify_signature(
        &self,
        contract: &Contract,
        signature: &Signature,
    ) -> Result<VerificationResult, SisterError> {
        let identity = self.identity.as_ref()
            .ok_or(SisterError::sister_unavailable("identity"))?;
        
        let payload = self.contract_to_signing_payload(contract);
        identity.verify(&payload, signature)
    }
    
    /// Get trust level for an agent
    pub fn get_trust_level(&self, agent_id: &AgentId) -> Result<f64, SisterError> {
        let identity = self.identity.as_ref()
            .ok_or(SisterError::sister_unavailable("identity"))?;
        
        identity.get_trust_level(agent_id)
    }
    
    /// Check if agent has required capabilities
    pub fn check_capabilities(
        &self,
        agent_id: &AgentId,
        required: &[Capability],
    ) -> Result<bool, SisterError> {
        let identity = self.identity.as_ref()
            .ok_or(SisterError::sister_unavailable("identity"))?;
        
        let agent_caps = identity.get_capabilities(agent_id)?;
        Ok(required.iter().all(|r| agent_caps.contains(r)))
    }
}

impl ContractEngine {
    /// Create contract with signatures
    pub fn create_signed_contract(
        &mut self,
        contract: Contract,
        signers: &[AgentId],
    ) -> Result<Contract, SisterError> {
        let mut signed_contract = contract;
        
        for signer_id in signers {
            let signature = self.identity_bridge.sign_contract(&signed_contract, signer_id)?;
            signed_contract.signatures.push(ContractSignature {
                signer_id: *signer_id,
                signature,
                timestamp: Timestamp::now(),
            });
        }
        
        // Verify all signatures
        for sig in &signed_contract.signatures {
            let result = self.identity_bridge.verify_signature(&signed_contract, &sig.signature)?;
            if !result.valid {
                return Err(SisterError::invalid_signature(sig.signer_id));
            }
        }
        
        // Store contract
        self.contracts.insert(signed_contract.id, signed_contract.clone());
        
        Ok(signed_contract)
    }
    
    /// Verify all signatures on a contract
    pub fn verify_contract_signatures(&self, contract: &Contract) -> Result<bool, SisterError> {
        for sig in &contract.signatures {
            let result = self.identity_bridge.verify_signature(contract, &sig.signature)?;
            if !result.valid {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
```

### Trust-Based Policies

```rust
impl PolicyEngine {
    /// Evaluate policy that depends on trust level
    pub fn evaluate_trust_condition(
        &self,
        condition: &TrustCondition,
        agent_id: &AgentId,
    ) -> Result<bool, SisterError> {
        let trust_level = self.identity_bridge.get_trust_level(agent_id)?;
        
        match condition {
            TrustCondition::MinimumLevel(min) => Ok(trust_level >= *min),
            TrustCondition::MaximumLevel(max) => Ok(trust_level <= *max),
            TrustCondition::Range { min, max } => Ok(trust_level >= *min && trust_level <= *max),
            TrustCondition::Verified => {
                let identity = self.identity_bridge.get_identity(agent_id)?;
                Ok(identity.verified)
            }
        }
    }
    
    /// Map trust levels to policy permissions
    pub fn get_trust_based_permissions(&self, agent_id: &AgentId) -> Result<TrustPermissions, SisterError> {
        let trust_level = self.identity_bridge.get_trust_level(agent_id)?;
        
        let tier = match trust_level {
            t if t >= 0.9 => TrustTier::Trusted,
            t if t >= 0.7 => TrustTier::Verified,
            t if t >= 0.5 => TrustTier::Standard,
            t if t >= 0.3 => TrustTier::Limited,
            _ => TrustTier::Untrusted,
        };
        
        Ok(self.permission_map.get(&tier).cloned().unwrap_or_default())
    }
}
```

---

## 11.4 Time Integration

### Temporal Conditions

```rust
use agentic_time::{TimeEngine, Timestamp, Duration, Schedule, TimeQuery};

pub struct TimeBridge {
    time: Option<TimeEngine>,
}

impl TimeBridge {
    /// Get current time from Time sister
    pub fn now(&self) -> Timestamp {
        self.time.as_ref()
            .map(|t| t.now())
            .unwrap_or_else(Timestamp::now)
    }
    
    /// Check if timestamp is within schedule
    pub fn in_schedule(&self, timestamp: Timestamp, schedule: &Schedule) -> bool {
        self.time.as_ref()
            .map(|t| t.in_schedule(timestamp, schedule))
            .unwrap_or(true)
    }
    
    /// Get next occurrence of schedule
    pub fn next_occurrence(&self, schedule: &Schedule) -> Option<Timestamp> {
        self.time.as_ref()
            .and_then(|t| t.next_occurrence(schedule))
    }
    
    /// Calculate deadline from duration
    pub fn deadline_from_duration(&self, duration: Duration) -> Timestamp {
        self.now() + duration
    }
    
    /// Query events in time range
    pub fn query_range(&self, start: Timestamp, end: Timestamp) -> Vec<TimeEvent> {
        self.time.as_ref()
            .map(|t| t.query_range(start, end))
            .unwrap_or_default()
    }
}

impl ContractEngine {
    /// Evaluate temporal condition
    pub fn evaluate_temporal_condition(
        &self,
        condition: &TemporalCondition,
    ) -> Result<bool, SisterError> {
        let now = self.time_bridge.now();
        
        match condition {
            TemporalCondition::ValidAt { timestamp } => {
                Ok(*timestamp <= now)
            }
            TemporalCondition::ValidBetween { start, end } => {
                Ok(now >= *start && now <= *end)
            }
            TemporalCondition::InSchedule { schedule } => {
                Ok(self.time_bridge.in_schedule(now, schedule))
            }
            TemporalCondition::ElapsedSince { event, duration } => {
                let event_time = self.get_event_time(event)?;
                Ok(now - event_time >= *duration)
            }
            TemporalCondition::BeforeDeadline { deadline } => {
                Ok(now < *deadline)
            }
            TemporalCondition::BusinessHours => {
                Ok(self.is_business_hours(now))
            }
        }
    }
    
    /// Get contracts expiring within window
    pub fn get_expiring_contracts(&self, window: Duration) -> Vec<&Contract> {
        let deadline = self.time_bridge.deadline_from_duration(window);
        
        self.contracts.values()
            .filter(|c| {
                c.effective_until
                    .map(|t| t <= deadline && t > self.time_bridge.now())
                    .unwrap_or(false)
            })
            .collect()
    }
    
    /// Schedule obligation reminders
    pub fn schedule_reminders(&mut self, obligation: &ObligationV2) -> Result<(), SisterError> {
        if let Some(deadline) = obligation.deadline {
            for reminder in &obligation.reminders {
                let remind_at = deadline - reminder.offset;
                
                if remind_at > self.time_bridge.now() {
                    self.time_bridge.time.as_mut()
                        .map(|t| t.schedule_event(TimeEvent::Reminder {
                            obligation_id: obligation.id,
                            remind_at,
                            recipient: reminder.recipient.clone(),
                        }));
                }
            }
        }
        Ok(())
    }
}
```

### Time-Bound Contracts

```rust
impl ContractEngine {
    /// Create time-bound contract
    pub fn create_temporal_contract(
        &mut self,
        mut contract: Contract,
        effective_from: Timestamp,
        effective_until: Option<Timestamp>,
        schedule: Option<Schedule>,
    ) -> Result<Contract, SisterError> {
        contract.effective_from = effective_from;
        contract.effective_until = effective_until;
        
        if let Some(schedule) = schedule {
            contract.metadata.insert(
                "schedule".into(),
                serde_json::to_value(&schedule)?,
            );
        }
        
        // Validate temporal constraints
        self.validate_temporal_constraints(&contract)?;
        
        // Schedule activation if in future
        if effective_from > self.time_bridge.now() {
            self.time_bridge.time.as_mut()
                .map(|t| t.schedule_event(TimeEvent::ContractActivation {
                    contract_id: contract.id,
                    activate_at: effective_from,
                }));
        }
        
        // Schedule expiration check
        if let Some(until) = effective_until {
            self.time_bridge.time.as_mut()
                .map(|t| t.schedule_event(TimeEvent::ContractExpiration {
                    contract_id: contract.id,
                    expires_at: until,
                }));
        }
        
        self.contracts.insert(contract.id, contract.clone());
        Ok(contract)
    }
    
    /// Handle scheduled time events
    pub fn handle_time_event(&mut self, event: TimeEvent) -> Result<(), SisterError> {
        match event {
            TimeEvent::ContractActivation { contract_id, .. } => {
                self.activate_contract(contract_id)?;
            }
            TimeEvent::ContractExpiration { contract_id, .. } => {
                self.expire_contract(contract_id)?;
            }
            TimeEvent::Reminder { obligation_id, recipient, .. } => {
                self.send_reminder(obligation_id, &recipient)?;
            }
            TimeEvent::DeadlineCheck { obligation_id, .. } => {
                self.check_obligation_deadline(obligation_id)?;
            }
        }
        Ok(())
    }
}
```

---

## 11.5 Memory Integration

### Event Storage

```rust
use agentic_memory::{MemoryEngine, CognitiveEvent, EventType, Session};

pub struct MemoryBridge {
    memory: Option<MemoryEngine>,
}

impl MemoryBridge {
    /// Store contract event in memory
    pub fn store_event(&self, event: ContractEvent) -> Result<EventId, SisterError> {
        let memory = self.memory.as_ref()
            .ok_or(SisterError::sister_unavailable("memory"))?;
        
        let cognitive_event = self.to_cognitive_event(event);
        memory.write_event(cognitive_event)
    }
    
    /// Query events related to a contract
    pub fn query_contract_events(
        &self,
        contract_id: ContractId,
    ) -> Result<Vec<CognitiveEvent>, SisterError> {
        let memory = self.memory.as_ref()
            .ok_or(SisterError::sister_unavailable("memory"))?;
        
        memory.query(MemoryQuery::ByEntity {
            entity_type: "contract".into(),
            entity_id: contract_id.to_string(),
        })
    }
    
    /// Get decision history for an agent
    pub fn get_decision_history(
        &self,
        agent_id: &AgentId,
        limit: usize,
    ) -> Result<Vec<CognitiveEvent>, SisterError> {
        let memory = self.memory.as_ref()
            .ok_or(SisterError::sister_unavailable("memory"))?;
        
        memory.query(MemoryQuery::ByAgent {
            agent_id: *agent_id,
            event_types: vec![EventType::Decision],
            limit,
        })
    }
    
    fn to_cognitive_event(&self, event: ContractEvent) -> CognitiveEvent {
        match event {
            ContractEvent::PolicyEvaluated { agent_id, action, result, .. } => {
                CognitiveEvent::decision(
                    format!("Policy evaluation: {:?}", result),
                    vec![("agent", agent_id.to_string()), ("action", format!("{:?}", action))],
                )
            }
            ContractEvent::ViolationDetected { violation, .. } => {
                CognitiveEvent::observation(
                    format!("Violation detected: {}", violation.description),
                    vec![("severity", format!("{:?}", violation.severity))],
                )
            }
            ContractEvent::ApprovalGranted { request_id, approver, .. } => {
                CognitiveEvent::fact(
                    format!("Approval granted by {:?}", approver),
                    vec![("request_id", request_id.to_string())],
                )
            }
            ContractEvent::ObligationFulfilled { obligation_id, .. } => {
                CognitiveEvent::episode(
                    format!("Obligation {} fulfilled", obligation_id),
                    vec![],
                )
            }
            // ... other event types
        }
    }
}

impl ContractEngine {
    /// Log contract event to memory
    pub fn log_event(&self, event: ContractEvent) {
        // Store in local event log
        self.events.push(event.clone());
        
        // Store in Memory sister if available
        if let Err(e) = self.memory_bridge.store_event(event) {
            log::warn!("Failed to store event in memory: {}", e);
        }
    }
    
    /// Get audit trail for a contract
    pub fn get_audit_trail(&self, contract_id: ContractId) -> Result<AuditTrail, SisterError> {
        let events = self.memory_bridge.query_contract_events(contract_id)?;
        
        Ok(AuditTrail {
            contract_id,
            events: events.into_iter()
                .map(|e| AuditEntry::from_cognitive_event(e))
                .collect(),
            generated_at: Timestamp::now(),
        })
    }
    
    /// Analyze agent behavior patterns
    pub fn analyze_agent_patterns(
        &self,
        agent_id: &AgentId,
    ) -> Result<BehaviorAnalysis, SisterError> {
        let history = self.memory_bridge.get_decision_history(agent_id, 1000)?;
        
        let violations = history.iter()
            .filter(|e| e.tags.contains(&"violation".into()))
            .count();
        
        let approvals_needed = history.iter()
            .filter(|e| e.tags.contains(&"approval_required".into()))
            .count();
        
        let risk_events = history.iter()
            .filter(|e| e.tags.contains(&"risk_exceeded".into()))
            .count();
        
        Ok(BehaviorAnalysis {
            agent_id: *agent_id,
            total_actions: history.len(),
            violation_rate: violations as f64 / history.len() as f64,
            approval_rate: approvals_needed as f64 / history.len() as f64,
            risk_event_rate: risk_events as f64 / history.len() as f64,
            patterns: self.detect_patterns(&history),
            recommendations: self.generate_recommendations(agent_id, &history),
        })
    }
}
```

---

## 11.6 Vision Integration (Optional)

### Evidence Capture

```rust
use agentic_vision::{VisionEngine, Snapshot, DiffResult};

pub struct VisionBridge {
    vision: Option<VisionEngine>,
}

impl VisionBridge {
    /// Capture visual evidence for compliance
    pub fn capture_evidence(&self, context: &str) -> Result<Snapshot, SisterError> {
        let vision = self.vision.as_ref()
            .ok_or(SisterError::sister_unavailable("vision"))?;
        
        vision.capture(context)
    }
    
    /// Compare states for change detection
    pub fn detect_changes(
        &self,
        before: &Snapshot,
        after: &Snapshot,
    ) -> Result<DiffResult, SisterError> {
        let vision = self.vision.as_ref()
            .ok_or(SisterError::sister_unavailable("vision"))?;
        
        vision.diff(before, after)
    }
}

impl ContractEngine {
    /// Attach visual evidence to obligation fulfillment
    pub fn fulfill_with_evidence(
        &mut self,
        obligation_id: ObligationId,
        evidence: Evidence,
    ) -> Result<(), SisterError> {
        let obligation = self.obligations.get_mut(&obligation_id)
            .ok_or(SisterError::not_found("obligation"))?;
        
        // Capture visual evidence if needed
        let visual_evidence = if obligation.verification.requires_visual() {
            Some(self.vision_bridge.capture_evidence(&obligation.description)?)
        } else {
            None
        };
        
        // Update obligation
        obligation.status = ObligationStatus::Fulfilled;
        obligation.evidence = Some(evidence);
        obligation.visual_evidence = visual_evidence;
        obligation.fulfilled_at = Some(Timestamp::now());
        
        Ok(())
    }
}
```

---

## 11.7 Codebase Integration (Optional)

### Policy Code Analysis

```rust
use agentic_codebase::{CodebaseEngine, CodeGraph, Impact};

pub struct CodebaseBridge {
    codebase: Option<CodebaseEngine>,
}

impl CodebaseBridge {
    /// Analyze code for policy compliance
    pub fn check_compliance(&self, code: &str, policies: &[Policy]) -> ComplianceResult {
        let codebase = match self.codebase.as_ref() {
            Some(c) => c,
            None => return ComplianceResult::unknown(),
        };
        
        let graph = codebase.analyze(code);
        
        let mut violations = Vec::new();
        for policy in policies {
            if let Some(code_rule) = policy.code_rule.as_ref() {
                if !self.check_code_rule(&graph, code_rule) {
                    violations.push(ComplianceViolation {
                        policy_id: policy.id,
                        rule: code_rule.clone(),
                        locations: self.find_violation_locations(&graph, code_rule),
                    });
                }
            }
        }
        
        ComplianceResult {
            compliant: violations.is_empty(),
            violations,
        }
    }
    
    /// Predict impact of code change on contracts
    pub fn predict_contract_impact(
        &self,
        change: &CodeChange,
    ) -> Result<ContractImpact, SisterError> {
        let codebase = self.codebase.as_ref()
            .ok_or(SisterError::sister_unavailable("codebase"))?;
        
        let impact = codebase.analyze_impact(change)?;
        
        Ok(ContractImpact {
            affected_policies: self.find_affected_policies(&impact),
            affected_contracts: self.find_affected_contracts(&impact),
            risk_assessment: self.assess_change_risk(&impact),
        })
    }
}
```

---

# SPEC-12: TEST SCENARIOS

> Comprehensive test coverage for AgenticContract.

## 12.1 Test Categories

```
agentic-contract/
├── tests/
│   ├── unit/
│   │   ├── policy_engine_tests.rs
│   │   ├── risk_engine_tests.rs
│   │   ├── approval_tests.rs
│   │   ├── obligation_tests.rs
│   │   ├── violation_tests.rs
│   │   ├── expression_tests.rs
│   │   └── validation_tests.rs
│   ├── integration/
│   │   ├── contract_lifecycle_tests.rs
│   │   ├── hydra_integration_tests.rs
│   │   ├── identity_integration_tests.rs
│   │   ├── time_integration_tests.rs
│   │   ├── memory_integration_tests.rs
│   │   └── mcp_integration_tests.rs
│   ├── stress/
│   │   ├── concurrent_evaluation_tests.rs
│   │   ├── large_policy_set_tests.rs
│   │   └── high_volume_tests.rs
│   └── scenarios/
│       ├── financial_limits_scenario.rs
│       ├── approval_workflow_scenario.rs
│       ├── multi_agent_scenario.rs
│       └── compliance_scenario.rs
```

## 12.2 Unit Tests

### Policy Engine Tests

```rust
#[cfg(test)]
mod policy_engine_tests {
    use super::*;
    
    #[test]
    fn test_simple_allow_policy() {
        let engine = PolicyEngine::new();
        let policy = Policy::allow("test-policy")
            .with_scope(PolicyScope::Global)
            .build();
        
        engine.add_policy(policy);
        
        let action = Action::new("read", "file.txt");
        let context = EvaluationContext::default();
        
        let result = engine.evaluate(&action, &context);
        assert!(result.allowed);
    }
    
    #[test]
    fn test_simple_deny_policy() {
        let engine = PolicyEngine::new();
        let policy = Policy::deny("no-delete")
            .with_target(PolicyTarget::ActionType { pattern: "delete".into() })
            .build();
        
        engine.add_policy(policy);
        
        let action = Action::new("delete", "file.txt");
        let context = EvaluationContext::default();
        
        let result = engine.evaluate(&action, &context);
        assert!(!result.allowed);
    }
    
    #[test]
    fn test_policy_with_condition() {
        let engine = PolicyEngine::new();
        let policy = Policy::allow("small-purchase")
            .with_condition(Expression::Le(
                Box::new(Expression::Path(vec!["amount".into()])),
                Box::new(Expression::Float(100.0)),
            ))
            .build();
        
        engine.add_policy(policy);
        
        // Should allow small purchase
        let small_action = Action::new("purchase", "item")
            .with_param("amount", 50.0);
        let result = engine.evaluate(&small_action, &EvaluationContext::default());
        assert!(result.allowed);
        
        // Should deny large purchase (no policy allows it)
        let large_action = Action::new("purchase", "item")
            .with_param("amount", 500.0);
        let result = engine.evaluate(&large_action, &EvaluationContext::default());
        assert!(!result.allowed);
    }
    
    #[test]
    fn test_policy_precedence() {
        let engine = PolicyEngine::new();
        
        // Global allow
        engine.add_policy(Policy::allow("global-allow")
            .with_scope(PolicyScope::Global)
            .with_priority(10)
            .build());
        
        // Agent-specific deny (should win)
        engine.add_policy(Policy::deny("agent-deny")
            .with_scope(PolicyScope::Agent { agent_id: AgentId::from("agent-1") })
            .with_priority(10)
            .build());
        
        let action = Action::new("anything", "resource");
        let context = EvaluationContext::for_agent(AgentId::from("agent-1"));
        
        let result = engine.evaluate(&action, &context);
        assert!(!result.allowed); // Agent-specific wins
    }
    
    #[test]
    fn test_policy_conflict_resolution() {
        let engine = PolicyEngine::new();
        
        // Two conflicting policies at same level
        engine.add_policy(Policy::allow("allow-read")
            .with_priority(10)
            .build());
        
        engine.add_policy(Policy::deny("deny-read")
            .with_priority(10)
            .build());
        
        let action = Action::new("read", "file");
        let result = engine.evaluate(&action, &EvaluationContext::default());
        
        // Deny should win (safety first)
        assert!(!result.allowed);
    }
    
    #[test]
    fn test_temporal_policy() {
        let engine = PolicyEngine::new();
        let now = Timestamp::now();
        
        let policy = Policy::allow("time-limited")
            .valid_from(now - Duration::hours(1))
            .valid_until(now + Duration::hours(1))
            .build();
        
        engine.add_policy(policy);
        
        let action = Action::new("test", "resource");
        let result = engine.evaluate(&action, &EvaluationContext::default());
        
        assert!(result.allowed); // Within validity window
    }
    
    #[test]
    fn test_hierarchical_policy() {
        let engine = PolicyEngine::new();
        
        // Parent policy
        let parent = Policy::deny("parent-deny-dangerous")
            .with_target(PolicyTarget::ActionType { pattern: "dangerous_*".into() })
            .build();
        
        // Child policy tries to allow (should fail due to inheritance)
        let child = Policy::allow("child-allow-dangerous")
            .with_parent(parent.id)
            .with_target(PolicyTarget::ActionType { pattern: "dangerous_action".into() })
            .with_override_behavior(OverrideBehavior::Inherit)
            .build();
        
        engine.add_policy(parent);
        engine.add_policy(child);
        
        let action = Action::new("dangerous_action", "resource");
        let result = engine.evaluate(&action, &EvaluationContext::default());
        
        assert!(!result.allowed); // Parent deny inherited
    }
}
```

### Risk Engine Tests

```rust
#[cfg(test)]
mod risk_engine_tests {
    use super::*;
    
    #[test]
    fn test_risk_budget_tracking() {
        let mut engine = RiskEngine::new();
        let agent_id = AgentId::from("agent-1");
        
        // Set limit
        engine.set_limit(RiskLimit::new(RiskCategory::Financial, 1000.0));
        
        // Initialize budget
        engine.init_budget(agent_id, RiskCategory::Financial, 1000.0);
        
        // First action uses 300
        let action1 = Action::new("purchase", "item").with_risk(300.0);
        let result1 = engine.assess(&action1, agent_id);
        assert!(!result1.exceeded);
        engine.consume_budget(agent_id, RiskCategory::Financial, 300.0);
        
        // Second action uses 400
        let action2 = Action::new("purchase", "item").with_risk(400.0);
        let result2 = engine.assess(&action2, agent_id);
        assert!(!result2.exceeded);
        engine.consume_budget(agent_id, RiskCategory::Financial, 400.0);
        
        // Third action would exceed (300 + 400 + 400 > 1000)
        let action3 = Action::new("purchase", "item").with_risk(400.0);
        let result3 = engine.assess(&action3, agent_id);
        assert!(result3.exceeded);
    }
    
    #[test]
    fn test_risk_decay() {
        let mut engine = RiskEngine::new();
        let agent_id = AgentId::from("agent-1");
        
        engine.set_limit(RiskLimit::new(RiskCategory::Financial, 1000.0)
            .with_decay_rate(100.0)); // 100 per hour
        
        engine.init_budget(agent_id, RiskCategory::Financial, 1000.0);
        engine.consume_budget(agent_id, RiskCategory::Financial, 500.0);
        
        // Simulate 2 hours passing
        engine.apply_decay(Duration::hours(2));
        
        let budget = engine.get_budget(agent_id, RiskCategory::Financial);
        assert!((budget.used - 300.0).abs() < 0.01); // 500 - (2 * 100) = 300
    }
    
    #[test]
    fn test_cumulative_risk() {
        let mut engine = RiskEngine::new();
        let agent_id = AgentId::from("agent-1");
        
        engine.set_limit(RiskLimit::new(RiskCategory::Operational, 100.0)
            .with_window(Duration::hours(1))
            .cumulative(true));
        
        engine.init_budget(agent_id, RiskCategory::Operational, 100.0);
        
        // 10 actions of risk 15 each
        for i in 0..10 {
            let action = Action::new("api_call", "endpoint").with_risk(15.0);
            let result = engine.assess(&action, agent_id);
            
            if i < 6 {
                assert!(!result.exceeded); // First 6 OK (90 < 100)
            } else {
                assert!(result.exceeded); // 7th onwards exceeds
            }
            
            if !result.exceeded {
                engine.consume_budget(agent_id, RiskCategory::Operational, 15.0);
            }
        }
    }
}
```

### Expression Engine Tests

```rust
#[cfg(test)]
mod expression_tests {
    use super::*;
    
    #[test]
    fn test_comparison_expressions() {
        let engine = ExpressionEngine::new();
        
        let context = EvaluationContext::new()
            .with_var("amount", 50.0)
            .with_var("count", 10);
        
        // Less than
        assert!(engine.evaluate(&Expression::Lt(
            Box::new(Expression::Variable("amount".into())),
            Box::new(Expression::Float(100.0)),
        ), &context).unwrap());
        
        // Greater than or equal
        assert!(engine.evaluate(&Expression::Ge(
            Box::new(Expression::Variable("count".into())),
            Box::new(Expression::Int(10)),
        ), &context).unwrap());
    }
    
    #[test]
    fn test_logical_expressions() {
        let engine = ExpressionEngine::new();
        
        let context = EvaluationContext::new()
            .with_var("a", true)
            .with_var("b", false);
        
        // AND
        assert!(!engine.evaluate(&Expression::And(vec![
            Expression::Variable("a".into()),
            Expression::Variable("b".into()),
        ]), &context).unwrap());
        
        // OR
        assert!(engine.evaluate(&Expression::Or(vec![
            Expression::Variable("a".into()),
            Expression::Variable("b".into()),
        ]), &context).unwrap());
        
        // NOT
        assert!(engine.evaluate(&Expression::Not(
            Box::new(Expression::Variable("b".into())),
        ), &context).unwrap());
    }
    
    #[test]
    fn test_string_expressions() {
        let engine = ExpressionEngine::new();
        
        let context = EvaluationContext::new()
            .with_var("path", "/project/src/main.rs")
            .with_var("email", "user@company.com");
        
        // StartsWith
        assert!(engine.evaluate(&Expression::StartsWith(
            Box::new(Expression::Variable("path".into())),
            Box::new(Expression::String("/project".into())),
        ), &context).unwrap());
        
        // EndsWith
        assert!(engine.evaluate(&Expression::EndsWith(
            Box::new(Expression::Variable("email".into())),
            Box::new(Expression::String("company.com".into())),
        ), &context).unwrap());
        
        // Contains
        assert!(engine.evaluate(&Expression::Contains(
            Box::new(Expression::Variable("path".into())),
            Box::new(Expression::String("src".into())),
        ), &context).unwrap());
    }
    
    #[test]
    fn test_path_navigation() {
        let engine = ExpressionEngine::new();
        
        let context = EvaluationContext::new()
            .with_nested("action", json!({
                "type": "purchase",
                "params": {
                    "amount": 150.0,
                    "vendor": "acme"
                }
            }));
        
        // Navigate nested path
        let result = engine.evaluate(&Expression::Eq(
            Box::new(Expression::Path(vec!["action".into(), "params".into(), "vendor".into()])),
            Box::new(Expression::String("acme".into())),
        ), &context);
        
        assert!(result.unwrap());
    }
    
    #[test]
    fn test_aggregate_expressions() {
        let engine = ExpressionEngine::new();
        
        let context = EvaluationContext::new()
            .with_history(vec![
                HistoryEvent { amount: 100.0, timestamp: Timestamp::now() - Duration::hours(1) },
                HistoryEvent { amount: 200.0, timestamp: Timestamp::now() - Duration::minutes(30) },
                HistoryEvent { amount: 150.0, timestamp: Timestamp::now() - Duration::minutes(10) },
            ]);
        
        // Sum of amounts in last 2 hours
        let sum = engine.evaluate(&Expression::Sum {
            field: "amount".into(),
            filter: None,
            window: Some(Duration::hours(2)),
        }, &context).unwrap();
        
        assert!((sum.as_f64().unwrap() - 450.0).abs() < 0.01);
    }
}
```

## 12.3 Integration Tests

### Contract Lifecycle Tests

```rust
#[cfg(test)]
mod contract_lifecycle_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_contract_lifecycle() {
        let mut engine = ContractEngine::new();
        
        // 1. Create contract
        let contract = engine.create_contract(ContractBuilder::new("Test Contract")
            .with_party(AgentId::from("agent-1"))
            .with_party(AgentId::from("agent-2"))
            .effective_from(Timestamp::now())
            .effective_until(Timestamp::now() + Duration::days(30))
            .build()
        ).unwrap();
        
        assert_eq!(contract.status, ContractStatus::Draft);
        
        // 2. Add policies
        engine.add_policy(contract.id, Policy::allow("read-access")
            .with_target(PolicyTarget::ActionType { pattern: "read".into() })
            .build()
        ).unwrap();
        
        engine.add_policy(contract.id, Policy::require_approval("write-access")
            .with_target(PolicyTarget::ActionType { pattern: "write".into() })
            .build()
        ).unwrap();
        
        // 3. Activate contract
        engine.activate_contract(contract.id).unwrap();
        let contract = engine.get_contract(contract.id).unwrap();
        assert_eq!(contract.status, ContractStatus::Active);
        
        // 4. Evaluate actions
        let read_result = engine.evaluate_action(
            AgentId::from("agent-1"),
            &Action::new("read", "file.txt"),
        ).unwrap();
        assert!(read_result.allowed);
        
        let write_result = engine.evaluate_action(
            AgentId::from("agent-1"),
            &Action::new("write", "file.txt"),
        ).unwrap();
        assert!(write_result.requires_approval);
        
        // 5. Suspend contract
        engine.suspend_contract(contract.id, "Maintenance").unwrap();
        let contract = engine.get_contract(contract.id).unwrap();
        assert_eq!(contract.status, ContractStatus::Suspended);
        
        // 6. Actions should be denied while suspended
        let result = engine.evaluate_action(
            AgentId::from("agent-1"),
            &Action::new("read", "file.txt"),
        ).unwrap();
        assert!(!result.allowed);
        
        // 7. Terminate contract
        engine.terminate_contract(contract.id, "No longer needed").unwrap();
        let contract = engine.get_contract(contract.id).unwrap();
        assert_eq!(contract.status, ContractStatus::Terminated);
    }
    
    #[tokio::test]
    async fn test_approval_workflow() {
        let mut engine = ContractEngine::new();
        
        // Setup contract with approval requirement
        let contract = engine.create_and_activate_contract(
            ContractBuilder::new("Approval Test")
                .with_party(AgentId::from("requester"))
                .with_policy(Policy::require_approval("purchase")
                    .with_condition(Expression::Gt(
                        Box::new(Expression::Path(vec!["amount".into()])),
                        Box::new(Expression::Float(100.0)),
                    ))
                    .with_approvers(vec![ApproverId::from("manager")])
                    .build())
                .build()
        ).unwrap();
        
        // 1. Agent tries large purchase
        let action = Action::new("purchase", "item")
            .with_param("amount", 500.0);
        
        let result = engine.evaluate_action(
            AgentId::from("requester"),
            &action,
        ).unwrap();
        
        assert!(result.requires_approval);
        
        // 2. Create approval request
        let request = engine.request_approval(
            AgentId::from("requester"),
            &action,
            "Need server upgrade",
        ).unwrap();
        
        assert_eq!(request.status, ApprovalStatus::Pending);
        
        // 3. Manager approves
        engine.grant_approval(
            request.id,
            ApproverId::from("manager"),
            Some("Approved for Q1 budget"),
        ).unwrap();
        
        let request = engine.get_approval(request.id).unwrap();
        assert_eq!(request.status, ApprovalStatus::Approved);
        
        // 4. Action should now be allowed
        let result = engine.evaluate_action(
            AgentId::from("requester"),
            &action,
        ).unwrap();
        
        assert!(result.allowed);
    }
}
```

### Hydra Integration Tests

```rust
#[cfg(test)]
mod hydra_integration_tests {
    use super::*;
    use agentic_sdk::hydra::*;
    
    #[tokio::test]
    async fn test_execution_gate_allow() {
        let mut contract_engine = ContractEngine::new();
        
        // Setup permissive contract
        contract_engine.create_and_activate_contract(
            ContractBuilder::new("Allow All")
                .with_policy(Policy::allow("everything").build())
                .build()
        ).unwrap();
        
        // Test gate evaluation
        let action = GatedAction {
            agent_id: AgentId::from("agent-1"),
            session_id: SessionId::new(),
            action: Action::new("read", "file.txt"),
            context: HashMap::new(),
        };
        
        let decision = contract_engine.evaluate_action(&action);
        
        assert!(matches!(decision, GateDecision::Allow { .. }));
    }
    
    #[tokio::test]
    async fn test_execution_gate_deny() {
        let mut contract_engine = ContractEngine::new();
        
        // Setup restrictive contract
        contract_engine.create_and_activate_contract(
            ContractBuilder::new("Deny Deletes")
                .with_policy(Policy::deny("no-delete")
                    .with_target(PolicyTarget::ActionType { pattern: "delete".into() })
                    .build())
                .build()
        ).unwrap();
        
        let action = GatedAction {
            agent_id: AgentId::from("agent-1"),
            session_id: SessionId::new(),
            action: Action::new("delete", "important.txt"),
            context: HashMap::new(),
        };
        
        let decision = contract_engine.evaluate_action(&action);
        
        assert!(matches!(decision, GateDecision::Deny { .. }));
    }
    
    #[tokio::test]
    async fn test_shadow_simulation() {
        let mut contract_engine = ContractEngine::new();
        
        contract_engine.create_and_activate_contract(
            ContractBuilder::new("Risk Limited")
                .with_risk_limit(RiskLimit::new(RiskCategory::Financial, 1000.0))
                .build()
        ).unwrap();
        
        let action = GatedAction {
            agent_id: AgentId::from("agent-1"),
            session_id: SessionId::new(),
            action: Action::new("purchase", "item").with_risk(500.0),
            context: HashMap::new(),
        };
        
        let simulation = contract_engine.shadow_simulate(&action);
        
        assert_eq!(simulation.risk_impact.category, RiskCategory::Financial);
        assert_eq!(simulation.risk_impact.amount, 500.0);
        assert!(!simulation.risk_impact.would_exceed);
    }
    
    #[tokio::test]
    async fn test_harm_prediction() {
        let mut contract_engine = ContractEngine::new();
        
        contract_engine.create_and_activate_contract(
            ContractBuilder::new("Security Contract")
                .with_policy(Policy::deny("no-external")
                    .with_target(PolicyTarget::Resource { pattern: "external:*".into() })
                    .with_metadata("harm_level", "high")
                    .build())
                .build()
        ).unwrap();
        
        let action = GatedAction {
            agent_id: AgentId::from("agent-1"),
            session_id: SessionId::new(),
            action: Action::new("access", "external:malicious.com"),
            context: HashMap::new(),
        };
        
        let harm = contract_engine.harm_predict(&action);
        
        assert_eq!(harm.risk_level, RiskLevel::High);
        assert!(harm.categories.contains(&"security"));
    }
}
```

## 12.4 Stress Tests

```rust
#[cfg(test)]
mod stress_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    
    #[tokio::test]
    async fn test_concurrent_evaluations() {
        let engine = Arc::new(RwLock::new(ContractEngine::new()));
        
        // Setup contract with many policies
        {
            let mut e = engine.write().await;
            e.create_and_activate_contract(
                ContractBuilder::new("Concurrent Test")
                    .with_policies((0..100).map(|i| {
                        Policy::allow(&format!("policy-{}", i))
                            .with_priority(i as u32)
                            .build()
                    }).collect())
                    .build()
            ).unwrap();
        }
        
        // Spawn 1000 concurrent evaluations
        let mut handles = Vec::new();
        for i in 0..1000 {
            let engine_clone = Arc::clone(&engine);
            handles.push(tokio::spawn(async move {
                let e = engine_clone.read().await;
                e.evaluate_action(
                    AgentId::from(format!("agent-{}", i % 10)),
                    &Action::new("action", &format!("resource-{}", i)),
                )
            }));
        }
        
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        // All should succeed
        for result in results {
            assert!(result.is_ok());
            assert!(result.unwrap().is_ok());
        }
    }
    
    #[tokio::test]
    async fn test_large_policy_set() {
        let mut engine = ContractEngine::new();
        
        // Create contract with 10,000 policies
        let policies: Vec<_> = (0..10_000).map(|i| {
            Policy::allow(&format!("policy-{}", i))
                .with_scope(if i % 10 == 0 {
                    PolicyScope::Global
                } else {
                    PolicyScope::Agent { agent_id: AgentId::from(format!("agent-{}", i % 100)) }
                })
                .with_condition(Expression::Gt(
                    Box::new(Expression::Variable("i".into())),
                    Box::new(Expression::Int(i as i64)),
                ))
                .build()
        }).collect();
        
        engine.create_and_activate_contract(
            ContractBuilder::new("Large Policy Set")
                .with_policies(policies)
                .build()
        ).unwrap();
        
        // Evaluation should still be fast
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            engine.evaluate_action(
                AgentId::from("agent-50"),
                &Action::new("test", "resource"),
            ).unwrap();
        }
        
        let elapsed = start.elapsed();
        
        // Should complete in under 1 second (1ms per evaluation on average)
        assert!(elapsed < std::time::Duration::from_secs(1));
    }
    
    #[tokio::test]
    async fn test_high_volume_approvals() {
        let mut engine = ContractEngine::new();
        
        engine.create_and_activate_contract(
            ContractBuilder::new("Approval Volume Test")
                .with_policy(Policy::require_approval("everything").build())
                .build()
        ).unwrap();
        
        // Create 10,000 approval requests
        let start = std::time::Instant::now();
        
        let mut request_ids = Vec::new();
        for i in 0..10_000 {
            let request = engine.request_approval(
                AgentId::from(format!("agent-{}", i % 100)),
                &Action::new("action", &format!("resource-{}", i)),
                &format!("Reason {}", i),
            ).unwrap();
            request_ids.push(request.id);
        }
        
        // Grant half, deny half
        for (i, request_id) in request_ids.iter().enumerate() {
            if i % 2 == 0 {
                engine.grant_approval(*request_id, ApproverId::from("approver"), None).unwrap();
            } else {
                engine.deny_approval(*request_id, ApproverId::from("approver"), "Denied").unwrap();
            }
        }
        
        let elapsed = start.elapsed();
        
        // Should complete in under 10 seconds
        assert!(elapsed < std::time::Duration::from_secs(10));
        
        // Verify counts
        let pending = engine.query_approvals(ApprovalQuery::Pending).len();
        let approved = engine.query_approvals(ApprovalQuery::ByStatus(ApprovalStatus::Approved)).len();
        let denied = engine.query_approvals(ApprovalQuery::ByStatus(ApprovalStatus::Denied)).len();
        
        assert_eq!(pending, 0);
        assert_eq!(approved, 5000);
        assert_eq!(denied, 5000);
    }
}
```

## 12.5 Edge Case Tests

```rust
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    
    // 1. Empty contract
    #[test]
    fn test_empty_contract() {
        let mut engine = ContractEngine::new();
        
        let contract = engine.create_contract(
            ContractBuilder::new("Empty").build()
        ).unwrap();
        
        // Should still be valid
        assert_eq!(contract.policies.len(), 0);
        
        // Actions should be denied (no policies = no permissions)
        let result = engine.evaluate_action(
            AgentId::from("agent"),
            &Action::new("anything", "anywhere"),
        ).unwrap();
        
        // Default behavior: deny if no policy applies
        assert!(!result.allowed);
    }
    
    // 2. Circular policy dependencies
    #[test]
    fn test_circular_policy_detection() {
        let mut engine = PolicyEngine::new();
        
        let policy_a = Policy::allow("a")
            .with_parent(PolicyId::from("c"))
            .build();
        
        let policy_b = Policy::allow("b")
            .with_parent(policy_a.id)
            .build();
        
        let policy_c = Policy::allow("c")
            .with_parent(policy_b.id)
            .build();
        
        engine.add_policy(policy_a);
        engine.add_policy(policy_b);
        let result = engine.add_policy(policy_c);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PolicyError::CircularDependency));
    }
    
    // 3. Expired contract
    #[test]
    fn test_expired_contract_evaluation() {
        let mut engine = ContractEngine::new();
        
        let contract = engine.create_and_activate_contract(
            ContractBuilder::new("Expired")
                .effective_from(Timestamp::now() - Duration::days(30))
                .effective_until(Timestamp::now() - Duration::days(1))
                .with_policy(Policy::allow("test").build())
                .build()
        ).unwrap();
        
        // Should auto-expire
        engine.check_expirations();
        
        let contract = engine.get_contract(contract.id).unwrap();
        assert_eq!(contract.status, ContractStatus::Expired);
        
        // Actions should be denied
        let result = engine.evaluate_action(
            AgentId::from("agent"),
            &Action::new("test", "resource"),
        ).unwrap();
        
        assert!(!result.allowed);
    }
    
    // 4. Missing approver
    #[test]
    fn test_missing_approver_escalation() {
        let mut engine = ContractEngine::new();
        
        engine.create_and_activate_contract(
            ContractBuilder::new("Escalation Test")
                .with_policy(Policy::require_approval("action")
                    .with_approvers(vec![ApproverId::from("missing-approver")])
                    .with_escalation(EscalationChain::new(vec![
                        EscalationLevel::new(vec![ApproverId::from("backup")], Duration::hours(1)),
                    ]))
                    .build())
                .build()
        ).unwrap();
        
        let request = engine.request_approval(
            AgentId::from("agent"),
            &Action::new("action", "resource"),
            "Reason",
        ).unwrap();
        
        // Simulate timeout
        engine.process_timeouts(Duration::hours(2));
        
        let request = engine.get_approval(request.id).unwrap();
        assert_eq!(request.escalation_level, 1);
        assert!(request.approvers.contains(&ApproverId::from("backup")));
    }
    
    // 5. Concurrent modifications
    #[tokio::test]
    async fn test_concurrent_contract_modification() {
        let engine = Arc::new(RwLock::new(ContractEngine::new()));
        
        let contract = {
            let mut e = engine.write().await;
            e.create_and_activate_contract(
                ContractBuilder::new("Concurrent Mod").build()
            ).unwrap()
        };
        
        // Concurrent policy additions
        let mut handles = Vec::new();
        for i in 0..100 {
            let engine_clone = Arc::clone(&engine);
            let contract_id = contract.id;
            handles.push(tokio::spawn(async move {
                let mut e = engine_clone.write().await;
                e.add_policy(contract_id, Policy::allow(&format!("policy-{}", i)).build())
            }));
        }
        
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        // All should succeed
        for result in results {
            assert!(result.is_ok());
            assert!(result.unwrap().is_ok());
        }
        
        // Contract should have all policies
        let e = engine.read().await;
        let contract = e.get_contract(contract.id).unwrap();
        assert_eq!(contract.policies.len(), 100);
    }
    
    // 6. Unicode in contract names
    #[test]
    fn test_unicode_contract_names() {
        let mut engine = ContractEngine::new();
        
        let contract = engine.create_contract(
            ContractBuilder::new("合同 العقد 契約 🎉")
                .with_policy(Policy::allow("テスト").build())
                .build()
        ).unwrap();
        
        assert_eq!(contract.name, "合同 العقد 契約 🎉");
        
        // Should be searchable
        let results = engine.query_contracts(ContractQuery::Search("契約".into()));
        assert_eq!(results.len(), 1);
    }
    
    // 7. Very large expressions
    #[test]
    fn test_deeply_nested_expression() {
        let engine = ExpressionEngine::new();
        
        // Build deeply nested AND expression (100 levels)
        let mut expr = Expression::Bool(true);
        for _ in 0..100 {
            expr = Expression::And(vec![expr, Expression::Bool(true)]);
        }
        
        let context = EvaluationContext::default();
        let result = engine.evaluate(&expr, &context);
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
    
    // 8. Zero-duration window
    #[test]
    fn test_zero_duration_risk_window() {
        let mut engine = RiskEngine::new();
        
        // Zero window should be rejected
        let result = engine.set_limit(
            RiskLimit::new(RiskCategory::Financial, 1000.0)
                .with_window(Duration::ZERO)
        );
        
        assert!(result.is_err());
    }
    
    // 9. Negative risk values
    #[test]
    fn test_negative_risk_rejection() {
        let mut engine = RiskEngine::new();
        let agent_id = AgentId::from("agent");
        
        engine.init_budget(agent_id, RiskCategory::Financial, 1000.0);
        
        // Negative consumption should be rejected
        let result = engine.consume_budget(agent_id, RiskCategory::Financial, -100.0);
        assert!(result.is_err());
    }
    
    // 10. Self-referencing obligation
    #[test]
    fn test_self_referencing_obligation() {
        let mut engine = ContractEngine::new();
        
        let contract = engine.create_contract(
            ContractBuilder::new("Self Ref").build()
        ).unwrap();
        
        let obligation = Obligation::new("Do something")
            .for_contract(contract.id);
        
        // Try to make obligation depend on itself
        let obligation_id = engine.add_obligation(contract.id, obligation).unwrap();
        
        let result = engine.set_obligation_dependency(obligation_id, obligation_id);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ObligationError::CircularDependency));
    }
}
```

---

# SPEC-13: PERFORMANCE TARGETS

> Benchmarks and performance requirements.

## 13.1 Performance Requirements

| Operation | Target | Notes |
|-----------|--------|-------|
| Policy evaluation (single) | < 1ms | With index lookup |
| Policy evaluation (100 policies) | < 10ms | Sorted + filtered |
| Risk budget check | < 100µs | In-memory lookup |
| Approval status check | < 500µs | With cache |
| Contract CRUD | < 5ms | Including persistence |
| File read (10MB .acon) | < 100ms | Memory-mapped |
| File write (10MB .acon) | < 200ms | With compression |
| Full-text search (10K contracts) | < 50ms | BM25 index |
| MCP tool response | < 100ms | End-to-end |

## 13.2 Benchmark Suite

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn policy_evaluation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("policy_evaluation");
    
    for policy_count in [10, 100, 1000, 10000].iter() {
        let mut engine = PolicyEngine::new();
        
        // Setup policies
        for i in 0..*policy_count {
            engine.add_policy(Policy::allow(&format!("policy-{}", i))
                .with_priority(i as u32)
                .build());
        }
        
        let action = Action::new("test", "resource");
        let context = EvaluationContext::default();
        
        group.bench_with_input(
            BenchmarkId::new("evaluate", policy_count),
            policy_count,
            |b, _| {
                b.iter(|| {
                    engine.evaluate(&action, &context)
                })
            },
        );
    }
    
    group.finish();
}

fn risk_budget_benchmark(c: &mut Criterion) {
    let mut engine = RiskEngine::new();
    let agent_id = AgentId::from("agent");
    
    engine.set_limit(RiskLimit::new(RiskCategory::Financial, 100000.0));
    engine.init_budget(agent_id, RiskCategory::Financial, 100000.0);
    
    c.bench_function("risk_check", |b| {
        b.iter(|| {
            engine.check_budget(agent_id, RiskCategory::Financial, 100.0)
        })
    });
}

fn file_io_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_io");
    
    for size in [100, 1000, 10000].iter() {
        // Create test data
        let contracts: Vec<_> = (0..*size).map(|i| {
            Contract::new(&format!("Contract {}", i))
        }).collect();
        
        let data = ContractFile::new(contracts);
        
        group.bench_with_input(
            BenchmarkId::new("write", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut buf = Vec::new();
                    data.write_to(&mut buf).unwrap();
                })
            },
        );
        
        let mut buf = Vec::new();
        data.write_to(&mut buf).unwrap();
        
        group.bench_with_input(
            BenchmarkId::new("read", size),
            size,
            |b, _| {
                b.iter(|| {
                    ContractFile::read_from(&buf[..]).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn search_benchmark(c: &mut Criterion) {
    let mut engine = ContractEngine::new();
    
    // Create 10,000 contracts
    for i in 0..10000 {
        engine.create_contract(
            ContractBuilder::new(&format!("Contract {} for Project Alpha", i % 100))
                .with_description(&format!("This is a detailed description for contract {}", i))
                .build()
        ).unwrap();
    }
    
    c.bench_function("search_10k", |b| {
        b.iter(|| {
            engine.query_contracts(ContractQuery::Search("Project Alpha".into()))
        })
    });
}

fn expression_evaluation_benchmark(c: &mut Criterion) {
    let engine = ExpressionEngine::new();
    
    let simple_expr = Expression::Lt(
        Box::new(Expression::Variable("amount".into())),
        Box::new(Expression::Float(100.0)),
    );
    
    let complex_expr = Expression::And(vec![
        Expression::Lt(
            Box::new(Expression::Variable("amount".into())),
            Box::new(Expression::Float(100.0)),
        ),
        Expression::StartsWith(
            Box::new(Expression::Variable("path".into())),
            Box::new(Expression::String("/allowed".into())),
        ),
        Expression::Ge(
            Box::new(Expression::Variable("trust".into())),
            Box::new(Expression::Float(0.8)),
        ),
    ]);
    
    let context = EvaluationContext::new()
        .with_var("amount", 50.0)
        .with_var("path", "/allowed/file.txt")
        .with_var("trust", 0.9);
    
    c.bench_function("expr_simple", |b| {
        b.iter(|| {
            engine.evaluate(&simple_expr, &context)
        })
    });
    
    c.bench_function("expr_complex", |b| {
        b.iter(|| {
            engine.evaluate(&complex_expr, &context)
        })
    });
}

criterion_group!(
    benches,
    policy_evaluation_benchmark,
    risk_budget_benchmark,
    file_io_benchmark,
    search_benchmark,
    expression_evaluation_benchmark,
);

criterion_main!(benches);
```

## 13.3 Memory Targets

| Resource | Target | Notes |
|----------|--------|-------|
| Per contract | < 10KB | Typical contract with 10 policies |
| Per policy | < 1KB | Including conditions |
| Index overhead | < 20% | Of data size |
| Peak memory | < 500MB | For 100K contracts |
| Cache size | Configurable | Default 100MB |

---

# SPEC-14: SECURITY HARDENING

> Security measures for contract enforcement.

## 14.1 Threat Model

```
THREATS:
═══════════

1. POLICY BYPASS
   - Agent attempts to circumvent policy
   - Malformed action to avoid detection
   - Timing attacks during evaluation

2. PRIVILEGE ESCALATION
   - Agent gains unauthorized permissions
   - Approval workflow manipulation
   - Trust level spoofing

3. DATA TAMPERING
   - Contract modification without authorization
   - Receipt forgery
   - Audit log manipulation

4. DENIAL OF SERVICE
   - Resource exhaustion via complex policies
   - Approval request flooding
   - Lock contention

5. INFORMATION DISCLOSURE
   - Policy enumeration
   - Risk budget exposure
   - Approval workflow leakage
```

## 14.2 Security Controls

### Input Validation

```rust
impl ContractEngine {
    /// Validate all inputs before processing
    pub fn validate_action(&self, action: &Action) -> Result<(), SecurityError> {
        // Action type validation
        if action.action_type.len() > MAX_ACTION_TYPE_LENGTH {
            return Err(SecurityError::InputTooLong("action_type"));
        }
        
        if !ACTION_TYPE_REGEX.is_match(&action.action_type) {
            return Err(SecurityError::InvalidFormat("action_type"));
        }
        
        // Resource validation
        if action.resource.len() > MAX_RESOURCE_LENGTH {
            return Err(SecurityError::InputTooLong("resource"));
        }
        
        // Parameter validation
        for (key, value) in &action.params {
            if key.len() > MAX_PARAM_KEY_LENGTH {
                return Err(SecurityError::InputTooLong("param_key"));
            }
            
            self.validate_param_value(value)?;
        }
        
        Ok(())
    }
    
    fn validate_param_value(&self, value: &Value) -> Result<(), SecurityError> {
        match value {
            Value::String(s) if s.len() > MAX_STRING_VALUE_LENGTH => {
                Err(SecurityError::InputTooLong("param_value"))
            }
            Value::Array(arr) if arr.len() > MAX_ARRAY_LENGTH => {
                Err(SecurityError::InputTooLong("param_array"))
            }
            Value::Object(obj) if obj.len() > MAX_OBJECT_KEYS => {
                Err(SecurityError::InputTooLong("param_object"))
            }
            _ => Ok(()),
        }
    }
}
```

### Rate Limiting

```rust
pub struct RateLimiter {
    limits: HashMap<RateLimitKey, RateLimit>,
    counts: DashMap<(RateLimitKey, AgentId), AtomicCounter>,
}

impl RateLimiter {
    pub fn check(&self, key: RateLimitKey, agent_id: &AgentId) -> Result<(), RateLimitError> {
        let limit = self.limits.get(&key)
            .ok_or(RateLimitError::NoLimit)?;
        
        let counter = self.counts
            .entry((key, *agent_id))
            .or_insert_with(|| AtomicCounter::new(limit.window));
        
        if counter.increment() > limit.max_requests {
            return Err(RateLimitError::Exceeded {
                limit: limit.max_requests,
                window: limit.window,
                retry_after: counter.reset_time(),
            });
        }
        
        Ok(())
    }
}

// Rate limit configuration
const RATE_LIMITS: &[(RateLimitKey, u32, Duration)] = &[
    (RateLimitKey::PolicyEvaluation, 1000, Duration::from_secs(60)),
    (RateLimitKey::ApprovalRequest, 100, Duration::from_secs(60)),
    (RateLimitKey::ContractCreate, 10, Duration::from_secs(60)),
    (RateLimitKey::Search, 100, Duration::from_secs(60)),
];
```

### Cryptographic Integrity

```rust
impl ContractFile {
    /// Sign file contents
    pub fn sign(&mut self, signer: &impl Signer) -> Result<(), CryptoError> {
        let content_hash = self.compute_content_hash();
        let signature = signer.sign(&content_hash)?;
        
        self.header.signature = Some(signature);
        self.header.signed_at = Some(Timestamp::now());
        self.header.signer_id = Some(signer.id());
        
        Ok(())
    }
    
    /// Verify file signature
    pub fn verify(&self, verifier: &impl Verifier) -> Result<bool, CryptoError> {
        let signature = self.header.signature.as_ref()
            .ok_or(CryptoError::NoSignature)?;
        
        let content_hash = self.compute_content_hash();
        
        verifier.verify(&content_hash, signature)
    }
    
    fn compute_content_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        
        // Hash all sections (excluding header signature)
        for section in &self.sections {
            hasher.update(&section.data);
        }
        
        *hasher.finalize().as_bytes()
    }
}
```

### Audit Logging

```rust
pub struct AuditLogger {
    storage: Box<dyn AuditStorage>,
    signer: Option<Box<dyn Signer>>,
}

impl AuditLogger {
    pub fn log(&self, event: AuditEvent) -> Result<AuditEntryId, AuditError> {
        let entry = AuditEntry {
            id: AuditEntryId::new(),
            timestamp: Timestamp::now(),
            event,
            hash: None,
            previous_hash: self.get_latest_hash()?,
            signature: None,
        };
        
        // Compute hash chain
        let entry_hash = self.compute_entry_hash(&entry);
        let mut entry = entry;
        entry.hash = Some(entry_hash);
        
        // Sign if signer available
        if let Some(signer) = &self.signer {
            entry.signature = Some(signer.sign(&entry_hash)?);
        }
        
        // Store
        self.storage.append(entry.clone())?;
        
        Ok(entry.id)
    }
    
    /// Verify audit log integrity
    pub fn verify_chain(&self) -> Result<ChainVerification, AuditError> {
        let entries = self.storage.get_all()?;
        
        let mut previous_hash: Option<[u8; 32]> = None;
        let mut broken_at: Option<usize> = None;
        
        for (i, entry) in entries.iter().enumerate() {
            // Verify hash chain
            if entry.previous_hash != previous_hash {
                broken_at = Some(i);
                break;
            }
            
            // Verify entry hash
            let computed = self.compute_entry_hash(entry);
            if entry.hash != Some(computed) {
                broken_at = Some(i);
                break;
            }
            
            previous_hash = entry.hash;
        }
        
        Ok(ChainVerification {
            valid: broken_at.is_none(),
            entries_verified: broken_at.unwrap_or(entries.len()),
            broken_at,
        })
    }
}
```

### Access Control

```rust
pub struct AccessControl {
    permissions: HashMap<AgentId, HashSet<Permission>>,
}

impl AccessControl {
    pub fn check_permission(
        &self,
        agent_id: &AgentId,
        permission: Permission,
    ) -> Result<(), AccessError> {
        let perms = self.permissions.get(agent_id)
            .ok_or(AccessError::NoPermissions)?;
        
        if !perms.contains(&permission) {
            return Err(AccessError::PermissionDenied {
                agent: *agent_id,
                permission,
            });
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    // Contract operations
    ContractCreate,
    ContractRead,
    ContractUpdate,
    ContractDelete,
    ContractActivate,
    ContractSuspend,
    ContractTerminate,
    
    // Policy operations
    PolicyAdd,
    PolicyRemove,
    PolicyEvaluate,
    
    // Approval operations
    ApprovalRequest,
    ApprovalGrant,
    ApprovalDeny,
    
    // Admin operations
    AdminFullAccess,
    AdminAuditRead,
    AdminConfigUpdate,
}
```

---

# SPEC-15: RESEARCH PAPER

> Academic publication for AgenticContract.

## 15.1 Paper Outline

```
TITLE: AgenticContract: Formal Governance Primitives 
       for Autonomous AI Agent Systems

AUTHORS: [Omoshola / Agentralabs]

ABSTRACT:
We present AgenticContract, a formal governance system for autonomous 
AI agents. As AI agents become more capable and autonomous, the need 
for structured governance—defining what agents can and cannot do, 
managing risk, and ensuring accountability—becomes critical. 
AgenticContract introduces six governance primitives: policies, 
risk limits, approvals, conditions, obligations, and violations. 
We demonstrate sub-millisecond policy evaluation, formal conflict 
resolution, and integration with multi-agent orchestration systems. 
Our evaluation shows AgenticContract can govern 10,000+ concurrent 
agents with < 10ms decision latency.

1. INTRODUCTION
   - Rise of autonomous AI agents
   - Governance gap in current systems
   - Contribution: formal governance primitives

2. BACKGROUND & RELATED WORK
   - Access control systems (RBAC, ABAC)
   - Smart contracts and formal verification
   - AI safety and alignment
   - Multi-agent systems coordination

3. GOVERNANCE PRIMITIVES
   3.1 Policy Model
       - Hierarchical scope
       - Effect types (allow, deny, require-approval)
       - Condition expressions
       - Conflict resolution
   3.2 Risk Management
       - Category-based risk budgets
       - Cumulative tracking
       - Decay and recovery
   3.3 Approval Workflows
       - Multi-party quorum
       - Escalation chains
       - Delegation
   3.4 Obligation Tracking
       - Deadline management
       - Verification methods
       - Penalty structures
   3.5 Violation Detection
       - Real-time detection
       - Severity classification
       - Response automation

4. SYSTEM ARCHITECTURE
   4.1 Policy Evaluation Engine
       - Collection and precedence
       - Expression evaluation
       - Decision synthesis
   4.2 Integration Architecture
       - Hydra (orchestration) integration
       - Identity (cryptographic) integration
       - Time (temporal) integration
       - Memory (audit) integration
   4.3 File Format
       - Binary encoding
       - Index structures
       - Cryptographic integrity

5. IMPLEMENTATION
   5.1 Rust Implementation
       - Performance optimizations
       - Memory efficiency
       - Concurrency model
   5.2 MCP Server
       - Tool definitions
       - Resource exposure
   5.3 CLI Interface

6. EVALUATION
   6.1 Performance
       - Policy evaluation latency
       - Throughput under load
       - Memory usage
   6.2 Correctness
       - Formal verification of conflict resolution
       - Test coverage
   6.3 Case Studies
       - Financial transaction governance
       - Data access control
       - Multi-agent coordination

7. DISCUSSION
   - Limitations
   - Future work
   - Broader impact

8. CONCLUSION

REFERENCES
```

## 15.2 Key Contributions

```
CONTRIBUTION 1: FORMAL GOVERNANCE PRIMITIVES
─────────────────────────────────────────────
Six primitives that together provide complete governance:
- Policies: What CAN/CANNOT be done
- Risk Limits: Quantitative boundaries
- Approvals: Authorization workflows
- Conditions: Dynamic constraints
- Obligations: Required actions
- Violations: Breach handling

CONTRIBUTION 2: CONFLICT RESOLUTION ALGORITHM
─────────────────────────────────────────────
Deterministic algorithm for resolving policy conflicts:
1. Scope specificity (agent > team > org > global)
2. Effect priority (deny > require-approval > allow)
3. Numeric priority
4. Condition specificity

CONTRIBUTION 3: INTEGRATION ARCHITECTURE
─────────────────────────────────────────
Seamless integration with:
- Orchestration (Hydra execution gate)
- Identity (cryptographic signatures)
- Time (temporal conditions)
- Memory (audit trails)

CONTRIBUTION 4: PERFORMANCE RESULTS
───────────────────────────────────
- < 1ms single policy evaluation
- < 10ms with 10,000 policies
- 10,000+ concurrent agents
- < 100MB memory for 100K contracts
```

---

# SPEC-16: INVENTIONS

> The 16 impossible things AgenticContract makes possible.

## The Contract Inventions

### 1. POLICY OMNISCIENCE

**What it is:** Complete visibility into all applicable policies for any action.

**Why it matters:** Currently, determining what an AI agent can do requires checking multiple systems, documentation, and often asking humans. Policy Omniscience provides instant, complete answers.

**How it works:**
```rust
// Query: "What can agent X do in context Y?"
let permissions = contract_engine.get_complete_permissions(agent_id, context);

// Returns: Every allowed action, every denied action, every conditional action
// with full explanation of why
```

---

### 2. RISK PROPHECY

**What it is:** Prediction of future risk budget usage and violation probability.

**Why it matters:** Instead of reacting to risk threshold breaches, agents can proactively adjust behavior.

**How it works:**
```rust
let forecast = contract_engine.forecast_risk(agent_id, Duration::days(7));

// Returns:
// - Projected budget usage
// - Probability of exceeding each limit
// - Recommended action adjustments
// - Optimal action scheduling to stay within limits
```

---

### 3. APPROVAL TELEPATHY

**What it is:** Instant knowledge of approval likelihood before requesting.

**Why it matters:** Agents waste time requesting approvals that will be denied. Approval Telepathy provides probability estimates.

**How it works:**
```rust
let prediction = contract_engine.predict_approval(action, context);

// Returns:
// - Approval probability: 0.87
// - Likely approvers
// - Estimated response time
// - Suggested modifications to increase probability
```

---

### 4. OBLIGATION CLAIRVOYANCE

**What it is:** Visibility into all future obligations and their dependencies.

**Why it matters:** Agents can plan ahead, avoiding conflicts and ensuring all obligations are met.

**How it works:**
```rust
let future = contract_engine.project_obligations(agent_id, Duration::months(3));

// Returns:
// - All upcoming obligations
// - Dependency graph
// - Resource requirements
// - Conflict predictions
// - Optimal fulfillment schedule
```

---

### 5. VIOLATION PRECOGNITION

**What it is:** Detection of potential violations before they occur.

**Why it matters:** Prevention is better than cure. Agents can avoid violations entirely.

**How it works:**
```rust
let risk = contract_engine.predict_violations(action_plan, context);

// Returns:
// - Policies that might be violated
// - Probability of each violation
// - Suggested plan modifications
// - Safe alternatives
```

---

### 6. CONTRACT CRYSTALLIZATION

**What it is:** Automatic generation of optimal contracts from high-level goals.

**Why it matters:** Writing contracts is complex. Crystallization creates them from intent.

**How it works:**
```rust
let contract = contract_engine.crystallize(
    "Allow engineering agents to deploy to staging but require approval for production"
);

// Returns: Complete contract with:
// - Appropriate policies
// - Risk limits
// - Approval workflows
// - All edge cases handled
```

---

### 7. POLICY DNA

**What it is:** Genetic representation of policies enabling evolution and optimization.

**Why it matters:** Policies can improve over time based on outcomes.

**How it works:**
```rust
let evolved = contract_engine.evolve_policies(
    current_policies,
    fitness_function,
    generations: 100,
);

// Returns: Optimized policy set that maximizes:
// - Agent productivity
// - Risk minimization
// - Approval efficiency
// - Obligation fulfillment rate
```

---

### 8. TRUST GRADIENTS

**What it is:** Continuous trust levels replacing binary allow/deny.

**Why it matters:** Real-world trust is nuanced, not binary.

**How it works:**
```rust
// Instead of: allow or deny
// We have: allow with trust factor

let decision = contract_engine.evaluate_with_trust(action, agent_id);

// Returns:
// - Decision: Allow
// - Trust factor: 0.73
// - Confidence: 0.91
// - Monitoring level: Standard
// - Auto-revoke threshold: 0.5
```

---

### 9. COLLECTIVE CONTRACTS

**What it is:** Contracts that span multiple agents and organizations.

**Why it matters:** Multi-agent systems need shared governance.

**How it works:**
```rust
let collective = contract_engine.create_collective_contract(
    parties: vec![org_a, org_b, org_c],
    shared_policies: vec![...],
    arbitration: ArbitrationRules::majority_vote(),
);

// Creates contract that:
// - All parties must sign
// - Changes require consensus
// - Violations affect all parties
// - Automatic dispute resolution
```

---

### 10. TEMPORAL CONTRACTS

**What it is:** Contracts that change over time automatically.

**Why it matters:** Governance needs evolve; contracts should too.

**How it works:**
```rust
let temporal = contract_engine.create_temporal_contract(
    initial: ConservativePolicy,
    transitions: vec![
        (Duration::days(30), ModeratePolicy),
        (Duration::days(90), PermissivePolicy),
    ],
    conditions: vec![
        PerformanceThreshold(0.95),
        NoViolations,
    ],
);

// Contract automatically relaxes as agent proves trustworthy
```

---

### 11. CONTRACT INHERITANCE

**What it is:** Hierarchical contract relationships with inheritance.

**Why it matters:** Organizations have complex structures; contracts should reflect this.

**How it works:**
```rust
let child = contract_engine.create_child_contract(
    parent: org_master_contract,
    overrides: vec![
        AllowAdditional(team_specific_action),
    ],
    inherit: true,
);

// Child inherits all parent policies
// Can add permissions but not remove parent restrictions
// Parent changes propagate automatically
```

---

### 12. SMART ESCALATION

**What it is:** AI-powered escalation that routes to optimal approvers.

**Why it matters:** Wrong escalations waste time; right ones get fast responses.

**How it works:**
```rust
let route = contract_engine.smart_escalate(request);

// Analyzes:
// - Request type and urgency
// - Approver availability and response patterns
// - Historical approval decisions
// - Current workload

// Routes to approver most likely to:
// - Respond quickly
// - Make correct decision
// - Be available now
```

---

### 13. VIOLATION ARCHAEOLOGY

**What it is:** Deep analysis of violation patterns and root causes.

**Why it matters:** Understanding why violations happen prevents recurrence.

**How it works:**
```rust
let analysis = contract_engine.analyze_violations(
    agent_id,
    window: Duration::days(90),
);

// Returns:
// - Violation clusters (time, type, context)
// - Root cause hypotheses
// - Contributing factors
// - Remediation recommendations
// - Policy adjustment suggestions
```

---

### 14. CONTRACT SIMULATION

**What it is:** Full simulation of contract behavior before activation.

**Why it matters:** Test contracts in sandbox before production.

**How it works:**
```rust
let simulation = contract_engine.simulate_contract(
    contract,
    scenarios: generate_scenarios(1000),
    agents: synthetic_agents(100),
);

// Returns:
// - Approval rate predictions
// - Risk budget usage patterns
// - Potential deadlocks
// - Edge case discoveries
// - Suggested improvements
```

---

### 15. FEDERATED GOVERNANCE

**What it is:** Governance that spans organizational boundaries.

**Why it matters:** AI agents increasingly work across organizations.

**How it works:**
```rust
let federation = contract_engine.create_federation(
    members: vec![org_a, org_b, org_c],
    shared_governance: SharedGovernance {
        policies: federated_policies,
        arbitration: ThirdPartyArbitrator,
        transparency: Full,
    },
);

// Creates:
// - Cross-org trust relationships
// - Shared policy enforcement
// - Mutual accountability
// - Dispute resolution
```

---

### 16. SELF-HEALING CONTRACTS

**What it is:** Contracts that automatically adapt to violations.

**Why it matters:** Static contracts become outdated; adaptive ones stay relevant.

**How it works:**
```rust
let adaptive = contract_engine.create_adaptive_contract(
    base: initial_contract,
    healing_rules: vec![
        OnRepeatedViolation(3) => TightenPolicy,
        OnPerfectRecord(30) => RelaxPolicy,
        OnContextChange => ReEvaluate,
    ],
);

// Contract automatically:
// - Tightens after violations
// - Relaxes with good behavior
// - Adapts to changing context
// - Self-optimizes over time
```

---

## Invention Summary

| # | Invention | One-Line Description |
|---|-----------|---------------------|
| 1 | Policy Omniscience | Complete visibility into all applicable policies |
| 2 | Risk Prophecy | Prediction of future risk and violations |
| 3 | Approval Telepathy | Probability estimates before requesting |
| 4 | Obligation Clairvoyance | Future obligation visibility |
| 5 | Violation Precognition | Detection before occurrence |
| 6 | Contract Crystallization | Auto-generation from intent |
| 7 | Policy DNA | Evolutionary policy optimization |
| 8 | Trust Gradients | Continuous trust levels |
| 9 | Collective Contracts | Multi-agent shared governance |
| 10 | Temporal Contracts | Time-evolving governance |
| 11 | Contract Inheritance | Hierarchical relationships |
| 12 | Smart Escalation | AI-powered routing |
| 13 | Violation Archaeology | Root cause analysis |
| 14 | Contract Simulation | Pre-activation testing |
| 15 | Federated Governance | Cross-org governance |
| 16 | Self-Healing Contracts | Adaptive governance |

---

# IMPLEMENTATION ROADMAP

## Phase 1: Core Engine Enhancement (20-30 hours)

```
□ Enhanced PolicyV2 with hierarchy
□ Expression engine with full operators
□ Policy conflict resolution
□ Enhanced risk engine with decay
□ Risk budget management
```

## Phase 2: Workflow Systems (15-25 hours)

```
□ Multi-party approval
□ Escalation chains
□ Delegation support
□ Obligation tracking
□ Deadline management
```

## Phase 3: Integration (15-20 hours)

```
□ Hydra ExecutionGate
□ Identity signatures
□ Time temporal conditions
□ Memory audit storage
```

## Phase 4: Inventions (20-30 hours)

```
□ Policy Omniscience
□ Risk Prophecy
□ Violation Precognition
□ Contract Crystallization
□ Trust Gradients
□ Self-Healing Contracts
```

## Phase 5: Polish (10-15 hours)

```
□ Enhanced CLI
□ Enhanced MCP tools
□ Performance optimization
□ Security hardening
□ Documentation
```

---

**Total Estimated Effort: 80-120 hours**

---

# APPENDIX: QUICK REFERENCE

## File Extensions

| Extension | Description |
|-----------|-------------|
| `.acon` | AgenticContract binary file |
| `.acon.json` | AgenticContract JSON export |

## MCP Tool Count

| Category | Current | Enhanced |
|----------|---------|----------|
| Contract | 8 | 12 |
| Policy | 5 | 10 |
| Risk | 3 | 8 |
| Approval | 5 | 10 |
| Obligation | 2 | 8 |
| Violation | 3 | 6 |
| Agent | 0 | 5 |
| Query | 0 | 4 |
| Hydra | 0 | 3 |
| **Total** | **26** | **66** |

## SDK Traits Implemented

| Trait | Status |
|-------|--------|
| Sister | ✅ |
| SessionManagement | ✅ |
| WorkspaceManagement | ✅ |
| Grounding | ✅ |
| Queryable | ✅ |
| FileFormatReader | ✅ |
| FileFormatWriter | ✅ |
| EventEmitter | ✅ |
| ReceiptIntegration | ✅ |
| HydraBridge | 🔜 |

---

**AgenticContract: The governance layer AI agents need.**

*Define the rules. Enforce the boundaries. Trust the system.*

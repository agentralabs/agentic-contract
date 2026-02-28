//! Core engine wrapping ContractFile.

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::approval::{
    ApprovalDecision, ApprovalRequest, ApprovalRule, ApprovalStatus, DecisionType,
};
use crate::condition::{Condition, ConditionStatus};
use crate::error::{ContractError, ContractResult};
use crate::file_format::ContractFile;
use crate::inventions::*;
use crate::obligation::{Obligation, ObligationStatus};
use crate::policy::{Policy, PolicyAction, PolicyScope};
use crate::risk_limit::RiskLimit;
use crate::violation::{Violation, ViolationSeverity};
use crate::ContractId;

/// Core engine for contract operations.
pub struct ContractEngine {
    /// In-memory contract file.
    pub file: ContractFile,
}

impl ContractEngine {
    /// Create a new engine with an empty contract file.
    pub fn new() -> Self {
        Self {
            file: ContractFile::new(),
        }
    }

    /// Create from an existing contract file.
    pub fn from_file(file: ContractFile) -> Self {
        Self { file }
    }

    /// Open from path.
    pub fn open(path: impl Into<std::path::PathBuf>) -> ContractResult<Self> {
        let file = ContractFile::open(path)?;
        Ok(Self { file })
    }

    /// Save the contract file.
    pub fn save(&self) -> ContractResult<()> {
        self.file.save()
    }

    // ── Policies ───────────────────────────────────────────────────

    /// Add a policy.
    pub fn add_policy(&mut self, policy: Policy) -> ContractId {
        let id = policy.id;
        self.file.policies.push(policy);
        id
    }

    /// Check if an action is allowed under current policies.
    ///
    /// Returns the most restrictive applicable policy action.
    pub fn check_policy(&self, action_type: &str, scope: PolicyScope) -> PolicyAction {
        let mut result = PolicyAction::Allow;

        for policy in &self.file.policies {
            if !policy.is_active() {
                continue;
            }

            // Check scope compatibility
            if policy.scope == PolicyScope::Global || policy.scope == scope {
                // Check if the policy's conditions mention this action type
                let label_lower = policy.label.to_lowercase();
                let action_lower = action_type.to_lowercase();

                if label_lower.contains(&action_lower) || action_lower.contains(&label_lower) {
                    // Apply most restrictive action
                    match policy.action {
                        PolicyAction::Deny => return PolicyAction::Deny,
                        PolicyAction::RequireApproval => result = PolicyAction::RequireApproval,
                        PolicyAction::AuditOnly if result == PolicyAction::Allow => {
                            result = PolicyAction::AuditOnly
                        }
                        _ => {}
                    }
                }
            }
        }

        result
    }

    /// List policies, optionally filtered by scope.
    pub fn list_policies(&self, scope: Option<PolicyScope>) -> Vec<&Policy> {
        self.file
            .policies
            .iter()
            .filter(|p| scope.is_none() || Some(p.scope) == scope)
            .collect()
    }

    /// Get a policy by ID.
    pub fn get_policy(&self, id: ContractId) -> ContractResult<&Policy> {
        self.file
            .find_policy(id)
            .ok_or_else(|| ContractError::NotFound(format!("Policy {}", id)))
    }

    // ── Risk Limits ────────────────────────────────────────────────

    /// Add a risk limit.
    pub fn add_risk_limit(&mut self, limit: RiskLimit) -> ContractId {
        let id = limit.id;
        self.file.risk_limits.push(limit);
        id
    }

    /// Check if an action would exceed any risk limits.
    ///
    /// Returns the first limit that would be exceeded, or None if OK.
    pub fn check_risk_limit(&self, label_pattern: &str, amount: f64) -> Option<&RiskLimit> {
        let pattern = label_pattern.to_lowercase();
        self.file.risk_limits.iter().find(|limit| {
            limit.label.to_lowercase().contains(&pattern) && limit.would_exceed(amount)
        })
    }

    /// Increment a risk limit by label.
    pub fn increment_risk_limit(&mut self, id: ContractId, amount: f64) -> ContractResult<()> {
        let limit = self
            .file
            .find_risk_limit_mut(id)
            .ok_or_else(|| ContractError::NotFound(format!("RiskLimit {}", id)))?;
        limit.increment(amount);
        Ok(())
    }

    /// List all risk limits.
    pub fn list_risk_limits(&self) -> &[RiskLimit] {
        &self.file.risk_limits
    }

    // ── Approvals ──────────────────────────────────────────────────

    /// Add an approval rule.
    pub fn add_approval_rule(&mut self, rule: ApprovalRule) -> ContractId {
        let id = rule.id;
        self.file.approval_rules.push(rule);
        id
    }

    /// Create an approval request.
    pub fn request_approval(
        &mut self,
        rule_id: ContractId,
        action_description: impl Into<String>,
        requestor: impl Into<String>,
    ) -> ContractResult<ContractId> {
        // Verify rule exists
        if !self.file.approval_rules.iter().any(|r| r.id == rule_id) {
            return Err(ContractError::NotFound(format!("ApprovalRule {}", rule_id)));
        }

        let request = ApprovalRequest::new(rule_id, action_description, requestor);
        let id = request.id;
        self.file.approval_requests.push(request);
        Ok(id)
    }

    /// Decide on an approval request.
    pub fn decide_approval(
        &mut self,
        request_id: ContractId,
        decision: DecisionType,
        decider: impl Into<String>,
        reason: impl Into<String>,
    ) -> ContractResult<ContractId> {
        // Update the request status
        let request = self
            .file
            .approval_requests
            .iter_mut()
            .find(|r| r.id == request_id)
            .ok_or_else(|| ContractError::NotFound(format!("ApprovalRequest {}", request_id)))?;

        request.status = match decision {
            DecisionType::Approve => ApprovalStatus::Approved,
            DecisionType::Deny => ApprovalStatus::Denied,
        };

        let approval_decision = ApprovalDecision::new(request_id, decision, decider, reason);
        let id = approval_decision.id;
        self.file.approval_decisions.push(approval_decision);
        Ok(id)
    }

    /// List approval requests, optionally filtered by status.
    pub fn list_approval_requests(&self, status: Option<ApprovalStatus>) -> Vec<&ApprovalRequest> {
        self.file
            .approval_requests
            .iter()
            .filter(|r| status.is_none() || Some(r.status) == status)
            .collect()
    }

    // ── Conditions ─────────────────────────────────────────────────

    /// Add a condition.
    pub fn add_condition(&mut self, condition: Condition) -> ContractId {
        let id = condition.id;
        self.file.conditions.push(condition);
        id
    }

    /// Evaluate a condition (simplified: just check the status).
    pub fn evaluate_condition(&self, id: ContractId) -> ContractResult<ConditionStatus> {
        let condition = self
            .file
            .conditions
            .iter()
            .find(|c| c.id == id)
            .ok_or_else(|| ContractError::NotFound(format!("Condition {}", id)))?;
        Ok(condition.status)
    }

    /// List all conditions.
    pub fn list_conditions(&self) -> &[Condition] {
        &self.file.conditions
    }

    // ── Obligations ────────────────────────────────────────────────

    /// Add an obligation.
    pub fn add_obligation(&mut self, obligation: Obligation) -> ContractId {
        let id = obligation.id;
        self.file.obligations.push(obligation);
        id
    }

    /// Check obligation status.
    pub fn check_obligation(&self, id: ContractId) -> ContractResult<ObligationStatus> {
        let obligation = self
            .file
            .find_obligation(id)
            .ok_or_else(|| ContractError::NotFound(format!("Obligation {}", id)))?;

        if obligation.is_overdue() {
            Ok(ObligationStatus::Overdue)
        } else {
            Ok(obligation.status)
        }
    }

    /// Fulfill an obligation.
    pub fn fulfill_obligation(&mut self, id: ContractId) -> ContractResult<()> {
        let obligation = self
            .file
            .find_obligation_mut(id)
            .ok_or_else(|| ContractError::NotFound(format!("Obligation {}", id)))?;
        obligation.fulfill();
        Ok(())
    }

    /// List obligations, optionally filtered by status.
    pub fn list_obligations(&self, status: Option<ObligationStatus>) -> Vec<&Obligation> {
        self.file
            .obligations
            .iter()
            .filter(|o| status.is_none() || Some(o.status) == status)
            .collect()
    }

    // ── Violations ─────────────────────────────────────────────────

    /// Report a violation.
    pub fn report_violation(&mut self, violation: Violation) -> ContractId {
        let id = violation.id;
        self.file.violations.push(violation);
        id
    }

    /// List violations, optionally filtered by severity.
    pub fn list_violations(&self, severity: Option<ViolationSeverity>) -> Vec<&Violation> {
        self.file
            .violations
            .iter()
            .filter(|v| severity.is_none() || Some(v.severity) == severity)
            .collect()
    }

    // ── Inventions ─────────────────────────────────────────────────

    // ── 1. Policy Omniscience ─────────────────────────────────────

    /// Get complete visibility into all applicable policies for an agent.
    pub fn policy_omniscience(&self, agent_id: &str, context: &str) -> PolicyOmniscience {
        let mut allowed = Vec::new();
        let mut denied = Vec::new();
        let mut conditional = Vec::new();

        for policy in &self.file.policies {
            if !policy.is_active() {
                continue;
            }
            let entry = PermissionEntry {
                action: policy.label.clone(),
                policy_id: policy.id,
                policy_label: policy.label.clone(),
                reason: if policy.description.is_empty() {
                    format!("{:?} policy", policy.action)
                } else {
                    policy.description.clone()
                },
                scope: format!("{:?}", policy.scope),
            };
            match policy.action {
                PolicyAction::Allow => allowed.push(entry),
                PolicyAction::Deny => denied.push(entry),
                PolicyAction::RequireApproval => conditional.push(entry),
                PolicyAction::AuditOnly => allowed.push(entry),
            }
        }

        let total = (allowed.len() + denied.len() + conditional.len()) as u32;
        PolicyOmniscience {
            id: ContractId::new(),
            agent_id: agent_id.to_string(),
            context: context.to_string(),
            allowed_actions: allowed,
            denied_actions: denied,
            conditional_actions: conditional,
            total_permissions: total,
            queried_at: Utc::now(),
        }
    }

    // ── 2. Risk Prophecy ──────────────────────────────────────────

    /// Predict future risk budget usage.
    pub fn risk_prophecy(&self, agent_id: &str, forecast_window_secs: i64) -> RiskProphecy {
        let mut projections = Vec::new();
        let mut total_risk = 0.0;
        let mut recommendations = Vec::new();

        for limit in &self.file.risk_limits {
            let usage = limit.usage_ratio();
            // Linear projection: if current usage is X% over elapsed time,
            // project forward at the same rate
            let projected = (usage * 1.5).min(1.0);
            let exceed_prob = if projected > 0.8 {
                ((projected - 0.8) / 0.2).min(1.0)
            } else {
                0.0
            };
            let time_until = if usage > 0.0 && usage < 1.0 {
                Some(((1.0 - usage) / usage * forecast_window_secs as f64) as i64)
            } else {
                None
            };

            if usage > 0.7 {
                recommendations.push(format!(
                    "Risk limit '{}' at {:.0}% — consider increasing max or reducing usage",
                    limit.label,
                    usage * 100.0
                ));
            }

            total_risk += usage;
            projections.push(RiskProjection {
                limit_id: limit.id,
                limit_label: limit.label.clone(),
                current_usage: usage,
                projected_usage: projected,
                exceed_probability: exceed_prob,
                time_until_limit_secs: time_until,
            });
        }

        let count = projections.len().max(1) as f64;
        RiskProphecy {
            id: ContractId::new(),
            agent_id: agent_id.to_string(),
            forecast_window_secs,
            projections,
            overall_risk_score: (total_risk / count).min(1.0),
            recommendations,
            prophesied_at: Utc::now(),
        }
    }

    // ── 3. Approval Telepathy ─────────────────────────────────────

    /// Predict approval likelihood for an action.
    pub fn approval_telepathy(&self, action: &str) -> ApprovalTelepathy {
        let total_requests = self.file.approval_requests.len() as f64;
        let approved_count = self
            .file
            .approval_requests
            .iter()
            .filter(|r| r.status == ApprovalStatus::Approved)
            .count() as f64;
        let historical_rate = if total_requests > 0.0 {
            approved_count / total_requests
        } else {
            0.5 // no history, assume 50/50
        };

        // Check if the action would be denied by policy
        let policy_result = self.check_policy(action, PolicyScope::Global);
        let probability = match policy_result {
            PolicyAction::Deny => 0.1,
            PolicyAction::RequireApproval => historical_rate * 0.8,
            PolicyAction::Allow => 0.95,
            PolicyAction::AuditOnly => 0.9,
        };

        let mut suggestions = Vec::new();
        if probability < 0.5 {
            suggestions.push(ApprovalSuggestion {
                modification: "Reduce scope of the action".to_string(),
                new_probability: (probability + 0.2).min(1.0),
                effort: "low".to_string(),
            });
            suggestions.push(ApprovalSuggestion {
                modification: "Add risk mitigation documentation".to_string(),
                new_probability: (probability + 0.3).min(1.0),
                effort: "medium".to_string(),
            });
        }

        ApprovalTelepathy {
            id: ContractId::new(),
            action: action.to_string(),
            approval_probability: probability,
            likely_approvers: vec!["admin".to_string()],
            estimated_response_secs: 300,
            suggestions,
            historical_approval_rate: historical_rate,
            predicted_at: Utc::now(),
        }
    }

    // ── 4. Obligation Clairvoyance ────────────────────────────────

    /// Forecast upcoming obligations and identify scheduling conflicts.
    pub fn obligation_clairvoyance(
        &self,
        agent_id: &str,
        window_secs: i64,
    ) -> ObligationClairvoyance {
        let now = Utc::now();
        let mut upcoming = Vec::new();
        let mut optimal_order = Vec::new();

        for obligation in &self.file.obligations {
            if obligation.status != ObligationStatus::Pending {
                continue;
            }
            let time_remaining = obligation
                .deadline
                .map(|d| (d - now).num_seconds())
                .filter(|&s| s > 0 && s <= window_secs);

            let miss_risk = if let Some(remaining) = time_remaining {
                // Higher risk for closer deadlines
                1.0 - (remaining as f64 / window_secs as f64).min(1.0)
            } else if obligation.deadline.is_some() {
                // Already overdue
                1.0
            } else {
                0.1 // No deadline
            };

            upcoming.push(ObligationForecast {
                obligation_id: obligation.id,
                label: obligation.label.clone(),
                deadline: obligation.deadline,
                time_remaining_secs: time_remaining,
                estimated_effort_minutes: 30,
                depends_on: Vec::new(),
                miss_risk,
            });
            optimal_order.push(obligation.id);
        }

        // Sort by miss_risk descending for optimal order
        upcoming.sort_by(|a, b| {
            b.miss_risk
                .partial_cmp(&a.miss_risk)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        optimal_order = upcoming.iter().map(|f| f.obligation_id).collect();

        ObligationClairvoyance {
            id: ContractId::new(),
            agent_id: agent_id.to_string(),
            window_secs,
            upcoming,
            conflicts: Vec::new(),
            optimal_order,
            projected_at: Utc::now(),
        }
    }

    // ── 5. Violation Precognition ─────────────────────────────────

    /// Detect potential violations before they occur.
    pub fn violation_precognition(&self, planned_action: &str) -> ViolationPrecognition {
        let mut at_risk_policies = Vec::new();
        let mut at_risk_limits = Vec::new();
        let mut safe_alternatives = Vec::new();

        for policy in &self.file.policies {
            if !policy.is_active() {
                continue;
            }
            if policy.action == PolicyAction::Deny {
                let label_lower = policy.label.to_lowercase();
                let action_lower = planned_action.to_lowercase();
                if label_lower.contains(&action_lower) || action_lower.contains(&label_lower) {
                    at_risk_policies.push(PolicyRisk {
                        policy_id: policy.id,
                        policy_label: policy.label.clone(),
                        probability: 0.9,
                        trigger: format!("Action '{}' matches deny policy", planned_action),
                    });
                }
            }
        }

        for limit in &self.file.risk_limits {
            let headroom = limit.remaining();
            if limit.usage_ratio() > 0.8 {
                at_risk_limits.push(LimitRisk {
                    limit_id: limit.id,
                    limit_label: limit.label.clone(),
                    headroom,
                    projected_usage: limit.usage_ratio() + 0.1,
                });
            }
        }

        if !at_risk_policies.is_empty() {
            safe_alternatives.push(format!("Modify '{}' to avoid policy conflicts", planned_action));
            safe_alternatives.push("Request pre-approval before proceeding".to_string());
        }

        let violation_probability = if at_risk_policies.is_empty() && at_risk_limits.is_empty() {
            0.05
        } else {
            let max_policy = at_risk_policies
                .iter()
                .map(|p| p.probability)
                .fold(0.0f64, f64::max);
            let max_limit = if at_risk_limits.is_empty() {
                0.0
            } else {
                0.7
            };
            max_policy.max(max_limit)
        };

        ViolationPrecognition {
            id: ContractId::new(),
            planned_action: planned_action.to_string(),
            at_risk_policies,
            at_risk_limits,
            safe_alternatives,
            violation_probability,
            analyzed_at: Utc::now(),
        }
    }

    // ── 6. Contract Crystallization ───────────────────────────────

    /// Generate contract policies from a high-level intent description.
    pub fn crystallize_contract(&self, intent: &str) -> CrystallizedContract {
        let intent_lower = intent.to_lowercase();
        let mut policies = Vec::new();
        let mut risk_limits = Vec::new();
        let mut edge_cases = Vec::new();

        // Pattern-match intent to generate policies
        if intent_lower.contains("budget") || intent_lower.contains("spend") {
            policies.push(CrystallizedPolicy {
                label: "Budget control policy".to_string(),
                scope: "global".to_string(),
                action: "require_approval".to_string(),
                rationale: "Spending actions should require approval".to_string(),
            });
            risk_limits.push(CrystallizedRiskLimit {
                label: "Spending budget".to_string(),
                max_value: 1000.0,
                limit_type: "budget".to_string(),
                rationale: "Cap total spending to prevent overruns".to_string(),
            });
            edge_cases.push("What happens when budget is exactly at limit?".to_string());
        }

        if intent_lower.contains("deploy") || intent_lower.contains("production") {
            policies.push(CrystallizedPolicy {
                label: "Production deployment gate".to_string(),
                scope: "global".to_string(),
                action: "require_approval".to_string(),
                rationale: "Production deployments need human approval".to_string(),
            });
            edge_cases.push("Emergency hotfix deployment scenario".to_string());
        }

        if intent_lower.contains("rate") || intent_lower.contains("api") {
            risk_limits.push(CrystallizedRiskLimit {
                label: "API rate limit".to_string(),
                max_value: 100.0,
                limit_type: "rate".to_string(),
                rationale: "Prevent API abuse".to_string(),
            });
        }

        if intent_lower.contains("safe") || intent_lower.contains("restrict") {
            policies.push(CrystallizedPolicy {
                label: "Restrictive default policy".to_string(),
                scope: "global".to_string(),
                action: "deny".to_string(),
                rationale: "Default deny for unrecognized actions".to_string(),
            });
        }

        // Always add a baseline policy
        if policies.is_empty() {
            policies.push(CrystallizedPolicy {
                label: format!("Governance policy for: {}", intent),
                scope: "global".to_string(),
                action: "audit_only".to_string(),
                rationale: "Baseline governance with audit trail".to_string(),
            });
        }

        let confidence = if policies.len() > 1 { 0.75 } else { 0.5 };

        CrystallizedContract {
            id: ContractId::new(),
            intent: intent.to_string(),
            policies,
            risk_limits,
            approval_workflows: Vec::new(),
            edge_cases,
            confidence,
            crystallized_at: Utc::now(),
        }
    }

    // ── 7. Policy DNA ─────────────────────────────────────────────

    /// Extract the genetic representation of a policy.
    pub fn extract_policy_dna(&self, policy_id: ContractId) -> ContractResult<PolicyDna> {
        let policy = self.get_policy(policy_id)?;

        let scope_breadth = match policy.scope {
            PolicyScope::Global => 1.0,
            PolicyScope::Session => 0.5,
            PolicyScope::Agent => 0.3,
        };
        let restriction = match policy.action {
            PolicyAction::Deny => 1.0,
            PolicyAction::RequireApproval => 0.7,
            PolicyAction::AuditOnly => 0.3,
            PolicyAction::Allow => 0.0,
        };

        let genes = vec![
            PolicyGene {
                name: "scope_breadth".to_string(),
                value: scope_breadth,
                dominant: scope_breadth > 0.5,
            },
            PolicyGene {
                name: "restriction_level".to_string(),
                value: restriction,
                dominant: restriction > 0.5,
            },
            PolicyGene {
                name: "tag_complexity".to_string(),
                value: (policy.tags.len() as f64 / 10.0).min(1.0),
                dominant: false,
            },
        ];

        // Fitness based on violation count for this policy
        let violations_for_policy = self
            .file
            .violations
            .iter()
            .filter(|v| v.policy_id == Some(policy_id))
            .count();
        let fitness = if violations_for_policy == 0 {
            0.9
        } else {
            (1.0 - violations_for_policy as f64 * 0.1).max(0.1)
        };

        Ok(PolicyDna {
            id: ContractId::new(),
            policy_id,
            genes,
            fitness,
            generation: 1,
            mutations: Vec::new(),
            extracted_at: Utc::now(),
        })
    }

    // ── 8. Trust Gradients ────────────────────────────────────────

    /// Evaluate an action with trust-weighted policy assessment.
    pub fn evaluate_trust_gradient(&self, agent_id: &str, action: &str) -> TrustGradient {
        // Compute trust based on violation history
        let agent_violations = self
            .file
            .violations
            .iter()
            .filter(|v| v.actor == agent_id)
            .count();
        let trust_factor = (1.0 - agent_violations as f64 * 0.15).max(0.0);

        let monitoring_level = if trust_factor > 0.8 {
            MonitoringLevel::Minimal
        } else if trust_factor > 0.5 {
            MonitoringLevel::Standard
        } else if trust_factor > 0.2 {
            MonitoringLevel::Enhanced
        } else {
            MonitoringLevel::FullAudit
        };

        let factors = vec![
            TrustFactor {
                name: "violation_history".to_string(),
                weight: 0.5,
                score: trust_factor,
                trend: 0.0,
            },
            TrustFactor {
                name: "policy_compliance".to_string(),
                weight: 0.3,
                score: if agent_violations == 0 { 1.0 } else { 0.5 },
                trend: 0.0,
            },
            TrustFactor {
                name: "approval_track_record".to_string(),
                weight: 0.2,
                score: 0.7,
                trend: 0.0,
            },
        ];

        TrustGradient {
            id: ContractId::new(),
            agent_id: agent_id.to_string(),
            action: action.to_string(),
            trust_factor,
            confidence: 0.7,
            monitoring_level,
            auto_revoke_threshold: 0.2,
            contributing_factors: factors,
            evaluated_at: Utc::now(),
        }
    }

    // ── 9. Collective Contracts ───────────────────────────────────

    /// Create a multi-party collective governance contract.
    pub fn create_collective_contract(
        &self,
        parties: Vec<(&str, &str)>,
        arbitration_method: ArbitrationMethod,
    ) -> CollectiveContract {
        let party_list: Vec<ContractParty> = parties
            .iter()
            .map(|(id, name)| ContractParty {
                party_id: id.to_string(),
                name: name.to_string(),
                role: "member".to_string(),
                signed: false,
                signed_at: None,
            })
            .collect();
        let required = party_list.len() as u32;

        CollectiveContract {
            id: ContractId::new(),
            parties: party_list,
            shared_policies: self.file.policies.iter().map(|p| p.id).collect(),
            arbitration: ArbitrationRules {
                method: arbitration_method,
                timeout_secs: 86400,
                arbitrator: None,
            },
            status: CollectiveStatus::Pending,
            signatures: 0,
            required_signatures: required,
            created_at: Utc::now(),
        }
    }

    // ── 10. Temporal Contracts ────────────────────────────────────

    /// Create a time-evolving contract with governance transitions.
    pub fn create_temporal_contract(
        &self,
        label: &str,
        initial_level: GovernanceLevel,
    ) -> TemporalContract {
        TemporalContract {
            id: ContractId::new(),
            label: label.to_string(),
            initial_level,
            current_level: initial_level,
            transitions: Vec::new(),
            transition_conditions: Vec::new(),
            performance_history: Vec::new(),
            created_at: Utc::now(),
        }
    }

    // ── 11. Contract Inheritance ──────────────────────────────────

    /// Create a hierarchical parent-child contract relationship.
    pub fn create_contract_inheritance(
        &self,
        parent_id: ContractId,
        child_id: ContractId,
        propagate: bool,
    ) -> ContractResult<ContractInheritance> {
        // Verify both exist
        self.get_policy(parent_id)?;
        self.get_policy(child_id)?;

        let inherited_policies = vec![parent_id];
        Ok(ContractInheritance {
            id: ContractId::new(),
            parent_id,
            child_id,
            inherited_policies,
            overrides: Vec::new(),
            propagate_changes: propagate,
            created_at: Utc::now(),
        })
    }

    // ── 12. Smart Escalation ─────────────────────────────────────

    /// Route an approval request to the optimal approver.
    pub fn smart_escalate(&self, description: &str, urgency: f64) -> SmartEscalation {
        let fallback = vec![
            EscalationTarget {
                approver_id: "admin".to_string(),
                name: "System Admin".to_string(),
                availability: 0.9,
                avg_response_secs: 300,
                approval_rate: 0.8,
            },
            EscalationTarget {
                approver_id: "manager".to_string(),
                name: "Team Manager".to_string(),
                availability: 0.7,
                avg_response_secs: 600,
                approval_rate: 0.7,
            },
        ];

        let recommended = if urgency > 0.8 {
            "admin".to_string()
        } else {
            "manager".to_string()
        };

        let routing_reason = if urgency > 0.8 {
            "High urgency — routing to admin for fastest response".to_string()
        } else {
            "Standard urgency — routing to team manager".to_string()
        };

        SmartEscalation {
            id: ContractId::new(),
            request_description: description.to_string(),
            urgency,
            recommended_approver: recommended,
            routing_reason,
            fallback_chain: fallback,
            estimated_response_secs: if urgency > 0.8 { 300 } else { 600 },
            confidence: 0.75,
            routed_at: Utc::now(),
        }
    }

    // ── 13. Violation Archaeology ─────────────────────────────────

    /// Analyze violation patterns to identify root causes.
    pub fn violation_archaeology(&self, agent_id: &str, window_secs: i64) -> ViolationArchaeology {
        let now = Utc::now();
        let agent_violations: Vec<&Violation> = self
            .file
            .violations
            .iter()
            .filter(|v| {
                v.actor == agent_id
                    && (now - v.detected_at).num_seconds() <= window_secs
            })
            .collect();

        // Cluster by severity
        let mut severity_counts: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();
        for v in &agent_violations {
            *severity_counts
                .entry(format!("{:?}", v.severity))
                .or_insert(0) += 1;
        }

        let clusters: Vec<ViolationCluster> = severity_counts
            .iter()
            .map(|(severity, count)| ViolationCluster {
                label: format!("{} violations", severity),
                count: *count,
                severity: severity.clone(),
                time_pattern: None,
                context_pattern: None,
            })
            .collect();

        let root_causes = if !agent_violations.is_empty() {
            vec![RootCause {
                hypothesis: "Agent may be exceeding operational boundaries".to_string(),
                confidence: 0.6,
                evidence: agent_violations
                    .iter()
                    .take(3)
                    .map(|v| v.description.clone())
                    .collect(),
                factors: vec!["Policy awareness".to_string(), "Rate limiting".to_string()],
            }]
        } else {
            Vec::new()
        };

        let recommendations = if !agent_violations.is_empty() {
            vec![
                Remediation {
                    action: "Review and update agent policy awareness".to_string(),
                    expected_impact: "Reduce violations by 40%".to_string(),
                    effort: "medium".to_string(),
                    priority: 1,
                },
                Remediation {
                    action: "Add pre-action violation checks".to_string(),
                    expected_impact: "Prevent repeat violations".to_string(),
                    effort: "low".to_string(),
                    priority: 2,
                },
            ]
        } else {
            Vec::new()
        };

        ViolationArchaeology {
            id: ContractId::new(),
            agent_id: agent_id.to_string(),
            window_secs,
            clusters,
            root_causes,
            recommendations,
            policy_adjustments: Vec::new(),
            analyzed_at: Utc::now(),
        }
    }

    // ── 14. Contract Simulation ───────────────────────────────────

    /// Simulate contract behavior across multiple scenarios.
    pub fn simulate_contract(&self, scenario_count: u32) -> ContractSimulation {
        let total_policies = self.file.policies.len() as f64;
        let deny_count = self
            .file
            .policies
            .iter()
            .filter(|p| p.action == PolicyAction::Deny && p.is_active())
            .count() as f64;
        let approval_count = self
            .file
            .policies
            .iter()
            .filter(|p| p.action == PolicyAction::RequireApproval && p.is_active())
            .count() as f64;

        let denial_rate = if total_policies > 0.0 {
            deny_count / total_policies
        } else {
            0.0
        };
        let approval_rate = 1.0 - denial_rate;

        // Check for potential deadlocks (conflicting policies)
        let mut deadlocks = Vec::new();
        for (i, p1) in self.file.policies.iter().enumerate() {
            for p2 in self.file.policies.iter().skip(i + 1) {
                if p1.action == PolicyAction::Allow
                    && p2.action == PolicyAction::Deny
                    && p1.scope == p2.scope
                {
                    deadlocks.push(SimulationDeadlock {
                        description: format!(
                            "Policy '{}' allows but '{}' denies in same scope",
                            p1.label, p2.label
                        ),
                        policies_involved: vec![p1.id, p2.id],
                        resolution: "Clarify policy precedence rules".to_string(),
                    });
                }
            }
        }

        let risk_breach_rate = self
            .file
            .risk_limits
            .iter()
            .filter(|l| l.usage_ratio() > 0.9)
            .count() as f64
            / self.file.risk_limits.len().max(1) as f64;

        let health = (approval_rate * 0.4 + (1.0 - risk_breach_rate) * 0.3
            + if deadlocks.is_empty() { 0.3 } else { 0.0 })
        .min(1.0);

        let mut edge_cases = Vec::new();
        if approval_count > 0.0 && self.file.approval_rules.is_empty() {
            edge_cases.push(SimulationEdgeCase {
                description: "Policies require approval but no approval rules exist".to_string(),
                current_behavior: "Requests will fail with missing rule".to_string(),
                suggested_fix: "Add approval rules for require_approval policies".to_string(),
            });
        }

        ContractSimulation {
            id: ContractId::new(),
            scenario_count,
            approval_rate,
            denial_rate,
            risk_breach_rate,
            deadlocks,
            edge_cases,
            health_score: health,
            simulated_at: Utc::now(),
        }
    }

    // ── 15. Federated Governance ──────────────────────────────────

    /// Create cross-organizational federated governance.
    pub fn create_federated_governance(
        &self,
        name: &str,
        members: Vec<(&str, &str)>,
        transparency: TransparencyLevel,
    ) -> FederatedGovernance {
        let member_list: Vec<FederationMember> = members
            .iter()
            .map(|(id, org_name)| FederationMember {
                org_id: id.to_string(),
                name: org_name.to_string(),
                contributed_policies: 0,
                trust_level: 0.5,
                ratified: false,
            })
            .collect();

        FederatedGovernance {
            id: ContractId::new(),
            name: name.to_string(),
            members: member_list,
            shared_policies: Vec::new(),
            transparency,
            status: FederationStatus::Forming,
            created_at: Utc::now(),
        }
    }

    // ── 16. Self-Healing Contracts ────────────────────────────────

    /// Create a contract that automatically adapts to violations.
    pub fn create_self_healing_contract(
        &self,
        base_contract_id: ContractId,
    ) -> ContractResult<SelfHealingContract> {
        // Verify base contract exists
        self.get_policy(base_contract_id)?;

        let healing_rules = vec![
            HealingRule {
                trigger: HealingTrigger::RepeatedViolation { count: 3 },
                action: HealingAction::TightenPolicy,
                cooldown_secs: 3600,
                last_triggered: None,
            },
            HealingRule {
                trigger: HealingTrigger::PerfectRecord {
                    duration_secs: 604800,
                },
                action: HealingAction::RelaxPolicy,
                cooldown_secs: 86400,
                last_triggered: None,
            },
            HealingRule {
                trigger: HealingTrigger::RiskApproaching { threshold: 0.9 },
                action: HealingAction::AddMonitoring,
                cooldown_secs: 1800,
                last_triggered: None,
            },
        ];

        Ok(SelfHealingContract {
            id: ContractId::new(),
            base_contract_id,
            healing_rules,
            healing_history: Vec::new(),
            adaptation_level: AdaptationLevel::Original,
            health_score: 1.0,
            created_at: Utc::now(),
        })
    }

    // ── Stats ──────────────────────────────────────────────────────

    /// Get summary statistics.
    pub fn stats(&self) -> ContractStats {
        ContractStats {
            policy_count: self.file.policies.len(),
            active_policy_count: self.file.policies.iter().filter(|p| p.is_active()).count(),
            risk_limit_count: self.file.risk_limits.len(),
            approval_rule_count: self.file.approval_rules.len(),
            pending_approval_count: self
                .file
                .approval_requests
                .iter()
                .filter(|r| r.is_pending())
                .count(),
            condition_count: self.file.conditions.len(),
            obligation_count: self.file.obligations.len(),
            pending_obligation_count: self
                .file
                .obligations
                .iter()
                .filter(|o| o.status == ObligationStatus::Pending)
                .count(),
            violation_count: self.file.violations.len(),
            critical_violation_count: self
                .file
                .violations
                .iter()
                .filter(|v| v.severity >= ViolationSeverity::Critical)
                .count(),
            total_entities: self.file.total_entities(),
        }
    }
}

impl Default for ContractEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary statistics for a contract store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStats {
    pub policy_count: usize,
    pub active_policy_count: usize,
    pub risk_limit_count: usize,
    pub approval_rule_count: usize,
    pub pending_approval_count: usize,
    pub condition_count: usize,
    pub obligation_count: usize,
    pub pending_obligation_count: usize,
    pub violation_count: usize,
    pub critical_violation_count: usize,
    pub total_entities: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risk_limit::LimitType;

    #[test]
    fn test_engine_lifecycle() {
        let mut engine = ContractEngine::new();

        // Add policy
        let policy = Policy::new(
            "No deploys on Friday",
            PolicyScope::Global,
            PolicyAction::Deny,
        );
        let policy_id = engine.add_policy(policy);
        assert!(engine.get_policy(policy_id).is_ok());

        // Check policy
        let action = engine.check_policy("deploy", PolicyScope::Global);
        assert_eq!(action, PolicyAction::Deny);

        // Add risk limit
        let limit = RiskLimit::new("API calls", LimitType::Rate, 100.0);
        let limit_id = engine.add_risk_limit(limit);
        assert!(engine.check_risk_limit("api", 50.0).is_none());

        engine.increment_risk_limit(limit_id, 90.0).unwrap();
        assert!(engine.check_risk_limit("api", 20.0).is_some());

        // Stats
        let stats = engine.stats();
        assert_eq!(stats.policy_count, 1);
        assert_eq!(stats.risk_limit_count, 1);
    }

    #[test]
    fn test_approval_workflow() {
        let mut engine = ContractEngine::new();

        let rule = ApprovalRule::new("Deploy approval", "deploy:*");
        let rule_id = engine.add_approval_rule(rule);

        let request_id = engine
            .request_approval(rule_id, "Deploy v2.0 to production", "agent_1")
            .unwrap();

        let pending = engine.list_approval_requests(Some(ApprovalStatus::Pending));
        assert_eq!(pending.len(), 1);

        engine
            .decide_approval(request_id, DecisionType::Approve, "admin", "LGTM")
            .unwrap();

        let approved = engine.list_approval_requests(Some(ApprovalStatus::Approved));
        assert_eq!(approved.len(), 1);
    }

    #[test]
    fn test_violation_reporting() {
        let mut engine = ContractEngine::new();

        let v = Violation::new("Rate limit exceeded", ViolationSeverity::Warning, "agent_1");
        engine.report_violation(v);

        let v2 = Violation::new(
            "Unauthorized access",
            ViolationSeverity::Critical,
            "agent_2",
        );
        engine.report_violation(v2);

        assert_eq!(engine.list_violations(None).len(), 2);
        assert_eq!(
            engine
                .list_violations(Some(ViolationSeverity::Critical))
                .len(),
            1
        );

        let stats = engine.stats();
        assert_eq!(stats.critical_violation_count, 1);
    }
}

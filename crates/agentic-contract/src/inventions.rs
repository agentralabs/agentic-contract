//! The 16 Contract Inventions — advanced governance capabilities.
//!
//! Organized into five categories:
//!
//! - **Visibility** (1-5): Policy Omniscience, Risk Prophecy, Approval Telepathy,
//!   Obligation Clairvoyance, Violation Precognition
//! - **Generation** (6-7): Contract Crystallization, Policy DNA
//! - **Trust** (8-9): Trust Gradients, Collective Contracts
//! - **Temporal** (10-11): Temporal Contracts, Contract Inheritance
//! - **Advanced** (12-16): Smart Escalation, Violation Archaeology,
//!   Contract Simulation, Federated Governance, Self-Healing Contracts

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ContractId;

// ─── VISIBILITY (1-5) ───────────────────────────────────────────────────────

// ─── 1. Policy Omniscience ──────────────────────────────────────────────────

/// Complete visibility into all applicable policies for an action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyOmniscience {
    /// Unique identifier.
    pub id: ContractId,
    /// The agent being queried about.
    pub agent_id: String,
    /// The context of the query.
    pub context: String,
    /// Actions explicitly allowed.
    pub allowed_actions: Vec<PermissionEntry>,
    /// Actions explicitly denied.
    pub denied_actions: Vec<PermissionEntry>,
    /// Actions requiring approval.
    pub conditional_actions: Vec<PermissionEntry>,
    /// Overall permission count.
    pub total_permissions: u32,
    /// When this query was made.
    pub queried_at: DateTime<Utc>,
}

/// A single permission entry in an omniscience result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntry {
    /// The action pattern.
    pub action: String,
    /// The policy granting/denying this.
    pub policy_id: ContractId,
    /// Policy label.
    pub policy_label: String,
    /// Explanation of why.
    pub reason: String,
    /// Scope of the permission.
    pub scope: String,
}

// ─── 2. Risk Prophecy ───────────────────────────────────────────────────────

/// Prediction of future risk budget usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProphecy {
    /// Unique identifier.
    pub id: ContractId,
    /// The agent being prophesied about.
    pub agent_id: String,
    /// Forecast window in seconds.
    pub forecast_window_secs: i64,
    /// Projected budget usage by limit.
    pub projections: Vec<RiskProjection>,
    /// Overall risk score (0.0-1.0).
    pub overall_risk_score: f64,
    /// Recommended adjustments.
    pub recommendations: Vec<String>,
    /// When this prophecy was made.
    pub prophesied_at: DateTime<Utc>,
}

/// A projection for a specific risk limit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProjection {
    /// The risk limit being projected.
    pub limit_id: ContractId,
    /// Risk limit label.
    pub limit_label: String,
    /// Current usage (0.0-1.0 fraction of limit).
    pub current_usage: f64,
    /// Projected usage at end of window.
    pub projected_usage: f64,
    /// Probability of exceeding limit (0.0-1.0).
    pub exceed_probability: f64,
    /// Estimated time until limit reached (seconds), if applicable.
    pub time_until_limit_secs: Option<i64>,
}

// ─── 3. Approval Telepathy ──────────────────────────────────────────────────

/// Prediction of approval likelihood.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTelepathy {
    /// Unique identifier.
    pub id: ContractId,
    /// The action being considered.
    pub action: String,
    /// Probability of approval (0.0-1.0).
    pub approval_probability: f64,
    /// Likely approvers.
    pub likely_approvers: Vec<String>,
    /// Estimated response time in seconds.
    pub estimated_response_secs: i64,
    /// Suggested modifications to increase probability.
    pub suggestions: Vec<ApprovalSuggestion>,
    /// Historical data informing this prediction.
    pub historical_approval_rate: f64,
    /// When this prediction was made.
    pub predicted_at: DateTime<Utc>,
}

/// A suggestion to improve approval chances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSuggestion {
    /// What to change.
    pub modification: String,
    /// New probability after this change.
    pub new_probability: f64,
    /// Effort required (low/medium/high).
    pub effort: String,
}

// ─── 4. Obligation Clairvoyance ─────────────────────────────────────────────

/// Visibility into future obligations and dependencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationClairvoyance {
    /// Unique identifier.
    pub id: ContractId,
    /// The agent being analyzed.
    pub agent_id: String,
    /// Projection window in seconds.
    pub window_secs: i64,
    /// Upcoming obligations sorted by deadline.
    pub upcoming: Vec<ObligationForecast>,
    /// Dependency conflicts detected.
    pub conflicts: Vec<ObligationConflict>,
    /// Optimal fulfillment schedule.
    pub optimal_order: Vec<ContractId>,
    /// When this projection was made.
    pub projected_at: DateTime<Utc>,
}

/// A forecast for a single obligation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationForecast {
    /// Obligation ID.
    pub obligation_id: ContractId,
    /// Obligation label.
    pub label: String,
    /// Deadline.
    pub deadline: Option<DateTime<Utc>>,
    /// Time remaining in seconds.
    pub time_remaining_secs: Option<i64>,
    /// Estimated fulfillment effort in minutes.
    pub estimated_effort_minutes: u32,
    /// Dependencies on other obligations.
    pub depends_on: Vec<ContractId>,
    /// Risk of missing deadline (0.0-1.0).
    pub miss_risk: f64,
}

/// A conflict between obligations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationConflict {
    /// First obligation.
    pub obligation_a: ContractId,
    /// Second obligation.
    pub obligation_b: ContractId,
    /// Nature of the conflict.
    pub conflict_type: String,
    /// Suggested resolution.
    pub resolution: String,
}

// ─── 5. Violation Precognition ──────────────────────────────────────────────

/// Detection of potential violations before they occur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationPrecognition {
    /// Unique identifier.
    pub id: ContractId,
    /// The planned action being analyzed.
    pub planned_action: String,
    /// Policies at risk of being violated.
    pub at_risk_policies: Vec<PolicyRisk>,
    /// Risk limits at risk of being exceeded.
    pub at_risk_limits: Vec<LimitRisk>,
    /// Safe alternative actions.
    pub safe_alternatives: Vec<String>,
    /// Overall violation probability (0.0-1.0).
    pub violation_probability: f64,
    /// When this analysis was made.
    pub analyzed_at: DateTime<Utc>,
}

/// A policy at risk of being violated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRisk {
    /// The policy at risk.
    pub policy_id: ContractId,
    /// Policy label.
    pub policy_label: String,
    /// Probability of violation (0.0-1.0).
    pub probability: f64,
    /// What would trigger the violation.
    pub trigger: String,
}

/// A risk limit at risk of being exceeded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitRisk {
    /// The risk limit.
    pub limit_id: ContractId,
    /// Limit label.
    pub limit_label: String,
    /// Current headroom before exceeding.
    pub headroom: f64,
    /// Projected usage from the planned action.
    pub projected_usage: f64,
}

// ─── GENERATION (6-7) ───────────────────────────────────────────────────────

// ─── 6. Contract Crystallization ────────────────────────────────────────────

/// A crystallized contract generated from high-level intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizedContract {
    /// Unique identifier.
    pub id: ContractId,
    /// The original intent string.
    pub intent: String,
    /// Generated policies.
    pub policies: Vec<CrystallizedPolicy>,
    /// Generated risk limits.
    pub risk_limits: Vec<CrystallizedRiskLimit>,
    /// Generated approval workflows.
    pub approval_workflows: Vec<String>,
    /// Edge cases identified.
    pub edge_cases: Vec<String>,
    /// Confidence in the crystallization (0.0-1.0).
    pub confidence: f64,
    /// When crystallized.
    pub crystallized_at: DateTime<Utc>,
}

/// A policy generated during crystallization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizedPolicy {
    /// Suggested label.
    pub label: String,
    /// Suggested scope.
    pub scope: String,
    /// Suggested action.
    pub action: String,
    /// Why this policy was generated.
    pub rationale: String,
}

/// A risk limit generated during crystallization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizedRiskLimit {
    /// Suggested label.
    pub label: String,
    /// Suggested maximum value.
    pub max_value: f64,
    /// Suggested limit type.
    pub limit_type: String,
    /// Why this limit was generated.
    pub rationale: String,
}

// ─── 7. Policy DNA ──────────────────────────────────────────────────────────

/// Genetic representation of a policy for evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDna {
    /// Unique identifier.
    pub id: ContractId,
    /// Source policy.
    pub policy_id: ContractId,
    /// DNA sequence (encoded traits).
    pub genes: Vec<PolicyGene>,
    /// Fitness score based on outcomes (0.0-1.0).
    pub fitness: f64,
    /// Generation number.
    pub generation: u32,
    /// Mutation history.
    pub mutations: Vec<PolicyMutation>,
    /// When this DNA was extracted.
    pub extracted_at: DateTime<Utc>,
}

/// A gene in a policy's DNA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyGene {
    /// Gene name (e.g., "scope_breadth", "restriction_level").
    pub name: String,
    /// Gene value (0.0-1.0).
    pub value: f64,
    /// Whether this gene is dominant.
    pub dominant: bool,
}

/// A mutation in policy evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMutation {
    /// What changed.
    pub gene_name: String,
    /// Previous value.
    pub old_value: f64,
    /// New value.
    pub new_value: f64,
    /// Whether this mutation improved fitness.
    pub beneficial: bool,
}

/// Result of evolving policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResult {
    /// Number of generations simulated.
    pub generations: u32,
    /// Best evolved policy set.
    pub evolved_policies: Vec<PolicyDna>,
    /// Final fitness score.
    pub best_fitness: f64,
    /// Improvements made.
    pub improvements: Vec<String>,
}

// ─── TRUST (8-9) ────────────────────────────────────────────────────────────

// ─── 8. Trust Gradients ─────────────────────────────────────────────────────

/// A trust-augmented policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustGradient {
    /// Unique identifier.
    pub id: ContractId,
    /// The agent being evaluated.
    pub agent_id: String,
    /// The action being evaluated.
    pub action: String,
    /// Trust factor (0.0-1.0).
    pub trust_factor: f64,
    /// Confidence in the trust assessment (0.0-1.0).
    pub confidence: f64,
    /// Monitoring level applied.
    pub monitoring_level: MonitoringLevel,
    /// Trust score threshold for auto-revocation.
    pub auto_revoke_threshold: f64,
    /// Factors contributing to the trust score.
    pub contributing_factors: Vec<TrustFactor>,
    /// When this evaluation was made.
    pub evaluated_at: DateTime<Utc>,
}

/// Monitoring level based on trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringLevel {
    /// Minimal monitoring.
    Minimal,
    /// Standard monitoring.
    Standard,
    /// Enhanced monitoring.
    Enhanced,
    /// Full audit trail.
    FullAudit,
}

/// A factor contributing to the trust score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustFactor {
    /// Factor name.
    pub name: String,
    /// Weight in trust calculation (0.0-1.0).
    pub weight: f64,
    /// Current score for this factor (0.0-1.0).
    pub score: f64,
    /// Trend (positive = improving).
    pub trend: f64,
}

// ─── 9. Collective Contracts ────────────────────────────────────────────────

/// A contract spanning multiple agents or organizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveContract {
    /// Unique identifier.
    pub id: ContractId,
    /// Parties to this contract.
    pub parties: Vec<ContractParty>,
    /// Shared policies.
    pub shared_policies: Vec<ContractId>,
    /// Arbitration rules.
    pub arbitration: ArbitrationRules,
    /// Current status.
    pub status: CollectiveStatus,
    /// Signature count.
    pub signatures: u32,
    /// Required signatures.
    pub required_signatures: u32,
    /// When created.
    pub created_at: DateTime<Utc>,
}

/// A party to a collective contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractParty {
    /// Party identifier.
    pub party_id: String,
    /// Party name.
    pub name: String,
    /// Role in the contract.
    pub role: String,
    /// Whether this party has signed.
    pub signed: bool,
    /// When they signed (if they have).
    pub signed_at: Option<DateTime<Utc>>,
}

/// Rules for arbitrating disputes in collective contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationRules {
    /// Arbitration method.
    pub method: ArbitrationMethod,
    /// Timeout before escalation (seconds).
    pub timeout_secs: i64,
    /// Who arbitrates.
    pub arbitrator: Option<String>,
}

/// Method of arbitration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArbitrationMethod {
    /// Simple majority vote.
    MajorityVote,
    /// Unanimous agreement.
    Unanimous,
    /// Designated third-party arbitrator.
    ThirdParty,
    /// Automated based on rules.
    Automated,
}

/// Status of a collective contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectiveStatus {
    /// Awaiting all signatures.
    Pending,
    /// All signed, contract is active.
    Active,
    /// Contract has been terminated.
    Terminated,
    /// In dispute.
    Disputed,
}

// ─── TEMPORAL (10-11) ───────────────────────────────────────────────────────

// ─── 10. Temporal Contracts ─────────────────────────────────────────────────

/// A contract that evolves over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContract {
    /// Unique identifier.
    pub id: ContractId,
    /// Contract label.
    pub label: String,
    /// Initial governance level.
    pub initial_level: GovernanceLevel,
    /// Current governance level.
    pub current_level: GovernanceLevel,
    /// Planned transitions.
    pub transitions: Vec<GovernanceTransition>,
    /// Conditions that must be met for transitions.
    pub transition_conditions: Vec<String>,
    /// Performance history (0.0-1.0 per period).
    pub performance_history: Vec<f64>,
    /// When created.
    pub created_at: DateTime<Utc>,
}

/// A governance level (how strict the contract is).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceLevel {
    /// Very restrictive.
    Conservative,
    /// Moderately restrictive.
    Moderate,
    /// Permissive.
    Permissive,
    /// Agent has full autonomy.
    Autonomous,
}

/// A planned transition between governance levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceTransition {
    /// From level.
    pub from: GovernanceLevel,
    /// To level.
    pub to: GovernanceLevel,
    /// When this transition should happen.
    pub scheduled_at: DateTime<Utc>,
    /// Whether the transition has occurred.
    pub completed: bool,
    /// Whether conditions were met.
    pub conditions_met: bool,
}

// ─── 11. Contract Inheritance ───────────────────────────────────────────────

/// A hierarchical contract relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInheritance {
    /// Unique identifier.
    pub id: ContractId,
    /// Parent contract ID.
    pub parent_id: ContractId,
    /// Child contract ID.
    pub child_id: ContractId,
    /// What the child inherits.
    pub inherited_policies: Vec<ContractId>,
    /// What the child overrides.
    pub overrides: Vec<InheritanceOverride>,
    /// Whether parent changes propagate.
    pub propagate_changes: bool,
    /// When this relationship was created.
    pub created_at: DateTime<Utc>,
}

/// An override in a child contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceOverride {
    /// Policy being overridden.
    pub policy_id: ContractId,
    /// Override type.
    pub override_type: OverrideType,
    /// Description of the override.
    pub description: String,
}

/// Type of inheritance override.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverrideType {
    /// Allow an additional action.
    AllowAdditional,
    /// Restrict further.
    RestrictFurther,
    /// Modify parameters.
    ModifyParameters,
}

// ─── ADVANCED (12-16) ───────────────────────────────────────────────────────

// ─── 12. Smart Escalation ───────────────────────────────────────────────────

/// AI-powered escalation routing result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartEscalation {
    /// Unique identifier.
    pub id: ContractId,
    /// The request being escalated.
    pub request_description: String,
    /// Request urgency (0.0-1.0).
    pub urgency: f64,
    /// Recommended approver.
    pub recommended_approver: String,
    /// Why this approver was chosen.
    pub routing_reason: String,
    /// Alternative approvers in order.
    pub fallback_chain: Vec<EscalationTarget>,
    /// Estimated response time in seconds.
    pub estimated_response_secs: i64,
    /// Confidence in routing (0.0-1.0).
    pub confidence: f64,
    /// When routed.
    pub routed_at: DateTime<Utc>,
}

/// A target in an escalation chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationTarget {
    /// Approver ID.
    pub approver_id: String,
    /// Approver name.
    pub name: String,
    /// Estimated availability (0.0-1.0).
    pub availability: f64,
    /// Historical response time in seconds.
    pub avg_response_secs: i64,
    /// Historical approval rate (0.0-1.0).
    pub approval_rate: f64,
}

// ─── 13. Violation Archaeology ──────────────────────────────────────────────

/// Deep analysis of violation patterns and root causes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationArchaeology {
    /// Unique identifier.
    pub id: ContractId,
    /// Agent being analyzed.
    pub agent_id: String,
    /// Analysis window in seconds.
    pub window_secs: i64,
    /// Violation clusters found.
    pub clusters: Vec<ViolationCluster>,
    /// Root cause hypotheses.
    pub root_causes: Vec<RootCause>,
    /// Remediation recommendations.
    pub recommendations: Vec<Remediation>,
    /// Policy adjustment suggestions.
    pub policy_adjustments: Vec<PolicyAdjustment>,
    /// When analyzed.
    pub analyzed_at: DateTime<Utc>,
}

/// A cluster of related violations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationCluster {
    /// Cluster label.
    pub label: String,
    /// Violation count in this cluster.
    pub count: u32,
    /// Common severity.
    pub severity: String,
    /// Common time pattern (e.g., "weekday mornings").
    pub time_pattern: Option<String>,
    /// Common context.
    pub context_pattern: Option<String>,
}

/// A root cause hypothesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    /// Hypothesis description.
    pub hypothesis: String,
    /// Confidence (0.0-1.0).
    pub confidence: f64,
    /// Supporting evidence.
    pub evidence: Vec<String>,
    /// Contributing factors.
    pub factors: Vec<String>,
}

/// A remediation recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remediation {
    /// Action to take.
    pub action: String,
    /// Expected impact.
    pub expected_impact: String,
    /// Effort required (low/medium/high).
    pub effort: String,
    /// Priority (1 = highest).
    pub priority: u32,
}

/// A suggested policy adjustment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAdjustment {
    /// Policy to adjust.
    pub policy_id: ContractId,
    /// Suggested change.
    pub adjustment: String,
    /// Reason for the adjustment.
    pub reason: String,
}

// ─── 14. Contract Simulation ────────────────────────────────────────────────

/// Results of simulating contract behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSimulation {
    /// Unique identifier.
    pub id: ContractId,
    /// Number of scenarios simulated.
    pub scenario_count: u32,
    /// Approval rate across simulations (0.0-1.0).
    pub approval_rate: f64,
    /// Denial rate (0.0-1.0).
    pub denial_rate: f64,
    /// Risk limit breach rate (0.0-1.0).
    pub risk_breach_rate: f64,
    /// Potential deadlocks discovered.
    pub deadlocks: Vec<SimulationDeadlock>,
    /// Edge cases discovered.
    pub edge_cases: Vec<SimulationEdgeCase>,
    /// Overall contract health score (0.0-1.0).
    pub health_score: f64,
    /// When simulated.
    pub simulated_at: DateTime<Utc>,
}

/// A deadlock found during simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDeadlock {
    /// Description of the deadlock.
    pub description: String,
    /// Policies involved.
    pub policies_involved: Vec<ContractId>,
    /// How to resolve it.
    pub resolution: String,
}

/// An edge case found during simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEdgeCase {
    /// Description of the edge case.
    pub description: String,
    /// How the contract handles it currently.
    pub current_behavior: String,
    /// Suggested improvement.
    pub suggested_fix: String,
}

// ─── 15. Federated Governance ───────────────────────────────────────────────

/// Governance spanning organizational boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedGovernance {
    /// Unique identifier.
    pub id: ContractId,
    /// Federation name.
    pub name: String,
    /// Member organizations.
    pub members: Vec<FederationMember>,
    /// Shared policy set.
    pub shared_policies: Vec<ContractId>,
    /// Transparency level.
    pub transparency: TransparencyLevel,
    /// Status.
    pub status: FederationStatus,
    /// When created.
    pub created_at: DateTime<Utc>,
}

/// A member of a federation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationMember {
    /// Organization ID.
    pub org_id: String,
    /// Organization name.
    pub name: String,
    /// Policies contributed.
    pub contributed_policies: u32,
    /// Trust level within federation (0.0-1.0).
    pub trust_level: f64,
    /// Whether this member has ratified.
    pub ratified: bool,
}

/// Transparency level of a federation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransparencyLevel {
    /// All members see everything.
    Full,
    /// Members see summaries only.
    Summary,
    /// Members see only their own actions.
    Minimal,
}

/// Status of a federation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederationStatus {
    /// Being formed.
    Forming,
    /// Active and operational.
    Active,
    /// Suspended.
    Suspended,
    /// Dissolved.
    Dissolved,
}

// ─── 16. Self-Healing Contracts ─────────────────────────────────────────────

/// A contract that automatically adapts to violations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHealingContract {
    /// Unique identifier.
    pub id: ContractId,
    /// Base contract ID.
    pub base_contract_id: ContractId,
    /// Healing rules.
    pub healing_rules: Vec<HealingRule>,
    /// Healing events that have occurred.
    pub healing_history: Vec<HealingEvent>,
    /// Current adaptation level.
    pub adaptation_level: AdaptationLevel,
    /// Overall contract health (0.0-1.0).
    pub health_score: f64,
    /// When created.
    pub created_at: DateTime<Utc>,
}

/// A rule for self-healing behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingRule {
    /// Trigger condition.
    pub trigger: HealingTrigger,
    /// Action to take.
    pub action: HealingAction,
    /// Cooldown before re-triggering (seconds).
    pub cooldown_secs: i64,
    /// Last triggered.
    pub last_triggered: Option<DateTime<Utc>>,
}

/// What triggers a healing action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealingTrigger {
    /// Repeated violations (threshold count).
    RepeatedViolation { count: u32 },
    /// Perfect record for a duration (seconds).
    PerfectRecord { duration_secs: i64 },
    /// Risk limit approaching threshold.
    RiskApproaching { threshold: f64 },
    /// Context change detected.
    ContextChange,
}

/// What action to take for healing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealingAction {
    /// Tighten policy restrictions.
    TightenPolicy,
    /// Relax policy restrictions.
    RelaxPolicy,
    /// Add monitoring.
    AddMonitoring,
    /// Remove monitoring.
    RemoveMonitoring,
    /// Require additional approval.
    AddApproval,
    /// Remove approval requirement.
    RemoveApproval,
}

/// A healing event that has occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingEvent {
    /// What triggered the healing.
    pub trigger: String,
    /// What action was taken.
    pub action: String,
    /// Policies affected.
    pub affected_policies: Vec<ContractId>,
    /// When the healing occurred.
    pub healed_at: DateTime<Utc>,
    /// Result of the healing.
    pub result: String,
}

/// Current adaptation level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdaptationLevel {
    /// Original contract, no adaptations.
    Original,
    /// Minor adaptations applied.
    MinorAdaptation,
    /// Significant adaptations applied.
    MajorAdaptation,
    /// Substantially different from original.
    FullyAdapted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_level_equality() {
        assert_eq!(MonitoringLevel::Standard, MonitoringLevel::Standard);
        assert_ne!(MonitoringLevel::Minimal, MonitoringLevel::FullAudit);
    }

    #[test]
    fn test_governance_level_equality() {
        assert_eq!(GovernanceLevel::Conservative, GovernanceLevel::Conservative);
        assert_ne!(GovernanceLevel::Moderate, GovernanceLevel::Autonomous);
    }

    #[test]
    fn test_collective_status_equality() {
        assert_eq!(CollectiveStatus::Active, CollectiveStatus::Active);
    }

    #[test]
    fn test_override_type_equality() {
        assert_eq!(OverrideType::AllowAdditional, OverrideType::AllowAdditional);
    }

    #[test]
    fn test_arbitration_method_equality() {
        assert_eq!(ArbitrationMethod::MajorityVote, ArbitrationMethod::MajorityVote);
    }

    #[test]
    fn test_healing_action_equality() {
        assert_eq!(HealingAction::TightenPolicy, HealingAction::TightenPolicy);
        assert_ne!(HealingAction::TightenPolicy, HealingAction::RelaxPolicy);
    }

    #[test]
    fn test_adaptation_level_equality() {
        assert_eq!(AdaptationLevel::Original, AdaptationLevel::Original);
    }

    #[test]
    fn test_federation_status_equality() {
        assert_eq!(FederationStatus::Active, FederationStatus::Active);
    }

    #[test]
    fn test_transparency_level_equality() {
        assert_eq!(TransparencyLevel::Full, TransparencyLevel::Full);
    }
}

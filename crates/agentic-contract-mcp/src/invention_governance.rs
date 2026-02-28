//! Inventions 8-12 (Governance category) — Trust Gradients, Collective Contracts,
//! Temporal Contracts, Contract Inheritance, and Smart Escalation.
//!
//! Each invention provides 4 deep tools for a total of 20 tools.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use agentic_contract::approval::{ApprovalStatus, DecisionType};
use agentic_contract::inventions::*;
use agentic_contract::ContractEngine;
use agentic_contract::ContractId;

use crate::tools::{require_id, require_str, ToolDefinition};

// ─────────────────────────────────────────────────────────────────────────────
// Tool definitions
// ─────────────────────────────────────────────────────────────────────────────

pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Invention 8: Trust Gradients (4 tools) ──────────────────────────
    ToolDefinition {
        name: "trust_gradient_evaluate",
        description: "Evaluate trust-weighted policy assessment for an agent and action",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to evaluate trust for"},"action":{"type":"string","description":"Action to evaluate"}},"required":["agent_id","action"]}"#,
    },
    ToolDefinition {
        name: "trust_gradient_history",
        description: "Get trust evolution over time for an agent",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to get trust history for"},"window_days":{"type":"integer","description":"Window in days to analyze","default":90}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "trust_gradient_predict",
        description: "Predict future trust trajectory for an agent",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to predict trust for"},"forecast_days":{"type":"integer","description":"Days to forecast ahead","default":30}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "trust_gradient_compare",
        description: "Compare trust profiles of two agents side-by-side",
        input_schema: r#"{"type":"object","properties":{"agent_a":{"type":"string","description":"First agent ID"},"agent_b":{"type":"string","description":"Second agent ID"}},"required":["agent_a","agent_b"]}"#,
    },
    // ── Invention 9: Collective Contracts (4 tools) ─────────────────────
    ToolDefinition {
        name: "collective_contract_create",
        description: "Create a multi-party governance contract with quorum rules",
        input_schema: r#"{"type":"object","properties":{"parties":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"name":{"type":"string"},"role":{"type":"string","default":"member"}},"required":["id","name"]},"description":"Parties involved"},"arbitration":{"type":"string","enum":["majority_vote","unanimous","third_party","automated"],"default":"majority_vote"},"quorum_ratio":{"type":"number","description":"Fraction of parties required to sign (0.0-1.0)","default":0.5}},"required":["parties"]}"#,
    },
    ToolDefinition {
        name: "collective_contract_sign",
        description: "Sign a collective contract as one of its parties",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Collective contract ID"},"signer_id":{"type":"string","description":"Party ID of the signer"}},"required":["contract_id","signer_id"]}"#,
    },
    ToolDefinition {
        name: "collective_contract_status",
        description: "Get detailed status of a collective contract including signature progress",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Collective contract ID"}},"required":["contract_id"]}"#,
    },
    ToolDefinition {
        name: "collective_contract_arbitrate",
        description: "Initiate dispute resolution for a collective contract",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Collective contract ID"},"dispute_description":{"type":"string","description":"Description of the dispute"},"filed_by":{"type":"string","description":"Party filing the dispute"}},"required":["contract_id","dispute_description","filed_by"]}"#,
    },
    // ── Invention 10: Temporal Contracts (4 tools) ──────────────────────
    ToolDefinition {
        name: "temporal_contract_create",
        description: "Create a time-evolving contract with governance level transitions",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"Contract label"},"initial_level":{"type":"string","enum":["conservative","moderate","permissive","autonomous"],"default":"conservative"},"transition_conditions":{"type":"array","items":{"type":"string"},"description":"Conditions for governance transitions"}},"required":["label"]}"#,
    },
    ToolDefinition {
        name: "temporal_contract_transition",
        description: "Evaluate and apply governance level transitions based on performance",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Temporal contract ID"},"performance_score":{"type":"number","description":"Current performance score 0.0-1.0"}},"required":["contract_id","performance_score"]}"#,
    },
    ToolDefinition {
        name: "temporal_contract_history",
        description: "Get full history of governance transitions for a temporal contract",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Temporal contract ID"}},"required":["contract_id"]}"#,
    },
    ToolDefinition {
        name: "temporal_contract_predict",
        description: "Predict when the next governance transition will occur",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Temporal contract ID"}},"required":["contract_id"]}"#,
    },
    // ── Invention 11: Contract Inheritance (4 tools) ────────────────────
    ToolDefinition {
        name: "contract_inheritance_create",
        description: "Create a parent-child relationship between policies with inheritance",
        input_schema: r#"{"type":"object","properties":{"parent_id":{"type":"string","description":"Parent policy ID"},"child_id":{"type":"string","description":"Child policy ID"},"propagate":{"type":"boolean","description":"Whether parent changes propagate to child","default":true}},"required":["parent_id","child_id"]}"#,
    },
    ToolDefinition {
        name: "contract_inheritance_tree",
        description: "Visualize the full inheritance tree from a root policy",
        input_schema: r#"{"type":"object","properties":{"root_id":{"type":"string","description":"Root policy ID to start traversal from"}},"required":["root_id"]}"#,
    },
    ToolDefinition {
        name: "contract_inheritance_resolve",
        description:
            "Resolve the effective policy for a given scope by walking the inheritance chain",
        input_schema: r#"{"type":"object","properties":{"policy_id":{"type":"string","description":"Policy ID to resolve effective rules for"},"scope":{"type":"string","description":"Scope context for resolution","default":"global"}},"required":["policy_id"]}"#,
    },
    ToolDefinition {
        name: "contract_inheritance_override",
        description: "Create an override in a child contract for a parent policy property",
        input_schema: r#"{"type":"object","properties":{"inheritance_id":{"type":"string","description":"Inheritance relationship ID"},"policy_id":{"type":"string","description":"Policy to override"},"override_type":{"type":"string","enum":["allow_additional","restrict_further","modify_parameters"],"default":"modify_parameters"},"description":{"type":"string","description":"Description of the override"}},"required":["inheritance_id","policy_id","description"]}"#,
    },
    // ── Invention 12: Smart Escalation (4 tools) ────────────────────────
    ToolDefinition {
        name: "smart_escalation_route",
        description: "Route an approval request to the optimal approver based on scoring",
        input_schema: r#"{"type":"object","properties":{"description":{"type":"string","description":"What needs approval"},"urgency":{"type":"number","description":"Urgency level 0.0-1.0","default":0.5}},"required":["description"]}"#,
    },
    ToolDefinition {
        name: "smart_escalation_history",
        description: "Analyze escalation patterns including response times and bottlenecks",
        input_schema: r#"{"type":"object","properties":{"window_days":{"type":"integer","description":"Analysis window in days","default":30}}}"#,
    },
    ToolDefinition {
        name: "smart_escalation_predict",
        description: "Predict response time for an escalation based on historical data",
        input_schema: r#"{"type":"object","properties":{"urgency":{"type":"number","description":"Urgency level 0.0-1.0","default":0.5},"approver_id":{"type":"string","description":"Specific approver to predict for (optional)"}}}"#,
    },
    ToolDefinition {
        name: "smart_escalation_configure",
        description: "Configure escalation routing rules, approvers, and timeout policies",
        input_schema: r#"{"type":"object","properties":{"add_approvers":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"name":{"type":"string"},"domains":{"type":"array","items":{"type":"string"}}},"required":["id","name"]},"description":"Approvers to add"},"urgency_thresholds":{"type":"object","properties":{"low":{"type":"number"},"medium":{"type":"number"},"high":{"type":"number"},"critical":{"type":"number"}},"description":"Custom urgency bucket thresholds"},"timeout_secs":{"type":"integer","description":"Default timeout in seconds"}}}"#,
    },
];

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────────

/// Compute Jaccard similarity between two strings by word overlap.
fn word_overlap(a: &str, b: &str) -> f64 {
    let words_a: HashSet<String> = a
        .to_lowercase()
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(String::from)
        .collect();
    let words_b: HashSet<String> = b
        .to_lowercase()
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(String::from)
        .collect();
    if words_a.is_empty() || words_b.is_empty() {
        return 0.0;
    }
    let intersection = words_a.intersection(&words_b).count() as f64;
    let union_count = words_a.union(&words_b).count() as f64;
    intersection / union_count
}

/// Severity weight: Info=0.1, Warning=0.3, Critical=0.7, Fatal=1.0.
fn severity_weight(sev: &agentic_contract::ViolationSeverity) -> f64 {
    match sev {
        agentic_contract::ViolationSeverity::Info => 0.1,
        agentic_contract::ViolationSeverity::Warning => 0.3,
        agentic_contract::ViolationSeverity::Critical => 0.7,
        agentic_contract::ViolationSeverity::Fatal => 1.0,
    }
}

/// Determine monitoring level from trust score.
fn monitoring_from_trust(trust: f64) -> MonitoringLevel {
    if trust >= 0.85 {
        MonitoringLevel::Minimal
    } else if trust >= 0.60 {
        MonitoringLevel::Standard
    } else if trust >= 0.35 {
        MonitoringLevel::Enhanced
    } else {
        MonitoringLevel::FullAudit
    }
}

/// Format a MonitoringLevel as string.
fn monitoring_label(ml: &MonitoringLevel) -> &'static str {
    match ml {
        MonitoringLevel::Minimal => "minimal",
        MonitoringLevel::Standard => "standard",
        MonitoringLevel::Enhanced => "enhanced",
        MonitoringLevel::FullAudit => "full_audit",
    }
}

/// Format a GovernanceLevel as string.
fn governance_label(gl: &GovernanceLevel) -> &'static str {
    match gl {
        GovernanceLevel::Conservative => "conservative",
        GovernanceLevel::Moderate => "moderate",
        GovernanceLevel::Permissive => "permissive",
        GovernanceLevel::Autonomous => "autonomous",
    }
}

/// Numeric order for governance levels (0=conservative .. 3=autonomous).
fn governance_ordinal(gl: &GovernanceLevel) -> u8 {
    match gl {
        GovernanceLevel::Conservative => 0,
        GovernanceLevel::Moderate => 1,
        GovernanceLevel::Permissive => 2,
        GovernanceLevel::Autonomous => 3,
    }
}

/// Governance level from ordinal, clamped to valid range.
fn governance_from_ordinal(o: i32) -> GovernanceLevel {
    match o.clamp(0, 3) {
        0 => GovernanceLevel::Conservative,
        1 => GovernanceLevel::Moderate,
        2 => GovernanceLevel::Permissive,
        _ => GovernanceLevel::Autonomous,
    }
}

/// Parse a GovernanceLevel from a string.
fn parse_governance_level(s: &str) -> Result<GovernanceLevel, String> {
    match s {
        "conservative" => Ok(GovernanceLevel::Conservative),
        "moderate" => Ok(GovernanceLevel::Moderate),
        "permissive" => Ok(GovernanceLevel::Permissive),
        "autonomous" => Ok(GovernanceLevel::Autonomous),
        other => Err(format!("Unknown governance level: {}", other)),
    }
}

/// Compute exponential decay trust score from violation history.
/// Returns (trust_score, weighted_violation_sum, violation_count_for_agent).
fn compute_trust_from_violations(
    violations: &[agentic_contract::Violation],
    agent_id: &str,
    now: DateTime<Utc>,
) -> (f64, f64, usize) {
    let agent_violations: Vec<_> = violations.iter().filter(|v| v.actor == agent_id).collect();

    if agent_violations.is_empty() {
        return (1.0, 0.0, 0);
    }

    let mut weighted_sum = 0.0;
    for v in &agent_violations {
        let age = now.signed_duration_since(v.detected_at);
        let age_days = age.num_seconds() as f64 / 86400.0;
        // Half-life of 30 days
        let decay_factor = (-0.693 * age_days / 30.0).exp();
        weighted_sum += severity_weight(&v.severity) * decay_factor;
    }

    // Trust = 1 / (1 + weighted_sum), sigmoid-like function
    let trust = 1.0 / (1.0 + weighted_sum);
    (trust, weighted_sum, agent_violations.len())
}

/// Compute the approval track record for an agent from approval decisions.
fn compute_approval_track_record(
    requests: &[agentic_contract::ApprovalRequest],
    decisions: &[agentic_contract::ApprovalDecision],
    agent_id: &str,
) -> f64 {
    let agent_requests: Vec<_> = requests
        .iter()
        .filter(|r| r.requestor == agent_id)
        .collect();

    if agent_requests.is_empty() {
        return 0.5; // neutral when no history
    }

    let request_ids: HashSet<_> = agent_requests.iter().map(|r| r.id).collect();

    let relevant_decisions: Vec<_> = decisions
        .iter()
        .filter(|d| request_ids.contains(&d.request_id))
        .collect();

    if relevant_decisions.is_empty() {
        return 0.5;
    }

    let approvals = relevant_decisions
        .iter()
        .filter(|d| d.decision == DecisionType::Approve)
        .count();

    approvals as f64 / relevant_decisions.len() as f64
}

/// Build a trust gradient evaluation for a single agent.
fn build_trust_gradient(engine: &ContractEngine, agent_id: &str, action: &str) -> Value {
    let now = Utc::now();

    // 1. Violation history factor (weight 0.4)
    let (trust_from_violations, weighted_violation_sum, violation_count) =
        compute_trust_from_violations(&engine.file.violations, agent_id, now);

    // 2. Policy compliance factor (weight 0.3)
    // Check how many policies the agent's action is compliant with
    let total_policies = engine.file.policies.len();
    let matching_deny_policies = engine
        .file
        .policies
        .iter()
        .filter(|p| {
            p.is_active()
                && p.action == agentic_contract::PolicyAction::Deny
                && word_overlap(&p.label, action) > 0.2
        })
        .count();
    let policy_compliance = if total_policies == 0 {
        1.0
    } else {
        1.0 - (matching_deny_policies as f64 / total_policies.max(1) as f64)
    };

    // 3. Approval track record factor (weight 0.2)
    let approval_record = compute_approval_track_record(
        &engine.file.approval_requests,
        &engine.file.approval_decisions,
        agent_id,
    );

    // 4. Tenure factor (weight 0.1)
    // Estimate tenure from earliest violation or approval request
    let earliest_activity = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id)
        .map(|v| v.detected_at)
        .chain(
            engine
                .file
                .approval_requests
                .iter()
                .filter(|r| r.requestor == agent_id)
                .map(|r| r.created_at),
        )
        .min();

    let tenure_score = match earliest_activity {
        Some(first) => {
            let tenure_days = now.signed_duration_since(first).num_days() as f64;
            // Logarithmic growth: ~0.5 at 7 days, ~0.8 at 90 days, ~0.95 at 365 days
            (tenure_days / (tenure_days + 30.0)).min(1.0)
        }
        None => 0.1, // brand new, very low tenure
    };

    // Weighted trust score
    let trust_factor = trust_from_violations * 0.4
        + policy_compliance * 0.3
        + approval_record * 0.2
        + tenure_score * 0.1;

    let monitoring = monitoring_from_trust(trust_factor);

    // Confidence based on amount of data
    let data_points = violation_count
        + engine
            .file
            .approval_requests
            .iter()
            .filter(|r| r.requestor == agent_id)
            .count();
    let confidence = (data_points as f64 / (data_points as f64 + 5.0)).min(0.99);

    // Contributing factors with trends
    // Trend: compare recent violations (last 7 days) vs older
    let recent_violations = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && now.signed_duration_since(v.detected_at).num_days() < 7)
        .count();
    let older_violations = engine
        .file
        .violations
        .iter()
        .filter(|v| {
            v.actor == agent_id
                && now.signed_duration_since(v.detected_at).num_days() >= 7
                && now.signed_duration_since(v.detected_at).num_days() < 37
        })
        .count();
    // Positive trend means improving (fewer recent violations)
    let violation_trend = if older_violations == 0 && recent_violations == 0 {
        0.0
    } else {
        (older_violations as f64 - recent_violations as f64 * 4.3)
            / (older_violations as f64 + recent_violations as f64 * 4.3 + 1.0)
    };

    let contributing_factors = vec![
        json!({
            "name": "violation_history",
            "weight": 0.4,
            "score": trust_from_violations,
            "trend": violation_trend
        }),
        json!({
            "name": "policy_compliance",
            "weight": 0.3,
            "score": policy_compliance,
            "trend": 0.0
        }),
        json!({
            "name": "approval_track_record",
            "weight": 0.2,
            "score": approval_record,
            "trend": 0.0
        }),
        json!({
            "name": "tenure",
            "weight": 0.1,
            "score": tenure_score,
            "trend": 0.01
        }),
    ];

    let auto_revoke_threshold = 0.15;

    json!({
        "id": ContractId::new().to_string(),
        "agent_id": agent_id,
        "action": action,
        "trust_factor": (trust_factor * 1000.0).round() / 1000.0,
        "confidence": (confidence * 1000.0).round() / 1000.0,
        "monitoring_level": monitoring_label(&monitoring),
        "auto_revoke_threshold": auto_revoke_threshold,
        "contributing_factors": contributing_factors,
        "violation_count": violation_count,
        "weighted_violation_sum": (weighted_violation_sum * 1000.0).round() / 1000.0,
        "evaluated_at": now.to_rfc3339()
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// In-memory stores for stateful governance inventions
// ─────────────────────────────────────────────────────────────────────────────

// We use thread-local stores for collective contracts, temporal contracts,
// inheritance records, and escalation config. In production, these would
// persist via the engine's file format.

use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref COLLECTIVE_CONTRACTS: Mutex<Vec<CollectiveContractRecord>> =
        Mutex::new(Vec::new());
    static ref TEMPORAL_CONTRACTS: Mutex<Vec<TemporalContractRecord>> =
        Mutex::new(Vec::new());
    static ref INHERITANCE_RECORDS: Mutex<Vec<InheritanceRecord>> =
        Mutex::new(Vec::new());
    static ref ESCALATION_CONFIG: Mutex<EscalationConfig> =
        Mutex::new(EscalationConfig::default());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CollectiveContractRecord {
    id: ContractId,
    parties: Vec<CollectivePartyRecord>,
    shared_policy_ids: Vec<ContractId>,
    arbitration_method: String,
    quorum_ratio: f64,
    required_signatures: u32,
    signatures: u32,
    status: String,
    disputes: Vec<DisputeRecord>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CollectivePartyRecord {
    party_id: String,
    name: String,
    role: String,
    signed: bool,
    signed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DisputeRecord {
    id: ContractId,
    description: String,
    filed_by: String,
    resolution: Option<String>,
    recommendation: Option<String>,
    filed_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TemporalContractRecord {
    id: ContractId,
    label: String,
    initial_level: String,
    current_level: String,
    transition_conditions: Vec<String>,
    performance_history: Vec<PerformanceEntry>,
    transitions: Vec<TransitionEntry>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceEntry {
    score: f64,
    recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransitionEntry {
    from_level: String,
    to_level: String,
    reason: String,
    performance_at_transition: f64,
    transitioned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InheritanceRecord {
    id: ContractId,
    parent_id: ContractId,
    child_id: ContractId,
    inherited_policy_ids: Vec<ContractId>,
    overrides: Vec<OverrideRecord>,
    propagate_changes: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverrideRecord {
    id: ContractId,
    policy_id: ContractId,
    override_type: String,
    description: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EscalationConfig {
    approvers: Vec<ApproverRecord>,
    urgency_thresholds: UrgencyThresholds,
    default_timeout_secs: u64,
}

impl Default for EscalationConfig {
    fn default() -> Self {
        Self {
            approvers: vec![
                ApproverRecord {
                    id: "admin".into(),
                    name: "Administrator".into(),
                    domains: vec!["security".into(), "infrastructure".into()],
                    availability: 0.8,
                    avg_response_secs: 300,
                    approval_rate: 0.75,
                },
                ApproverRecord {
                    id: "lead".into(),
                    name: "Team Lead".into(),
                    domains: vec!["deployment".into(), "code_review".into()],
                    availability: 0.9,
                    avg_response_secs: 600,
                    approval_rate: 0.85,
                },
                ApproverRecord {
                    id: "manager".into(),
                    name: "Engineering Manager".into(),
                    domains: vec!["budget".into(), "hiring".into(), "strategy".into()],
                    availability: 0.6,
                    avg_response_secs: 1800,
                    approval_rate: 0.65,
                },
            ],
            urgency_thresholds: UrgencyThresholds::default(),
            default_timeout_secs: 3600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApproverRecord {
    id: String,
    name: String,
    domains: Vec<String>,
    availability: f64,
    avg_response_secs: i64,
    approval_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UrgencyThresholds {
    low: f64,
    medium: f64,
    high: f64,
    critical: f64,
}

impl Default for UrgencyThresholds {
    fn default() -> Self {
        Self {
            low: 0.25,
            medium: 0.5,
            high: 0.75,
            critical: 0.9,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Main dispatcher
// ─────────────────────────────────────────────────────────────────────────────

/// Try to handle a tool call for inventions 8-12.
/// Returns `None` if the tool name is not recognized by this module.
pub fn try_handle(
    name: &str,
    args: Value,
    engine: &mut ContractEngine,
) -> Option<Result<Value, String>> {
    match name {
        // ── Invention 8: Trust Gradients ────────────────────────────────
        "trust_gradient_evaluate" => Some(handle_trust_gradient_evaluate(args, engine)),
        "trust_gradient_history" => Some(handle_trust_gradient_history(args, engine)),
        "trust_gradient_predict" => Some(handle_trust_gradient_predict(args, engine)),
        "trust_gradient_compare" => Some(handle_trust_gradient_compare(args, engine)),

        // ── Invention 9: Collective Contracts ──────────────────────────
        "collective_contract_create" => Some(handle_collective_contract_create(args, engine)),
        "collective_contract_sign" => Some(handle_collective_contract_sign(args, engine)),
        "collective_contract_status" => Some(handle_collective_contract_status(args, engine)),
        "collective_contract_arbitrate" => Some(handle_collective_contract_arbitrate(args, engine)),

        // ── Invention 10: Temporal Contracts ───────────────────────────
        "temporal_contract_create" => Some(handle_temporal_contract_create(args, engine)),
        "temporal_contract_transition" => Some(handle_temporal_contract_transition(args, engine)),
        "temporal_contract_history" => Some(handle_temporal_contract_history(args, engine)),
        "temporal_contract_predict" => Some(handle_temporal_contract_predict(args, engine)),

        // ── Invention 11: Contract Inheritance ─────────────────────────
        "contract_inheritance_create" => Some(handle_contract_inheritance_create(args, engine)),
        "contract_inheritance_tree" => Some(handle_contract_inheritance_tree(args, engine)),
        "contract_inheritance_resolve" => Some(handle_contract_inheritance_resolve(args, engine)),
        "contract_inheritance_override" => Some(handle_contract_inheritance_override(args, engine)),

        // ── Invention 12: Smart Escalation ─────────────────────────────
        "smart_escalation_route" => Some(handle_smart_escalation_route(args, engine)),
        "smart_escalation_history" => Some(handle_smart_escalation_history(args, engine)),
        "smart_escalation_predict" => Some(handle_smart_escalation_predict(args, engine)),
        "smart_escalation_configure" => Some(handle_smart_escalation_configure(args, engine)),

        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 8: Trust Gradients
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_trust_gradient_evaluate(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let action = require_str(&args, "action")?;

    let result = build_trust_gradient(engine, agent_id, action);
    Ok(result)
}

fn handle_trust_gradient_history(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let window_days = args
        .get("window_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(90);
    let now = Utc::now();
    let window_start = now - Duration::days(window_days);

    // Collect agent violations sorted chronologically
    let mut agent_violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && v.detected_at >= window_start)
        .collect();
    agent_violations.sort_by_key(|v| v.detected_at);

    // Build trust time-series: compute trust at each violation event
    let mut time_series: Vec<Value> = Vec::new();

    // Starting point: trust before any violations in window
    let violations_before_window: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && v.detected_at < window_start)
        .collect();
    let (initial_trust, _, _) = if violations_before_window.is_empty() {
        (1.0, 0.0, 0)
    } else {
        compute_trust_from_violations(&engine.file.violations, agent_id, window_start)
    };

    time_series.push(json!({
        "timestamp": window_start.to_rfc3339(),
        "trust_score": (initial_trust * 1000.0).round() / 1000.0,
        "event": "window_start",
        "annotation": "Trust at beginning of analysis window"
    }));

    // At each violation, recompute trust as of that moment
    for violation in &agent_violations {
        let (trust_at_point, weighted_sum, _) =
            compute_trust_from_violations(&engine.file.violations, agent_id, violation.detected_at);

        time_series.push(json!({
            "timestamp": violation.detected_at.to_rfc3339(),
            "trust_score": (trust_at_point * 1000.0).round() / 1000.0,
            "event": "violation",
            "annotation": format!(
                "Violation: {} (severity: {:?}, weighted_sum: {:.3})",
                violation.description,
                violation.severity,
                weighted_sum
            ),
            "violation_id": violation.id.to_string(),
            "severity": format!("{:?}", violation.severity)
        }));
    }

    // Current trust
    let (current_trust, current_sum, total_count) =
        compute_trust_from_violations(&engine.file.violations, agent_id, now);

    time_series.push(json!({
        "timestamp": now.to_rfc3339(),
        "trust_score": (current_trust * 1000.0).round() / 1000.0,
        "event": "current",
        "annotation": "Current trust level"
    }));

    // Compute overall trend
    let trend = if time_series.len() >= 3 {
        let first = time_series[0]["trust_score"].as_f64().unwrap_or(1.0);
        let last = time_series
            .last()
            .and_then(|v| v["trust_score"].as_f64())
            .unwrap_or(1.0);
        last - first
    } else {
        0.0
    };

    Ok(json!({
        "agent_id": agent_id,
        "window_days": window_days,
        "time_series": time_series,
        "current_trust": (current_trust * 1000.0).round() / 1000.0,
        "total_violations_in_window": agent_violations.len(),
        "total_violations_all_time": total_count,
        "weighted_violation_sum": (current_sum * 1000.0).round() / 1000.0,
        "trend": (trend * 1000.0).round() / 1000.0,
        "trend_label": if trend > 0.05 { "improving" } else if trend < -0.05 { "declining" } else { "stable" },
        "analyzed_at": now.to_rfc3339()
    }))
}

fn handle_trust_gradient_predict(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let forecast_days = args
        .get("forecast_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(30);
    let now = Utc::now();

    // Current trust
    let (current_trust, _, _) =
        compute_trust_from_violations(&engine.file.violations, agent_id, now);

    // Compute violation velocity: violations per week in last 30 days
    let recent_violations = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && now.signed_duration_since(v.detected_at).num_days() < 30)
        .count();
    let violation_velocity = recent_violations as f64 / 4.0; // per week

    // Average severity of recent violations
    let avg_severity = if recent_violations > 0 {
        engine
            .file
            .violations
            .iter()
            .filter(|v| {
                v.actor == agent_id && now.signed_duration_since(v.detected_at).num_days() < 30
            })
            .map(|v| severity_weight(&v.severity))
            .sum::<f64>()
            / recent_violations as f64
    } else {
        0.0
    };

    // Project trust over forecast window
    let mut projections: Vec<Value> = Vec::new();
    let step_days = if forecast_days <= 7 { 1 } else { 7 };

    for day in (0..=forecast_days).step_by(step_days as usize) {
        let projected_time = now + Duration::days(day);

        if violation_velocity > 0.0 {
            // Project new violations accumulating
            let projected_new_violations = violation_velocity * (day as f64 / 7.0);
            let projected_new_weight = projected_new_violations * avg_severity;

            // Also account for existing violations decaying
            let (_, existing_weight, _) =
                compute_trust_from_violations(&engine.file.violations, agent_id, projected_time);

            let total_weight = existing_weight + projected_new_weight;
            let projected_trust = 1.0 / (1.0 + total_weight);

            projections.push(json!({
                "day": day,
                "timestamp": projected_time.to_rfc3339(),
                "projected_trust": (projected_trust * 1000.0).round() / 1000.0,
                "projected_new_violations": (projected_new_violations * 10.0).round() / 10.0,
                "monitoring_level": monitoring_label(&monitoring_from_trust(projected_trust))
            }));
        } else {
            // Clean record: existing violations decay, trust recovers
            let (projected_trust, _, _) =
                compute_trust_from_violations(&engine.file.violations, agent_id, projected_time);

            projections.push(json!({
                "day": day,
                "timestamp": projected_time.to_rfc3339(),
                "projected_trust": (projected_trust * 1000.0).round() / 1000.0,
                "projected_new_violations": 0,
                "monitoring_level": monitoring_label(&monitoring_from_trust(projected_trust))
            }));
        }
    }

    // Determine if trust will cross thresholds
    let mut threshold_crossings: Vec<Value> = Vec::new();
    let thresholds = [
        (0.85, "minimal_monitoring"),
        (0.60, "standard_monitoring"),
        (0.35, "enhanced_monitoring"),
        (0.15, "auto_revoke"),
    ];

    for (threshold, label) in &thresholds {
        // Find when projected trust crosses this threshold
        if current_trust > *threshold {
            // Will it drop below?
            for p in &projections {
                let pt = p["projected_trust"].as_f64().unwrap_or(1.0);
                if pt < *threshold {
                    threshold_crossings.push(json!({
                        "threshold": threshold,
                        "label": label,
                        "direction": "downward",
                        "estimated_day": p["day"],
                        "estimated_date": p["timestamp"]
                    }));
                    break;
                }
            }
        } else {
            // Will it rise above?
            for p in &projections {
                let pt = p["projected_trust"].as_f64().unwrap_or(0.0);
                if pt >= *threshold {
                    threshold_crossings.push(json!({
                        "threshold": threshold,
                        "label": label,
                        "direction": "upward",
                        "estimated_day": p["day"],
                        "estimated_date": p["timestamp"]
                    }));
                    break;
                }
            }
        }
    }

    let trajectory = if violation_velocity > 1.0 {
        "declining_fast"
    } else if violation_velocity > 0.0 {
        "declining_slow"
    } else if current_trust < 0.95 {
        "recovering"
    } else {
        "stable_high"
    };

    Ok(json!({
        "agent_id": agent_id,
        "current_trust": (current_trust * 1000.0).round() / 1000.0,
        "forecast_days": forecast_days,
        "violation_velocity_per_week": (violation_velocity * 100.0).round() / 100.0,
        "avg_violation_severity": (avg_severity * 1000.0).round() / 1000.0,
        "trajectory": trajectory,
        "projections": projections,
        "threshold_crossings": threshold_crossings,
        "predicted_at": now.to_rfc3339()
    }))
}

fn handle_trust_gradient_compare(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_a = require_str(&args, "agent_a")?;
    let agent_b = require_str(&args, "agent_b")?;
    let now = Utc::now();

    let gradient_a = build_trust_gradient(engine, agent_a, "*");
    let gradient_b = build_trust_gradient(engine, agent_b, "*");

    let trust_a = gradient_a["trust_factor"].as_f64().unwrap_or(0.0);
    let trust_b = gradient_b["trust_factor"].as_f64().unwrap_or(0.0);

    let (_, _, violations_a) = compute_trust_from_violations(&engine.file.violations, agent_a, now);
    let (_, _, violations_b) = compute_trust_from_violations(&engine.file.violations, agent_b, now);

    let approval_a = compute_approval_track_record(
        &engine.file.approval_requests,
        &engine.file.approval_decisions,
        agent_a,
    );
    let approval_b = compute_approval_track_record(
        &engine.file.approval_requests,
        &engine.file.approval_decisions,
        agent_b,
    );

    let more_trusted = if trust_a > trust_b {
        agent_a
    } else if trust_b > trust_a {
        agent_b
    } else {
        "equal"
    };

    let mut advantages_a: Vec<String> = Vec::new();
    let mut advantages_b: Vec<String> = Vec::new();

    if trust_a > trust_b + 0.05 {
        advantages_a.push(format!(
            "Higher overall trust ({:.3} vs {:.3})",
            trust_a, trust_b
        ));
    } else if trust_b > trust_a + 0.05 {
        advantages_b.push(format!(
            "Higher overall trust ({:.3} vs {:.3})",
            trust_b, trust_a
        ));
    }

    if violations_a < violations_b {
        advantages_a.push(format!(
            "Fewer violations ({} vs {})",
            violations_a, violations_b
        ));
    } else if violations_b < violations_a {
        advantages_b.push(format!(
            "Fewer violations ({} vs {})",
            violations_b, violations_a
        ));
    }

    if approval_a > approval_b + 0.05 {
        advantages_a.push(format!(
            "Better approval record ({:.1}% vs {:.1}%)",
            approval_a * 100.0,
            approval_b * 100.0
        ));
    } else if approval_b > approval_a + 0.05 {
        advantages_b.push(format!(
            "Better approval record ({:.1}% vs {:.1}%)",
            approval_b * 100.0,
            approval_a * 100.0
        ));
    }

    Ok(json!({
        "agent_a": {
            "id": agent_a,
            "trust_factor": trust_a,
            "monitoring_level": gradient_a["monitoring_level"],
            "violation_count": violations_a,
            "approval_rate": (approval_a * 1000.0).round() / 1000.0,
            "contributing_factors": gradient_a["contributing_factors"],
            "advantages": advantages_a
        },
        "agent_b": {
            "id": agent_b,
            "trust_factor": trust_b,
            "monitoring_level": gradient_b["monitoring_level"],
            "violation_count": violations_b,
            "approval_rate": (approval_b * 1000.0).round() / 1000.0,
            "contributing_factors": gradient_b["contributing_factors"],
            "advantages": advantages_b
        },
        "more_trusted": more_trusted,
        "trust_delta": ((trust_a - trust_b).abs() * 1000.0).round() / 1000.0,
        "compared_at": now.to_rfc3339()
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 9: Collective Contracts
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_collective_contract_create(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let parties_arr = args
        .get("parties")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Missing required parameter: parties".to_string())?;

    let parties: Vec<CollectivePartyRecord> = parties_arr
        .iter()
        .filter_map(|p| {
            let id = p.get("id").and_then(|v| v.as_str())?;
            let name = p.get("name").and_then(|v| v.as_str())?;
            let role = p.get("role").and_then(|v| v.as_str()).unwrap_or("member");
            Some(CollectivePartyRecord {
                party_id: id.to_string(),
                name: name.to_string(),
                role: role.to_string(),
                signed: false,
                signed_at: None,
            })
        })
        .collect();

    if parties.len() < 2 {
        return Err("Collective contract requires at least 2 parties".to_string());
    }

    let arbitration = args
        .get("arbitration")
        .and_then(|v| v.as_str())
        .unwrap_or("majority_vote")
        .to_string();

    let quorum_ratio = args
        .get("quorum_ratio")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5)
        .clamp(0.0, 1.0);

    let required_signatures = ((parties.len() as f64 * quorum_ratio).ceil() as u32).max(1);

    // Gather shared policies from engine
    let shared_policy_ids: Vec<ContractId> = engine
        .file
        .policies
        .iter()
        .filter(|p| p.is_active() && p.scope == agentic_contract::PolicyScope::Global)
        .map(|p| p.id)
        .collect();

    let id = ContractId::new();
    let now = Utc::now();

    let record = CollectiveContractRecord {
        id,
        parties: parties.clone(),
        shared_policy_ids: shared_policy_ids.clone(),
        arbitration_method: arbitration.clone(),
        quorum_ratio,
        required_signatures,
        signatures: 0,
        status: "pending".to_string(),
        disputes: Vec::new(),
        created_at: now,
    };

    let mut store = COLLECTIVE_CONTRACTS.lock().unwrap();
    store.push(record);

    Ok(json!({
        "id": id.to_string(),
        "parties": parties.iter().map(|p| json!({
            "party_id": p.party_id,
            "name": p.name,
            "role": p.role,
            "signed": p.signed
        })).collect::<Vec<_>>(),
        "shared_policies": shared_policy_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
        "shared_policy_count": shared_policy_ids.len(),
        "arbitration_method": arbitration,
        "quorum_ratio": quorum_ratio,
        "required_signatures": required_signatures,
        "signatures": 0,
        "status": "pending",
        "created_at": now.to_rfc3339()
    }))
}

fn handle_collective_contract_sign(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id_str = require_str(&args, "contract_id")?;
    let signer_id = require_str(&args, "signer_id")?;
    let contract_id: ContractId = contract_id_str
        .parse()
        .map_err(|e| format!("Invalid contract_id: {}", e))?;
    let now = Utc::now();

    let mut store = COLLECTIVE_CONTRACTS.lock().unwrap();
    let contract = store
        .iter_mut()
        .find(|c| c.id == contract_id)
        .ok_or_else(|| format!("Collective contract not found: {}", contract_id))?;

    // Check if signer is a party
    let party = contract
        .parties
        .iter_mut()
        .find(|p| p.party_id == signer_id)
        .ok_or_else(|| {
            format!(
                "Signer '{}' is not a party to contract {}",
                signer_id, contract_id
            )
        })?;

    // Check if already signed
    if party.signed {
        return Err(format!(
            "Party '{}' has already signed this contract",
            signer_id
        ));
    }

    // Apply signature
    party.signed = true;
    party.signed_at = Some(now);
    contract.signatures += 1;

    // Check quorum
    let quorum_reached = contract.signatures >= contract.required_signatures;
    if quorum_reached {
        contract.status = "active".to_string();
    }

    let unsigned_parties: Vec<String> = contract
        .parties
        .iter()
        .filter(|p| !p.signed)
        .map(|p| p.party_id.clone())
        .collect();

    Ok(json!({
        "contract_id": contract_id.to_string(),
        "signer_id": signer_id,
        "signed_at": now.to_rfc3339(),
        "signatures": contract.signatures,
        "required_signatures": contract.required_signatures,
        "quorum_reached": quorum_reached,
        "status": contract.status,
        "remaining_unsigned": unsigned_parties,
        "remaining_count": unsigned_parties.len()
    }))
}

fn handle_collective_contract_status(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id_str = require_str(&args, "contract_id")?;
    let contract_id: ContractId = contract_id_str
        .parse()
        .map_err(|e| format!("Invalid contract_id: {}", e))?;
    let now = Utc::now();

    let store = COLLECTIVE_CONTRACTS.lock().unwrap();
    let contract = store
        .iter()
        .find(|c| c.id == contract_id)
        .ok_or_else(|| format!("Collective contract not found: {}", contract_id))?;

    let signed_parties: Vec<Value> = contract
        .parties
        .iter()
        .filter(|p| p.signed)
        .map(|p| {
            json!({
                "party_id": p.party_id,
                "name": p.name,
                "role": p.role,
                "signed_at": p.signed_at.map(|t| t.to_rfc3339())
            })
        })
        .collect();

    let unsigned_parties: Vec<Value> = contract
        .parties
        .iter()
        .filter(|p| !p.signed)
        .map(|p| {
            json!({
                "party_id": p.party_id,
                "name": p.name,
                "role": p.role
            })
        })
        .collect();

    let age_secs = now.signed_duration_since(contract.created_at).num_seconds();

    // Estimate time to full signature based on signing velocity
    let estimated_completion_secs =
        if contract.signatures > 0 && contract.signatures < contract.parties.len() as u32 {
            let secs_per_sig = age_secs as f64 / contract.signatures as f64;
            let remaining = contract.parties.len() as u32 - contract.signatures;
            Some((secs_per_sig * remaining as f64) as i64)
        } else {
            None
        };

    // Stall risk: high if no signatures after 24 hours
    let stall_risk = if contract.signatures == 0 && age_secs > 86400 {
        "high"
    } else if contract.signatures < contract.required_signatures && age_secs > 172800 {
        "medium"
    } else {
        "low"
    };

    Ok(json!({
        "contract_id": contract_id.to_string(),
        "status": contract.status,
        "parties_total": contract.parties.len(),
        "signatures": contract.signatures,
        "required_signatures": contract.required_signatures,
        "quorum_ratio": contract.quorum_ratio,
        "signed_parties": signed_parties,
        "unsigned_parties": unsigned_parties,
        "age_seconds": age_secs,
        "age_human": format_duration(age_secs),
        "estimated_completion_secs": estimated_completion_secs,
        "stall_risk": stall_risk,
        "arbitration_method": contract.arbitration_method,
        "shared_policy_count": contract.shared_policy_ids.len(),
        "active_disputes": contract.disputes.iter().filter(|d| d.resolved_at.is_none()).count(),
        "created_at": contract.created_at.to_rfc3339()
    }))
}

fn handle_collective_contract_arbitrate(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id_str = require_str(&args, "contract_id")?;
    let dispute_description = require_str(&args, "dispute_description")?;
    let filed_by = require_str(&args, "filed_by")?;
    let contract_id: ContractId = contract_id_str
        .parse()
        .map_err(|e| format!("Invalid contract_id: {}", e))?;
    let now = Utc::now();

    let mut store = COLLECTIVE_CONTRACTS.lock().unwrap();
    let contract = store
        .iter_mut()
        .find(|c| c.id == contract_id)
        .ok_or_else(|| format!("Collective contract not found: {}", contract_id))?;

    // Validate filer is a party
    if !contract.parties.iter().any(|p| p.party_id == filed_by) {
        return Err(format!(
            "Filer '{}' is not a party to contract {}",
            filed_by, contract_id
        ));
    }

    // Analyze dispute against policies
    let relevant_policies: Vec<Value> = engine
        .file
        .policies
        .iter()
        .filter(|p| word_overlap(&p.label, dispute_description) > 0.1)
        .map(|p| {
            json!({
                "policy_id": p.id.to_string(),
                "label": p.label,
                "action": format!("{:?}", p.action),
                "relevance": (word_overlap(&p.label, dispute_description) * 100.0).round() / 100.0
            })
        })
        .collect();

    // Generate resolution recommendation based on arbitration method
    let recommendation = match contract.arbitration_method.as_str() {
        "majority_vote" => {
            let party_count = contract.parties.len();
            format!(
                "Submit dispute to majority vote. {} of {} parties must agree on resolution. \
                 Based on {} relevant policies, recommend evaluating compliance of all parties.",
                party_count / 2 + 1,
                party_count,
                relevant_policies.len()
            )
        }
        "unanimous" => "All parties must agree on resolution. Schedule mediation session \
             to reach consensus."
            .to_string(),
        "third_party" => "Refer dispute to designated third-party arbitrator. Both parties \
             must submit evidence within 72 hours."
            .to_string(),
        "automated" => {
            if relevant_policies.is_empty() {
                "No directly relevant policies found. Recommend adding specific \
                 governance rules to prevent future ambiguity."
                    .to_string()
            } else {
                format!(
                    "Automated resolution: {} relevant policies found. \
                     Applying strictest applicable policy interpretation.",
                    relevant_policies.len()
                )
            }
        }
        _ => "Unknown arbitration method. Falling back to majority vote.".to_string(),
    };

    // Record dispute
    let dispute_id = ContractId::new();
    contract.disputes.push(DisputeRecord {
        id: dispute_id,
        description: dispute_description.to_string(),
        filed_by: filed_by.to_string(),
        resolution: None,
        recommendation: Some(recommendation.clone()),
        filed_at: now,
        resolved_at: None,
    });

    // Update contract status
    contract.status = "disputed".to_string();

    Ok(json!({
        "dispute_id": dispute_id.to_string(),
        "contract_id": contract_id.to_string(),
        "dispute_description": dispute_description,
        "filed_by": filed_by,
        "arbitration_method": contract.arbitration_method,
        "relevant_policies": relevant_policies,
        "recommendation": recommendation,
        "contract_status": "disputed",
        "filed_at": now.to_rfc3339()
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 10: Temporal Contracts
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_temporal_contract_create(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let label = require_str(&args, "label")?;
    let initial_level_str = args
        .get("initial_level")
        .and_then(|v| v.as_str())
        .unwrap_or("conservative");
    let initial_level = parse_governance_level(initial_level_str)?;

    let transition_conditions: Vec<String> = args
        .get("transition_conditions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_else(|| {
            vec![
                "performance >= 0.8 for 7 consecutive days".to_string(),
                "zero critical violations in 14 days".to_string(),
                "approval rate >= 90% over 30 days".to_string(),
            ]
        });

    let id = ContractId::new();
    let now = Utc::now();

    let record = TemporalContractRecord {
        id,
        label: label.to_string(),
        initial_level: governance_label(&initial_level).to_string(),
        current_level: governance_label(&initial_level).to_string(),
        transition_conditions: transition_conditions.clone(),
        performance_history: Vec::new(),
        transitions: Vec::new(),
        created_at: now,
    };

    let mut store = TEMPORAL_CONTRACTS.lock().unwrap();
    store.push(record);

    Ok(json!({
        "id": id.to_string(),
        "label": label,
        "initial_level": governance_label(&initial_level),
        "current_level": governance_label(&initial_level),
        "transition_conditions": transition_conditions,
        "next_possible_level": governance_label(&governance_from_ordinal(
            governance_ordinal(&initial_level) as i32 + 1
        )),
        "performance_history": [],
        "created_at": now.to_rfc3339()
    }))
}

fn handle_temporal_contract_transition(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id_str = require_str(&args, "contract_id")?;
    let performance_score = args
        .get("performance_score")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| "Missing required parameter: performance_score".to_string())?;
    let performance_score = performance_score.clamp(0.0, 1.0);

    let contract_id: ContractId = contract_id_str
        .parse()
        .map_err(|e| format!("Invalid contract_id: {}", e))?;
    let now = Utc::now();

    let mut store = TEMPORAL_CONTRACTS.lock().unwrap();
    let contract = store
        .iter_mut()
        .find(|c| c.id == contract_id)
        .ok_or_else(|| format!("Temporal contract not found: {}", contract_id))?;

    // Record performance
    contract.performance_history.push(PerformanceEntry {
        score: performance_score,
        recorded_at: now,
    });

    let current_level = parse_governance_level(&contract.current_level)?;
    let current_ordinal = governance_ordinal(&current_level);

    // Evaluate transition conditions
    // Upward transition: performance >= 0.8 for last 7 entries
    // Downward transition: performance < 0.4 for last 3 entries
    let history_len = contract.performance_history.len();

    let recent_avg = if history_len >= 3 {
        let recent: Vec<f64> = contract
            .performance_history
            .iter()
            .rev()
            .take(7)
            .map(|e| e.score)
            .collect();
        recent.iter().sum::<f64>() / recent.len() as f64
    } else {
        performance_score
    };

    let should_upgrade = history_len >= 7
        && contract
            .performance_history
            .iter()
            .rev()
            .take(7)
            .all(|e| e.score >= 0.8)
        && current_ordinal < 3;

    let should_downgrade = history_len >= 3
        && contract
            .performance_history
            .iter()
            .rev()
            .take(3)
            .all(|e| e.score < 0.4)
        && current_ordinal > 0;

    let mut transition_applied = false;
    let mut new_level = governance_label(&current_level).to_string();
    let mut reason = String::new();

    if should_upgrade {
        let next_level = governance_from_ordinal(current_ordinal as i32 + 1);
        new_level = governance_label(&next_level).to_string();
        reason = format!(
            "Performance consistently >= 0.8 for 7 entries (avg: {:.3}). Upgrading governance.",
            recent_avg
        );
        contract.current_level = new_level.clone();
        contract.transitions.push(TransitionEntry {
            from_level: governance_label(&current_level).to_string(),
            to_level: new_level.clone(),
            reason: reason.clone(),
            performance_at_transition: performance_score,
            transitioned_at: now,
        });
        transition_applied = true;
    } else if should_downgrade {
        let prev_level = governance_from_ordinal(current_ordinal as i32 - 1);
        new_level = governance_label(&prev_level).to_string();
        reason = format!(
            "Performance consistently < 0.4 for 3 entries (avg: {:.3}). Downgrading governance.",
            recent_avg
        );
        contract.current_level = new_level.clone();
        contract.transitions.push(TransitionEntry {
            from_level: governance_label(&current_level).to_string(),
            to_level: new_level.clone(),
            reason: reason.clone(),
            performance_at_transition: performance_score,
            transitioned_at: now,
        });
        transition_applied = true;
    }

    // Compute governance delta
    let governance_delta = if transition_applied {
        let new_gl = parse_governance_level(&new_level).unwrap_or(current_level);
        governance_ordinal(&new_gl) as i32 - current_ordinal as i32
    } else {
        0
    };

    Ok(json!({
        "contract_id": contract_id.to_string(),
        "performance_score": performance_score,
        "recent_average": (recent_avg * 1000.0).round() / 1000.0,
        "previous_level": governance_label(&current_level),
        "current_level": new_level,
        "transition_applied": transition_applied,
        "governance_delta": governance_delta,
        "reason": if transition_applied { reason } else { "No transition conditions met".to_string() },
        "conditions_for_upgrade": "Performance >= 0.8 for 7 consecutive entries",
        "conditions_for_downgrade": "Performance < 0.4 for 3 consecutive entries",
        "total_performance_entries": contract.performance_history.len(),
        "total_transitions": contract.transitions.len(),
        "evaluated_at": now.to_rfc3339()
    }))
}

fn handle_temporal_contract_history(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id_str = require_str(&args, "contract_id")?;
    let contract_id: ContractId = contract_id_str
        .parse()
        .map_err(|e| format!("Invalid contract_id: {}", e))?;

    let store = TEMPORAL_CONTRACTS.lock().unwrap();
    let contract = store
        .iter()
        .find(|c| c.id == contract_id)
        .ok_or_else(|| format!("Temporal contract not found: {}", contract_id))?;

    let transitions: Vec<Value> = contract
        .transitions
        .iter()
        .map(|t| {
            json!({
                "from_level": t.from_level,
                "to_level": t.to_level,
                "reason": t.reason,
                "performance_at_transition": t.performance_at_transition,
                "transitioned_at": t.transitioned_at.to_rfc3339()
            })
        })
        .collect();

    let performance_entries: Vec<Value> = contract
        .performance_history
        .iter()
        .map(|e| {
            json!({
                "score": e.score,
                "recorded_at": e.recorded_at.to_rfc3339()
            })
        })
        .collect();

    // Compute summary stats
    let avg_performance = if contract.performance_history.is_empty() {
        0.0
    } else {
        contract
            .performance_history
            .iter()
            .map(|e| e.score)
            .sum::<f64>()
            / contract.performance_history.len() as f64
    };

    let min_performance = contract
        .performance_history
        .iter()
        .map(|e| e.score)
        .fold(f64::INFINITY, f64::min);
    let max_performance = contract
        .performance_history
        .iter()
        .map(|e| e.score)
        .fold(f64::NEG_INFINITY, f64::max);

    let upgrades = contract
        .transitions
        .iter()
        .filter(|t| {
            let from = parse_governance_level(&t.from_level).ok();
            let to = parse_governance_level(&t.to_level).ok();
            match (from, to) {
                (Some(f), Some(t)) => governance_ordinal(&t) > governance_ordinal(&f),
                _ => false,
            }
        })
        .count();

    let downgrades = contract.transitions.len() - upgrades;

    Ok(json!({
        "contract_id": contract_id.to_string(),
        "label": contract.label,
        "initial_level": contract.initial_level,
        "current_level": contract.current_level,
        "total_transitions": contract.transitions.len(),
        "upgrades": upgrades,
        "downgrades": downgrades,
        "transitions": transitions,
        "performance_summary": {
            "total_entries": contract.performance_history.len(),
            "average": (avg_performance * 1000.0).round() / 1000.0,
            "min": if min_performance.is_finite() { min_performance } else { 0.0 },
            "max": if max_performance.is_finite() { max_performance } else { 0.0 },
        },
        "performance_history": performance_entries,
        "created_at": contract.created_at.to_rfc3339()
    }))
}

fn handle_temporal_contract_predict(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id_str = require_str(&args, "contract_id")?;
    let contract_id: ContractId = contract_id_str
        .parse()
        .map_err(|e| format!("Invalid contract_id: {}", e))?;
    let now = Utc::now();

    let store = TEMPORAL_CONTRACTS.lock().unwrap();
    let contract = store
        .iter()
        .find(|c| c.id == contract_id)
        .ok_or_else(|| format!("Temporal contract not found: {}", contract_id))?;

    let current_level =
        parse_governance_level(&contract.current_level).unwrap_or(GovernanceLevel::Conservative);
    let current_ordinal = governance_ordinal(&current_level);

    if contract.performance_history.is_empty() {
        return Ok(json!({
            "contract_id": contract_id.to_string(),
            "current_level": contract.current_level,
            "prediction": "Insufficient data",
            "message": "No performance history available for prediction. Submit performance scores first.",
            "predicted_at": now.to_rfc3339()
        }));
    }

    // Compute performance trend using linear regression over recent entries
    let recent_count = contract.performance_history.len().min(20);
    let recent: Vec<f64> = contract
        .performance_history
        .iter()
        .rev()
        .take(recent_count)
        .map(|e| e.score)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let n = recent.len() as f64;
    let sum_x: f64 = (0..recent.len()).map(|i| i as f64).sum();
    let sum_y: f64 = recent.iter().sum();
    let sum_xy: f64 = recent.iter().enumerate().map(|(i, y)| i as f64 * y).sum();
    let sum_x2: f64 = (0..recent.len()).map(|i| (i as f64) * (i as f64)).sum();

    let slope = if n * sum_x2 - sum_x * sum_x > 0.0 {
        (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x)
    } else {
        0.0
    };

    let intercept = (sum_y - slope * sum_x) / n;
    let current_projected = intercept + slope * (n - 1.0);

    // Predict entries needed to reach upgrade threshold (0.8)
    let entries_to_upgrade = if current_ordinal < 3 {
        if slope > 0.001 {
            // Need 7 consecutive entries >= 0.8
            let entries_to_reach_0_8 = if current_projected >= 0.8 {
                0.0
            } else {
                (0.8 - current_projected) / slope
            };
            // Plus 7 entries at >= 0.8
            Some((entries_to_reach_0_8 + 7.0).ceil() as i64)
        } else if current_projected >= 0.8 {
            // Already there, just need 7 consecutive
            let consecutive_above = contract
                .performance_history
                .iter()
                .rev()
                .take_while(|e| e.score >= 0.8)
                .count();
            if consecutive_above >= 7 {
                Some(0)
            } else {
                Some((7 - consecutive_above) as i64)
            }
        } else {
            None // flat or declining below threshold
        }
    } else {
        None // already at highest level
    };

    // Predict entries to downgrade (performance < 0.4 for 3 entries)
    let entries_to_downgrade = if current_ordinal > 0 {
        if slope < -0.001 {
            let entries_to_reach_0_4 = if current_projected < 0.4 {
                0.0
            } else {
                (current_projected - 0.4) / (-slope)
            };
            Some((entries_to_reach_0_4 + 3.0).ceil() as i64)
        } else {
            None
        }
    } else {
        None
    };

    let prediction = if let Some(entries) = entries_to_upgrade {
        if entries == 0 {
            "Upgrade imminent — conditions already met"
        } else if entries <= 7 {
            "Upgrade likely within next few performance entries"
        } else if entries <= 30 {
            "Upgrade possible within moderate time horizon"
        } else {
            "Upgrade distant — sustained improvement needed"
        }
    } else if let Some(entries) = entries_to_downgrade {
        if entries <= 3 {
            "Downgrade imminent — performance declining rapidly"
        } else if entries <= 10 {
            "Downgrade risk — performance trend is negative"
        } else {
            "Downgrade possible if decline continues"
        }
    } else if current_ordinal == 3 {
        "At maximum governance level — no further upgrade possible"
    } else {
        "Stable — no transition expected with current trend"
    };

    Ok(json!({
        "contract_id": contract_id.to_string(),
        "current_level": contract.current_level,
        "performance_trend_slope": (slope * 10000.0).round() / 10000.0,
        "current_projected_performance": (current_projected * 1000.0).round() / 1000.0,
        "prediction": prediction,
        "entries_to_upgrade": entries_to_upgrade,
        "entries_to_downgrade": entries_to_downgrade,
        "next_upgrade_level": if current_ordinal < 3 {
            Some(governance_label(&governance_from_ordinal(current_ordinal as i32 + 1)))
        } else { None },
        "next_downgrade_level": if current_ordinal > 0 {
            Some(governance_label(&governance_from_ordinal(current_ordinal as i32 - 1)))
        } else { None },
        "data_points_analyzed": recent_count,
        "predicted_at": now.to_rfc3339()
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 11: Contract Inheritance
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_contract_inheritance_create(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let parent_id = require_id(&args, "parent_id")?;
    let child_id = require_id(&args, "child_id")?;
    let propagate = args
        .get("propagate")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    // Verify both policies exist
    let parent = engine
        .file
        .policies
        .iter()
        .find(|p| p.id == parent_id)
        .ok_or_else(|| format!("Parent policy not found: {}", parent_id))?;
    let child = engine
        .file
        .policies
        .iter()
        .find(|p| p.id == child_id)
        .ok_or_else(|| format!("Child policy not found: {}", child_id))?;

    // Check for circular inheritance
    {
        let store = INHERITANCE_RECORDS.lock().unwrap();
        if has_circular_inheritance(&store, child_id, parent_id) {
            return Err(format!(
                "Circular inheritance detected: {} -> {} would create a cycle",
                parent_id, child_id
            ));
        }
    }

    // Determine inherited properties
    let inherited_policy_ids = vec![parent_id];

    let id = ContractId::new();
    let now = Utc::now();

    let record = InheritanceRecord {
        id,
        parent_id,
        child_id,
        inherited_policy_ids: inherited_policy_ids.clone(),
        overrides: Vec::new(),
        propagate_changes: propagate,
        created_at: now,
    };

    let parent_label = parent.label.clone();
    let child_label = child.label.clone();
    let parent_scope = format!("{:?}", parent.scope);
    let child_scope = format!("{:?}", child.scope);

    let mut store = INHERITANCE_RECORDS.lock().unwrap();
    store.push(record);

    Ok(json!({
        "id": id.to_string(),
        "parent_id": parent_id.to_string(),
        "parent_label": parent_label,
        "parent_scope": parent_scope,
        "child_id": child_id.to_string(),
        "child_label": child_label,
        "child_scope": child_scope,
        "inherited_policies": inherited_policy_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
        "propagate_changes": propagate,
        "overrides": [],
        "created_at": now.to_rfc3339()
    }))
}

fn handle_contract_inheritance_tree(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let root_id = require_id(&args, "root_id")?;

    let store = INHERITANCE_RECORDS.lock().unwrap();

    // Build tree recursively
    fn build_tree_node(
        policy_id: ContractId,
        store: &[InheritanceRecord],
        engine: &ContractEngine,
        depth: usize,
        max_depth: usize,
    ) -> Value {
        let policy_info = engine
            .file
            .policies
            .iter()
            .find(|p| p.id == policy_id)
            .map(|p| {
                json!({
                    "id": p.id.to_string(),
                    "label": p.label,
                    "scope": format!("{:?}", p.scope),
                    "action": format!("{:?}", p.action),
                    "active": p.is_active()
                })
            })
            .unwrap_or_else(|| {
                json!({
                    "id": policy_id.to_string(),
                    "label": "unknown",
                    "error": "Policy not found"
                })
            });

        let children_records: Vec<_> = store.iter().filter(|r| r.parent_id == policy_id).collect();

        let children: Vec<Value> = if depth < max_depth {
            children_records
                .iter()
                .map(|r| {
                    let mut child_node =
                        build_tree_node(r.child_id, store, engine, depth + 1, max_depth);
                    if let Some(obj) = child_node.as_object_mut() {
                        obj.insert("inheritance_id".to_string(), json!(r.id.to_string()));
                        obj.insert("propagate_changes".to_string(), json!(r.propagate_changes));
                        obj.insert("override_count".to_string(), json!(r.overrides.len()));
                    }
                    child_node
                })
                .collect()
        } else {
            vec![]
        };

        json!({
            "policy": policy_info,
            "depth": depth,
            "children": children,
            "child_count": children.len()
        })
    }

    let tree = build_tree_node(root_id, &store, engine, 0, 10);

    // Count total nodes in tree
    fn count_nodes(node: &Value) -> usize {
        let children_count: usize = node["children"]
            .as_array()
            .map(|arr| arr.iter().map(count_nodes).sum())
            .unwrap_or(0);
        1 + children_count
    }

    let total_nodes = count_nodes(&tree);

    // Compute max depth
    fn max_depth_of(node: &Value) -> usize {
        let d = node["depth"].as_u64().unwrap_or(0) as usize;
        let children_max = node["children"]
            .as_array()
            .map(|arr| arr.iter().map(max_depth_of).max().unwrap_or(d))
            .unwrap_or(d);
        children_max.max(d)
    }

    let tree_depth = max_depth_of(&tree);

    Ok(json!({
        "root_id": root_id.to_string(),
        "tree": tree,
        "total_nodes": total_nodes,
        "max_depth": tree_depth,
        "total_inheritance_records": store.len()
    }))
}

fn handle_contract_inheritance_resolve(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let policy_id = require_id(&args, "policy_id")?;
    let scope = args
        .get("scope")
        .and_then(|v| v.as_str())
        .unwrap_or("global");

    let store = INHERITANCE_RECORDS.lock().unwrap();

    // Walk up the inheritance chain
    let mut chain: Vec<Value> = Vec::new();
    let mut current_id = policy_id;
    let mut visited: HashSet<ContractId> = HashSet::new();
    let mut effective_action = None;
    let mut effective_scope = None;
    let mut applied_overrides: Vec<Value> = Vec::new();

    loop {
        if visited.contains(&current_id) {
            break; // prevent infinite loops
        }
        visited.insert(current_id);

        let policy = engine.file.policies.iter().find(|p| p.id == current_id);

        if let Some(p) = policy {
            chain.push(json!({
                "policy_id": p.id.to_string(),
                "label": p.label,
                "scope": format!("{:?}", p.scope),
                "action": format!("{:?}", p.action),
                "active": p.is_active()
            }));

            if effective_action.is_none() {
                effective_action = Some(format!("{:?}", p.action));
            }
            if effective_scope.is_none() {
                effective_scope = Some(format!("{:?}", p.scope));
            }
        }

        // Find inheritance record where current_id is the child
        let parent_record = store.iter().find(|r| r.child_id == current_id);

        match parent_record {
            Some(record) => {
                // Collect overrides at this level
                for o in &record.overrides {
                    applied_overrides.push(json!({
                        "override_id": o.id.to_string(),
                        "policy_id": o.policy_id.to_string(),
                        "override_type": o.override_type,
                        "description": o.description,
                        "applied_at_level": chain.len() - 1
                    }));

                    // Apply override to effective policy
                    match o.override_type.as_str() {
                        "restrict_further" => {
                            effective_action = Some("Deny".to_string());
                        }
                        "allow_additional" => {
                            effective_action = Some("Allow".to_string());
                        }
                        "modify_parameters" => {
                            // Parameters modified, keep current action
                        }
                        _ => {}
                    }
                }

                current_id = record.parent_id;
            }
            None => break, // no parent, reached root
        }
    }

    // Reverse chain to show root first
    chain.reverse();

    Ok(json!({
        "policy_id": policy_id.to_string(),
        "scope": scope,
        "inheritance_chain": chain,
        "chain_depth": chain.len(),
        "effective_action": effective_action.unwrap_or_else(|| "unknown".to_string()),
        "effective_scope": effective_scope.unwrap_or_else(|| scope.to_string()),
        "applied_overrides": applied_overrides,
        "override_count": applied_overrides.len(),
        "resolved_at": Utc::now().to_rfc3339()
    }))
}

fn handle_contract_inheritance_override(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let inheritance_id_str = require_str(&args, "inheritance_id")?;
    let policy_id = require_id(&args, "policy_id")?;
    let override_type_str = args
        .get("override_type")
        .and_then(|v| v.as_str())
        .unwrap_or("modify_parameters");
    let description = require_str(&args, "description")?;

    let inheritance_id: ContractId = inheritance_id_str
        .parse()
        .map_err(|e| format!("Invalid inheritance_id: {}", e))?;

    // Validate override type
    let override_type = match override_type_str {
        "allow_additional" | "restrict_further" | "modify_parameters" => {
            override_type_str.to_string()
        }
        other => {
            return Err(format!(
            "Unknown override_type: {}. Use: allow_additional, restrict_further, modify_parameters",
            other
        ))
        }
    };

    let now = Utc::now();
    let override_id = ContractId::new();

    let mut store = INHERITANCE_RECORDS.lock().unwrap();
    let record = store
        .iter_mut()
        .find(|r| r.id == inheritance_id)
        .ok_or_else(|| format!("Inheritance record not found: {}", inheritance_id))?;

    // Verify the policy is in the inheritance chain
    let policy_in_chain =
        record.parent_id == policy_id || record.inherited_policy_ids.contains(&policy_id);

    if !policy_in_chain {
        return Err(format!(
            "Policy {} is not part of inheritance chain {}",
            policy_id, inheritance_id
        ));
    }

    record.overrides.push(OverrideRecord {
        id: override_id,
        policy_id,
        override_type: override_type.clone(),
        description: description.to_string(),
        created_at: now,
    });

    Ok(json!({
        "override_id": override_id.to_string(),
        "inheritance_id": inheritance_id.to_string(),
        "policy_id": policy_id.to_string(),
        "override_type": override_type,
        "description": description,
        "total_overrides": record.overrides.len(),
        "parent_id": record.parent_id.to_string(),
        "child_id": record.child_id.to_string(),
        "created_at": now.to_rfc3339()
    }))
}

/// Check if adding parent_id as a parent of child_id would create a cycle.
fn has_circular_inheritance(
    store: &[InheritanceRecord],
    child_id: ContractId,
    proposed_parent_id: ContractId,
) -> bool {
    // Walk upward from proposed_parent_id to see if we reach child_id
    let mut current = proposed_parent_id;
    let mut visited: HashSet<ContractId> = HashSet::new();

    loop {
        if current == child_id {
            return true;
        }
        if visited.contains(&current) {
            return false;
        }
        visited.insert(current);

        match store.iter().find(|r| r.child_id == current) {
            Some(record) => current = record.parent_id,
            None => return false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 12: Smart Escalation
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_smart_escalation_route(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let description = require_str(&args, "description")?;
    let urgency = args
        .get("urgency")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5)
        .clamp(0.0, 1.0);
    let now = Utc::now();

    let config = ESCALATION_CONFIG.lock().unwrap();

    if config.approvers.is_empty() {
        return Err("No approvers configured. Use smart_escalation_configure first.".to_string());
    }

    // Compute domain relevance from description
    let description_lower = description.to_lowercase();

    // Score each approver
    let mut scored_approvers: Vec<(usize, f64, String)> = config
        .approvers
        .iter()
        .enumerate()
        .map(|(idx, approver)| {
            // Domain expertise score
            let domain_score: f64 = approver
                .domains
                .iter()
                .map(|d| {
                    if description_lower.contains(&d.to_lowercase()) {
                        1.0
                    } else {
                        word_overlap(d, description) * 0.5
                    }
                })
                .fold(0.0_f64, f64::max);

            // Response time factor (lower is better)
            let response_factor = if approver.avg_response_secs > 0 {
                1.0 / (approver.avg_response_secs as f64 / 300.0).max(0.1)
            } else {
                1.0
            };

            // Composite score
            let score = urgency
                * approver.availability
                * response_factor
                * approver.approval_rate
                * (0.5 + domain_score * 0.5);

            let reason = format!(
                "availability={:.2}, response_time={}s, approval_rate={:.2}, domain_match={:.2}",
                approver.availability,
                approver.avg_response_secs,
                approver.approval_rate,
                domain_score
            );

            (idx, score, reason)
        })
        .collect();

    // Sort by score descending
    scored_approvers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let best_idx = scored_approvers[0].0;
    let best = &config.approvers[best_idx];
    let routing_reason = format!(
        "Highest composite score ({:.3}): {}",
        scored_approvers[0].1, scored_approvers[0].2
    );

    // Build fallback chain
    let fallback_chain: Vec<Value> = scored_approvers
        .iter()
        .skip(1)
        .map(|(idx, score, _)| {
            let approver = &config.approvers[*idx];
            json!({
                "approver_id": approver.id,
                "name": approver.name,
                "availability": approver.availability,
                "avg_response_secs": approver.avg_response_secs,
                "approval_rate": approver.approval_rate,
                "score": (score * 1000.0).round() / 1000.0
            })
        })
        .collect();

    // Also compute from actual engine approval data
    let actual_response_times = compute_actual_response_times(engine, &best.id);
    let estimated_response_secs = actual_response_times.unwrap_or(best.avg_response_secs);

    // Urgency bucket
    let urgency_bucket = if urgency >= config.urgency_thresholds.critical {
        "critical"
    } else if urgency >= config.urgency_thresholds.high {
        "high"
    } else if urgency >= config.urgency_thresholds.medium {
        "medium"
    } else {
        "low"
    };

    Ok(json!({
        "id": ContractId::new().to_string(),
        "request_description": description,
        "urgency": urgency,
        "urgency_bucket": urgency_bucket,
        "recommended_approver": best.id,
        "recommended_approver_name": best.name,
        "routing_reason": routing_reason,
        "routing_score": (scored_approvers[0].1 * 1000.0).round() / 1000.0,
        "fallback_chain": fallback_chain,
        "estimated_response_secs": estimated_response_secs,
        "timeout_secs": config.default_timeout_secs,
        "confidence": ((scored_approvers[0].1 / (scored_approvers[0].1 + 0.5)) * 1000.0).round() / 1000.0,
        "routed_at": now.to_rfc3339()
    }))
}

fn handle_smart_escalation_history(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let window_days = args
        .get("window_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(30);
    let now = Utc::now();
    let window_start = now - Duration::days(window_days);

    // Analyze approval requests and decisions within window
    let requests_in_window: Vec<_> = engine
        .file
        .approval_requests
        .iter()
        .filter(|r| r.created_at >= window_start)
        .collect();

    let decisions_in_window: Vec<_> = engine
        .file
        .approval_decisions
        .iter()
        .filter(|d| d.decided_at >= window_start)
        .collect();

    // Group by decider to find bottlenecks
    let mut decider_stats: HashMap<String, DeciderStats> = HashMap::new();

    for decision in &decisions_in_window {
        let stats = decider_stats
            .entry(decision.decider.clone())
            .or_insert_with(|| DeciderStats {
                total_decisions: 0,
                approvals: 0,
                denials: 0,
                total_response_secs: 0,
                response_count: 0,
            });

        stats.total_decisions += 1;
        match decision.decision {
            DecisionType::Approve => stats.approvals += 1,
            DecisionType::Deny => stats.denials += 1,
        }

        // Compute response time
        if let Some(request) = engine
            .file
            .approval_requests
            .iter()
            .find(|r| r.id == decision.request_id)
        {
            let response_secs = decision
                .decided_at
                .signed_duration_since(request.created_at)
                .num_seconds();
            if response_secs >= 0 {
                stats.total_response_secs += response_secs;
                stats.response_count += 1;
            }
        }
    }

    // Build decider summary
    let mut decider_summaries: Vec<Value> = decider_stats
        .iter()
        .map(|(decider, stats)| {
            let avg_response = if stats.response_count > 0 {
                stats.total_response_secs / stats.response_count
            } else {
                0
            };
            let approval_rate = if stats.total_decisions > 0 {
                stats.approvals as f64 / stats.total_decisions as f64
            } else {
                0.0
            };

            json!({
                "decider": decider,
                "total_decisions": stats.total_decisions,
                "approvals": stats.approvals,
                "denials": stats.denials,
                "approval_rate": (approval_rate * 1000.0).round() / 1000.0,
                "avg_response_secs": avg_response,
                "avg_response_human": format_duration(avg_response),
                "is_bottleneck": avg_response > 1800 || stats.total_decisions > 10
            })
        })
        .collect();

    decider_summaries.sort_by(|a, b| {
        let resp_a = a["avg_response_secs"].as_i64().unwrap_or(0);
        let resp_b = b["avg_response_secs"].as_i64().unwrap_or(0);
        resp_b.cmp(&resp_a)
    });

    // Overall stats
    let total_requests = requests_in_window.len();
    let total_decisions = decisions_in_window.len();
    let pending = requests_in_window
        .iter()
        .filter(|r| r.status == ApprovalStatus::Pending)
        .count();

    // Bottleneck identification
    let bottlenecks: Vec<Value> = decider_summaries
        .iter()
        .filter(|d| d["is_bottleneck"].as_bool().unwrap_or(false))
        .cloned()
        .collect();

    Ok(json!({
        "window_days": window_days,
        "total_requests": total_requests,
        "total_decisions": total_decisions,
        "pending_requests": pending,
        "decider_analysis": decider_summaries,
        "bottlenecks": bottlenecks,
        "bottleneck_count": bottlenecks.len(),
        "analyzed_at": now.to_rfc3339()
    }))
}

fn handle_smart_escalation_predict(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let urgency = args
        .get("urgency")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5)
        .clamp(0.0, 1.0);
    let specific_approver = args.get("approver_id").and_then(|v| v.as_str());
    let now = Utc::now();

    let config = ESCALATION_CONFIG.lock().unwrap();

    // Compute historical response times by urgency bucket
    let urgency_bucket = if urgency >= config.urgency_thresholds.critical {
        "critical"
    } else if urgency >= config.urgency_thresholds.high {
        "high"
    } else if urgency >= config.urgency_thresholds.medium {
        "medium"
    } else {
        "low"
    };

    // Get actual response time data from engine
    let mut response_times: Vec<i64> = Vec::new();

    for decision in &engine.file.approval_decisions {
        if let Some(request) = engine
            .file
            .approval_requests
            .iter()
            .find(|r| r.id == decision.request_id)
        {
            if let Some(approver_id) = specific_approver {
                if decision.decider != approver_id {
                    continue;
                }
            }

            let response_secs = decision
                .decided_at
                .signed_duration_since(request.created_at)
                .num_seconds();
            if response_secs >= 0 {
                response_times.push(response_secs);
            }
        }
    }

    // If we have data, compute statistics
    let (mean_response, median_response, p95_response, confidence) = if !response_times.is_empty() {
        response_times.sort();
        let n = response_times.len();
        let mean = response_times.iter().sum::<i64>() / n as i64;
        let median = response_times[n / 2];
        let p95_idx = (n as f64 * 0.95) as usize;
        let p95 = response_times[p95_idx.min(n - 1)];
        let confidence = (n as f64 / (n as f64 + 5.0)).min(0.95);
        (mean, median, p95, confidence)
    } else {
        // Fall back to configured values
        let approver = specific_approver
            .and_then(|id| config.approvers.iter().find(|a| a.id == id))
            .or_else(|| config.approvers.first());

        let base_response = approver.map(|a| a.avg_response_secs).unwrap_or(600);

        // Adjust for urgency
        let urgency_factor = 1.0 + (1.0 - urgency) * 2.0; // higher urgency = faster
        let adjusted = (base_response as f64 * urgency_factor) as i64;

        (adjusted, adjusted, (adjusted as f64 * 1.5) as i64, 0.3)
    };

    // Compute confidence interval
    let ci_lower = (mean_response as f64 * 0.6) as i64;
    let ci_upper = (mean_response as f64 * 1.8) as i64;

    Ok(json!({
        "urgency": urgency,
        "urgency_bucket": urgency_bucket,
        "approver_id": specific_approver,
        "estimated_response_secs": mean_response,
        "estimated_response_human": format_duration(mean_response),
        "median_response_secs": median_response,
        "p95_response_secs": p95_response,
        "confidence_interval": {
            "lower_secs": ci_lower,
            "upper_secs": ci_upper,
            "lower_human": format_duration(ci_lower),
            "upper_human": format_duration(ci_upper)
        },
        "confidence": (confidence * 1000.0).round() / 1000.0,
        "data_points": response_times.len(),
        "predicted_at": now.to_rfc3339()
    }))
}

fn handle_smart_escalation_configure(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let now = Utc::now();
    let mut config = ESCALATION_CONFIG.lock().unwrap();
    let mut changes: Vec<String> = Vec::new();

    // Add approvers
    if let Some(add_arr) = args.get("add_approvers").and_then(|v| v.as_array()) {
        for item in add_arr {
            if let (Some(id), Some(name)) = (
                item.get("id").and_then(|v| v.as_str()),
                item.get("name").and_then(|v| v.as_str()),
            ) {
                let domains: Vec<String> = item
                    .get("domains")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                // Check for duplicate
                if config.approvers.iter().any(|a| a.id == id) {
                    changes.push(format!("Skipped duplicate approver: {}", id));
                    continue;
                }

                config.approvers.push(ApproverRecord {
                    id: id.to_string(),
                    name: name.to_string(),
                    domains: domains.clone(),
                    availability: 0.8,
                    avg_response_secs: 600,
                    approval_rate: 0.75,
                });
                changes.push(format!(
                    "Added approver: {} ({}) with domains {:?}",
                    name, id, domains
                ));
            }
        }
    }

    // Update urgency thresholds
    if let Some(thresholds) = args.get("urgency_thresholds") {
        if let Some(low) = thresholds.get("low").and_then(|v| v.as_f64()) {
            config.urgency_thresholds.low = low.clamp(0.0, 1.0);
            changes.push(format!("Updated low threshold: {:.2}", low));
        }
        if let Some(medium) = thresholds.get("medium").and_then(|v| v.as_f64()) {
            config.urgency_thresholds.medium = medium.clamp(0.0, 1.0);
            changes.push(format!("Updated medium threshold: {:.2}", medium));
        }
        if let Some(high) = thresholds.get("high").and_then(|v| v.as_f64()) {
            config.urgency_thresholds.high = high.clamp(0.0, 1.0);
            changes.push(format!("Updated high threshold: {:.2}", high));
        }
        if let Some(critical) = thresholds.get("critical").and_then(|v| v.as_f64()) {
            config.urgency_thresholds.critical = critical.clamp(0.0, 1.0);
            changes.push(format!("Updated critical threshold: {:.2}", critical));
        }
    }

    // Update timeout
    if let Some(timeout) = args.get("timeout_secs").and_then(|v| v.as_u64()) {
        config.default_timeout_secs = timeout;
        changes.push(format!("Updated default timeout: {}s", timeout));
    }

    if changes.is_empty() {
        changes.push(
            "No changes applied. Provide add_approvers, urgency_thresholds, or timeout_secs."
                .to_string(),
        );
    }

    Ok(json!({
        "changes": changes,
        "current_config": {
            "approvers": config.approvers.iter().map(|a| json!({
                "id": a.id,
                "name": a.name,
                "domains": a.domains,
                "availability": a.availability,
                "avg_response_secs": a.avg_response_secs,
                "approval_rate": a.approval_rate
            })).collect::<Vec<_>>(),
            "urgency_thresholds": {
                "low": config.urgency_thresholds.low,
                "medium": config.urgency_thresholds.medium,
                "high": config.urgency_thresholds.high,
                "critical": config.urgency_thresholds.critical
            },
            "default_timeout_secs": config.default_timeout_secs,
            "total_approvers": config.approvers.len()
        },
        "configured_at": now.to_rfc3339()
    }))
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility helpers
// ─────────────────────────────────────────────────────────────────────────────

struct DeciderStats {
    total_decisions: usize,
    approvals: usize,
    denials: usize,
    total_response_secs: i64,
    response_count: i64,
}

/// Compute actual average response time for an approver from engine data.
fn compute_actual_response_times(engine: &ContractEngine, approver_id: &str) -> Option<i64> {
    let mut total_secs: i64 = 0;
    let mut count: i64 = 0;

    for decision in &engine.file.approval_decisions {
        if decision.decider != approver_id {
            continue;
        }

        if let Some(request) = engine
            .file
            .approval_requests
            .iter()
            .find(|r| r.id == decision.request_id)
        {
            let response_secs = decision
                .decided_at
                .signed_duration_since(request.created_at)
                .num_seconds();
            if response_secs >= 0 {
                total_secs += response_secs;
                count += 1;
            }
        }
    }

    if count > 0 {
        Some(total_secs / count)
    } else {
        None
    }
}

/// Format a duration in seconds to a human-readable string.
fn format_duration(secs: i64) -> String {
    if secs < 0 {
        return "negative".to_string();
    }
    if secs < 60 {
        return format!("{}s", secs);
    }
    if secs < 3600 {
        let mins = secs / 60;
        let remaining = secs % 60;
        if remaining > 0 {
            format!("{}m {}s", mins, remaining)
        } else {
            format!("{}m", mins)
        }
    } else if secs < 86400 {
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        if mins > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}h", hours)
        }
    } else {
        let days = secs / 86400;
        let hours = (secs % 86400) / 3600;
        if hours > 0 {
            format!("{}d {}h", days, hours)
        } else {
            format!("{}d", days)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_overlap_identical() {
        let score = word_overlap("deploy production server", "deploy production server");
        assert!((score - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_word_overlap_partial() {
        let score = word_overlap("deploy production server", "deploy staging server");
        assert!(score > 0.3);
        assert!(score < 1.0);
    }

    #[test]
    fn test_word_overlap_empty() {
        assert_eq!(word_overlap("", "test"), 0.0);
        assert_eq!(word_overlap("test", ""), 0.0);
    }

    #[test]
    fn test_word_overlap_short_words_filtered() {
        // Words <= 2 chars are filtered
        let score = word_overlap("a b c", "a b c");
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_severity_weight_ordering() {
        use agentic_contract::ViolationSeverity;
        assert!(
            severity_weight(&ViolationSeverity::Fatal)
                > severity_weight(&ViolationSeverity::Critical)
        );
        assert!(
            severity_weight(&ViolationSeverity::Critical)
                > severity_weight(&ViolationSeverity::Warning)
        );
        assert!(
            severity_weight(&ViolationSeverity::Warning)
                > severity_weight(&ViolationSeverity::Info)
        );
    }

    #[test]
    fn test_monitoring_from_trust_levels() {
        assert_eq!(monitoring_from_trust(0.9), MonitoringLevel::Minimal);
        assert_eq!(monitoring_from_trust(0.7), MonitoringLevel::Standard);
        assert_eq!(monitoring_from_trust(0.5), MonitoringLevel::Enhanced);
        assert_eq!(monitoring_from_trust(0.2), MonitoringLevel::FullAudit);
    }

    #[test]
    fn test_governance_ordinal_roundtrip() {
        for level in &[
            GovernanceLevel::Conservative,
            GovernanceLevel::Moderate,
            GovernanceLevel::Permissive,
            GovernanceLevel::Autonomous,
        ] {
            let ordinal = governance_ordinal(level);
            let recovered = governance_from_ordinal(ordinal as i32);
            assert_eq!(*level, recovered);
        }
    }

    #[test]
    fn test_governance_from_ordinal_clamped() {
        assert_eq!(governance_from_ordinal(-5), GovernanceLevel::Conservative);
        assert_eq!(governance_from_ordinal(100), GovernanceLevel::Autonomous);
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(45), "45s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(150), "2m 30s");
        assert_eq!(format_duration(120), "2m");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(3700), "1h 1m");
        assert_eq!(format_duration(3600), "1h");
    }

    #[test]
    fn test_format_duration_days() {
        assert_eq!(format_duration(90000), "1d 1h");
        assert_eq!(format_duration(86400), "1d");
    }

    #[test]
    fn test_trust_from_violations_no_violations() {
        let violations: Vec<agentic_contract::Violation> = vec![];
        let (trust, sum, count) = compute_trust_from_violations(&violations, "agent_1", Utc::now());
        assert_eq!(trust, 1.0);
        assert_eq!(sum, 0.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_trust_from_violations_with_violations() {
        let mut violations = vec![agentic_contract::Violation::new(
            "Test violation",
            agentic_contract::ViolationSeverity::Warning,
            "agent_1",
        )];
        // Set detected_at to now for predictable test
        violations[0].detected_at = Utc::now();

        let (trust, sum, count) = compute_trust_from_violations(&violations, "agent_1", Utc::now());
        assert!(trust < 1.0);
        assert!(trust > 0.0);
        assert!(sum > 0.0);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_trust_from_violations_different_agent() {
        let violations = vec![agentic_contract::Violation::new(
            "Test violation",
            agentic_contract::ViolationSeverity::Critical,
            "agent_other",
        )];

        let (trust, _, count) = compute_trust_from_violations(&violations, "agent_1", Utc::now());
        assert_eq!(trust, 1.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_has_circular_inheritance_no_cycle() {
        let id_a = ContractId::new();
        let id_b = ContractId::new();
        let id_c = ContractId::new();

        let records = vec![InheritanceRecord {
            id: ContractId::new(),
            parent_id: id_a,
            child_id: id_b,
            inherited_policy_ids: vec![id_a],
            overrides: vec![],
            propagate_changes: true,
            created_at: Utc::now(),
        }];

        assert!(!has_circular_inheritance(&records, id_c, id_b));
    }

    #[test]
    fn test_has_circular_inheritance_with_cycle() {
        let id_a = ContractId::new();
        let id_b = ContractId::new();

        let records = vec![InheritanceRecord {
            id: ContractId::new(),
            parent_id: id_a,
            child_id: id_b,
            inherited_policy_ids: vec![id_a],
            overrides: vec![],
            propagate_changes: true,
            created_at: Utc::now(),
        }];

        // Adding id_b as parent of id_a would create A->B->A cycle
        assert!(has_circular_inheritance(&records, id_a, id_b));
    }

    #[test]
    fn test_try_handle_unknown_tool() {
        let mut engine = ContractEngine::new();
        let result = try_handle("nonexistent_tool", json!({}), &mut engine);
        assert!(result.is_none());
    }

    #[test]
    fn test_try_handle_known_tool() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "trust_gradient_evaluate",
            json!({"agent_id": "test_agent", "action": "deploy"}),
            &mut engine,
        );
        assert!(result.is_some());
        let val = result.unwrap().unwrap();
        assert_eq!(val["agent_id"], "test_agent");
        assert!(val["trust_factor"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn test_tool_defs_count() {
        assert_eq!(TOOL_DEFS.len(), 20);
    }

    #[test]
    fn test_tool_defs_names_unique() {
        let names: HashSet<&str> = TOOL_DEFS.iter().map(|t| t.name).collect();
        assert_eq!(names.len(), TOOL_DEFS.len());
    }

    #[test]
    fn test_parse_governance_level() {
        assert_eq!(
            parse_governance_level("conservative").unwrap(),
            GovernanceLevel::Conservative
        );
        assert_eq!(
            parse_governance_level("autonomous").unwrap(),
            GovernanceLevel::Autonomous
        );
        assert!(parse_governance_level("invalid").is_err());
    }

    #[test]
    fn test_approval_track_record_no_data() {
        let requests: Vec<agentic_contract::ApprovalRequest> = vec![];
        let decisions: Vec<agentic_contract::ApprovalDecision> = vec![];
        let rate = compute_approval_track_record(&requests, &decisions, "agent_1");
        assert_eq!(rate, 0.5); // neutral
    }
}

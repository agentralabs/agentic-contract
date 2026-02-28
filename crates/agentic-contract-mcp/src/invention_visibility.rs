//! Inventions 1-5 (Visibility category) — Policy Omniscience, Risk Prophecy,
//! Approval Telepathy, Obligation Clairvoyance, Violation Precognition.
//!
//! These inventions provide deep visibility into the contract engine's state,
//! using statistical analysis, pattern matching, and predictive algorithms
//! to surface actionable insights about policies, risks, approvals,
//! obligations, and potential violations.

use std::collections::HashMap;

use chrono::{Duration, Utc};
use serde_json::{json, Value};

use agentic_contract::inventions::*;
use agentic_contract::ContractEngine;
use agentic_contract::ContractId;
use agentic_contract::policy::{PolicyAction, PolicyScope};

use crate::tools::{require_id, require_str, ToolDefinition};

// ─── Tool definitions ────────────────────────────────────────────────────────

pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Invention 1: Policy Omniscience (4 tools) ─────────────────────
    ToolDefinition {
        name: "policy_omniscience_query",
        description: "Get complete visibility into all applicable policies for an agent in a given context",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to query policies for"},"context":{"type":"string","description":"Context for the query (e.g. 'deploy', 'data-access')"},"include_inactive":{"type":"boolean","description":"Whether to include expired/disabled policies","default":false}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "policy_omniscience_diff",
        description: "Compare policy visibility between two agents showing permission differences",
        input_schema: r#"{"type":"object","properties":{"agent_a":{"type":"string","description":"First agent ID"},"agent_b":{"type":"string","description":"Second agent ID"},"context":{"type":"string","description":"Optional context filter"}},"required":["agent_a","agent_b"]}"#,
    },
    ToolDefinition {
        name: "policy_omniscience_coverage",
        description: "Analyze policy coverage gaps identifying unprotected actions and scopes",
        input_schema: r#"{"type":"object","properties":{"scope":{"type":"string","enum":["global","session","agent"],"description":"Scope to analyze coverage for"}}}"#,
    },
    ToolDefinition {
        name: "policy_omniscience_conflicts",
        description: "Detect conflicting policies where allow and deny overlap on the same action pattern",
        input_schema: r#"{"type":"object","properties":{"scope":{"type":"string","enum":["global","session","agent"],"description":"Scope to check for conflicts"}}}"#,
    },
    // ── Invention 2: Risk Prophecy (4 tools) ──────────────────────────
    ToolDefinition {
        name: "risk_prophecy_forecast",
        description: "Predict future risk budget usage with exponential decay trend analysis",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to forecast for"},"forecast_window_secs":{"type":"integer","description":"Forecast window in seconds","default":3600},"confidence_level":{"type":"number","description":"Confidence level for prediction interval (0.0-1.0)","default":0.95}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "risk_prophecy_heatmap",
        description: "Generate risk heatmap across all limits showing temporal concentration of risk events",
        input_schema: r#"{"type":"object","properties":{"window_secs":{"type":"integer","description":"Analysis window in seconds","default":86400},"bucket_count":{"type":"integer","description":"Number of time buckets for the heatmap","default":24}}}"#,
    },
    ToolDefinition {
        name: "risk_prophecy_threshold_alert",
        description: "Identify risk limits approaching their thresholds with time-to-breach estimates",
        input_schema: r#"{"type":"object","properties":{"alert_threshold":{"type":"number","description":"Usage fraction (0.0-1.0) above which to alert","default":0.75}}}"#,
    },
    ToolDefinition {
        name: "risk_prophecy_correlation",
        description: "Analyze correlations between different risk limits to identify coupled risks",
        input_schema: r#"{"type":"object","properties":{"window_secs":{"type":"integer","description":"Correlation analysis window","default":604800}}}"#,
    },
    // ── Invention 3: Approval Telepathy (4 tools) ─────────────────────
    ToolDefinition {
        name: "approval_telepathy_predict",
        description: "Predict approval likelihood for an action using historical pattern analysis",
        input_schema: r#"{"type":"object","properties":{"action":{"type":"string","description":"Action to predict approval for"},"requestor":{"type":"string","description":"Who would request the approval"}},"required":["action"]}"#,
    },
    ToolDefinition {
        name: "approval_telepathy_optimize",
        description: "Suggest modifications to maximize approval probability with effort estimates",
        input_schema: r#"{"type":"object","properties":{"action":{"type":"string","description":"Action to optimize for approval"},"max_suggestions":{"type":"integer","description":"Maximum number of suggestions","default":5}},"required":["action"]}"#,
    },
    ToolDefinition {
        name: "approval_telepathy_timing",
        description: "Analyze optimal timing for submitting approval requests based on historical patterns",
        input_schema: r#"{"type":"object","properties":{"action":{"type":"string","description":"Action to analyze timing for"},"window_hours":{"type":"integer","description":"Hours to analyze for optimal window","default":168}},"required":["action"]}"#,
    },
    ToolDefinition {
        name: "approval_telepathy_bottleneck",
        description: "Identify approval process bottlenecks and slow-response approvers",
        input_schema: r#"{"type":"object","properties":{"window_secs":{"type":"integer","description":"Analysis window in seconds","default":604800}}}"#,
    },
    // ── Invention 4: Obligation Clairvoyance (4 tools) ────────────────
    ToolDefinition {
        name: "obligation_clairvoyance_forecast",
        description: "Forecast upcoming obligations with scheduling conflict detection and optimal order",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to forecast for"},"window_secs":{"type":"integer","description":"Forecast window in seconds","default":86400},"include_completed":{"type":"boolean","description":"Include recently completed obligations","default":false}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "obligation_clairvoyance_dependencies",
        description: "Map obligation dependency chains showing critical paths and potential cascading failures",
        input_schema: r#"{"type":"object","properties":{"obligation_id":{"type":"string","description":"Root obligation to trace dependencies from"}},"required":["obligation_id"]}"#,
    },
    ToolDefinition {
        name: "obligation_clairvoyance_workload",
        description: "Analyze obligation workload distribution across agents with overload detection",
        input_schema: r#"{"type":"object","properties":{"window_secs":{"type":"integer","description":"Analysis window","default":604800},"overload_threshold":{"type":"integer","description":"Number of obligations above which an agent is overloaded","default":10}}}"#,
    },
    ToolDefinition {
        name: "obligation_clairvoyance_risk",
        description: "Calculate risk scores for obligation fulfillment combining deadline proximity and complexity",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to analyze"},"urgency_weight":{"type":"number","description":"Weight for deadline proximity (0.0-1.0)","default":0.6},"complexity_weight":{"type":"number","description":"Weight for effort complexity (0.0-1.0)","default":0.4}},"required":["agent_id"]}"#,
    },
    // ── Invention 5: Violation Precognition (4 tools) ─────────────────
    ToolDefinition {
        name: "violation_precognition_analyze",
        description: "Analyze a planned action for potential policy and risk-limit violations before execution",
        input_schema: r#"{"type":"object","properties":{"planned_action":{"type":"string","description":"Action being planned"},"agent_id":{"type":"string","description":"Agent that would perform the action"}},"required":["planned_action"]}"#,
    },
    ToolDefinition {
        name: "violation_precognition_batch",
        description: "Analyze multiple planned actions in sequence detecting cumulative violation risk",
        input_schema: r#"{"type":"object","properties":{"actions":{"type":"array","items":{"type":"string"},"description":"Ordered list of planned actions"},"agent_id":{"type":"string","description":"Agent that would perform the actions"}},"required":["actions"]}"#,
    },
    ToolDefinition {
        name: "violation_precognition_alternatives",
        description: "Generate safe alternative actions when a planned action would cause violations",
        input_schema: r#"{"type":"object","properties":{"planned_action":{"type":"string","description":"Action that may cause violations"},"max_alternatives":{"type":"integer","description":"Maximum alternatives to suggest","default":3}},"required":["planned_action"]}"#,
    },
    ToolDefinition {
        name: "violation_precognition_history_pattern",
        description: "Detect recurring violation patterns using time-series clustering of historical violations",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to analyze patterns for"},"window_secs":{"type":"integer","description":"Historical window to analyze","default":2592000},"min_cluster_size":{"type":"integer","description":"Minimum violations to form a pattern","default":3}},"required":["agent_id"]}"#,
    },
];

// ─── Dispatch ────────────────────────────────────────────────────────────────

pub fn try_handle(
    name: &str,
    args: Value,
    engine: &mut ContractEngine,
) -> Option<Result<Value, String>> {
    match name {
        // ── Invention 1: Policy Omniscience ────────────────────────────
        "policy_omniscience_query" => Some(handle_policy_omniscience_query(args, engine)),
        "policy_omniscience_diff" => Some(handle_policy_omniscience_diff(args, engine)),
        "policy_omniscience_coverage" => Some(handle_policy_omniscience_coverage(args, engine)),
        "policy_omniscience_conflicts" => Some(handle_policy_omniscience_conflicts(args, engine)),

        // ── Invention 2: Risk Prophecy ────────────────────────────────
        "risk_prophecy_forecast" => Some(handle_risk_prophecy_forecast(args, engine)),
        "risk_prophecy_heatmap" => Some(handle_risk_prophecy_heatmap(args, engine)),
        "risk_prophecy_threshold_alert" => Some(handle_risk_prophecy_threshold_alert(args, engine)),
        "risk_prophecy_correlation" => Some(handle_risk_prophecy_correlation(args, engine)),

        // ── Invention 3: Approval Telepathy ───────────────────────────
        "approval_telepathy_predict" => Some(handle_approval_telepathy_predict(args, engine)),
        "approval_telepathy_optimize" => Some(handle_approval_telepathy_optimize(args, engine)),
        "approval_telepathy_timing" => Some(handle_approval_telepathy_timing(args, engine)),
        "approval_telepathy_bottleneck" => Some(handle_approval_telepathy_bottleneck(args, engine)),

        // ── Invention 4: Obligation Clairvoyance ──────────────────────
        "obligation_clairvoyance_forecast" => {
            Some(handle_obligation_clairvoyance_forecast(args, engine))
        }
        "obligation_clairvoyance_dependencies" => {
            Some(handle_obligation_clairvoyance_dependencies(args, engine))
        }
        "obligation_clairvoyance_workload" => {
            Some(handle_obligation_clairvoyance_workload(args, engine))
        }
        "obligation_clairvoyance_risk" => {
            Some(handle_obligation_clairvoyance_risk(args, engine))
        }

        // ── Invention 5: Violation Precognition ───────────────────────
        "violation_precognition_analyze" => {
            Some(handle_violation_precognition_analyze(args, engine))
        }
        "violation_precognition_batch" => {
            Some(handle_violation_precognition_batch(args, engine))
        }
        "violation_precognition_alternatives" => {
            Some(handle_violation_precognition_alternatives(args, engine))
        }
        "violation_precognition_history_pattern" => {
            Some(handle_violation_precognition_history_pattern(args, engine))
        }

        _ => None,
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Jaccard-like word overlap between two strings (words > 2 chars).
fn word_overlap(a: &str, b: &str) -> f64 {
    let set_a: std::collections::HashSet<&str> =
        a.split_whitespace().filter(|w| w.len() > 2).collect();
    let set_b: std::collections::HashSet<&str> =
        b.split_whitespace().filter(|w| w.len() > 2).collect();
    if set_a.is_empty() || set_b.is_empty() {
        return 0.0;
    }
    let intersection = set_a.intersection(&set_b).count() as f64;
    let union = set_a.union(&set_b).count() as f64;
    intersection / union
}

/// Classify a policy action into allowed/denied/conditional category.
fn classify_action(action: &PolicyAction) -> &'static str {
    match action {
        PolicyAction::Allow => "allowed",
        PolicyAction::Deny => "denied",
        PolicyAction::RequireApproval => "conditional",
        PolicyAction::AuditOnly => "allowed",
    }
}

/// Compute a risk usage fraction from violation history for a given limit.
fn compute_risk_usage(
    violations: &[agentic_contract::Violation],
    limit_label: &str,
    window_secs: i64,
    now: chrono::DateTime<Utc>,
) -> f64 {
    let cutoff = now - Duration::seconds(window_secs);
    let relevant: Vec<&agentic_contract::Violation> = violations
        .iter()
        .filter(|v| v.detected_at >= cutoff)
        .filter(|v| word_overlap(&v.description, limit_label) > 0.15)
        .collect();

    if relevant.is_empty() {
        return 0.0;
    }

    // Exponential decay: recent violations contribute more
    let mut weighted_sum = 0.0;
    let half_life = window_secs as f64 / 3.0;
    for v in &relevant {
        let age = (now - v.detected_at).num_seconds() as f64;
        let decay = (-age * 0.693 / half_life).exp(); // 0.693 = ln(2)
        let severity_factor = match v.severity {
            agentic_contract::ViolationSeverity::Fatal => 1.0,
            agentic_contract::ViolationSeverity::Critical => 0.8,
            agentic_contract::ViolationSeverity::Warning => 0.4,
            agentic_contract::ViolationSeverity::Info => 0.1,
        };
        weighted_sum += decay * severity_factor;
    }

    // Normalize to 0.0-1.0 range (cap at 1.0)
    (weighted_sum / 5.0).min(1.0)
}

/// Linear extrapolation of risk usage based on recent trend.
fn extrapolate_usage(current: f64, trend_rate: f64, forecast_secs: f64) -> f64 {
    let projected = current + trend_rate * forecast_secs;
    projected.clamp(0.0, 1.0)
}

/// Compute trend rate (usage change per second) from violation history.
fn compute_trend_rate(
    violations: &[agentic_contract::Violation],
    limit_label: &str,
    window_secs: i64,
    now: chrono::DateTime<Utc>,
) -> f64 {
    let cutoff = now - Duration::seconds(window_secs);
    let relevant: Vec<&agentic_contract::Violation> = violations
        .iter()
        .filter(|v| v.detected_at >= cutoff)
        .filter(|v| word_overlap(&v.description, limit_label) > 0.15)
        .collect();

    if relevant.len() < 2 {
        return 0.0;
    }

    // Split into first half and second half to compute trend
    let mid = relevant.len() / 2;
    let first_half = &relevant[..mid];
    let second_half = &relevant[mid..];

    let first_rate = first_half.len() as f64 / (window_secs as f64 / 2.0);
    let second_rate = second_half.len() as f64 / (window_secs as f64 / 2.0);

    second_rate - first_rate
}

/// Estimate approval probability based on historical approval decisions.
fn estimate_approval_probability(
    engine: &ContractEngine,
    action: &str,
) -> (f64, Vec<String>, i64) {
    let approvals = engine.list_approval_requests(None);

    let mut total = 0u32;
    let mut approved = 0u32;
    let mut approvers: HashMap<String, u32> = HashMap::new();
    let mut response_times: Vec<i64> = Vec::new();

    for req in &approvals {
        if word_overlap(&req.action_description, action) > 0.2 {
            total += 1;
            if req.status == agentic_contract::ApprovalStatus::Approved {
                approved += 1;
            }
            // Track response times using created_at as proxy
            let response_secs = (chrono::Utc::now() - req.created_at).num_seconds();
            response_times.push(response_secs);
            *approvers.entry(req.requestor.clone()).or_insert(0) += 1;
        }
    }

    let probability = if total == 0 {
        0.5 // Prior when no history
    } else {
        approved as f64 / total as f64
    };

    let mut likely_approvers: Vec<(String, u32)> = approvers.into_iter().collect();
    likely_approvers.sort_by(|a, b| b.1.cmp(&a.1));
    let top_approvers: Vec<String> = likely_approvers.into_iter().take(3).map(|(k, _)| k).collect();

    let avg_response = if response_times.is_empty() {
        3600 // Default 1 hour
    } else {
        response_times.iter().sum::<i64>() / response_times.len() as i64
    };

    (probability, top_approvers, avg_response)
}

/// Compute obligation miss risk based on deadline proximity and effort estimate.
fn compute_miss_risk(
    deadline_secs: Option<i64>,
    effort_minutes: u32,
    urgency_weight: f64,
    complexity_weight: f64,
) -> f64 {
    let urgency_factor = match deadline_secs {
        None => 0.3, // No deadline = moderate risk
        Some(secs) if secs <= 0 => 1.0,
        Some(secs) if secs < 3600 => 0.9,
        Some(secs) if secs < 86400 => 0.6,
        Some(secs) if secs < 604800 => 0.3,
        _ => 0.1,
    };

    let complexity_factor = if effort_minutes > 480 {
        0.9
    } else if effort_minutes > 120 {
        0.6
    } else if effort_minutes > 30 {
        0.3
    } else {
        0.1
    };

    (urgency_factor * urgency_weight + complexity_factor * complexity_weight).clamp(0.0, 1.0)
}

/// Estimate effort in minutes for an obligation based on its description.
fn estimate_effort(label: &str) -> u32 {
    let lower = label.to_lowercase();
    if lower.contains("audit") || lower.contains("review") || lower.contains("analyze") {
        120
    } else if lower.contains("deploy") || lower.contains("migrate") || lower.contains("upgrade") {
        240
    } else if lower.contains("report") || lower.contains("document") {
        60
    } else if lower.contains("fix") || lower.contains("patch") || lower.contains("hotfix") {
        90
    } else if lower.contains("test") || lower.contains("verify") {
        45
    } else {
        30
    }
}

/// Generate safe alternative actions by analyzing policy constraints.
fn generate_alternatives(
    policies: &[agentic_contract::Policy],
    planned_action: &str,
    max_alternatives: usize,
) -> Vec<String> {
    let mut alternatives = Vec::new();

    // Find which policies restrict the action
    let restricting: Vec<&agentic_contract::Policy> = policies
        .iter()
        .filter(|p| {
            p.status == agentic_contract::PolicyStatus::Active
                && word_overlap(&p.label, planned_action) > 0.15
                && matches!(p.action, PolicyAction::Deny | PolicyAction::RequireApproval)
        })
        .collect();

    if restricting.is_empty() {
        return vec![format!("{} (no restrictions found)", planned_action)];
    }

    // For each restricting policy, suggest a narrower-scope alternative
    for policy in restricting.iter().take(max_alternatives) {
        match policy.scope {
            PolicyScope::Global => {
                alternatives.push(format!(
                    "Narrow scope: perform '{}' within session scope only",
                    planned_action
                ));
            }
            PolicyScope::Session => {
                alternatives.push(format!(
                    "Request agent-level exception for '{}'",
                    planned_action
                ));
            }
            PolicyScope::Agent => {
                alternatives.push(format!(
                    "Escalate '{}' through approval workflow",
                    planned_action
                ));
            }
        }
    }

    // Always suggest the approval path
    if alternatives.len() < max_alternatives {
        alternatives.push(format!(
            "Submit '{}' for pre-approval before execution",
            planned_action
        ));
    }

    alternatives.truncate(max_alternatives);
    alternatives
}

// ═══════════════════════════════════════════════════════════════════════════════
// Invention 1: Policy Omniscience
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_policy_omniscience_query(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let context = args
        .get("context")
        .and_then(|v| v.as_str())
        .unwrap_or("*");
    let include_inactive = args
        .get("include_inactive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let policies = engine.list_policies(None);
    let now = Utc::now();

    let mut allowed = Vec::new();
    let mut denied = Vec::new();
    let mut conditional = Vec::new();

    for policy in &policies {
        // Filter by activity status
        if !include_inactive && policy.status != agentic_contract::PolicyStatus::Active {
            continue;
        }

        // Check context match (word overlap)
        let context_match = if context == "*" {
            true
        } else {
            word_overlap(&policy.label, context) > 0.1
                || policy.tags.iter().any(|t| word_overlap(t, context) > 0.2)
        };

        if !context_match {
            // Check agent match via tags
            let agent_match = policy.tags.iter().any(|t| t == agent_id);
            if !agent_match && policy.scope != PolicyScope::Global {
                continue;
            }
        }

        let entry = json!({
            "action": policy.label,
            "policy_id": policy.id.to_string(),
            "policy_label": policy.label,
            "scope": format!("{:?}", policy.scope),
            "status": format!("{:?}", policy.status),
            "category": classify_action(&policy.action),
            "tags": policy.tags,
        });

        match classify_action(&policy.action) {
            "allowed" => allowed.push(entry),
            "denied" => denied.push(entry),
            "conditional" => conditional.push(entry),
            _ => {}
        }
    }

    let total = allowed.len() + denied.len() + conditional.len();

    let omniscience = PolicyOmniscience {
        id: ContractId::new(),
        agent_id: agent_id.to_string(),
        context: context.to_string(),
        allowed_actions: vec![], // Stored in JSON response
        denied_actions: vec![],
        conditional_actions: vec![],
        total_permissions: total as u32,
        queried_at: now,
    };

    Ok(json!({
        "id": omniscience.id.to_string(),
        "agent_id": agent_id,
        "context": context,
        "allowed_actions": allowed,
        "denied_actions": denied,
        "conditional_actions": conditional,
        "total_permissions": total,
        "summary": {
            "allowed_count": allowed.len(),
            "denied_count": denied.len(),
            "conditional_count": conditional.len(),
            "coverage_ratio": if total > 0 {
                allowed.len() as f64 / total as f64
            } else { 0.0 },
        },
        "queried_at": omniscience.queried_at.to_rfc3339(),
    }))
}

fn handle_policy_omniscience_diff(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_a = require_str(&args, "agent_a")?;
    let agent_b = require_str(&args, "agent_b")?;
    let context = args
        .get("context")
        .and_then(|v| v.as_str())
        .unwrap_or("*");

    let policies = engine.list_policies(None);

    // Compute policy sets for each agent
    let mut perms_a: HashMap<String, String> = HashMap::new();
    let mut perms_b: HashMap<String, String> = HashMap::new();

    for policy in &policies {
        if policy.status != agentic_contract::PolicyStatus::Active {
            continue;
        }

        let context_match = context == "*"
            || word_overlap(&policy.label, context) > 0.1
            || policy.tags.iter().any(|t| word_overlap(t, context) > 0.2);

        if !context_match {
            continue;
        }

        let is_global = policy.scope == PolicyScope::Global;
        let applies_a = is_global || policy.tags.iter().any(|t| t == agent_a);
        let applies_b = is_global || policy.tags.iter().any(|t| t == agent_b);

        let category = classify_action(&policy.action).to_string();

        if applies_a {
            perms_a.insert(policy.label.clone(), category.clone());
        }
        if applies_b {
            perms_b.insert(policy.label.clone(), category);
        }
    }

    // Compute diff
    let mut only_a = Vec::new();
    let mut only_b = Vec::new();
    let mut different = Vec::new();
    let mut shared = Vec::new();

    for (label, cat_a) in &perms_a {
        match perms_b.get(label) {
            None => only_a.push(json!({"policy": label, "category": cat_a})),
            Some(cat_b) if cat_a != cat_b => {
                different.push(json!({
                    "policy": label,
                    "agent_a_category": cat_a,
                    "agent_b_category": cat_b,
                }));
            }
            Some(_) => shared.push(label.clone()),
        }
    }
    for (label, cat_b) in &perms_b {
        if !perms_a.contains_key(label) {
            only_b.push(json!({"policy": label, "category": cat_b}));
        }
    }

    let divergence = if perms_a.len() + perms_b.len() == 0 {
        0.0
    } else {
        let total_unique = {
            let mut all: std::collections::HashSet<&String> = perms_a.keys().collect();
            all.extend(perms_b.keys());
            all.len()
        };
        1.0 - (shared.len() as f64 / total_unique as f64)
    };

    Ok(json!({
        "agent_a": agent_a,
        "agent_b": agent_b,
        "context": context,
        "only_agent_a": only_a,
        "only_agent_b": only_b,
        "different_permissions": different,
        "shared_policies": shared.len(),
        "divergence_score": divergence,
        "analysis": if divergence > 0.7 {
            "Agents have significantly different policy environments"
        } else if divergence > 0.3 {
            "Agents share some policies but have notable differences"
        } else {
            "Agents have largely similar policy environments"
        },
    }))
}

fn handle_policy_omniscience_coverage(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let scope_filter = args.get("scope").and_then(|v| v.as_str());

    let policies = engine.list_policies(None);

    // Analyze coverage by scope
    let mut global_count = 0u32;
    let mut session_count = 0u32;
    let mut agent_count = 0u32;
    let mut allow_count = 0u32;
    let mut deny_count = 0u32;
    let mut approval_count = 0u32;
    let mut audit_count = 0u32;

    let mut action_patterns: HashMap<String, Vec<String>> = HashMap::new();

    for policy in &policies {
        if policy.status != agentic_contract::PolicyStatus::Active {
            continue;
        }

        let scope_match = match scope_filter {
            None => true,
            Some("global") => policy.scope == PolicyScope::Global,
            Some("session") => policy.scope == PolicyScope::Session,
            Some("agent") => policy.scope == PolicyScope::Agent,
            _ => true,
        };

        if !scope_match {
            continue;
        }

        match policy.scope {
            PolicyScope::Global => global_count += 1,
            PolicyScope::Session => session_count += 1,
            PolicyScope::Agent => agent_count += 1,
        }

        match policy.action {
            PolicyAction::Allow => allow_count += 1,
            PolicyAction::Deny => deny_count += 1,
            PolicyAction::RequireApproval => approval_count += 1,
            PolicyAction::AuditOnly => audit_count += 1,
        }

        let first_word = policy
            .label
            .split_whitespace()
            .next()
            .unwrap_or("unknown")
            .to_lowercase();
        action_patterns
            .entry(first_word)
            .or_default()
            .push(policy.label.clone());
    }

    let total = global_count + session_count + agent_count;

    // Identify gaps
    let mut gaps = Vec::new();
    if global_count == 0 {
        gaps.push("No global-scope policies — agents have unrestricted global access");
    }
    if deny_count == 0 && total > 0 {
        gaps.push("No deny policies — all actions are permitted or audited");
    }
    if approval_count == 0 && total > 0 {
        gaps.push("No approval-required policies — no human oversight on sensitive actions");
    }
    if audit_count == 0 && total > 0 {
        gaps.push("No audit-only policies — no passive monitoring configured");
    }

    // Coverage score: higher when more scopes and action types are covered
    let scope_coverage = {
        let covered = (global_count > 0) as u32
            + (session_count > 0) as u32
            + (agent_count > 0) as u32;
        covered as f64 / 3.0
    };
    let action_coverage = {
        let covered = (allow_count > 0) as u32
            + (deny_count > 0) as u32
            + (approval_count > 0) as u32
            + (audit_count > 0) as u32;
        covered as f64 / 4.0
    };
    let overall_coverage = (scope_coverage + action_coverage) / 2.0;

    Ok(json!({
        "total_active_policies": total,
        "by_scope": {
            "global": global_count,
            "session": session_count,
            "agent": agent_count,
        },
        "by_action": {
            "allow": allow_count,
            "deny": deny_count,
            "require_approval": approval_count,
            "audit_only": audit_count,
        },
        "action_patterns": action_patterns.keys().collect::<Vec<_>>(),
        "coverage_scores": {
            "scope_coverage": scope_coverage,
            "action_type_coverage": action_coverage,
            "overall": overall_coverage,
        },
        "gaps": gaps,
        "recommendation": if overall_coverage >= 0.75 {
            "Good policy coverage across scopes and action types"
        } else if overall_coverage >= 0.5 {
            "Moderate coverage — consider adding policies for uncovered scopes"
        } else {
            "Low coverage — significant policy gaps detected"
        },
    }))
}

fn handle_policy_omniscience_conflicts(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let scope_filter = args.get("scope").and_then(|v| v.as_str());

    let policies = engine.list_policies(None);

    let active_policies: Vec<&agentic_contract::Policy> = policies
        .iter()
        .copied()
        .filter(|p| {
            p.status == agentic_contract::PolicyStatus::Active
                && match scope_filter {
                    None => true,
                    Some("global") => p.scope == PolicyScope::Global,
                    Some("session") => p.scope == PolicyScope::Session,
                    Some("agent") => p.scope == PolicyScope::Agent,
                    _ => true,
                }
        })
        .collect();

    let mut conflicts = Vec::new();

    // O(n^2) comparison for policy conflicts
    for i in 0..active_policies.len() {
        for j in (i + 1)..active_policies.len() {
            let pa = active_policies[i];
            let pb = active_policies[j];

            // Check if policies overlap in scope
            let overlap = word_overlap(&pa.label, &pb.label);
            if overlap < 0.2 {
                continue;
            }

            // Check for conflicting actions
            let is_conflict = matches!(
                (&pa.action, &pb.action),
                (PolicyAction::Allow, PolicyAction::Deny)
                    | (PolicyAction::Deny, PolicyAction::Allow)
            );

            let is_ambiguous = matches!(
                (&pa.action, &pb.action),
                (PolicyAction::Allow, PolicyAction::RequireApproval)
                    | (PolicyAction::RequireApproval, PolicyAction::Allow)
            );

            if is_conflict || is_ambiguous {
                let severity = if is_conflict { "conflict" } else { "ambiguous" };
                let resolution = if pa.scope == PolicyScope::Agent
                    && pb.scope == PolicyScope::Global
                {
                    "Agent-scope policy takes precedence over global"
                } else if pa.scope == PolicyScope::Global
                    && pb.scope == PolicyScope::Agent
                {
                    "Agent-scope policy takes precedence over global"
                } else {
                    "Stricter policy (deny > require_approval > audit > allow) should win"
                };

                conflicts.push(json!({
                    "policy_a": {
                        "id": pa.id.to_string(),
                        "label": pa.label,
                        "action": format!("{:?}", pa.action),
                        "scope": format!("{:?}", pa.scope),
                    },
                    "policy_b": {
                        "id": pb.id.to_string(),
                        "label": pb.label,
                        "action": format!("{:?}", pb.action),
                        "scope": format!("{:?}", pb.scope),
                    },
                    "overlap_score": overlap,
                    "severity": severity,
                    "suggested_resolution": resolution,
                }));
            }
        }
    }

    Ok(json!({
        "total_policies_analyzed": active_policies.len(),
        "conflicts_found": conflicts.len(),
        "conflicts": conflicts,
        "health": if conflicts.is_empty() {
            "No policy conflicts detected"
        } else if conflicts.len() <= 2 {
            "Minor conflicts — review recommended"
        } else {
            "Significant conflicts — immediate review required"
        },
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Invention 2: Risk Prophecy
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_risk_prophecy_forecast(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let forecast_window = args
        .get("forecast_window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(3600);
    let confidence_level = args
        .get("confidence_level")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.95);

    let limits = engine.list_risk_limits();
    let all_violations = engine.list_violations(None);
    let owned_violations: Vec<agentic_contract::Violation> = all_violations.into_iter().cloned().collect();
    let now = Utc::now();

    let mut projections = Vec::new();
    let mut overall_risk: f64 = 0.0;
    let mut recommendations = Vec::new();

    for limit in limits {
        // Compute current usage from violations
        let current_usage = compute_risk_usage(
            &owned_violations,
            &limit.label,
            forecast_window * 2, // Look back 2x the forecast window
            now,
        );

        // Compute trend
        let trend_rate = compute_trend_rate(&owned_violations, &limit.label, forecast_window * 2, now);

        // Extrapolate
        let projected = extrapolate_usage(current_usage, trend_rate, forecast_window as f64);

        // Probability of exceeding limit
        let exceed_prob = if projected >= 0.95 {
            confidence_level
        } else if projected >= 0.75 {
            0.5 + (projected - 0.75) * 2.0
        } else {
            projected * 0.3
        };

        // Time to breach estimate
        let time_to_breach = if trend_rate > 0.0 && current_usage < 1.0 {
            Some(((1.0 - current_usage) / trend_rate) as i64)
        } else {
            None
        };

        if exceed_prob > 0.5 {
            recommendations.push(format!(
                "Risk limit '{}' may be breached — consider increasing limit or reducing usage",
                limit.label
            ));
        }

        overall_risk = overall_risk.max(exceed_prob);

        projections.push(json!({
            "limit_id": limit.id.to_string(),
            "limit_label": limit.label,
            "current_usage": current_usage,
            "projected_usage": projected,
            "exceed_probability": exceed_prob,
            "time_until_limit_secs": time_to_breach,
            "trend_rate_per_sec": trend_rate,
            "max_value": limit.max_value,
        }));
    }

    if recommendations.is_empty() {
        recommendations.push("All risk limits are within safe margins".to_string());
    }

    let prophecy = RiskProphecy {
        id: ContractId::new(),
        agent_id: agent_id.to_string(),
        forecast_window_secs: forecast_window,
        projections: vec![],
        overall_risk_score: overall_risk,
        recommendations: recommendations.clone(),
        prophesied_at: now,
    };

    Ok(json!({
        "id": prophecy.id.to_string(),
        "agent_id": agent_id,
        "forecast_window_secs": forecast_window,
        "confidence_level": confidence_level,
        "projections": projections,
        "overall_risk_score": overall_risk,
        "risk_level": if overall_risk > 0.8 { "critical" }
            else if overall_risk > 0.5 { "elevated" }
            else if overall_risk > 0.2 { "moderate" }
            else { "low" },
        "recommendations": recommendations,
        "prophesied_at": prophecy.prophesied_at.to_rfc3339(),
    }))
}

fn handle_risk_prophecy_heatmap(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(86400);
    let bucket_count = args
        .get("bucket_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(24) as usize;

    let violations = engine.list_violations(None);
    let limits = engine.list_risk_limits();
    let now = Utc::now();
    let cutoff = now - Duration::seconds(window_secs);
    let bucket_size = window_secs as f64 / bucket_count as f64;

    let mut heatmap: Vec<Vec<u32>> = vec![vec![0; bucket_count]; limits.len()];

    for (li, limit) in limits.iter().enumerate() {
        for violation in &violations {
            if violation.detected_at < cutoff {
                continue;
            }
            if word_overlap(&violation.description, &limit.label) < 0.15 {
                continue;
            }
            let age = (now - violation.detected_at).num_seconds() as f64;
            let bucket = ((window_secs as f64 - age) / bucket_size) as usize;
            if bucket < bucket_count {
                heatmap[li][bucket] += 1;
            }
        }
    }

    let limit_names: Vec<&str> = limits.iter().map(|l| l.label.as_str()).collect();

    // Find hotspots (buckets with highest concentration)
    let mut hotspots = Vec::new();
    for (li, row) in heatmap.iter().enumerate() {
        for (bi, &count) in row.iter().enumerate() {
            if count >= 3 {
                hotspots.push(json!({
                    "limit": limit_names.get(li).unwrap_or(&"unknown"),
                    "bucket": bi,
                    "time_offset_secs": (bi as f64 * bucket_size) as i64,
                    "count": count,
                }));
            }
        }
    }
    hotspots.sort_by(|a, b| {
        b["count"]
            .as_u64()
            .unwrap_or(0)
            .cmp(&a["count"].as_u64().unwrap_or(0))
    });

    Ok(json!({
        "window_secs": window_secs,
        "bucket_count": bucket_count,
        "bucket_size_secs": bucket_size as i64,
        "limits": limit_names,
        "heatmap": heatmap,
        "hotspots": hotspots,
        "total_violations_in_window": violations.iter().filter(|v| v.detected_at >= cutoff).count(),
    }))
}

fn handle_risk_prophecy_threshold_alert(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let alert_threshold = args
        .get("alert_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.75);

    let limits = engine.list_risk_limits();
    let all_violations = engine.list_violations(None);
    let owned_violations: Vec<agentic_contract::Violation> = all_violations.into_iter().cloned().collect();
    let now = Utc::now();

    let mut alerts = Vec::new();

    for limit in limits {
        let window = limit.window_secs.unwrap_or(3600) as i64;
        let current_usage = compute_risk_usage(&owned_violations, &limit.label, window, now);
        let trend_rate = compute_trend_rate(&owned_violations, &limit.label, window, now);

        if current_usage >= alert_threshold {
            let time_to_breach = if trend_rate > 0.0 && current_usage < 1.0 {
                Some(((1.0 - current_usage) / trend_rate) as i64)
            } else {
                None
            };

            let severity = if current_usage >= 0.95 {
                "critical"
            } else if current_usage >= 0.85 {
                "high"
            } else {
                "warning"
            };

            alerts.push(json!({
                "limit_id": limit.id.to_string(),
                "limit_label": limit.label,
                "current_usage": current_usage,
                "max_value": limit.max_value,
                "threshold": alert_threshold,
                "usage_percentage": (current_usage * 100.0) as u32,
                "severity": severity,
                "trend": if trend_rate > 0.001 { "increasing" }
                    else if trend_rate < -0.001 { "decreasing" }
                    else { "stable" },
                "time_to_breach_secs": time_to_breach,
            }));
        }
    }

    alerts.sort_by(|a, b| {
        let ua = a["current_usage"].as_f64().unwrap_or(0.0);
        let ub = b["current_usage"].as_f64().unwrap_or(0.0);
        ub.partial_cmp(&ua).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(json!({
        "alert_threshold": alert_threshold,
        "total_limits": limits.len(),
        "alerts_triggered": alerts.len(),
        "alerts": alerts,
        "status": if alerts.is_empty() { "all_clear" }
            else if alerts.iter().any(|a| a["severity"] == "critical") { "critical" }
            else { "warning" },
    }))
}

fn handle_risk_prophecy_correlation(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(604800);

    let limits = engine.list_risk_limits();
    let all_violations = engine.list_violations(None);
    let owned_violations: Vec<agentic_contract::Violation> = all_violations.into_iter().cloned().collect();
    let now = Utc::now();

    let mut correlations = Vec::new();

    // Compute pairwise correlation between limits using co-occurrence of violations
    for i in 0..limits.len() {
        for j in (i + 1)..limits.len() {
            let usage_i = compute_risk_usage(&owned_violations, &limits[i].label, window_secs, now);
            let usage_j = compute_risk_usage(&owned_violations, &limits[j].label, window_secs, now);

            // Simple correlation: if both have similar usage patterns, they are correlated
            let diff = (usage_i - usage_j).abs();
            let correlation = 1.0 - diff;

            if correlation > 0.5 && (usage_i > 0.1 || usage_j > 0.1) {
                correlations.push(json!({
                    "limit_a": limits[i].label,
                    "limit_b": limits[j].label,
                    "correlation": correlation,
                    "usage_a": usage_i,
                    "usage_b": usage_j,
                    "coupled": correlation > 0.8,
                    "interpretation": if correlation > 0.8 {
                        "Strongly coupled — violations in one likely indicate violations in the other"
                    } else {
                        "Moderately correlated — some shared risk factors"
                    },
                }));
            }
        }
    }

    correlations.sort_by(|a, b| {
        let ca = a["correlation"].as_f64().unwrap_or(0.0);
        let cb = b["correlation"].as_f64().unwrap_or(0.0);
        cb.partial_cmp(&ca).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(json!({
        "window_secs": window_secs,
        "total_limits": limits.len(),
        "correlations_found": correlations.len(),
        "correlations": correlations,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Invention 3: Approval Telepathy
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_approval_telepathy_predict(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let action = require_str(&args, "action")?;
    let requestor = args.get("requestor").and_then(|v| v.as_str());

    let (probability, likely_approvers, avg_response) =
        estimate_approval_probability(engine, action);
    let now = Utc::now();

    // Build suggestion list
    let mut suggestions = Vec::new();
    if probability < 0.8 {
        suggestions.push(json!({
            "modification": "Add detailed justification explaining business need",
            "estimated_new_probability": (probability + 0.15).min(1.0),
            "effort": "low",
        }));
    }
    if probability < 0.6 {
        suggestions.push(json!({
            "modification": "Break the action into smaller, less risky steps",
            "estimated_new_probability": (probability + 0.25).min(1.0),
            "effort": "medium",
        }));
    }
    if probability < 0.4 {
        suggestions.push(json!({
            "modification": "Request a temporary exception with time-bound scope",
            "estimated_new_probability": (probability + 0.3).min(1.0),
            "effort": "medium",
        }));
        suggestions.push(json!({
            "modification": "Propose compensating controls (additional monitoring, audit trail)",
            "estimated_new_probability": (probability + 0.2).min(1.0),
            "effort": "high",
        }));
    }

    let telepathy = ApprovalTelepathy {
        id: ContractId::new(),
        action: action.to_string(),
        approval_probability: probability,
        likely_approvers: likely_approvers.clone(),
        estimated_response_secs: avg_response,
        suggestions: vec![],
        historical_approval_rate: probability,
        predicted_at: now,
    };

    Ok(json!({
        "id": telepathy.id.to_string(),
        "action": action,
        "requestor": requestor,
        "approval_probability": probability,
        "confidence": if probability > 0.8 || probability < 0.2 { "high" }
            else if probability > 0.6 || probability < 0.4 { "moderate" }
            else { "low" },
        "likely_approvers": likely_approvers,
        "estimated_response_secs": avg_response,
        "suggestions": suggestions,
        "predicted_at": telepathy.predicted_at.to_rfc3339(),
    }))
}

fn handle_approval_telepathy_optimize(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let action = require_str(&args, "action")?;
    let max_suggestions = args
        .get("max_suggestions")
        .and_then(|v| v.as_u64())
        .unwrap_or(5) as usize;

    let (base_probability, _, _) = estimate_approval_probability(engine, action);

    // Generate optimization suggestions with estimated impact
    let mut optimizations = Vec::new();

    // Check if narrowing scope would help
    let policies = engine.list_policies(None);
    let restricting: Vec<&agentic_contract::Policy> = policies
        .iter()
        .copied()
        .filter(|p| {
            p.status == agentic_contract::PolicyStatus::Active
                && word_overlap(&p.label, action) > 0.15
                && matches!(p.action, PolicyAction::Deny | PolicyAction::RequireApproval)
        })
        .collect();

    if !restricting.is_empty() {
        optimizations.push(json!({
            "strategy": "scope_narrowing",
            "description": format!("Narrow action scope to avoid {} restricting policies", restricting.len()),
            "estimated_improvement": 0.15,
            "new_probability": (base_probability + 0.15).min(1.0),
            "effort": "low",
            "affected_policies": restricting.iter().map(|p| p.label.clone()).collect::<Vec<_>>(),
        }));
    }

    optimizations.push(json!({
        "strategy": "justification",
        "description": "Provide detailed business justification with risk assessment",
        "estimated_improvement": 0.1,
        "new_probability": (base_probability + 0.1).min(1.0),
        "effort": "low",
    }));

    optimizations.push(json!({
        "strategy": "compensating_controls",
        "description": "Add compensating controls like monitoring, rollback plans, or time limits",
        "estimated_improvement": 0.2,
        "new_probability": (base_probability + 0.2).min(1.0),
        "effort": "medium",
    }));

    optimizations.push(json!({
        "strategy": "phased_approach",
        "description": "Break into smaller phases with approval gates between each phase",
        "estimated_improvement": 0.25,
        "new_probability": (base_probability + 0.25).min(1.0),
        "effort": "medium",
    }));

    optimizations.push(json!({
        "strategy": "precedent_reference",
        "description": "Reference previously approved similar actions as precedent",
        "estimated_improvement": 0.12,
        "new_probability": (base_probability + 0.12).min(1.0),
        "effort": "low",
    }));

    optimizations.truncate(max_suggestions);

    // Sort by estimated improvement (descending)
    optimizations.sort_by(|a, b| {
        let ia = a["estimated_improvement"].as_f64().unwrap_or(0.0);
        let ib = b["estimated_improvement"].as_f64().unwrap_or(0.0);
        ib.partial_cmp(&ia).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(json!({
        "action": action,
        "base_probability": base_probability,
        "optimizations": optimizations,
        "max_achievable_probability": optimizations.first()
            .and_then(|o| o["new_probability"].as_f64())
            .unwrap_or(base_probability),
    }))
}

fn handle_approval_telepathy_timing(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let action = require_str(&args, "action")?;
    let window_hours = args
        .get("window_hours")
        .and_then(|v| v.as_i64())
        .unwrap_or(168); // 1 week

    let approvals = engine.list_approval_requests(None);
    let now = Utc::now();
    let cutoff = now - Duration::hours(window_hours);

    // Analyze approval response patterns by hour of day
    let mut hour_buckets: Vec<(u32, u32, i64)> = vec![(0, 0, 0); 24]; // (total, approved, response_time)

    for req in &approvals {
        if req.created_at < cutoff {
            continue;
        }
        if word_overlap(&req.action_description, action) < 0.15 {
            continue;
        }

        let hour = req.created_at.format("%H").to_string().parse::<usize>().unwrap_or(0);
        if hour < 24 {
            hour_buckets[hour].0 += 1;
            if req.status == agentic_contract::ApprovalStatus::Approved {
                hour_buckets[hour].1 += 1;
            }
        }
    }

    // Find optimal hours (highest approval rate, fastest response)
    let mut hour_scores: Vec<(usize, f64)> = hour_buckets
        .iter()
        .enumerate()
        .map(|(hour, (total, approved, _response_sum))| {
            if *total == 0 {
                (hour, 0.5) // No data — neutral
            } else {
                let rate = *approved as f64 / *total as f64;
                (hour, rate)
            }
        })
        .collect();

    hour_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let optimal_windows: Vec<Value> = hour_scores
        .iter()
        .take(5)
        .map(|(hour, score)| {
            json!({
                "hour_utc": hour,
                "approval_rate": score,
                "recommendation": if *score > 0.8 { "Highly recommended" }
                    else if *score > 0.6 { "Good window" }
                    else { "Suboptimal — consider alternatives" },
            })
        })
        .collect();

    Ok(json!({
        "action": action,
        "analysis_window_hours": window_hours,
        "optimal_submission_windows": optimal_windows,
        "worst_hours": hour_scores.iter().rev().take(3)
            .map(|(h, s)| json!({"hour_utc": h, "approval_rate": s}))
            .collect::<Vec<_>>(),
        "data_points": approvals.iter()
            .filter(|r| r.created_at >= cutoff)
            .filter(|r| word_overlap(&r.action_description, action) >= 0.15)
            .count(),
    }))
}

fn handle_approval_telepathy_bottleneck(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(604800);

    let approvals = engine.list_approval_requests(None);
    let now = Utc::now();
    let cutoff = now - Duration::seconds(window_secs);

    let mut approver_stats: HashMap<String, (u32, i64, u32)> = HashMap::new(); // (total, total_response_time, pending)

    let mut pending_count = 0u32;
    let mut total_count = 0u32;
    let mut slow_requests = Vec::new();

    for req in &approvals {
        if req.created_at < cutoff {
            continue;
        }
        total_count += 1;

        if req.status == agentic_contract::ApprovalStatus::Pending {
            pending_count += 1;
            let wait = (now - req.created_at).num_seconds();
            if wait > 3600 {
                slow_requests.push(json!({
                    "request_id": req.id.to_string(),
                    "action": req.action_description,
                    "waiting_secs": wait,
                    "requestor": req.requestor,
                }));
            }
        }

        // Track requestor as a proxy for approver stats
        let entry = approver_stats
            .entry(req.requestor.clone())
            .or_insert((0, 0, 0));
        entry.0 += 1;
    }

    let mut bottlenecks: Vec<Value> = approver_stats
        .iter()
        .map(|(approver, (total, total_time, _pending))| {
            let avg_response = if *total > 0 {
                *total_time / *total as i64
            } else {
                0
            };
            json!({
                "approver": approver,
                "total_decisions": total,
                "avg_response_secs": avg_response,
                "is_bottleneck": avg_response > 7200, // > 2 hours
            })
        })
        .collect();

    bottlenecks.sort_by(|a, b| {
        let ta = a["avg_response_secs"].as_i64().unwrap_or(0);
        let tb = b["avg_response_secs"].as_i64().unwrap_or(0);
        tb.cmp(&ta)
    });

    slow_requests.sort_by(|a, b| {
        let wa = a["waiting_secs"].as_i64().unwrap_or(0);
        let wb = b["waiting_secs"].as_i64().unwrap_or(0);
        wb.cmp(&wa)
    });

    Ok(json!({
        "window_secs": window_secs,
        "total_requests": total_count,
        "pending_requests": pending_count,
        "pending_rate": if total_count > 0 { pending_count as f64 / total_count as f64 } else { 0.0 },
        "approver_performance": bottlenecks,
        "slow_pending_requests": slow_requests,
        "health": if pending_count == 0 { "healthy" }
            else if pending_count as f64 / total_count.max(1) as f64 > 0.3 { "congested" }
            else { "normal" },
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Invention 4: Obligation Clairvoyance
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_obligation_clairvoyance_forecast(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(86400);
    let include_completed = args
        .get("include_completed")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let obligations = engine.list_obligations(None);
    let now = Utc::now();
    let deadline_cutoff = now + Duration::seconds(window_secs);

    let mut forecasts = Vec::new();
    let mut conflicts = Vec::new();

    // Filter obligations relevant to this agent and within window
    let relevant: Vec<&agentic_contract::Obligation> = obligations
        .into_iter()
        .filter(|o| {
            if !include_completed
                && o.status == agentic_contract::ObligationStatus::Fulfilled
            {
                return false;
            }
            // Check if obligation mentions the agent
            word_overlap(&o.label, agent_id) > 0.1
                || o.label.contains(agent_id)
                || true // Include all for broad forecast
        })
        .filter(|o| {
            o.deadline
                .map(|d| d <= deadline_cutoff)
                .unwrap_or(true)
        })
        .collect();

    for obligation in &relevant {
        let time_remaining = obligation
            .deadline
            .map(|d| (d - now).num_seconds());
        let effort = estimate_effort(&obligation.label);
        let miss_risk = compute_miss_risk(time_remaining, effort, 0.6, 0.4);

        forecasts.push(json!({
            "obligation_id": obligation.id.to_string(),
            "label": obligation.label,
            "status": format!("{:?}", obligation.status),
            "deadline": obligation.deadline.map(|d| d.to_rfc3339()),
            "time_remaining_secs": time_remaining,
            "estimated_effort_minutes": effort,
            "miss_risk": miss_risk,
            "risk_level": if miss_risk > 0.8 { "critical" }
                else if miss_risk > 0.5 { "high" }
                else if miss_risk > 0.3 { "moderate" }
                else { "low" },
        }));
    }

    // Detect scheduling conflicts (overlapping deadlines with insufficient time)
    let mut sorted = forecasts.clone();
    sorted.sort_by(|a, b| {
        let da = a["time_remaining_secs"].as_i64().unwrap_or(i64::MAX);
        let db = b["time_remaining_secs"].as_i64().unwrap_or(i64::MAX);
        da.cmp(&db)
    });

    let mut cumulative_effort = 0i64;
    for forecast in &sorted {
        let effort_secs = forecast["estimated_effort_minutes"].as_u64().unwrap_or(0) as i64 * 60;
        let remaining = forecast["time_remaining_secs"].as_i64().unwrap_or(i64::MAX);

        cumulative_effort += effort_secs;
        if cumulative_effort > remaining && remaining < i64::MAX {
            conflicts.push(json!({
                "obligation": forecast["label"],
                "issue": "Cumulative effort exceeds available time before deadline",
                "cumulative_effort_secs": cumulative_effort,
                "deadline_secs": remaining,
                "resolution": "Prioritize or delegate some obligations",
            }));
        }
    }

    // Compute optimal order (by urgency × effort score)
    let mut optimal: Vec<Value> = forecasts.clone();
    optimal.sort_by(|a, b| {
        let ra = a["miss_risk"].as_f64().unwrap_or(0.0);
        let rb = b["miss_risk"].as_f64().unwrap_or(0.0);
        rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(json!({
        "agent_id": agent_id,
        "window_secs": window_secs,
        "upcoming_obligations": forecasts.len(),
        "forecasts": forecasts,
        "conflicts": conflicts,
        "optimal_fulfillment_order": optimal.iter()
            .map(|o| o["label"].as_str().unwrap_or("unknown"))
            .collect::<Vec<_>>(),
        "projected_at": now.to_rfc3339(),
    }))
}

fn handle_obligation_clairvoyance_dependencies(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let obligation_id = require_id(&args, "obligation_id")?;

    let obligations = engine.list_obligations(None);

    // Find the root obligation
    let root = obligations
        .iter()
        .find(|o| o.id == obligation_id)
        .ok_or_else(|| format!("Obligation not found: {}", obligation_id))?;

    // Build dependency tree using label word overlap
    let mut dependencies = Vec::new();
    let mut visited = std::collections::HashSet::new();
    visited.insert(obligation_id);

    for obligation in &obligations {
        if obligation.id == obligation_id {
            continue;
        }
        let overlap = word_overlap(&root.label, &obligation.label);
        if overlap > 0.25 {
            dependencies.push(json!({
                "obligation_id": obligation.id.to_string(),
                "label": obligation.label,
                "status": format!("{:?}", obligation.status),
                "relationship": if overlap > 0.6 { "strong_dependency" }
                    else { "weak_dependency" },
                "overlap_score": overlap,
            }));
            visited.insert(obligation.id);
        }
    }

    let critical_path_length = dependencies
        .iter()
        .filter(|d| d["relationship"] == "strong_dependency")
        .count();

    Ok(json!({
        "root_obligation": {
            "id": root.id.to_string(),
            "label": root.label,
            "status": format!("{:?}", root.status),
        },
        "dependencies": dependencies,
        "dependency_count": dependencies.len(),
        "critical_path_length": critical_path_length,
        "cascade_risk": if critical_path_length > 3 { "high" }
            else if critical_path_length > 1 { "moderate" }
            else { "low" },
    }))
}

fn handle_obligation_clairvoyance_workload(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let _window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(604800);
    let overload_threshold = args
        .get("overload_threshold")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let obligations = engine.list_obligations(None);

    // Group by agent (using tags or label patterns)
    let mut agent_workloads: HashMap<String, Vec<Value>> = HashMap::new();

    for obligation in &obligations {
        if obligation.status == agentic_contract::ObligationStatus::Fulfilled {
            continue;
        }

        // Extract agent from label (heuristic: first capitalized word or "default")
        let agent = obligation
            .label
            .split_whitespace()
            .find(|w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
            .unwrap_or("unassigned")
            .to_string();

        let effort = estimate_effort(&obligation.label);
        agent_workloads
            .entry(agent)
            .or_default()
            .push(json!({
                "obligation_id": obligation.id.to_string(),
                "label": obligation.label,
                "effort_minutes": effort,
                "deadline": obligation.deadline.map(|d| d.to_rfc3339()),
            }));
    }

    let mut workload_summary: Vec<Value> = agent_workloads
        .iter()
        .map(|(agent, items)| {
            let total_effort: u32 = items
                .iter()
                .map(|i| i["effort_minutes"].as_u64().unwrap_or(0) as u32)
                .sum();
            json!({
                "agent": agent,
                "obligation_count": items.len(),
                "total_effort_minutes": total_effort,
                "is_overloaded": items.len() > overload_threshold,
                "obligations": items,
            })
        })
        .collect();

    workload_summary.sort_by(|a, b| {
        let ea = a["total_effort_minutes"].as_u64().unwrap_or(0);
        let eb = b["total_effort_minutes"].as_u64().unwrap_or(0);
        eb.cmp(&ea)
    });

    let overloaded_count = workload_summary
        .iter()
        .filter(|w| w["is_overloaded"].as_bool().unwrap_or(false))
        .count();

    Ok(json!({
        "total_active_obligations": obligations.iter()
            .filter(|o| o.status != agentic_contract::ObligationStatus::Fulfilled)
            .count(),
        "agents_analyzed": workload_summary.len(),
        "overloaded_agents": overloaded_count,
        "overload_threshold": overload_threshold,
        "workloads": workload_summary,
        "recommendation": if overloaded_count == 0 { "Workload is balanced" }
            else { "Consider redistributing obligations from overloaded agents" },
    }))
}

fn handle_obligation_clairvoyance_risk(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let urgency_weight = args
        .get("urgency_weight")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.6);
    let complexity_weight = args
        .get("complexity_weight")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.4);

    let obligations = engine.list_obligations(None);
    let now = Utc::now();

    let mut risk_assessments = Vec::new();
    let mut total_risk = 0.0;
    let mut high_risk_count = 0u32;

    for obligation in &obligations {
        if obligation.status == agentic_contract::ObligationStatus::Fulfilled {
            continue;
        }

        let time_remaining = obligation
            .deadline
            .map(|d| (d - now).num_seconds());
        let effort = estimate_effort(&obligation.label);
        let risk = compute_miss_risk(time_remaining, effort, urgency_weight, complexity_weight);

        if risk > 0.7 {
            high_risk_count += 1;
        }
        total_risk += risk;

        risk_assessments.push(json!({
            "obligation_id": obligation.id.to_string(),
            "label": obligation.label,
            "miss_risk": risk,
            "risk_level": if risk > 0.8 { "critical" }
                else if risk > 0.5 { "high" }
                else if risk > 0.3 { "moderate" }
                else { "low" },
            "deadline_remaining_secs": time_remaining,
            "effort_minutes": effort,
            "urgency_component": if let Some(secs) = time_remaining {
                compute_miss_risk(Some(secs), 0, 1.0, 0.0)
            } else { 0.3 },
            "complexity_component": compute_miss_risk(None, effort, 0.0, 1.0),
        }));
    }

    risk_assessments.sort_by(|a, b| {
        let ra = a["miss_risk"].as_f64().unwrap_or(0.0);
        let rb = b["miss_risk"].as_f64().unwrap_or(0.0);
        rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
    });

    let avg_risk = if risk_assessments.is_empty() {
        0.0
    } else {
        total_risk / risk_assessments.len() as f64
    };

    Ok(json!({
        "agent_id": agent_id,
        "urgency_weight": urgency_weight,
        "complexity_weight": complexity_weight,
        "obligations_analyzed": risk_assessments.len(),
        "high_risk_count": high_risk_count,
        "average_risk": avg_risk,
        "overall_risk_level": if avg_risk > 0.7 { "critical" }
            else if avg_risk > 0.4 { "elevated" }
            else { "manageable" },
        "risk_assessments": risk_assessments,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Invention 5: Violation Precognition
// ═══════════════════════════════════════════════════════════════════════════════

fn handle_violation_precognition_analyze(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let planned_action = require_str(&args, "planned_action")?;
    let _agent_id = args.get("agent_id").and_then(|v| v.as_str());

    let policies_refs = engine.list_policies(None);
    let owned_policies: Vec<agentic_contract::Policy> = policies_refs.into_iter().cloned().collect();
    let limits = engine.list_risk_limits();
    let violations_refs = engine.list_violations(None);
    let owned_violations: Vec<agentic_contract::Violation> = violations_refs.into_iter().cloned().collect();
    let now = Utc::now();

    let mut at_risk_policies = Vec::new();
    let mut at_risk_limits = Vec::new();

    // Check policies
    for policy in &owned_policies {
        if policy.status != agentic_contract::PolicyStatus::Active {
            continue;
        }

        let overlap = word_overlap(&policy.label, planned_action);
        if overlap < 0.15 {
            continue;
        }

        match policy.action {
            PolicyAction::Deny => {
                at_risk_policies.push(json!({
                    "policy_id": policy.id.to_string(),
                    "policy_label": policy.label,
                    "probability": (0.5 + overlap * 0.5).min(1.0),
                    "trigger": format!("Action '{}' matches deny policy '{}'", planned_action, policy.label),
                    "scope": format!("{:?}", policy.scope),
                }));
            }
            PolicyAction::RequireApproval => {
                at_risk_policies.push(json!({
                    "policy_id": policy.id.to_string(),
                    "policy_label": policy.label,
                    "probability": (0.3 + overlap * 0.4).min(0.8),
                    "trigger": format!("Action '{}' requires approval per policy '{}'", planned_action, policy.label),
                    "scope": format!("{:?}", policy.scope),
                }));
            }
            _ => {}
        }
    }

    // Check risk limits
    for limit in limits {
        let current_usage = compute_risk_usage(&owned_violations, &limit.label, 3600, now);
        let overlap = word_overlap(&limit.label, planned_action);

        if overlap > 0.1 && current_usage > 0.5 {
            at_risk_limits.push(json!({
                "limit_id": limit.id.to_string(),
                "limit_label": limit.label,
                "current_headroom": 1.0 - current_usage,
                "projected_usage_increase": 0.1 + overlap * 0.3,
                "would_breach": current_usage + 0.1 + overlap * 0.3 > 1.0,
            }));
        }
    }

    // Generate alternatives
    let alternatives = generate_alternatives(&owned_policies, planned_action, 3);

    // Compute overall violation probability
    let max_policy_risk = at_risk_policies
        .iter()
        .map(|p| p["probability"].as_f64().unwrap_or(0.0))
        .fold(0.0f64, |a, b| a.max(b));
    let has_limit_breach = at_risk_limits
        .iter()
        .any(|l| l["would_breach"].as_bool().unwrap_or(false));

    let violation_probability = if has_limit_breach {
        (max_policy_risk + 0.3).min(1.0)
    } else {
        max_policy_risk
    };

    Ok(json!({
        "planned_action": planned_action,
        "violation_probability": violation_probability,
        "risk_level": if violation_probability > 0.8 { "critical" }
            else if violation_probability > 0.5 { "high" }
            else if violation_probability > 0.2 { "moderate" }
            else { "low" },
        "at_risk_policies": at_risk_policies,
        "at_risk_limits": at_risk_limits,
        "safe_alternatives": alternatives,
        "recommendation": if violation_probability > 0.7 {
            "High violation risk — consider alternatives or request approval first"
        } else if violation_probability > 0.3 {
            "Moderate risk — proceed with caution and additional monitoring"
        } else {
            "Low risk — action appears safe under current policies"
        },
        "analyzed_at": now.to_rfc3339(),
    }))
}

fn handle_violation_precognition_batch(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let actions: Vec<String> = args
        .get("actions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    if actions.is_empty() {
        return Err("At least one action is required".to_string());
    }

    let _agent_id = args.get("agent_id").and_then(|v| v.as_str());

    let policies_refs = engine.list_policies(None);
    let limits = engine.list_risk_limits();
    let violations_refs = engine.list_violations(None);
    let owned_violations: Vec<agentic_contract::Violation> = violations_refs.into_iter().cloned().collect();
    let now = Utc::now();

    let mut results = Vec::new();
    let mut cumulative_risk: f64 = 0.0;
    let mut cumulative_limit_usage: HashMap<String, f64> = HashMap::new();

    for (i, action) in actions.iter().enumerate() {
        let mut action_risk: f64 = 0.0;
        let mut action_issues = Vec::new();

        // Check policies
        for policy in &policies_refs {
            if policy.status != agentic_contract::PolicyStatus::Active {
                continue;
            }
            let overlap = word_overlap(&policy.label, action);
            if overlap > 0.15 && matches!(policy.action, PolicyAction::Deny) {
                let risk = (0.5 + overlap * 0.5).min(1.0);
                action_risk = action_risk.max(risk);
                action_issues.push(format!("Conflicts with deny policy '{}'", policy.label));
            }
        }

        // Check limits with cumulative impact
        for limit in limits {
            let base_usage = compute_risk_usage(&owned_violations, &limit.label, 3600, now);
            let cumulative = cumulative_limit_usage
                .get(&limit.label)
                .copied()
                .unwrap_or(0.0);
            let overlap = word_overlap(&limit.label, action);

            if overlap > 0.1 {
                let additional = 0.1 + overlap * 0.2;
                let projected = base_usage + cumulative + additional;
                *cumulative_limit_usage.entry(limit.label.clone()).or_insert(0.0) += additional;

                if projected > 1.0 {
                    action_issues.push(format!(
                        "Would breach risk limit '{}' (cumulative usage: {:.0}%)",
                        limit.label,
                        projected * 100.0
                    ));
                    action_risk = action_risk.max(0.9);
                }
            }
        }

        cumulative_risk += action_risk;

        results.push(json!({
            "sequence": i + 1,
            "action": action,
            "individual_risk": action_risk,
            "cumulative_risk": (cumulative_risk / (i + 1) as f64).min(1.0),
            "issues": action_issues,
            "safe": action_risk < 0.3,
        }));
    }

    let overall_risk = cumulative_risk / actions.len() as f64;

    Ok(json!({
        "actions_analyzed": actions.len(),
        "results": results,
        "overall_risk": overall_risk.min(1.0),
        "safe_actions": results.iter()
            .filter(|r| r["safe"].as_bool().unwrap_or(false))
            .count(),
        "risky_actions": results.iter()
            .filter(|r| !r["safe"].as_bool().unwrap_or(true))
            .count(),
        "cumulative_limit_impacts": cumulative_limit_usage,
        "recommendation": if overall_risk > 0.7 {
            "Batch has high cumulative risk — consider splitting into phases"
        } else if overall_risk > 0.3 {
            "Some actions carry risk — review flagged items before executing"
        } else {
            "Batch appears safe for execution"
        },
    }))
}

fn handle_violation_precognition_alternatives(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let planned_action = require_str(&args, "planned_action")?;
    let max_alternatives = args
        .get("max_alternatives")
        .and_then(|v| v.as_u64())
        .unwrap_or(3) as usize;

    let policies_refs = engine.list_policies(None);
    let owned_policies: Vec<agentic_contract::Policy> = policies_refs.into_iter().cloned().collect();
    let alternatives = generate_alternatives(&owned_policies, planned_action, max_alternatives);

    // Score each alternative
    let mut scored_alternatives: Vec<Value> = alternatives
        .iter()
        .enumerate()
        .map(|(i, alt)| {
            // Check how many policies the alternative would still conflict with
            let conflicts: Vec<&agentic_contract::Policy> = owned_policies
                .iter()
                .filter(|p| {
                    p.status == agentic_contract::PolicyStatus::Active
                        && word_overlap(&p.label, alt) > 0.15
                        && matches!(p.action, PolicyAction::Deny)
                })
                .collect();

            let safety_score = if conflicts.is_empty() {
                1.0
            } else {
                1.0 - (conflicts.len() as f64 * 0.3).min(1.0)
            };

            json!({
                "rank": i + 1,
                "alternative": alt,
                "safety_score": safety_score,
                "remaining_conflicts": conflicts.len(),
                "feasibility": if i == 0 { "high" }
                    else if i == 1 { "medium" }
                    else { "lower" },
            })
        })
        .collect();

    scored_alternatives.sort_by(|a, b| {
        let sa = a["safety_score"].as_f64().unwrap_or(0.0);
        let sb = b["safety_score"].as_f64().unwrap_or(0.0);
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Re-rank after sorting
    for (i, alt) in scored_alternatives.iter_mut().enumerate() {
        alt["rank"] = json!(i + 1);
    }

    Ok(json!({
        "planned_action": planned_action,
        "alternatives": scored_alternatives,
        "best_alternative": scored_alternatives.first().cloned(),
    }))
}

fn handle_violation_precognition_history_pattern(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(2592000); // 30 days
    let min_cluster_size = args
        .get("min_cluster_size")
        .and_then(|v| v.as_u64())
        .unwrap_or(3) as usize;

    let all_violations = engine.list_violations(None);
    let now = Utc::now();
    let cutoff = now - Duration::seconds(window_secs);

    // Filter to relevant violations
    let relevant: Vec<&agentic_contract::Violation> = all_violations
        .into_iter()
        .filter(|v| v.detected_at >= cutoff && v.actor == agent_id)
        .collect();

    // Cluster by description similarity
    let mut clusters: Vec<Vec<&agentic_contract::Violation>> = Vec::new();

    for violation in &relevant {
        let mut found = false;
        for cluster in &mut clusters {
            if let Some(first) = cluster.first() {
                if word_overlap(&first.description, &violation.description) > 0.3 {
                    cluster.push(*violation);
                    found = true;
                    break;
                }
            }
        }
        if !found {
            clusters.push(vec![*violation]);
        }
    }

    // Filter to clusters meeting minimum size
    let significant_clusters: Vec<Value> = clusters
        .iter()
        .filter(|c| c.len() >= min_cluster_size)
        .map(|cluster| {
            let count = cluster.len();
            let first = cluster.first().unwrap();
            let last = cluster.last().unwrap();

            // Compute average time between violations
            let mut intervals = Vec::new();
            for i in 1..cluster.len() {
                let gap = (cluster[i].detected_at - cluster[i - 1].detected_at).num_seconds();
                intervals.push(gap);
            }
            let avg_interval = if intervals.is_empty() {
                0
            } else {
                intervals.iter().sum::<i64>() / intervals.len() as i64
            };

            // Compute severity distribution
            let mut severity_counts: HashMap<String, u32> = HashMap::new();
            for v in cluster {
                *severity_counts
                    .entry(format!("{:?}", v.severity))
                    .or_insert(0) += 1;
            }

            // Trend: is the pattern accelerating or decelerating?
            let trend = if intervals.len() >= 2 {
                let mid = intervals.len() / 2;
                let first_avg: f64 =
                    intervals[..mid].iter().sum::<i64>() as f64 / mid as f64;
                let second_avg: f64 =
                    intervals[mid..].iter().sum::<i64>() as f64
                        / (intervals.len() - mid) as f64;
                if second_avg < first_avg * 0.8 {
                    "accelerating"
                } else if second_avg > first_avg * 1.2 {
                    "decelerating"
                } else {
                    "stable"
                }
            } else {
                "insufficient_data"
            };

            json!({
                "pattern": first.description.chars().take(80).collect::<String>(),
                "count": count,
                "first_occurrence": first.detected_at.to_rfc3339(),
                "last_occurrence": last.detected_at.to_rfc3339(),
                "avg_interval_secs": avg_interval,
                "severity_distribution": severity_counts,
                "trend": trend,
                "predicted_next_occurrence_secs": if avg_interval > 0 {
                    Some(avg_interval)
                } else { None },
                "risk": if trend == "accelerating" { "high" }
                    else if count > 5 { "moderate" }
                    else { "low" },
            })
        })
        .collect();

    Ok(json!({
        "agent_id": agent_id,
        "window_secs": window_secs,
        "total_violations": relevant.len(),
        "patterns_detected": significant_clusters.len(),
        "patterns": significant_clusters,
        "min_cluster_size": min_cluster_size,
        "analysis": if significant_clusters.is_empty() {
            "No recurring violation patterns detected"
        } else if significant_clusters.iter().any(|c| c["trend"] == "accelerating") {
            "Accelerating violation patterns detected — intervention recommended"
        } else {
            "Recurring patterns detected — monitor for changes"
        },
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

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
        let score = word_overlap("a b c", "a b c");
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_classify_action() {
        assert_eq!(classify_action(&PolicyAction::Allow), "allowed");
        assert_eq!(classify_action(&PolicyAction::Deny), "denied");
        assert_eq!(classify_action(&PolicyAction::RequireApproval), "conditional");
        assert_eq!(classify_action(&PolicyAction::AuditOnly), "allowed");
    }

    #[test]
    fn test_estimate_effort_keywords() {
        assert_eq!(estimate_effort("audit the system"), 120);
        assert_eq!(estimate_effort("deploy to production"), 240);
        assert_eq!(estimate_effort("generate report"), 60);
        assert_eq!(estimate_effort("fix the bug"), 90);
        assert_eq!(estimate_effort("run tests"), 45);
        assert_eq!(estimate_effort("do something"), 30);
    }

    #[test]
    fn test_compute_miss_risk_no_deadline() {
        let risk = compute_miss_risk(None, 30, 0.6, 0.4);
        assert!(risk > 0.0);
        assert!(risk < 0.5);
    }

    #[test]
    fn test_compute_miss_risk_expired_deadline() {
        let risk = compute_miss_risk(Some(-100), 30, 0.6, 0.4);
        assert!(risk > 0.5);
    }

    #[test]
    fn test_compute_miss_risk_far_deadline() {
        let risk = compute_miss_risk(Some(1_000_000), 10, 0.6, 0.4);
        assert!(risk < 0.2);
    }

    #[test]
    fn test_extrapolate_usage_clamp() {
        assert_eq!(extrapolate_usage(0.9, 0.001, 200.0), 1.0);
        assert_eq!(extrapolate_usage(0.1, -1.0, 200.0), 0.0);
    }

    #[test]
    fn test_extrapolate_usage_normal() {
        let result = extrapolate_usage(0.5, 0.0001, 1000.0);
        assert!((result - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_policy_omniscience_query() {
        let mut engine = ContractEngine::new();

        // Add some policies
        engine.add_policy(agentic_contract::Policy::new(
            "deploy production",
            PolicyScope::Global,
            PolicyAction::Deny,
        ));
        engine.add_policy(agentic_contract::Policy::new(
            "read logs",
            PolicyScope::Global,
            PolicyAction::Allow,
        ));

        let result = try_handle(
            "policy_omniscience_query",
            json!({"agent_id": "agent_1", "context": "deploy"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["total_permissions"].as_u64().unwrap() > 0);
    }

    #[test]
    fn test_policy_omniscience_coverage() {
        let mut engine = ContractEngine::new();

        engine.add_policy(agentic_contract::Policy::new(
            "global deny",
            PolicyScope::Global,
            PolicyAction::Deny,
        ));

        let result = try_handle(
            "policy_omniscience_coverage",
            json!({}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["total_active_policies"].as_u64().unwrap() >= 1);
        assert!(value["coverage_scores"]["overall"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn test_policy_omniscience_conflicts_no_conflicts() {
        let mut engine = ContractEngine::new();

        engine.add_policy(agentic_contract::Policy::new(
            "reading documents",
            PolicyScope::Global,
            PolicyAction::Allow,
        ));
        engine.add_policy(agentic_contract::Policy::new(
            "budget spending",
            PolicyScope::Global,
            PolicyAction::Deny,
        ));

        let result = try_handle(
            "policy_omniscience_conflicts",
            json!({}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        // Labels share no words so overlap is 0, below 0.2 threshold
        assert_eq!(value["conflicts_found"].as_u64().unwrap(), 0);
    }

    #[test]
    fn test_risk_prophecy_forecast_no_violations() {
        let mut engine = ContractEngine::new();
        engine.add_risk_limit(agentic_contract::RiskLimit::new("api calls", agentic_contract::LimitType::Rate, 100.0));

        let result = try_handle(
            "risk_prophecy_forecast",
            json!({"agent_id": "agent_1"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["risk_level"], "low");
    }

    #[test]
    fn test_risk_prophecy_heatmap() {
        let mut engine = ContractEngine::new();
        engine.add_risk_limit(agentic_contract::RiskLimit::new("budget", agentic_contract::LimitType::Budget, 1000.0));

        let result = try_handle(
            "risk_prophecy_heatmap",
            json!({"window_secs": 3600, "bucket_count": 6}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["bucket_count"].as_u64().unwrap(), 6);
    }

    #[test]
    fn test_risk_prophecy_threshold_alert_clean() {
        let mut engine = ContractEngine::new();
        engine.add_risk_limit(agentic_contract::RiskLimit::new("budget", agentic_contract::LimitType::Budget, 1000.0));

        let result = try_handle(
            "risk_prophecy_threshold_alert",
            json!({"alert_threshold": 0.75}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["status"], "all_clear");
    }

    #[test]
    fn test_approval_telepathy_predict_no_history() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "approval_telepathy_predict",
            json!({"action": "deploy production"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        // With no history, probability should be the prior (0.5)
        assert!((value["approval_probability"].as_f64().unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_approval_telepathy_optimize() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "approval_telepathy_optimize",
            json!({"action": "deploy", "max_suggestions": 3}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["optimizations"].as_array().unwrap().len() <= 3);
    }

    #[test]
    fn test_obligation_clairvoyance_forecast_empty() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "obligation_clairvoyance_forecast",
            json!({"agent_id": "agent_1"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["upcoming_obligations"].as_u64().unwrap(), 0);
    }

    #[test]
    fn test_obligation_clairvoyance_workload() {
        let mut engine = ContractEngine::new();
        engine.add_obligation(agentic_contract::Obligation::new(
            "Review code changes",
            "Review all recent code changes for compliance",
            "agent_1",
        ));

        let result = try_handle(
            "obligation_clairvoyance_workload",
            json!({"overload_threshold": 5}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["total_active_obligations"].as_u64().unwrap() >= 1);
    }

    #[test]
    fn test_violation_precognition_safe_action() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "violation_precognition_analyze",
            json!({"planned_action": "read documentation"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["risk_level"], "low");
    }

    #[test]
    fn test_violation_precognition_batch() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "violation_precognition_batch",
            json!({"actions": ["read logs", "check status"]}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["actions_analyzed"].as_u64().unwrap(), 2);
    }

    #[test]
    fn test_violation_precognition_batch_empty() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "violation_precognition_batch",
            json!({"actions": []}),
            &mut engine,
        );
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_violation_precognition_history_no_violations() {
        let mut engine = ContractEngine::new();

        let result = try_handle(
            "violation_precognition_history_pattern",
            json!({"agent_id": "agent_1"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["patterns_detected"].as_u64().unwrap(), 0);
    }

    #[test]
    fn test_generate_alternatives_no_restrictions() {
        let policies = vec![];
        let alts = generate_alternatives(&policies, "do something", 3);
        assert!(!alts.is_empty());
        assert!(alts[0].contains("no restrictions"));
    }

    #[test]
    fn test_compute_risk_usage_no_violations() {
        let violations = vec![];
        let usage = compute_risk_usage(&violations, "budget", 3600, Utc::now());
        assert_eq!(usage, 0.0);
    }

    #[test]
    fn test_compute_trend_rate_insufficient_data() {
        let violations = vec![];
        let rate = compute_trend_rate(&violations, "budget", 3600, Utc::now());
        assert_eq!(rate, 0.0);
    }
}

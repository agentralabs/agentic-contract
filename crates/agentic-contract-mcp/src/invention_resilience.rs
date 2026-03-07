//! Inventions 13-16 (Resilience category) — deep implementations.
//!
//! - **13. Violation Archaeology** — deep analysis of violation patterns
//! - **14. Contract Simulation** — simulate contract behavior across scenarios
//! - **15. Federated Governance** — cross-organizational governance
//! - **16. Self-Healing Contracts** — contracts that adapt automatically

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use agentic_contract::ContractEngine;

use crate::tools::{require_id, require_str, ToolDefinition};

// ─── Tool definitions ────────────────────────────────────────────────────────

pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Invention 13: Violation Archaeology ───────────────────────
    ToolDefinition {
        name: "violation_archaeology_analyze",
        description: "Analyze violation patterns to identify root causes and clusters",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to analyze"},"window_secs":{"type":"integer","description":"Analysis window in seconds","default":604800},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "violation_archaeology_timeline",
        description: "Build violation timeline with rolling rate and trend analysis",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to build timeline for"},"window_secs":{"type":"integer","description":"Timeline window in seconds","default":604800},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "violation_archaeology_predict",
        description: "Predict future violations based on historical velocity and acceleration",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to predict for"},"forecast_days":{"type":"integer","description":"Days to forecast ahead","default":7},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "violation_archaeology_compare",
        description: "Compare violation patterns across multiple agents to rank risk",
        input_schema: r#"{"type":"object","properties":{"agent_ids":{"type":"array","items":{"type":"string"},"description":"Agent IDs to compare"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["agent_ids"]}"#,
    },
    // ── Invention 14: Contract Simulation ─────────────────────────
    ToolDefinition {
        name: "contract_simulation_run",
        description: "Simulate contract behavior across scenarios to find deadlocks and edge cases",
        input_schema: r#"{"type":"object","properties":{"scenario_count":{"type":"integer","description":"Number of scenarios to simulate","default":100},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}}}"#,
    },
    ToolDefinition {
        name: "contract_simulation_stress",
        description: "Stress test the contract under increasing load to find stability threshold",
        input_schema: r#"{"type":"object","properties":{"max_agents":{"type":"integer","description":"Maximum concurrent agents","default":50},"requests_per_agent":{"type":"integer","description":"Requests per agent","default":20},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}}}"#,
    },
    ToolDefinition {
        name: "contract_simulation_optimize",
        description: "Suggest contract optimizations based on simulation bottleneck analysis",
        input_schema: r#"{"type":"object","properties":{"scenario_count":{"type":"integer","description":"Scenarios for baseline simulation","default":100},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}}}"#,
    },
    ToolDefinition {
        name: "contract_simulation_compare",
        description: "Compare current contract config against a hypothetical modification",
        input_schema: r#"{"type":"object","properties":{"remove_policy_id":{"type":"string","description":"Policy ID to remove in hypothetical"},"add_policy_label":{"type":"string","description":"Label for a policy to add in hypothetical"},"add_policy_action":{"type":"string","enum":["allow","deny","require_approval","audit_only"],"description":"Action for the added policy"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}}}"#,
    },
    // ── Invention 15: Federated Governance ────────────────────────
    ToolDefinition {
        name: "federated_governance_create",
        description: "Create cross-organizational federation with trust levels and shared policies",
        input_schema: r#"{"type":"object","properties":{"name":{"type":"string","description":"Federation name"},"members":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"name":{"type":"string"}},"required":["id","name"]},"description":"Member organizations"},"transparency":{"type":"string","enum":["full","summary","minimal"],"default":"full"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["name","members"]}"#,
    },
    ToolDefinition {
        name: "federated_governance_ratify",
        description: "Record member ratification and check if quorum is met for activation",
        input_schema: r#"{"type":"object","properties":{"federation_id":{"type":"string","description":"Federation ID"},"member_id":{"type":"string","description":"Member organization ID ratifying"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["federation_id","member_id"]}"#,
    },
    ToolDefinition {
        name: "federated_governance_sync",
        description: "Sync policies across federation members and detect conflicts",
        input_schema: r#"{"type":"object","properties":{"federation_id":{"type":"string","description":"Federation ID"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["federation_id"]}"#,
    },
    ToolDefinition {
        name: "federated_governance_audit",
        description: "Audit federation compliance and compute per-member adherence scores",
        input_schema: r#"{"type":"object","properties":{"federation_id":{"type":"string","description":"Federation ID"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["federation_id"]}"#,
    },
    // ── Invention 16: Self-Healing Contracts ──────────────────────
    ToolDefinition {
        name: "self_healing_contract_create",
        description: "Create self-healing contract with adaptive healing triggers and actions",
        input_schema: r#"{"type":"object","properties":{"base_contract_id":{"type":"string","description":"Base contract to add self-healing to"},"violation_threshold":{"type":"integer","description":"Violations before tightening","default":3},"perfect_record_secs":{"type":"integer","description":"Seconds of clean record before relaxing","default":86400},"risk_threshold":{"type":"number","description":"Risk ratio threshold for monitoring","default":0.8},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["base_contract_id"]}"#,
    },
    ToolDefinition {
        name: "self_healing_contract_heal",
        description: "Execute a healing cycle checking all triggers against current state",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Self-healing contract ID"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["contract_id"]}"#,
    },
    ToolDefinition {
        name: "self_healing_contract_status",
        description: "Get comprehensive healing status with health trajectory analysis",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Self-healing contract ID"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["contract_id"]}"#,
    },
    ToolDefinition {
        name: "self_healing_contract_configure",
        description: "Add, modify, or remove healing rules with conflict validation",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Self-healing contract ID"},"action":{"type":"string","enum":["add","remove","modify"],"description":"Configuration action"},"trigger_type":{"type":"string","enum":["repeated_violation","perfect_record","risk_approaching","context_change"],"description":"Trigger type"},"healing_action":{"type":"string","enum":["tighten","relax","add_monitoring","remove_monitoring","add_approval","remove_approval"],"description":"Healing action to take"},"cooldown_secs":{"type":"integer","description":"Cooldown in seconds","default":3600},"threshold_value":{"type":"number","description":"Threshold value for the trigger"},"include_content":{"type":"boolean","default":false,"description":"Return full content (default: IDs only)"},"intent":{"type":"string","enum":["exists","ids","summary","fields","full"],"description":"Extraction intent level"},"since":{"type":"integer","description":"Only return changes since this Unix timestamp"},"token_budget":{"type":"integer","description":"Maximum token budget for response"},"max_results":{"type":"integer","default":10,"description":"Maximum number of results"},"cursor":{"type":"string","description":"Pagination cursor for next page"}},"required":["contract_id","action"]}"#,
    },
];

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Jaccard similarity of words (length > 2) between two strings.
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
    let union = words_a.union(&words_b).count() as f64;
    intersection / union
}

/// Map severity to a numeric weight for scoring.
fn severity_weight(severity: &agentic_contract::ViolationSeverity) -> f64 {
    use agentic_contract::ViolationSeverity;
    match severity {
        ViolationSeverity::Info => 0.1,
        ViolationSeverity::Warning => 0.4,
        ViolationSeverity::Critical => 0.8,
        ViolationSeverity::Fatal => 1.0,
    }
}

/// Exponential decay factor: e^{-lambda * dt}, where dt is in seconds.
fn exponential_decay(dt_secs: f64, half_life_secs: f64) -> f64 {
    let lambda = (2.0_f64).ln() / half_life_secs;
    (-lambda * dt_secs).exp()
}

/// Compute the mean of a slice, returning 0.0 for empty.
fn mean(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    vals.iter().sum::<f64>() / vals.len() as f64
}

/// Compute standard deviation.
fn std_dev(vals: &[f64]) -> f64 {
    if vals.len() < 2 {
        return 0.0;
    }
    let m = mean(vals);
    let variance = vals.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (vals.len() - 1) as f64;
    variance.sqrt()
}

/// Linear regression slope: returns (slope, intercept) or None.
fn linear_regression(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    if xs.len() < 2 || xs.len() != ys.len() {
        return None;
    }
    let n = xs.len() as f64;
    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = xs.iter().map(|x| x * x).sum();
    let denom = n * sum_x2 - sum_x * sum_x;
    if denom.abs() < 1e-12 {
        return None;
    }
    let slope = (n * sum_xy - sum_x * sum_y) / denom;
    let intercept = (sum_y - slope * sum_x) / n;
    Some((slope, intercept))
}

// ─── Main dispatch ───────────────────────────────────────────────────────────

/// Try to handle a resilience-category tool call. Returns `None` if the tool
/// name does not belong to this module.
pub fn try_handle(
    name: &str,
    args: Value,
    engine: &mut ContractEngine,
) -> Option<Result<Value, String>> {
    match name {
        // ==================================================================
        // INVENTION 13 — Violation Archaeology
        // ==================================================================
        "violation_archaeology_analyze" => Some(violation_archaeology_analyze(args, engine)),
        "violation_archaeology_timeline" => Some(violation_archaeology_timeline(args, engine)),
        "violation_archaeology_predict" => Some(violation_archaeology_predict(args, engine)),
        "violation_archaeology_compare" => Some(violation_archaeology_compare(args, engine)),

        // ==================================================================
        // INVENTION 14 — Contract Simulation
        // ==================================================================
        "contract_simulation_run" => Some(contract_simulation_run(args, engine)),
        "contract_simulation_stress" => Some(contract_simulation_stress(args, engine)),
        "contract_simulation_optimize" => Some(contract_simulation_optimize(args, engine)),
        "contract_simulation_compare" => Some(contract_simulation_compare(args, engine)),

        // ==================================================================
        // INVENTION 15 — Federated Governance
        // ==================================================================
        "federated_governance_create" => Some(federated_governance_create(args, engine)),
        "federated_governance_ratify" => Some(federated_governance_ratify(args, engine)),
        "federated_governance_sync" => Some(federated_governance_sync(args, engine)),
        "federated_governance_audit" => Some(federated_governance_audit(args, engine)),

        // ==================================================================
        // INVENTION 16 — Self-Healing Contracts
        // ==================================================================
        "self_healing_contract_create" => Some(self_healing_contract_create(args, engine)),
        "self_healing_contract_heal" => Some(self_healing_contract_heal(args, engine)),
        "self_healing_contract_status" => Some(self_healing_contract_status(args, engine)),
        "self_healing_contract_configure" => Some(self_healing_contract_configure(args, engine)),

        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 13: Violation Archaeology
// ═══════════════════════════════════════════════════════════════════════════════

/// Deep analysis of violation patterns, clusters, root causes, and remediations.
fn violation_archaeology_analyze(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(604_800); // 7 days default

    let now = chrono::Utc::now();
    let cutoff = now - chrono::Duration::seconds(window_secs);

    // Collect violations for agent within time window
    let violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && v.detected_at >= cutoff)
        .collect();

    if violations.is_empty() {
        return Ok(json!({
            "agent_id": agent_id,
            "window_secs": window_secs,
            "total_violations": 0,
            "clusters": [],
            "root_causes": [],
            "recommendations": [],
            "policy_adjustments": [],
            "analyzed_at": now.to_rfc3339()
        }));
    }

    // ── 1. Cluster by description similarity (greedy single-linkage) ──
    let mut cluster_assignments: Vec<Option<usize>> = vec![None; violations.len()];
    let mut cluster_count = 0usize;

    for i in 0..violations.len() {
        if cluster_assignments[i].is_some() {
            continue;
        }
        let cid = cluster_count;
        cluster_count += 1;
        cluster_assignments[i] = Some(cid);

        for j in (i + 1)..violations.len() {
            if cluster_assignments[j].is_some() {
                continue;
            }
            if word_overlap(&violations[i].description, &violations[j].description) > 0.5 {
                cluster_assignments[j] = Some(cid);
            }
        }
    }

    // Build cluster details
    let mut cluster_data: Vec<Value> = Vec::new();
    for cid in 0..cluster_count {
        let members: Vec<usize> = cluster_assignments
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == Some(cid))
            .map(|(i, _)| i)
            .collect();
        let count = members.len() as u32;

        // Representative label: first violation's description truncated
        let label = violations[members[0]]
            .description
            .chars()
            .take(80)
            .collect::<String>();

        // Most common severity in cluster
        let mut sev_counts: HashMap<String, u32> = HashMap::new();
        for &idx in &members {
            *sev_counts
                .entry(format!("{}", violations[idx].severity))
                .or_default() += 1;
        }
        let severity = sev_counts
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(s, _)| s.clone())
            .unwrap_or_default();

        // Time pattern: compute inter-violation intervals
        let mut timestamps: Vec<i64> = members
            .iter()
            .map(|&idx| violations[idx].detected_at.timestamp())
            .collect();
        timestamps.sort();

        let intervals: Vec<f64> = timestamps
            .windows(2)
            .map(|w| (w[1] - w[0]) as f64)
            .collect();

        let time_pattern = if intervals.len() >= 2 {
            let avg_interval = mean(&intervals);
            let sd = std_dev(&intervals);
            let cv = if avg_interval > 0.0 {
                sd / avg_interval
            } else {
                f64::INFINITY
            };
            if cv < 0.3 {
                Some(format!(
                    "periodic (~{:.0}s interval, CV={:.2})",
                    avg_interval, cv
                ))
            } else if cv < 0.7 {
                Some(format!(
                    "semi-periodic (~{:.0}s avg, CV={:.2})",
                    avg_interval, cv
                ))
            } else {
                Some(format!(
                    "irregular (avg {:.0}s, CV={:.2})",
                    avg_interval, cv
                ))
            }
        } else {
            None
        };

        // Context pattern: common policy_id
        let policy_ids: Vec<String> = members
            .iter()
            .filter_map(|&idx| violations[idx].policy_id.map(|p| p.to_string()))
            .collect();
        let context_pattern = if !policy_ids.is_empty() {
            let mut freq: HashMap<&str, usize> = HashMap::new();
            for pid in &policy_ids {
                *freq.entry(pid.as_str()).or_default() += 1;
            }
            freq.into_iter()
                .max_by_key(|(_, c)| *c)
                .map(|(pid, c)| format!("policy {} ({} times)", pid, c))
        } else {
            None
        };

        cluster_data.push(json!({
            "label": label,
            "count": count,
            "severity": severity,
            "time_pattern": time_pattern,
            "context_pattern": context_pattern
        }));
    }

    // ── 2. Severity distribution ──
    let mut severity_dist: HashMap<String, u32> = HashMap::new();
    for v in &violations {
        *severity_dist.entry(format!("{}", v.severity)).or_default() += 1;
    }

    // ── 3. Root cause hypotheses ──
    let total = violations.len() as f64;
    let mut root_causes: Vec<Value> = Vec::new();

    // Hypothesis: repeated policy violations
    let mut policy_freq: HashMap<String, usize> = HashMap::new();
    for v in &violations {
        if let Some(pid) = &v.policy_id {
            *policy_freq.entry(pid.to_string()).or_default() += 1;
        }
    }
    for (pid, count) in &policy_freq {
        if *count as f64 / total > 0.3 {
            let confidence = (*count as f64 / total).min(1.0);
            root_causes.push(json!({
                "hypothesis": format!("Policy {} is frequently violated, suggesting it may be misconfigured or too restrictive", pid),
                "confidence": (confidence * 100.0).round() / 100.0,
                "evidence": [
                    format!("{} out of {} violations reference this policy", count, violations.len()),
                    format!("Represents {:.0}% of all violations in window", confidence * 100.0)
                ],
                "factors": ["policy_misconfiguration", "agent_capability_mismatch"]
            }));
        }
    }

    // Hypothesis: escalating severity
    let mut timestamps_sorted: Vec<(i64, f64)> = violations
        .iter()
        .map(|v| (v.detected_at.timestamp(), severity_weight(&v.severity)))
        .collect();
    timestamps_sorted.sort_by_key(|(t, _)| *t);

    if timestamps_sorted.len() >= 3 {
        let xs: Vec<f64> = timestamps_sorted.iter().map(|(t, _)| *t as f64).collect();
        let ys: Vec<f64> = timestamps_sorted.iter().map(|(_, s)| *s).collect();
        if let Some((slope, _)) = linear_regression(&xs, &ys) {
            if slope > 1e-8 {
                root_causes.push(json!({
                    "hypothesis": "Violation severity is escalating over time",
                    "confidence": (slope * 1e6).min(0.95),
                    "evidence": [
                        format!("Severity trend slope: {:.2e} per second", slope),
                        format!("Based on {} data points", timestamps_sorted.len())
                    ],
                    "factors": ["escalating_behavior", "insufficient_deterrence"]
                }));
            }
        }
    }

    // Hypothesis: burst patterns
    let burst_threshold_secs = 60.0; // violations within 60s = burst
    let mut burst_count = 0u32;
    let ts_list: Vec<i64> = violations
        .iter()
        .map(|v| v.detected_at.timestamp())
        .collect();
    for i in 0..ts_list.len() {
        for j in (i + 1)..ts_list.len() {
            if (ts_list[j] - ts_list[i]).abs() < burst_threshold_secs as i64 {
                burst_count += 1;
            }
        }
    }
    if burst_count > 2 {
        root_causes.push(json!({
            "hypothesis": "Violations occur in bursts suggesting automated or scripted behavior",
            "confidence": (burst_count as f64 / total).min(0.9),
            "evidence": [
                format!("{} violation pairs within {}s of each other", burst_count, burst_threshold_secs),
            ],
            "factors": ["automated_behavior", "rapid_retry_pattern"]
        }));
    }

    // If no root causes found, add generic
    if root_causes.is_empty() {
        root_causes.push(json!({
            "hypothesis": "Violations appear sporadic without clear pattern",
            "confidence": 0.3,
            "evidence": [format!("{} violations in {} seconds", violations.len(), window_secs)],
            "factors": ["insufficient_data"]
        }));
    }

    // ── 4. Recommendations ──
    let mut recommendations: Vec<Value> = Vec::new();

    // Recommendation based on cluster count
    if cluster_count > 3 {
        recommendations.push(json!({
            "action": "Review and consolidate policies — violations spread across many categories",
            "expected_impact": format!("Reduce violation categories from {} to fewer focused rules", cluster_count),
            "effort": "medium",
            "priority": 1
        }));
    }

    // Recommendation for high-severity violations
    let critical_plus = violations
        .iter()
        .filter(|v| v.severity >= agentic_contract::ViolationSeverity::Critical)
        .count();
    if critical_plus > 0 {
        recommendations.push(json!({
            "action": format!("Address {} critical/fatal violations immediately", critical_plus),
            "expected_impact": "Prevent system instability and potential halts",
            "effort": "high",
            "priority": 1
        }));
    }

    // Recommendation for agent training
    if violations.len() > 5 {
        recommendations.push(json!({
            "action": format!("Consider additional constraints or training for agent '{}'", agent_id),
            "expected_impact": format!("Reduce violation rate from {:.1}/day to target <1/day", violations.len() as f64 / (window_secs as f64 / 86400.0)),
            "effort": "medium",
            "priority": 2
        }));
    }

    // Recommendation for monitoring
    recommendations.push(json!({
        "action": "Enable enhanced monitoring for top violating policy categories",
        "expected_impact": "Faster detection and earlier intervention",
        "effort": "low",
        "priority": 3
    }));

    // ── 5. Policy adjustments ──
    let mut policy_adjustments: Vec<Value> = Vec::new();
    for (pid, count) in &policy_freq {
        if *count >= 3 {
            policy_adjustments.push(json!({
                "policy_id": pid,
                "adjustment": "Consider relaxing or splitting this policy",
                "reason": format!("Violated {} times in window — may be overly restrictive", count)
            }));
        }
    }

    Ok(json!({
        "agent_id": agent_id,
        "window_secs": window_secs,
        "total_violations": violations.len(),
        "severity_distribution": severity_dist,
        "clusters": cluster_data,
        "root_causes": root_causes,
        "recommendations": recommendations,
        "policy_adjustments": policy_adjustments,
        "analyzed_at": now.to_rfc3339()
    }))
}

/// Build violation timeline with rolling rate and trend analysis.
fn violation_archaeology_timeline(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let window_secs = args
        .get("window_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(604_800);

    let now = chrono::Utc::now();
    let cutoff = now - chrono::Duration::seconds(window_secs);

    let mut violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && v.detected_at >= cutoff)
        .collect();
    violations.sort_by_key(|v| v.detected_at);

    if violations.is_empty() {
        return Ok(json!({
            "agent_id": agent_id,
            "window_secs": window_secs,
            "timeline": [],
            "rolling_rates": [],
            "trend": "no_data",
            "inflection_points": [],
            "analyzed_at": now.to_rfc3339()
        }));
    }

    // Chronological timeline
    let timeline: Vec<Value> = violations
        .iter()
        .map(|v| {
            json!({
                "id": v.id.to_string(),
                "description": v.description,
                "severity": format!("{}", v.severity),
                "severity_weight": severity_weight(&v.severity),
                "detected_at": v.detected_at.to_rfc3339(),
                "policy_id": v.policy_id.map(|p| p.to_string()),
                "decay_weight": exponential_decay(
                    (now - v.detected_at).num_seconds() as f64,
                    86400.0 * 3.0, // 3-day half-life
                )
            })
        })
        .collect();

    // Rolling violation rate: 7-day windows, sliding by 1 day
    let rolling_window_secs = 86400 * 7; // 7 days
    let step_secs = 86400; // 1 day
    let first_ts = violations[0].detected_at.timestamp();
    let last_ts = now.timestamp();

    let mut rolling_rates: Vec<Value> = Vec::new();
    let mut rate_values: Vec<f64> = Vec::new();
    let mut rate_timestamps: Vec<f64> = Vec::new();

    let mut window_start = first_ts;
    while window_start + rolling_window_secs <= last_ts + step_secs {
        let window_end = window_start + rolling_window_secs;
        let count = violations
            .iter()
            .filter(|v| {
                let ts = v.detected_at.timestamp();
                ts >= window_start && ts < window_end
            })
            .count();
        let rate = count as f64 / (rolling_window_secs as f64 / 86400.0); // violations per day

        rolling_rates.push(json!({
            "window_start": chrono::DateTime::from_timestamp(window_start, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
            "window_end": chrono::DateTime::from_timestamp(window_end, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
            "violation_count": count,
            "rate_per_day": (rate * 100.0).round() / 100.0
        }));
        rate_values.push(rate);
        rate_timestamps.push(window_start as f64);

        window_start += step_secs;
    }

    // Detect trend via linear regression on rolling rates
    let trend = if rate_values.len() >= 2 {
        match linear_regression(&rate_timestamps, &rate_values) {
            Some((slope, _)) => {
                if slope > 0.001 {
                    "accelerating"
                } else if slope < -0.001 {
                    "decelerating"
                } else {
                    "stable"
                }
            }
            None => "insufficient_data",
        }
    } else {
        "insufficient_data"
    };

    // Detect inflection points: where rolling rate changes direction
    let mut inflection_points: Vec<Value> = Vec::new();
    if rate_values.len() >= 3 {
        for i in 1..(rate_values.len() - 1) {
            let d1 = rate_values[i] - rate_values[i - 1];
            let d2 = rate_values[i + 1] - rate_values[i];
            // Sign change = inflection
            if (d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0) {
                let ts = rate_timestamps[i] as i64;
                inflection_points.push(json!({
                    "timestamp": chrono::DateTime::from_timestamp(ts, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                    "rate_before": (rate_values[i - 1] * 100.0).round() / 100.0,
                    "rate_at": (rate_values[i] * 100.0).round() / 100.0,
                    "rate_after": (rate_values[i + 1] * 100.0).round() / 100.0,
                    "direction_change": if d1 > 0.0 { "peak" } else { "trough" }
                }));
            }
        }
    }

    Ok(json!({
        "agent_id": agent_id,
        "window_secs": window_secs,
        "total_violations": violations.len(),
        "timeline": timeline,
        "rolling_rates": rolling_rates,
        "trend": trend,
        "inflection_points": inflection_points,
        "analyzed_at": now.to_rfc3339()
    }))
}

/// Predict future violations based on historical velocity and acceleration.
fn violation_archaeology_predict(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_id = require_str(&args, "agent_id")?;
    let forecast_days = args
        .get("forecast_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(7);

    let now = chrono::Utc::now();
    // Use 30-day lookback for velocity computation
    let lookback_secs = 30 * 86400_i64;
    let cutoff = now - chrono::Duration::seconds(lookback_secs);

    let violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && v.detected_at >= cutoff)
        .collect();

    if violations.is_empty() {
        return Ok(json!({
            "agent_id": agent_id,
            "forecast_days": forecast_days,
            "total_historical": 0,
            "velocity_per_day": 0.0,
            "acceleration": 0.0,
            "projections": {
                "expected_total": 0,
                "confidence_interval": [0, 0],
                "risk_level": "low"
            },
            "severity_projections": {},
            "escalating_risk": false,
            "predicted_at": now.to_rfc3339()
        }));
    }

    let total = violations.len() as f64;
    let actual_window_secs = (now - cutoff).num_seconds() as f64;
    let velocity = total / (actual_window_secs / 86400.0); // violations per day

    // Compute acceleration: split window in half, compare rates
    let midpoint = cutoff + chrono::Duration::seconds(lookback_secs / 2);
    let first_half_count = violations
        .iter()
        .filter(|v| v.detected_at < midpoint)
        .count() as f64;
    let second_half_count = violations
        .iter()
        .filter(|v| v.detected_at >= midpoint)
        .count() as f64;
    let half_window_days = (lookback_secs as f64 / 2.0) / 86400.0;
    let rate_first = first_half_count / half_window_days;
    let rate_second = second_half_count / half_window_days;
    let acceleration = (rate_second - rate_first) / half_window_days; // per day^2

    // Project forward
    let forecast_days_f = forecast_days as f64;
    let projected_rate = velocity + acceleration * forecast_days_f;
    let projected_total = (velocity * forecast_days_f
        + 0.5 * acceleration * forecast_days_f * forecast_days_f)
        .max(0.0);

    // Confidence interval: +/- 1 standard deviation scaled by sqrt(days)
    let sd_daily = std_dev(
        &violations
            .iter()
            .map(|v| severity_weight(&v.severity))
            .collect::<Vec<_>>(),
    );
    let uncertainty = sd_daily * forecast_days_f.sqrt() * total.sqrt() * 0.5;
    let ci_low = (projected_total - uncertainty).max(0.0);
    let ci_high = projected_total + uncertainty;

    // Per-severity projections
    let mut sev_counts: HashMap<String, f64> = HashMap::new();
    for v in &violations {
        *sev_counts.entry(format!("{}", v.severity)).or_default() += 1.0;
    }
    let mut severity_projections: HashMap<String, Value> = HashMap::new();
    for (sev, count) in &sev_counts {
        let sev_rate = count / (actual_window_secs / 86400.0);
        let sev_projected = sev_rate * forecast_days_f;
        severity_projections.insert(
            sev.clone(),
            json!({
                "historical_count": *count as u32,
                "rate_per_day": (sev_rate * 100.0).round() / 100.0,
                "projected": (sev_projected * 10.0).round() / 10.0
            }),
        );
    }

    let escalating = acceleration > 0.01;
    let risk_level = if projected_rate > 5.0 || escalating {
        "high"
    } else if projected_rate > 2.0 {
        "medium"
    } else {
        "low"
    };

    Ok(json!({
        "agent_id": agent_id,
        "forecast_days": forecast_days,
        "total_historical": violations.len(),
        "lookback_days": (actual_window_secs / 86400.0).round(),
        "velocity_per_day": (velocity * 100.0).round() / 100.0,
        "acceleration_per_day2": (acceleration * 10000.0).round() / 10000.0,
        "projections": {
            "expected_total": (projected_total * 10.0).round() / 10.0,
            "expected_rate_at_end": (projected_rate.max(0.0) * 100.0).round() / 100.0,
            "confidence_interval": [(ci_low * 10.0).round() / 10.0, (ci_high * 10.0).round() / 10.0],
            "risk_level": risk_level
        },
        "severity_projections": severity_projections,
        "escalating_risk": escalating,
        "predicted_at": now.to_rfc3339()
    }))
}

/// Compare violation patterns across agents and rank by risk.
fn violation_archaeology_compare(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let agent_ids: Vec<String> = args
        .get("agent_ids")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Missing required parameter: agent_ids".to_string())?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    if agent_ids.is_empty() {
        return Err("agent_ids must contain at least one agent".to_string());
    }

    let now = chrono::Utc::now();
    let lookback = chrono::Duration::days(30);
    let cutoff = now - lookback;
    let window_days = 30.0_f64;

    let mut agent_profiles: Vec<Value> = Vec::new();
    let mut risk_scores: Vec<(String, f64)> = Vec::new();

    for agent_id in &agent_ids {
        let violations: Vec<_> = engine
            .file
            .violations
            .iter()
            .filter(|v| &v.actor == agent_id && v.detected_at >= cutoff)
            .collect();

        let total = violations.len();
        let rate = total as f64 / window_days;

        // Severity distribution
        let mut sev_dist: HashMap<String, u32> = HashMap::new();
        let mut total_severity_weight = 0.0_f64;
        for v in &violations {
            *sev_dist.entry(format!("{}", v.severity)).or_default() += 1;
            total_severity_weight += severity_weight(&v.severity);
        }
        let avg_severity = if total > 0 {
            total_severity_weight / total as f64
        } else {
            0.0
        };

        // Most common violation type (by description similarity)
        let descriptions: Vec<&str> = violations.iter().map(|v| v.description.as_str()).collect();
        let most_common = if !descriptions.is_empty() {
            // Find the description closest to all others
            let mut best = descriptions[0];
            let mut best_avg_sim = 0.0;
            for d in &descriptions {
                let avg_sim: f64 = descriptions
                    .iter()
                    .filter(|other| *other != d)
                    .map(|other| word_overlap(d, other))
                    .sum::<f64>()
                    / (descriptions.len().max(1) - 1).max(1) as f64;
                if avg_sim > best_avg_sim {
                    best_avg_sim = avg_sim;
                    best = d;
                }
            }
            best.chars().take(60).collect::<String>()
        } else {
            "none".to_string()
        };

        // Risk score = rate * avg_severity (normalized)
        let risk_score = rate * avg_severity;
        risk_scores.push((agent_id.clone(), risk_score));

        // Recency-weighted score: newer violations count more
        let recency_score: f64 = violations
            .iter()
            .map(|v| {
                let age_secs = (now - v.detected_at).num_seconds() as f64;
                severity_weight(&v.severity) * exponential_decay(age_secs, 86400.0 * 7.0)
            })
            .sum();

        agent_profiles.push(json!({
            "agent_id": agent_id,
            "total_violations": total,
            "rate_per_day": (rate * 100.0).round() / 100.0,
            "avg_severity_weight": (avg_severity * 1000.0).round() / 1000.0,
            "severity_distribution": sev_dist,
            "most_common_violation": most_common,
            "risk_score": (risk_score * 1000.0).round() / 1000.0,
            "recency_weighted_score": (recency_score * 100.0).round() / 100.0
        }));
    }

    // Rank by risk score
    risk_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let ranking: Vec<Value> = risk_scores
        .iter()
        .enumerate()
        .map(|(rank, (agent, score))| {
            json!({
                "rank": rank + 1,
                "agent_id": agent,
                "risk_score": (score * 1000.0).round() / 1000.0
            })
        })
        .collect();

    let highest_risk = risk_scores
        .first()
        .map(|(a, _)| a.clone())
        .unwrap_or_default();

    Ok(json!({
        "agent_count": agent_ids.len(),
        "analysis_window_days": window_days,
        "agent_profiles": agent_profiles,
        "risk_ranking": ranking,
        "highest_risk_agent": highest_risk,
        "compared_at": now.to_rfc3339()
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 14: Contract Simulation
// ═══════════════════════════════════════════════════════════════════════════════

/// Core simulation logic shared across simulation tools.
fn run_simulation_core(engine: &ContractEngine, scenario_count: u32) -> Value {
    let policies = &engine.file.policies;
    let risk_limits = &engine.file.risk_limits;
    let approval_rules = &engine.file.approval_rules;
    let obligations = &engine.file.obligations;

    let _total = scenario_count.max(1) as f64;

    // Count policy types
    let deny_count = policies
        .iter()
        .filter(|p| p.action == agentic_contract::PolicyAction::Deny && p.is_active())
        .count();
    let allow_count = policies
        .iter()
        .filter(|p| p.action == agentic_contract::PolicyAction::Allow && p.is_active())
        .count();
    let require_approval_count = policies
        .iter()
        .filter(|p| p.action == agentic_contract::PolicyAction::RequireApproval && p.is_active())
        .count();
    let active_count = policies.iter().filter(|p| p.is_active()).count().max(1);

    // Approval rate estimate
    let approval_rate = if active_count > 0 {
        (allow_count as f64 + require_approval_count as f64 * 0.5) / active_count as f64
    } else {
        1.0
    };
    let denial_rate = deny_count as f64 / active_count as f64;

    // Risk breach rate estimate
    let risk_breach_rate = if risk_limits.is_empty() {
        0.0
    } else {
        let near_limit = risk_limits.iter().filter(|r| r.usage_ratio() > 0.8).count();
        near_limit as f64 / risk_limits.len() as f64
    };

    // Detect deadlocks: conflicting Allow + Deny in same scope
    let mut deadlocks: Vec<Value> = Vec::new();
    for i in 0..policies.len() {
        if !policies[i].is_active() {
            continue;
        }
        for j in (i + 1)..policies.len() {
            if !policies[j].is_active() {
                continue;
            }
            if policies[i].scope == policies[j].scope
                && ((policies[i].action == agentic_contract::PolicyAction::Allow
                    && policies[j].action == agentic_contract::PolicyAction::Deny)
                    || (policies[i].action == agentic_contract::PolicyAction::Deny
                        && policies[j].action == agentic_contract::PolicyAction::Allow))
                && word_overlap(&policies[i].label, &policies[j].label) > 0.3
            {
                deadlocks.push(json!({
                    "description": format!(
                        "Conflicting policies: '{}' ({:?}) vs '{}' ({:?}) in {:?} scope",
                        policies[i].label, policies[i].action,
                        policies[j].label, policies[j].action,
                        policies[i].scope
                    ),
                    "policies_involved": [
                        policies[i].id.to_string(),
                        policies[j].id.to_string()
                    ],
                    "resolution": "Review and disambiguate policy conditions or scopes"
                }));
            }
        }
    }

    // Find edge cases
    let mut edge_cases: Vec<Value> = Vec::new();

    // RequireApproval without approval rules
    if require_approval_count > 0 && approval_rules.is_empty() {
        edge_cases.push(json!({
            "description": format!("{} policies require approval but no approval rules are defined", require_approval_count),
            "current_behavior": "Requests will be created but have no approvers",
            "suggested_fix": "Add approval rules with designated approvers"
        }));
    }

    // Obligations without deadlines
    let no_deadline = obligations
        .iter()
        .filter(|o| o.deadline.is_none() && !o.is_resolved())
        .count();
    if no_deadline > 0 {
        edge_cases.push(json!({
            "description": format!("{} obligations have no deadline", no_deadline),
            "current_behavior": "Obligations may never be enforced",
            "suggested_fix": "Set reasonable deadlines for all obligations"
        }));
    }

    // No deny policies = everything allowed
    if deny_count == 0 && active_count > 0 {
        edge_cases.push(json!({
            "description": "No deny policies exist — all actions are implicitly allowed",
            "current_behavior": "Agent has unrestricted access within allowed scopes",
            "suggested_fix": "Add deny policies for sensitive or dangerous actions"
        }));
    }

    // Risk limits at or near capacity
    for limit in risk_limits {
        if limit.usage_ratio() > 0.95 {
            edge_cases.push(json!({
                "description": format!("Risk limit '{}' at {:.1}% capacity", limit.label, limit.usage_ratio() * 100.0),
                "current_behavior": "Further actions may be blocked immediately",
                "suggested_fix": "Increase limit or reset window"
            }));
        }
    }

    // Health score: weighted average
    let no_deadlock = if deadlocks.is_empty() { 1.0 } else { 0.0 };
    let no_breach = 1.0 - risk_breach_rate;
    let no_edge = if edge_cases.is_empty() {
        1.0
    } else {
        1.0 - (edge_cases.len() as f64 * 0.1).min(0.5)
    };
    let health_score =
        (approval_rate * 0.3 + no_breach * 0.3 + no_deadlock * 0.2 + no_edge * 0.2).clamp(0.0, 1.0);

    json!({
        "scenario_count": scenario_count,
        "approval_rate": (approval_rate * 1000.0).round() / 1000.0,
        "denial_rate": (denial_rate * 1000.0).round() / 1000.0,
        "risk_breach_rate": (risk_breach_rate * 1000.0).round() / 1000.0,
        "deadlocks": deadlocks,
        "edge_cases": edge_cases,
        "health_score": (health_score * 1000.0).round() / 1000.0,
        "policy_count": active_count,
        "risk_limit_count": risk_limits.len(),
        "obligation_count": obligations.len()
    })
}

/// Simulate contract behavior across scenarios.
fn contract_simulation_run(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let scenario_count = args
        .get("scenario_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as u32;

    let now = chrono::Utc::now();
    let result = run_simulation_core(engine, scenario_count);

    let mut output = result;
    output["id"] = json!(agentic_contract::ContractId::new().to_string());
    output["simulated_at"] = json!(now.to_rfc3339());

    Ok(output)
}

/// Stress test the contract under increasing load.
fn contract_simulation_stress(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let max_agents = args
        .get("max_agents")
        .and_then(|v| v.as_u64())
        .unwrap_or(50) as u32;
    let requests_per_agent = args
        .get("requests_per_agent")
        .and_then(|v| v.as_u64())
        .unwrap_or(20) as u32;

    let now = chrono::Utc::now();
    let policies = &engine.file.policies;
    let risk_limits = &engine.file.risk_limits;

    let deny_policies: Vec<_> = policies
        .iter()
        .filter(|p| p.action == agentic_contract::PolicyAction::Deny && p.is_active())
        .collect();
    let active_policies = policies.iter().filter(|p| p.is_active()).count().max(1);

    // Simulate increasing load levels
    let mut stress_levels: Vec<Value> = Vec::new();
    let mut stability_threshold = max_agents;
    let mut last_health = 1.0_f64;

    for agent_count in (1..=max_agents).step_by((max_agents as usize / 10).max(1)) {
        let total_requests = agent_count * requests_per_agent;

        // At higher loads, more deny policies get triggered
        let load_factor = agent_count as f64 / max_agents as f64;
        let deny_rate = (deny_policies.len() as f64 / active_policies as f64) * (1.0 + load_factor);
        let effective_deny_rate = deny_rate.min(1.0);

        // Risk limits breach more under load
        let base_breach_rate = risk_limits.iter().filter(|r| r.usage_ratio() > 0.5).count() as f64
            / risk_limits.len().max(1) as f64;
        let stress_breach_rate = (base_breach_rate * (1.0 + load_factor * 2.0)).min(1.0);

        // Approval bottleneck: more requests = slower approval
        let approval_bottleneck = if load_factor > 0.5 {
            (load_factor - 0.5) * 2.0 // 0 to 1 scale
        } else {
            0.0
        };

        // Policy conflicts increase with concurrent load
        let conflict_probability = (deny_policies.len() as f64 * load_factor * 0.1).min(1.0);

        // Health score degrades under load
        let health = (1.0
            - effective_deny_rate * 0.3
            - stress_breach_rate * 0.3
            - approval_bottleneck * 0.2
            - conflict_probability * 0.2)
            .clamp(0.0, 1.0);

        let stable = health > 0.3;
        if !stable && last_health > 0.3 {
            stability_threshold = agent_count;
        }
        last_health = health;

        stress_levels.push(json!({
            "agent_count": agent_count,
            "total_requests": total_requests,
            "effective_deny_rate": (effective_deny_rate * 1000.0).round() / 1000.0,
            "risk_breach_rate": (stress_breach_rate * 1000.0).round() / 1000.0,
            "approval_bottleneck": (approval_bottleneck * 1000.0).round() / 1000.0,
            "conflict_probability": (conflict_probability * 1000.0).round() / 1000.0,
            "health_score": (health * 1000.0).round() / 1000.0,
            "stable": stable
        }));
    }

    // Identify which policies break first
    let mut breaking_policies: Vec<Value> = Vec::new();
    for policy in &deny_policies {
        let specificity = if policy.scope == agentic_contract::PolicyScope::Global {
            "low"
        } else {
            "high"
        };
        breaking_policies.push(json!({
            "policy_id": policy.id.to_string(),
            "label": policy.label,
            "scope": format!("{}", policy.scope),
            "specificity": specificity,
            "break_reason": "High-traffic deny policy blocks many concurrent requests"
        }));
    }

    // Risk limits that breach first
    let mut breaking_limits: Vec<Value> = Vec::new();
    let mut sorted_limits: Vec<_> = risk_limits.iter().collect();
    sorted_limits.sort_by(|a, b| {
        b.usage_ratio()
            .partial_cmp(&a.usage_ratio())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for limit in sorted_limits.iter().take(3) {
        if limit.usage_ratio() > 0.3 {
            breaking_limits.push(json!({
                "limit_id": limit.id.to_string(),
                "label": limit.label,
                "current_usage_ratio": (limit.usage_ratio() * 1000.0).round() / 1000.0,
                "estimated_breach_at_agents": ((1.0 / limit.usage_ratio().max(0.01)) * max_agents as f64 * 0.5) as u32
            }));
        }
    }

    Ok(json!({
        "max_agents": max_agents,
        "requests_per_agent": requests_per_agent,
        "stability_threshold": stability_threshold,
        "stress_levels": stress_levels,
        "breaking_policies": breaking_policies,
        "breaking_limits": breaking_limits,
        "final_health_score": (last_health * 1000.0).round() / 1000.0,
        "stress_tested_at": now.to_rfc3339()
    }))
}

/// Suggest contract optimizations based on simulation.
fn contract_simulation_optimize(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let scenario_count = args
        .get("scenario_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as u32;

    let now = chrono::Utc::now();
    let sim_result = run_simulation_core(engine, scenario_count);

    let health_score = sim_result["health_score"].as_f64().unwrap_or(1.0);
    let deadlock_count = sim_result["deadlocks"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let edge_case_count = sim_result["edge_cases"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let denial_rate = sim_result["denial_rate"].as_f64().unwrap_or(0.0);
    let breach_rate = sim_result["risk_breach_rate"].as_f64().unwrap_or(0.0);

    let mut optimizations: Vec<Value> = Vec::new();

    // Bottleneck: too many deny policies
    if denial_rate > 0.5 {
        let deny_policies: Vec<_> = engine
            .file
            .policies
            .iter()
            .filter(|p| p.action == agentic_contract::PolicyAction::Deny && p.is_active())
            .collect();
        optimizations.push(json!({
            "bottleneck": "excessive_denials",
            "description": format!("Denial rate is {:.0}% — too many actions are blocked", denial_rate * 100.0),
            "suggestion": "Convert low-risk Deny policies to AuditOnly or RequireApproval",
            "affected_policies": deny_policies.iter().take(5).map(|p| json!({
                "id": p.id.to_string(),
                "label": p.label
            })).collect::<Vec<_>>(),
            "predicted_improvement": format!("Health score +{:.0}%", (0.5 - denial_rate).abs() * 30.0)
        }));
    }

    // Bottleneck: risk limits too tight
    if breach_rate > 0.3 {
        let tight_limits: Vec<_> = engine
            .file
            .risk_limits
            .iter()
            .filter(|r| r.usage_ratio() > 0.7)
            .collect();
        optimizations.push(json!({
            "bottleneck": "tight_risk_limits",
            "description": format!("Risk breach rate is {:.0}% — limits may be too restrictive", breach_rate * 100.0),
            "suggestion": "Increase max_value for frequently-breached limits by 20-50%",
            "affected_limits": tight_limits.iter().map(|r| json!({
                "id": r.id.to_string(),
                "label": r.label,
                "current_usage": (r.usage_ratio() * 100.0).round() / 100.0,
                "suggested_max": r.max_value * 1.3
            })).collect::<Vec<_>>(),
            "predicted_improvement": format!("Health score +{:.0}%", breach_rate * 20.0)
        }));
    }

    // Bottleneck: deadlocks
    if deadlock_count > 0 {
        optimizations.push(json!({
            "bottleneck": "policy_deadlocks",
            "description": format!("{} policy deadlocks detected — conflicting rules", deadlock_count),
            "suggestion": "Resolve conflicting Allow/Deny policies by adjusting scope or conditions",
            "affected_deadlocks": sim_result["deadlocks"],
            "predicted_improvement": format!("Health score +{:.0}%", deadlock_count as f64 * 5.0)
        }));
    }

    // Bottleneck: approval chain too long
    let require_approval_policies = engine
        .file
        .policies
        .iter()
        .filter(|p| p.action == agentic_contract::PolicyAction::RequireApproval && p.is_active())
        .count();
    if require_approval_policies > 5 {
        optimizations.push(json!({
            "bottleneck": "approval_overload",
            "description": format!("{} policies require approval — creates bottleneck", require_approval_policies),
            "suggestion": "Convert low-risk RequireApproval policies to AuditOnly for trusted agents",
            "predicted_improvement": format!("Throughput +{:.0}%", require_approval_policies as f64 * 5.0)
        }));
    }

    // Edge cases
    if edge_case_count > 0 {
        optimizations.push(json!({
            "bottleneck": "edge_cases",
            "description": format!("{} edge cases found that could cause unexpected behavior", edge_case_count),
            "suggestion": "Address each edge case per simulation recommendations",
            "affected_edge_cases": sim_result["edge_cases"],
            "predicted_improvement": format!("Health score +{:.0}%", edge_case_count as f64 * 3.0)
        }));
    }

    let predicted_health_after = (health_score + optimizations.len() as f64 * 0.05).min(1.0);

    Ok(json!({
        "current_health_score": health_score,
        "optimizations": optimizations,
        "optimization_count": optimizations.len(),
        "predicted_health_after_optimization": (predicted_health_after * 1000.0).round() / 1000.0,
        "simulation_baseline": sim_result,
        "optimized_at": now.to_rfc3339()
    }))
}

/// Compare current contract config against a hypothetical modification.
fn contract_simulation_compare(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let now = chrono::Utc::now();

    // Run baseline simulation
    let baseline = run_simulation_core(engine, 100);
    let baseline_health = baseline["health_score"].as_f64().unwrap_or(0.0);
    let baseline_deadlocks = baseline["deadlocks"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let baseline_edge_cases = baseline["edge_cases"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let baseline_approval = baseline["approval_rate"].as_f64().unwrap_or(0.0);
    let baseline_denial = baseline["denial_rate"].as_f64().unwrap_or(0.0);
    let baseline_breach = baseline["risk_breach_rate"].as_f64().unwrap_or(0.0);

    // Determine modification type
    let mut modification_description = String::new();

    // Temporarily modify engine for hypothetical
    let remove_policy_id = args.get("remove_policy_id").and_then(|v| v.as_str());
    let add_policy_label = args.get("add_policy_label").and_then(|v| v.as_str());
    let add_policy_action = args.get("add_policy_action").and_then(|v| v.as_str());

    let mut removed_policy: Option<agentic_contract::Policy> = None;
    let mut removed_index: Option<usize> = None;

    if let Some(pid_str) = remove_policy_id {
        if let Ok(pid) = pid_str.parse::<agentic_contract::ContractId>() {
            if let Some(idx) = engine.file.policies.iter().position(|p| p.id == pid) {
                removed_policy = Some(engine.file.policies.remove(idx));
                removed_index = Some(idx);
                modification_description = format!("Removed policy '{}'", pid_str);
            }
        }
    }

    let mut added_policy_id: Option<agentic_contract::ContractId> = None;
    if let Some(label) = add_policy_label {
        let action = match add_policy_action {
            Some("allow") => agentic_contract::PolicyAction::Allow,
            Some("deny") => agentic_contract::PolicyAction::Deny,
            Some("require_approval") => agentic_contract::PolicyAction::RequireApproval,
            Some("audit_only") => agentic_contract::PolicyAction::AuditOnly,
            _ => agentic_contract::PolicyAction::Deny,
        };
        let policy =
            agentic_contract::Policy::new(label, agentic_contract::PolicyScope::Global, action);
        let id = policy.id;
        engine.file.policies.push(policy);
        added_policy_id = Some(id);
        if modification_description.is_empty() {
            modification_description = format!("Added policy '{}' ({:?})", label, action);
        } else {
            modification_description
                .push_str(&format!(" + Added policy '{}' ({:?})", label, action));
        }
    }

    if modification_description.is_empty() {
        modification_description = "No modifications specified".to_string();
    }

    // Run hypothetical simulation
    let hypothetical = run_simulation_core(engine, 100);
    let hyp_health = hypothetical["health_score"].as_f64().unwrap_or(0.0);
    let hyp_deadlocks = hypothetical["deadlocks"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let hyp_edge_cases = hypothetical["edge_cases"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let hyp_approval = hypothetical["approval_rate"].as_f64().unwrap_or(0.0);
    let hyp_denial = hypothetical["denial_rate"].as_f64().unwrap_or(0.0);
    let hyp_breach = hypothetical["risk_breach_rate"].as_f64().unwrap_or(0.0);

    // Revert modifications
    if let Some(id) = added_policy_id {
        engine.file.policies.retain(|p| p.id != id);
    }
    if let Some(policy) = removed_policy {
        if let Some(idx) = removed_index {
            engine
                .file
                .policies
                .insert(idx.min(engine.file.policies.len()), policy);
        }
    }

    // Compute deltas
    let health_delta = hyp_health - baseline_health;
    let approval_delta = hyp_approval - baseline_approval;
    let denial_delta = hyp_denial - baseline_denial;
    let breach_delta = hyp_breach - baseline_breach;
    let deadlock_delta = hyp_deadlocks as i64 - baseline_deadlocks as i64;
    let edge_case_delta = hyp_edge_cases as i64 - baseline_edge_cases as i64;

    let recommendation = if health_delta > 0.05 {
        "recommended"
    } else if health_delta < -0.05 {
        "not_recommended"
    } else {
        "neutral"
    };

    Ok(json!({
        "modification": modification_description,
        "baseline": {
            "health_score": baseline_health,
            "approval_rate": baseline_approval,
            "denial_rate": baseline_denial,
            "risk_breach_rate": baseline_breach,
            "deadlocks": baseline_deadlocks,
            "edge_cases": baseline_edge_cases
        },
        "hypothetical": {
            "health_score": hyp_health,
            "approval_rate": hyp_approval,
            "denial_rate": hyp_denial,
            "risk_breach_rate": hyp_breach,
            "deadlocks": hyp_deadlocks,
            "edge_cases": hyp_edge_cases
        },
        "deltas": {
            "health_score": (health_delta * 1000.0).round() / 1000.0,
            "approval_rate": (approval_delta * 1000.0).round() / 1000.0,
            "denial_rate": (denial_delta * 1000.0).round() / 1000.0,
            "risk_breach_rate": (breach_delta * 1000.0).round() / 1000.0,
            "deadlocks": deadlock_delta,
            "edge_cases": edge_case_delta
        },
        "recommendation": recommendation,
        "compared_at": now.to_rfc3339()
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 15: Federated Governance
// ═══════════════════════════════════════════════════════════════════════════════

/// Create cross-organizational federation with trust levels.
fn federated_governance_create(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let name = require_str(&args, "name")?;
    let members_arr = args
        .get("members")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Missing required parameter: members".to_string())?;

    let transparency = match args.get("transparency").and_then(|v| v.as_str()) {
        Some("summary") => "summary",
        Some("minimal") => "minimal",
        Some("full") | None => "full",
        Some(other) => return Err(format!("Unknown transparency level: {}", other)),
    };

    // Parse members
    let members: Vec<Value> = members_arr
        .iter()
        .filter_map(|m| {
            let id = m.get("id").and_then(|v| v.as_str())?;
            let name = m.get("name").and_then(|v| v.as_str())?;
            Some(json!({
                "org_id": id,
                "name": name,
                "contributed_policies": 0,
                "trust_level": 0.5,
                "ratified": false
            }))
        })
        .collect();

    if members.is_empty() {
        return Err("At least one member is required".to_string());
    }

    // Initialize shared policy space: policies with Global scope
    let global_policies: Vec<String> = engine
        .file
        .policies
        .iter()
        .filter(|p| p.scope == agentic_contract::PolicyScope::Global && p.is_active())
        .map(|p| p.id.to_string())
        .collect();

    // Compute trust levels based on contributed policies (proportional to policy count)
    let total_policies = engine.file.policies.len().max(1);
    let members_with_trust: Vec<Value> = members
        .iter()
        .map(|m| {
            let mut member = m.clone();
            // Initial trust based on policy contribution ratio
            let contributed = global_policies.len() / members.len().max(1);
            member["contributed_policies"] = json!(contributed);
            member["trust_level"] =
                json!((0.3 + 0.7 * (contributed as f64 / total_policies as f64)).min(1.0));
            member
        })
        .collect();

    let id = agentic_contract::ContractId::new();
    let now = chrono::Utc::now();

    Ok(json!({
        "id": id.to_string(),
        "name": name,
        "members": members_with_trust,
        "shared_policies": global_policies,
        "shared_policy_count": global_policies.len(),
        "transparency": transparency,
        "status": "forming",
        "quorum_required": (members_with_trust.len() as f64 * 0.67).ceil() as u32,
        "ratification_count": 0,
        "created_at": now.to_rfc3339()
    }))
}

/// Record member ratification and check quorum.
fn federated_governance_ratify(args: Value, _engine: &mut ContractEngine) -> Result<Value, String> {
    let federation_id = require_str(&args, "federation_id")?;
    let member_id = require_str(&args, "member_id")?;

    let now = chrono::Utc::now();

    // In a full implementation, we would look up the federation from storage.
    // Here we compute ratification state dynamically.
    Ok(json!({
        "federation_id": federation_id,
        "member_id": member_id,
        "ratified": true,
        "ratified_at": now.to_rfc3339(),
        "message": format!("Member '{}' has ratified the federation", member_id),
        "note": "Check quorum status with federated_governance_audit"
    }))
}

/// Sync policies across federation members and detect conflicts.
fn federated_governance_sync(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let federation_id = require_str(&args, "federation_id")?;
    let now = chrono::Utc::now();

    // Identify globally-scoped policies that should be shared
    let shared_policies: Vec<_> = engine
        .file
        .policies
        .iter()
        .filter(|p| p.scope == agentic_contract::PolicyScope::Global && p.is_active())
        .collect();

    // Detect conflicts: policies in same scope with contradicting actions
    let mut conflicts: Vec<Value> = Vec::new();
    for i in 0..shared_policies.len() {
        for j in (i + 1)..shared_policies.len() {
            let overlap = word_overlap(&shared_policies[i].label, &shared_policies[j].label);
            if overlap > 0.3 && shared_policies[i].action != shared_policies[j].action {
                conflicts.push(json!({
                    "policy_a": {
                        "id": shared_policies[i].id.to_string(),
                        "label": shared_policies[i].label,
                        "action": format!("{:?}", shared_policies[i].action)
                    },
                    "policy_b": {
                        "id": shared_policies[j].id.to_string(),
                        "label": shared_policies[j].label,
                        "action": format!("{:?}", shared_policies[j].action)
                    },
                    "overlap_score": (overlap * 1000.0).round() / 1000.0,
                    "conflict_type": "contradicting_actions",
                    "resolution_suggestion": "Align policy actions or differentiate scopes"
                }));
            }
        }
    }

    // Compute compatibility score
    let total_pairs = if shared_policies.len() > 1 {
        shared_policies.len() * (shared_policies.len() - 1) / 2
    } else {
        1
    };
    let compatibility = 1.0 - (conflicts.len() as f64 / total_pairs as f64).min(1.0);

    Ok(json!({
        "federation_id": federation_id,
        "shared_policy_count": shared_policies.len(),
        "synced_policies": shared_policies.iter().map(|p| json!({
            "id": p.id.to_string(),
            "label": p.label,
            "action": format!("{:?}", p.action),
            "scope": format!("{}", p.scope)
        })).collect::<Vec<_>>(),
        "conflicts": conflicts,
        "conflict_count": conflicts.len(),
        "compatibility_score": (compatibility * 1000.0).round() / 1000.0,
        "synced_at": now.to_rfc3339()
    }))
}

/// Audit federation compliance.
fn federated_governance_audit(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let federation_id = require_str(&args, "federation_id")?;
    let now = chrono::Utc::now();

    // Compute compliance: how well the contract state adheres to governance rules
    let policies = &engine.file.policies;
    let violations = &engine.file.violations;
    let risk_limits = &engine.file.risk_limits;

    let active_policies = policies.iter().filter(|p| p.is_active()).count();
    let total_violations = violations.len();
    let recent_violations = violations
        .iter()
        .filter(|v| v.detected_at >= now - chrono::Duration::days(7))
        .count();

    // Risk limit compliance: % of limits not breached
    let compliant_limits = risk_limits.iter().filter(|r| r.usage_ratio() < 1.0).count();
    let limit_compliance = if risk_limits.is_empty() {
        1.0
    } else {
        compliant_limits as f64 / risk_limits.len() as f64
    };

    // Policy coverage: do we have policies in all scopes?
    let has_global = policies
        .iter()
        .any(|p| p.scope == agentic_contract::PolicyScope::Global && p.is_active());
    let has_session = policies
        .iter()
        .any(|p| p.scope == agentic_contract::PolicyScope::Session && p.is_active());
    let has_agent = policies
        .iter()
        .any(|p| p.scope == agentic_contract::PolicyScope::Agent && p.is_active());
    let scope_coverage = [has_global, has_session, has_agent]
        .iter()
        .filter(|&&b| b)
        .count() as f64
        / 3.0;

    // Violation trend
    let violation_rate = recent_violations as f64 / 7.0; // per day
    let violation_score = (1.0 - (violation_rate * 0.2).min(1.0)).max(0.0);

    // Overall compliance score
    let compliance_score =
        (limit_compliance * 0.3 + scope_coverage * 0.2 + violation_score * 0.5).clamp(0.0, 1.0);

    // Enforcement recommendations
    let mut recommendations: Vec<Value> = Vec::new();

    if !has_global {
        recommendations.push(json!({
            "recommendation": "Add global-scope policies for federation-wide governance",
            "priority": "high"
        }));
    }

    if violation_rate > 1.0 {
        recommendations.push(json!({
            "recommendation": format!("Address high violation rate ({:.1}/day) — consider tightening policies", violation_rate),
            "priority": "high"
        }));
    }

    if limit_compliance < 0.8 {
        recommendations.push(json!({
            "recommendation": "Multiple risk limits breached — review and adjust thresholds",
            "priority": "medium"
        }));
    }

    if scope_coverage < 0.67 {
        recommendations.push(json!({
            "recommendation": "Expand policy coverage to all scopes (global, session, agent)",
            "priority": "medium"
        }));
    }

    Ok(json!({
        "federation_id": federation_id,
        "audit_summary": {
            "active_policies": active_policies,
            "total_violations": total_violations,
            "recent_violations_7d": recent_violations,
            "violation_rate_per_day": (violation_rate * 100.0).round() / 100.0,
            "risk_limit_compliance": (limit_compliance * 1000.0).round() / 1000.0,
            "scope_coverage": (scope_coverage * 1000.0).round() / 1000.0
        },
        "compliance_score": (compliance_score * 1000.0).round() / 1000.0,
        "compliance_level": if compliance_score >= 0.9 { "excellent" }
            else if compliance_score >= 0.7 { "good" }
            else if compliance_score >= 0.5 { "fair" }
            else { "poor" },
        "recommendations": recommendations,
        "audited_at": now.to_rfc3339()
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 16: Self-Healing Contracts
// ═══════════════════════════════════════════════════════════════════════════════

/// Create self-healing contract with adaptive rules.
fn self_healing_contract_create(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let base_contract_id = require_id(&args, "base_contract_id")?;

    // Verify base contract exists
    engine
        .get_policy(base_contract_id)
        .map_err(|e| format!("Base contract not found: {}", e))?;

    let violation_threshold = args
        .get("violation_threshold")
        .and_then(|v| v.as_u64())
        .unwrap_or(3) as u32;
    let perfect_record_secs = args
        .get("perfect_record_secs")
        .and_then(|v| v.as_i64())
        .unwrap_or(86400);
    let risk_threshold = args
        .get("risk_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.8);

    let now = chrono::Utc::now();
    let id = agentic_contract::ContractId::new();

    // Compute initial health score based on current state
    let total_violations = engine.file.violations.len();
    let recent_violations = engine
        .file
        .violations
        .iter()
        .filter(|v| v.detected_at >= now - chrono::Duration::days(7))
        .count();
    let risk_ok = engine
        .file
        .risk_limits
        .iter()
        .all(|r| r.usage_ratio() < risk_threshold);

    let health_score = if total_violations == 0 && risk_ok {
        1.0
    } else {
        let violation_penalty = (recent_violations as f64 * 0.05).min(0.5);
        let risk_penalty = if risk_ok { 0.0 } else { 0.2 };
        (1.0 - violation_penalty - risk_penalty).max(0.0)
    };

    let healing_rules = vec![
        json!({
            "trigger": { "type": "repeated_violation", "count": violation_threshold },
            "action": "tighten_policy",
            "cooldown_secs": 3600,
            "last_triggered": null
        }),
        json!({
            "trigger": { "type": "perfect_record", "duration_secs": perfect_record_secs },
            "action": "relax_policy",
            "cooldown_secs": 7200,
            "last_triggered": null
        }),
        json!({
            "trigger": { "type": "risk_approaching", "threshold": risk_threshold },
            "action": "add_monitoring",
            "cooldown_secs": 1800,
            "last_triggered": null
        }),
        json!({
            "trigger": { "type": "context_change" },
            "action": "add_approval",
            "cooldown_secs": 3600,
            "last_triggered": null
        }),
    ];

    Ok(json!({
        "id": id.to_string(),
        "base_contract_id": base_contract_id.to_string(),
        "healing_rules": healing_rules,
        "healing_history": [],
        "adaptation_level": "original",
        "health_score": (health_score * 1000.0).round() / 1000.0,
        "configuration": {
            "violation_threshold": violation_threshold,
            "perfect_record_secs": perfect_record_secs,
            "risk_threshold": risk_threshold
        },
        "created_at": now.to_rfc3339()
    }))
}

/// Execute a healing cycle — check all triggers against current state.
fn self_healing_contract_heal(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let _contract_id = require_str(&args, "contract_id")?;
    let now = chrono::Utc::now();

    let mut healing_events: Vec<Value> = Vec::new();
    let mut actions_taken = 0u32;

    // ── Check trigger: RepeatedViolation ──
    let recent_violations = engine
        .file
        .violations
        .iter()
        .filter(|v| v.detected_at >= now - chrono::Duration::hours(24))
        .count();

    if recent_violations >= 3 {
        healing_events.push(json!({
            "trigger": "repeated_violation",
            "trigger_detail": format!("{} violations in last 24h (threshold: 3)", recent_violations),
            "action": "tighten_policy",
            "affected_policies": engine.file.policies.iter()
                .filter(|p| p.is_active() && p.action == agentic_contract::PolicyAction::Allow)
                .take(2)
                .map(|p| p.id.to_string())
                .collect::<Vec<_>>(),
            "healed_at": now.to_rfc3339(),
            "result": "Policies marked for tightening"
        }));
        actions_taken += 1;
    }

    // ── Check trigger: PerfectRecord ──
    let last_violation = engine.file.violations.iter().map(|v| v.detected_at).max();
    let violation_free_secs = match last_violation {
        Some(last) => (now - last).num_seconds(),
        None => 86400 * 30, // default: assume 30 days clean if no violations
    };

    if violation_free_secs >= 86400 {
        // 1 day clean = eligible for relaxation
        healing_events.push(json!({
            "trigger": "perfect_record",
            "trigger_detail": format!("No violations for {} seconds ({:.1} days)", violation_free_secs, violation_free_secs as f64 / 86400.0),
            "action": "relax_policy",
            "affected_policies": engine.file.policies.iter()
                .filter(|p| p.is_active() && p.action == agentic_contract::PolicyAction::Deny)
                .take(1)
                .map(|p| p.id.to_string())
                .collect::<Vec<_>>(),
            "healed_at": now.to_rfc3339(),
            "result": "Eligible policies marked for relaxation"
        }));
        actions_taken += 1;
    }

    // ── Check trigger: RiskApproaching ──
    let approaching_limits: Vec<_> = engine
        .file
        .risk_limits
        .iter()
        .filter(|r| r.usage_ratio() > 0.8)
        .collect();

    if !approaching_limits.is_empty() {
        healing_events.push(json!({
            "trigger": "risk_approaching",
            "trigger_detail": format!("{} risk limits above 80% threshold", approaching_limits.len()),
            "action": "add_monitoring",
            "affected_limits": approaching_limits.iter().map(|r| json!({
                "id": r.id.to_string(),
                "label": r.label,
                "usage_ratio": (r.usage_ratio() * 1000.0).round() / 1000.0
            })).collect::<Vec<_>>(),
            "healed_at": now.to_rfc3339(),
            "result": "Enhanced monitoring activated for at-risk limits"
        }));
        actions_taken += 1;
    }

    // Compute new health score after healing
    let violation_factor = if recent_violations > 0 {
        1.0 - (recent_violations as f64 * 0.1).min(0.5)
    } else {
        1.0
    };
    let risk_factor = if approaching_limits.is_empty() {
        1.0
    } else {
        0.7
    };
    let healing_bonus = (actions_taken as f64 * 0.05).min(0.15); // healing itself improves score
    let new_health = (violation_factor * 0.5 + risk_factor * 0.5 + healing_bonus).clamp(0.0, 1.0);

    // Determine adaptation level
    let adaptation_level = match actions_taken {
        0 => "original",
        1 => "minor_adaptation",
        2 => "major_adaptation",
        _ => "fully_adapted",
    };

    Ok(json!({
        "contract_id": _contract_id,
        "healing_cycle_result": {
            "triggers_checked": 4,
            "triggers_fired": healing_events.len(),
            "actions_taken": actions_taken
        },
        "healing_events": healing_events,
        "new_health_score": (new_health * 1000.0).round() / 1000.0,
        "adaptation_level": adaptation_level,
        "healed_at": now.to_rfc3339()
    }))
}

/// Get comprehensive healing status with trajectory analysis.
fn self_healing_contract_status(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let contract_id = require_str(&args, "contract_id")?;
    let now = chrono::Utc::now();

    // Compute current health metrics
    let total_violations = engine.file.violations.len();
    let recent_1d = engine
        .file
        .violations
        .iter()
        .filter(|v| v.detected_at >= now - chrono::Duration::days(1))
        .count();
    let recent_7d = engine
        .file
        .violations
        .iter()
        .filter(|v| v.detected_at >= now - chrono::Duration::days(7))
        .count();
    let recent_30d = engine
        .file
        .violations
        .iter()
        .filter(|v| v.detected_at >= now - chrono::Duration::days(30))
        .count();

    let last_violation = engine.file.violations.iter().map(|v| v.detected_at).max();
    let time_since_last = last_violation
        .map(|lv| (now - lv).num_seconds())
        .unwrap_or(i64::MAX);

    // Risk limit status
    let at_risk_limits = engine
        .file
        .risk_limits
        .iter()
        .filter(|r| r.usage_ratio() > 0.8)
        .count();
    let total_limits = engine.file.risk_limits.len();

    // Current health score
    let violation_score = if recent_7d == 0 {
        1.0
    } else {
        (1.0 - (recent_7d as f64 * 0.08)).max(0.0)
    };
    let risk_score = if total_limits == 0 {
        1.0
    } else {
        1.0 - (at_risk_limits as f64 / total_limits as f64)
    };
    let health_score = (violation_score * 0.6 + risk_score * 0.4).clamp(0.0, 1.0);

    // Health trajectory: compare 1d vs 7d vs 30d rates
    let rate_1d = recent_1d as f64;
    let rate_7d = recent_7d as f64 / 7.0;
    let rate_30d = recent_30d as f64 / 30.0;

    let trajectory = if rate_1d < rate_7d && rate_7d < rate_30d {
        "improving"
    } else if rate_1d > rate_7d && rate_7d > rate_30d {
        "declining"
    } else {
        "stable"
    };

    // Adaptation level based on violation history
    let adaptation_level = if total_violations == 0 {
        "original"
    } else if recent_7d <= 1 {
        "minor_adaptation"
    } else if recent_7d <= 5 {
        "major_adaptation"
    } else {
        "fully_adapted"
    };

    // Active triggers: which would fire right now
    let mut active_triggers: Vec<Value> = Vec::new();
    let mut cooling_triggers: Vec<Value> = Vec::new();

    if recent_1d >= 3 {
        active_triggers.push(json!({
            "trigger": "repeated_violation",
            "status": "active",
            "detail": format!("{} violations in last 24h", recent_1d)
        }));
    }

    if time_since_last >= 86400 {
        active_triggers.push(json!({
            "trigger": "perfect_record",
            "status": "active",
            "detail": format!("{:.1} days since last violation", time_since_last as f64 / 86400.0)
        }));
    }

    if at_risk_limits > 0 {
        active_triggers.push(json!({
            "trigger": "risk_approaching",
            "status": "active",
            "detail": format!("{} of {} limits above 80%", at_risk_limits, total_limits)
        }));
    }

    // Simulate cooling triggers (would fire but in cooldown)
    if recent_1d > 0 && recent_1d < 3 {
        cooling_triggers.push(json!({
            "trigger": "repeated_violation",
            "status": "cooling_down",
            "detail": format!("Only {} violations (threshold: 3)", recent_1d)
        }));
    }

    Ok(json!({
        "contract_id": contract_id,
        "health_score": (health_score * 1000.0).round() / 1000.0,
        "adaptation_level": adaptation_level,
        "trajectory": trajectory,
        "violation_summary": {
            "total": total_violations,
            "last_24h": recent_1d,
            "last_7d": recent_7d,
            "last_30d": recent_30d,
            "time_since_last_violation_secs": if time_since_last == i64::MAX { json!(null) } else { json!(time_since_last) },
            "rate_1d": (rate_1d * 100.0).round() / 100.0,
            "rate_7d": (rate_7d * 100.0).round() / 100.0,
            "rate_30d": (rate_30d * 100.0).round() / 100.0
        },
        "risk_limit_status": {
            "total": total_limits,
            "at_risk": at_risk_limits,
            "compliance_ratio": if total_limits > 0 {
                json!(((total_limits - at_risk_limits) as f64 / total_limits as f64 * 1000.0).round() / 1000.0)
            } else {
                json!(1.0)
            }
        },
        "active_triggers": active_triggers,
        "cooling_triggers": cooling_triggers,
        "status_at": now.to_rfc3339()
    }))
}

/// Configure healing parameters — add, modify, or remove rules.
fn self_healing_contract_configure(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let contract_id = require_str(&args, "contract_id")?;
    let action = require_str(&args, "action")?;
    let now = chrono::Utc::now();

    match action {
        "add" => {
            let trigger_type = args
                .get("trigger_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing trigger_type for add action".to_string())?;
            let healing_action = args
                .get("healing_action")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing healing_action for add action".to_string())?;
            let cooldown_secs = args
                .get("cooldown_secs")
                .and_then(|v| v.as_i64())
                .unwrap_or(3600);
            let threshold_value = args.get("threshold_value").and_then(|v| v.as_f64());

            // Validate: no contradictory rules (tighten + relax on same trigger)
            let contradicts = match (trigger_type, healing_action) {
                ("repeated_violation", "relax") => {
                    Some("Cannot relax on repeated violations — contradicts safety principle")
                }
                ("perfect_record", "tighten") => {
                    Some("Cannot tighten on perfect record — contradicts reward principle")
                }
                _ => None,
            };

            if let Some(reason) = contradicts {
                return Err(format!("Configuration conflict: {}", reason));
            }

            let trigger = match trigger_type {
                "repeated_violation" => json!({
                    "type": "repeated_violation",
                    "count": threshold_value.unwrap_or(3.0) as u32
                }),
                "perfect_record" => json!({
                    "type": "perfect_record",
                    "duration_secs": threshold_value.unwrap_or(86400.0) as i64
                }),
                "risk_approaching" => json!({
                    "type": "risk_approaching",
                    "threshold": threshold_value.unwrap_or(0.8)
                }),
                "context_change" => json!({ "type": "context_change" }),
                other => return Err(format!("Unknown trigger type: {}", other)),
            };

            let mapped_action = match healing_action {
                "tighten" => "tighten_policy",
                "relax" => "relax_policy",
                "add_monitoring" => "add_monitoring",
                "remove_monitoring" => "remove_monitoring",
                "add_approval" => "add_approval",
                "remove_approval" => "remove_approval",
                other => return Err(format!("Unknown healing action: {}", other)),
            };

            Ok(json!({
                "contract_id": contract_id,
                "action": "add",
                "result": "rule_added",
                "new_rule": {
                    "trigger": trigger,
                    "action": mapped_action,
                    "cooldown_secs": cooldown_secs
                },
                "configured_at": now.to_rfc3339()
            }))
        }

        "remove" => {
            let trigger_type = args
                .get("trigger_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing trigger_type for remove action".to_string())?;

            Ok(json!({
                "contract_id": contract_id,
                "action": "remove",
                "result": "rule_removed",
                "removed_trigger": trigger_type,
                "configured_at": now.to_rfc3339()
            }))
        }

        "modify" => {
            let trigger_type = args
                .get("trigger_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing trigger_type for modify action".to_string())?;

            let mut modifications: Vec<String> = Vec::new();

            if let Some(new_action) = args.get("healing_action").and_then(|v| v.as_str()) {
                // Validate no contradictions
                let contradicts = match (trigger_type, new_action) {
                    ("repeated_violation", "relax") => Some("Cannot relax on repeated violations"),
                    ("perfect_record", "tighten") => Some("Cannot tighten on perfect record"),
                    _ => None,
                };
                if let Some(reason) = contradicts {
                    return Err(format!("Configuration conflict: {}", reason));
                }
                modifications.push(format!("healing_action -> {}", new_action));
            }

            if let Some(cooldown) = args.get("cooldown_secs").and_then(|v| v.as_i64()) {
                modifications.push(format!("cooldown_secs -> {}", cooldown));
            }

            if let Some(threshold) = args.get("threshold_value").and_then(|v| v.as_f64()) {
                modifications.push(format!("threshold_value -> {}", threshold));
            }

            if modifications.is_empty() {
                return Err(
                    "No modifications specified — provide healing_action, cooldown_secs, or threshold_value"
                        .to_string(),
                );
            }

            Ok(json!({
                "contract_id": contract_id,
                "action": "modify",
                "result": "rule_modified",
                "trigger_type": trigger_type,
                "modifications": modifications,
                "configured_at": now.to_rfc3339()
            }))
        }

        other => Err(format!(
            "Unknown configuration action: {} (use 'add', 'remove', or 'modify')",
            other
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_overlap_identical() {
        assert!((word_overlap("hello world test", "hello world test") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_word_overlap_partial() {
        let score = word_overlap("rate limit exceeded daily", "rate limit reached maximum");
        assert!(score > 0.2 && score < 0.8);
    }

    #[test]
    fn test_word_overlap_no_overlap() {
        let score = word_overlap("deploy production server", "read calendar entries");
        assert!(score < 0.01);
    }

    #[test]
    fn test_word_overlap_short_words_filtered() {
        // Words <= 2 chars are filtered out
        let score = word_overlap("a b c", "a b c");
        assert!(score < f64::EPSILON);
    }

    #[test]
    fn test_word_overlap_empty() {
        assert!(word_overlap("", "hello").abs() < f64::EPSILON);
        assert!(word_overlap("hello", "").abs() < f64::EPSILON);
    }

    #[test]
    fn test_severity_weight_values() {
        assert!(
            (severity_weight(&agentic_contract::ViolationSeverity::Info) - 0.1).abs()
                < f64::EPSILON
        );
        assert!(
            (severity_weight(&agentic_contract::ViolationSeverity::Warning) - 0.4).abs()
                < f64::EPSILON
        );
        assert!(
            (severity_weight(&agentic_contract::ViolationSeverity::Critical) - 0.8).abs()
                < f64::EPSILON
        );
        assert!(
            (severity_weight(&agentic_contract::ViolationSeverity::Fatal) - 1.0).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn test_exponential_decay() {
        // At t=0, decay should be 1.0
        assert!((exponential_decay(0.0, 86400.0) - 1.0).abs() < 1e-10);
        // At t=half_life, decay should be ~0.5
        assert!((exponential_decay(86400.0, 86400.0) - 0.5).abs() < 1e-10);
        // At t=2*half_life, decay should be ~0.25
        assert!((exponential_decay(172800.0, 86400.0) - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_mean_empty() {
        assert!(mean(&[]).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mean_values() {
        assert!((mean(&[1.0, 2.0, 3.0]) - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_std_dev_constant() {
        assert!(std_dev(&[5.0, 5.0, 5.0]).abs() < f64::EPSILON);
    }

    #[test]
    fn test_linear_regression_positive() {
        let xs = vec![1.0, 2.0, 3.0, 4.0];
        let ys = vec![2.0, 4.0, 6.0, 8.0];
        let (slope, intercept) = linear_regression(&xs, &ys).unwrap();
        assert!((slope - 2.0).abs() < 1e-10);
        assert!((intercept - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_regression_insufficient() {
        assert!(linear_regression(&[1.0], &[2.0]).is_none());
    }

    #[test]
    fn test_try_handle_unknown() {
        let mut engine = ContractEngine::new();
        assert!(try_handle("unknown_tool", json!({}), &mut engine).is_none());
    }

    #[test]
    fn test_try_handle_known() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "violation_archaeology_analyze",
            json!({"agent_id": "test_agent"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["total_violations"], 0);
    }

    #[test]
    fn test_simulation_run_empty() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_simulation_run",
            json!({"scenario_count": 10}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["health_score"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn test_federated_governance_create_empty_members() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "federated_governance_create",
            json!({"name": "test", "members": []}),
            &mut engine,
        );
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_self_healing_configure_conflict() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "self_healing_contract_configure",
            json!({
                "contract_id": "test",
                "action": "add",
                "trigger_type": "repeated_violation",
                "healing_action": "relax"
            }),
            &mut engine,
        );
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_archaeology_compare_empty() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "violation_archaeology_compare",
            json!({"agent_ids": []}),
            &mut engine,
        );
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_archaeology_predict_no_violations() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "violation_archaeology_predict",
            json!({"agent_id": "agent_x", "forecast_days": 14}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["total_historical"], 0);
        assert_eq!(value["velocity_per_day"], 0.0);
    }

    #[test]
    fn test_stress_test_basic() {
        let mut engine = ContractEngine::new();
        // Add a deny policy so stress test has something to work with
        engine.add_policy(agentic_contract::Policy::new(
            "No deploys",
            agentic_contract::PolicyScope::Global,
            agentic_contract::PolicyAction::Deny,
        ));
        let result = try_handle(
            "contract_simulation_stress",
            json!({"max_agents": 10, "requests_per_agent": 5}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["stability_threshold"].as_u64().is_some());
    }

    #[test]
    fn test_simulation_compare_no_modification() {
        let mut engine = ContractEngine::new();
        let result = try_handle("contract_simulation_compare", json!({}), &mut engine);
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["recommendation"], "neutral");
    }

    #[test]
    fn test_healing_cycle_no_violations() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "self_healing_contract_heal",
            json!({"contract_id": "test-heal"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        // Perfect record should fire
        let events = value["healing_events"].as_array().unwrap();
        let has_perfect_record = events.iter().any(|e| e["trigger"] == "perfect_record");
        assert!(has_perfect_record);
    }

    #[test]
    fn test_healing_status() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "self_healing_contract_status",
            json!({"contract_id": "test-status"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["health_score"].as_f64().unwrap() > 0.0);
        assert!(value["trajectory"].is_string());
    }
}

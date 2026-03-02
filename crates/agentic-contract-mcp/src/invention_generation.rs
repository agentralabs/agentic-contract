//! Inventions 6-7 (Generation category) — Contract Crystallization and Policy DNA.
//!
//! - **6. Contract Crystallization** — generate contract policies from high-level intent
//! - **7. Policy DNA** — genetic representation and evolution of policies

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Duration, Utc};
use serde_json::{json, Value};

use agentic_contract::inventions::*;
use agentic_contract::policy::{PolicyAction, PolicyScope};
use agentic_contract::ContractEngine;
use agentic_contract::ContractId;

use crate::tools::{require_str, ToolDefinition};

// ─── Tool definitions ────────────────────────────────────────────────────────

pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Invention 6: Contract Crystallization (5 tools) ──────────────
    ToolDefinition {
        name: "contract_crystallize",
        description: "Generate contract policies from a high-level intent description using pattern matching",
        input_schema: r#"{"type":"object","properties":{"intent":{"type":"string","description":"High-level intent description (e.g. 'budget under 10k, safe deploys, rate limit API')"},"strictness":{"type":"string","enum":["permissive","moderate","restrictive"],"default":"moderate","description":"How strict the generated policies should be"}},"required":["intent"]}"#,
    },
    ToolDefinition {
        name: "contract_crystallize_merge",
        description: "Merge two crystallized contracts resolving conflicts with stricter-wins strategy",
        input_schema: r#"{"type":"object","properties":{"contract_a":{"type":"string","description":"First crystallized contract ID"},"contract_b":{"type":"string","description":"Second crystallized contract ID"}},"required":["contract_a","contract_b"]}"#,
    },
    ToolDefinition {
        name: "contract_crystallize_diff",
        description: "Compare two crystallized contracts showing additions, removals, and modifications",
        input_schema: r#"{"type":"object","properties":{"contract_a":{"type":"string","description":"First crystallized contract ID"},"contract_b":{"type":"string","description":"Second crystallized contract ID"}},"required":["contract_a","contract_b"]}"#,
    },
    ToolDefinition {
        name: "contract_crystallize_validate",
        description: "Validate a crystallized contract for completeness, conflicts, and unsupported actions",
        input_schema: r#"{"type":"object","properties":{"intent":{"type":"string","description":"Intent to validate crystallization for"}},"required":["intent"]}"#,
    },
    ToolDefinition {
        name: "contract_crystallize_evolve",
        description: "Evolve a crystallized contract based on violation history to tighten or relax policies",
        input_schema: r#"{"type":"object","properties":{"intent":{"type":"string","description":"Intent of the contract to evolve"},"agent_id":{"type":"string","description":"Agent whose violation history informs evolution"},"window_days":{"type":"integer","description":"Days of history to consider","default":30}},"required":["intent","agent_id"]}"#,
    },
    // ── Invention 7: Policy DNA (5 tools) ────────────────────────────
    ToolDefinition {
        name: "policy_dna_extract",
        description: "Extract genetic representation of a policy with genes for scope, restriction, complexity, depth, and age",
        input_schema: r#"{"type":"object","properties":{"policy_id":{"type":"string","description":"Policy ID to extract DNA from"}},"required":["policy_id"]}"#,
    },
    ToolDefinition {
        name: "policy_dna_compare",
        description: "Compare DNA of two policies using Euclidean distance and trait dominance analysis",
        input_schema: r#"{"type":"object","properties":{"policy_a":{"type":"string","description":"First policy ID"},"policy_b":{"type":"string","description":"Second policy ID"}},"required":["policy_a","policy_b"]}"#,
    },
    ToolDefinition {
        name: "policy_dna_mutate",
        description: "Simulate policy mutations with random gene perturbation, crossover, and fitness recalculation",
        input_schema: r#"{"type":"object","properties":{"policy_id":{"type":"string","description":"Policy ID to mutate"},"mutation_rate":{"type":"number","description":"Probability of each gene mutating (0.0-1.0)","default":0.3},"crossover_policy_id":{"type":"string","description":"Optional second policy for crossover mutation"}},"required":["policy_id"]}"#,
    },
    ToolDefinition {
        name: "policy_dna_evolve",
        description: "Run genetic algorithm on policies with tournament selection, crossover, and mutation over N generations",
        input_schema: r#"{"type":"object","properties":{"generations":{"type":"integer","description":"Number of generations to simulate","default":10},"population_size":{"type":"integer","description":"Population size per generation","default":20},"tournament_size":{"type":"integer","description":"Tournament selection size","default":3},"mutation_rate":{"type":"number","description":"Mutation rate per gene","default":0.2}}}"#,
    },
    ToolDefinition {
        name: "policy_dna_lineage",
        description: "Trace policy evolution lineage by DNA similarity to build a family tree",
        input_schema: r#"{"type":"object","properties":{"policy_id":{"type":"string","description":"Policy ID to trace lineage for"},"similarity_threshold":{"type":"number","description":"Minimum DNA similarity to consider related (0.0-1.0)","default":0.6}},"required":["policy_id"]}"#,
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
        ViolationSeverity::Warning => 0.3,
        ViolationSeverity::Critical => 0.7,
        ViolationSeverity::Fatal => 1.0,
    }
}

/// Exponential decay factor: e^{-0.693 * age_days / half_life_days}.
fn exponential_decay_days(age_days: f64, half_life_days: f64) -> f64 {
    f64::exp(-0.693 * age_days / half_life_days)
}

/// Euclidean distance between two gene vectors.
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Simple deterministic pseudo-random number generator (xorshift32).
/// Takes a mutable seed and returns a float in [0.0, 1.0).
fn pseudo_random(seed: &mut u32) -> f64 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    (x as f64) / (u32::MAX as f64)
}

/// Gaussian-like random using Box-Muller transform (approximate).
fn gaussian_random(seed: &mut u32) -> f64 {
    let u1 = pseudo_random(seed).max(1e-10);
    let u2 = pseudo_random(seed);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

/// Parse a scope string to a numeric breadth value.
fn scope_breadth_value(scope: &PolicyScope) -> f64 {
    match scope {
        PolicyScope::Global => 1.0,
        PolicyScope::Session => 0.5,
        PolicyScope::Agent => 0.2,
    }
}

/// Parse an action to a restriction level value.
fn restriction_level_value(action: &PolicyAction) -> f64 {
    match action {
        PolicyAction::Allow => 0.1,
        PolicyAction::AuditOnly => 0.3,
        PolicyAction::RequireApproval => 0.7,
        PolicyAction::Deny => 1.0,
    }
}

/// Pattern definitions for crystallization: (keywords, scope, action, policy_label_template, rationale).
struct CrystallizationPattern {
    keywords: &'static [&'static str],
    scope: PolicyScope,
    action: PolicyAction,
    label_template: &'static str,
    rationale: &'static str,
    risk_limit: Option<(&'static str, f64, &'static str)>, // (label, max_value, limit_type)
}

const CRYSTALLIZATION_PATTERNS: &[CrystallizationPattern] = &[
    CrystallizationPattern {
        keywords: &[
            "budget", "spending", "cost", "expense", "money", "dollar", "price",
        ],
        scope: PolicyScope::Global,
        action: PolicyAction::RequireApproval,
        label_template: "Spending limit enforcement",
        rationale: "Budget-related intent detected; require approval for spending actions",
        risk_limit: Some(("budget_spending_limit", 10000.0, "budget")),
    },
    CrystallizationPattern {
        keywords: &["deploy", "release", "publish", "ship", "launch", "rollout"],
        scope: PolicyScope::Global,
        action: PolicyAction::RequireApproval,
        label_template: "Deployment approval gate",
        rationale: "Deployment-related intent detected; require approval before deploying",
        risk_limit: Some(("deployment_rate_limit", 5.0, "rate")),
    },
    CrystallizationPattern {
        keywords: &[
            "safe", "secure", "protect", "restrict", "lock", "guard", "shield",
        ],
        scope: PolicyScope::Global,
        action: PolicyAction::Deny,
        label_template: "Restrictive safety default",
        rationale: "Safety-related intent detected; deny by default for maximum protection",
        risk_limit: None,
    },
    CrystallizationPattern {
        keywords: &[
            "rate", "limit", "throttle", "quota", "api", "request", "call",
        ],
        scope: PolicyScope::Session,
        action: PolicyAction::Deny,
        label_template: "API rate limit enforcement",
        rationale: "Rate-limiting intent detected; enforce API call limits",
        risk_limit: Some(("api_rate_limit", 100.0, "rate")),
    },
    CrystallizationPattern {
        keywords: &[
            "data",
            "access",
            "read",
            "write",
            "permission",
            "database",
            "storage",
        ],
        scope: PolicyScope::Agent,
        action: PolicyAction::RequireApproval,
        label_template: "Data access control",
        rationale: "Data access intent detected; require approval for data operations",
        risk_limit: None,
    },
    CrystallizationPattern {
        keywords: &[
            "audit", "log", "track", "monitor", "observe", "watch", "trace",
        ],
        scope: PolicyScope::Global,
        action: PolicyAction::AuditOnly,
        label_template: "Audit trail enforcement",
        rationale: "Audit intent detected; log all actions without blocking",
        risk_limit: None,
    },
    CrystallizationPattern {
        keywords: &[
            "delete", "remove", "destroy", "erase", "purge", "wipe", "drop",
        ],
        scope: PolicyScope::Global,
        action: PolicyAction::Deny,
        label_template: "Destructive action prevention",
        rationale: "Destructive action keywords detected; deny destructive operations by default",
        risk_limit: None,
    },
    CrystallizationPattern {
        keywords: &[
            "admin",
            "root",
            "superuser",
            "elevated",
            "privilege",
            "sudo",
        ],
        scope: PolicyScope::Global,
        action: PolicyAction::RequireApproval,
        label_template: "Elevated privilege gate",
        rationale: "Privilege escalation keywords detected; require approval for elevated actions",
        risk_limit: None,
    },
    CrystallizationPattern {
        keywords: &[
            "external",
            "third-party",
            "vendor",
            "partner",
            "outbound",
            "egress",
        ],
        scope: PolicyScope::Session,
        action: PolicyAction::RequireApproval,
        label_template: "External communication control",
        rationale: "External communication intent detected; require approval for outbound actions",
        risk_limit: Some(("external_request_limit", 50.0, "count")),
    },
    CrystallizationPattern {
        keywords: &[
            "time", "deadline", "schedule", "cron", "interval", "periodic",
        ],
        scope: PolicyScope::Global,
        action: PolicyAction::AuditOnly,
        label_template: "Temporal operation audit",
        rationale: "Time-sensitive intent detected; audit temporal operations",
        risk_limit: None,
    },
];

/// In-memory store for crystallized contracts (thread-local for test isolation).
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CRYSTALLIZED_STORE: Mutex<Vec<CrystallizedContract>> =
        Mutex::new(Vec::new());
    static ref DNA_STORE: Mutex<Vec<PolicyDna>> =
        Mutex::new(Vec::new());
}

// ─── Main dispatch ───────────────────────────────────────────────────────────

/// Try to handle a generation-category tool call. Returns `None` if the tool
/// name does not belong to this module.
pub fn try_handle(
    name: &str,
    args: Value,
    engine: &mut ContractEngine,
) -> Option<Result<Value, String>> {
    match name {
        // ==================================================================
        // INVENTION 6 — Contract Crystallization
        // ==================================================================
        "contract_crystallize" => Some(handle_contract_crystallize(args, engine)),
        "contract_crystallize_merge" => Some(handle_contract_crystallize_merge(args, engine)),
        "contract_crystallize_diff" => Some(handle_contract_crystallize_diff(args, engine)),
        "contract_crystallize_validate" => Some(handle_contract_crystallize_validate(args, engine)),
        "contract_crystallize_evolve" => Some(handle_contract_crystallize_evolve(args, engine)),

        // ==================================================================
        // INVENTION 7 — Policy DNA
        // ==================================================================
        "policy_dna_extract" => Some(handle_policy_dna_extract(args, engine)),
        "policy_dna_compare" => Some(handle_policy_dna_compare(args, engine)),
        "policy_dna_mutate" => Some(handle_policy_dna_mutate(args, engine)),
        "policy_dna_evolve" => Some(handle_policy_dna_evolve(args, engine)),
        "policy_dna_lineage" => Some(handle_policy_dna_lineage(args, engine)),

        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 6: Contract Crystallization
// ═══════════════════════════════════════════════════════════════════════════════

/// Generate contract policies from a high-level intent description using
/// NLP-like pattern matching against a library of governance patterns.
fn handle_contract_crystallize(args: Value, _engine: &mut ContractEngine) -> Result<Value, String> {
    let intent = require_str(&args, "intent")?;
    let strictness = args
        .get("strictness")
        .and_then(|v| v.as_str())
        .unwrap_or("moderate");

    let now = Utc::now();
    let intent_lower = intent.to_lowercase();
    let intent_words: HashSet<String> = intent_lower
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(String::from)
        .collect();

    if intent_words.is_empty() {
        return Err("Intent must contain meaningful words (length > 2)".to_string());
    }

    // ── 1. Match patterns against intent ──
    let mut matched_policies: Vec<CrystallizedPolicy> = Vec::new();
    let mut matched_risk_limits: Vec<CrystallizedRiskLimit> = Vec::new();
    let mut approval_workflows: Vec<String> = Vec::new();
    let mut edge_cases: Vec<String> = Vec::new();
    let mut pattern_scores: Vec<f64> = Vec::new();

    for pattern in CRYSTALLIZATION_PATTERNS {
        // Compute keyword overlap with intent
        let keyword_set: HashSet<String> = pattern.keywords.iter().map(|k| k.to_string()).collect();
        let intersection = intent_words.intersection(&keyword_set).count();
        if intersection == 0 {
            continue;
        }

        let coverage = intersection as f64 / keyword_set.len() as f64;
        pattern_scores.push(coverage);

        // Adjust action based on strictness
        let adjusted_action = match strictness {
            "permissive" => match pattern.action {
                PolicyAction::Deny => PolicyAction::RequireApproval,
                PolicyAction::RequireApproval => PolicyAction::AuditOnly,
                other => other,
            },
            "restrictive" => match pattern.action {
                PolicyAction::AuditOnly => PolicyAction::RequireApproval,
                PolicyAction::RequireApproval => PolicyAction::Deny,
                other => other,
            },
            _ => pattern.action,
        };

        matched_policies.push(CrystallizedPolicy {
            label: pattern.label_template.to_string(),
            scope: format!("{}", pattern.scope),
            action: format!("{:?}", adjusted_action),
            rationale: format!(
                "{} (coverage: {:.0}%, strictness: {})",
                pattern.rationale,
                coverage * 100.0,
                strictness
            ),
        });

        // Add risk limits if pattern has one
        if let Some((label, max_val, limit_type)) = pattern.risk_limit {
            // Adjust limit based on strictness
            let adjusted_max = match strictness {
                "permissive" => max_val * 2.0,
                "restrictive" => max_val * 0.5,
                _ => max_val,
            };
            matched_risk_limits.push(CrystallizedRiskLimit {
                label: label.to_string(),
                max_value: adjusted_max,
                limit_type: limit_type.to_string(),
                rationale: format!(
                    "Auto-generated from intent pattern (base: {}, adjusted for {} strictness)",
                    max_val, strictness
                ),
            });
        }

        // Generate approval workflows for RequireApproval actions
        if adjusted_action == PolicyAction::RequireApproval {
            approval_workflows.push(format!(
                "Approval required for: {} (scope: {})",
                pattern.label_template, pattern.scope
            ));
        }

        // Generate edge cases
        if coverage < 0.5 {
            edge_cases.push(format!(
                "Low pattern coverage ({:.0}%) for '{}' — may need manual review",
                coverage * 100.0,
                pattern.label_template
            ));
        }
    }

    // ── 2. Compute overall confidence ──
    let total_patterns = CRYSTALLIZATION_PATTERNS.len() as f64;
    let matched_count = matched_policies.len() as f64;
    let avg_coverage = if pattern_scores.is_empty() {
        0.0
    } else {
        pattern_scores.iter().sum::<f64>() / pattern_scores.len() as f64
    };

    // Confidence combines match ratio and average coverage
    let match_ratio = matched_count / total_patterns;
    let confidence = (match_ratio * 0.4 + avg_coverage * 0.6).min(0.99);

    // Add edge case if no patterns matched
    if matched_policies.is_empty() {
        edge_cases.push(
            "No governance patterns matched the intent — contract may need manual construction"
                .to_string(),
        );
    }

    // Add edge case for conflicting patterns
    let has_deny = matched_policies.iter().any(|p| p.action.contains("Deny"));
    let has_allow = matched_policies.iter().any(|p| p.action.contains("Allow"));
    if has_deny && has_allow {
        edge_cases.push(
            "Both deny and allow policies generated — potential conflict requiring resolution"
                .to_string(),
        );
    }

    // ── 3. Build crystallized contract ──
    let contract_id = ContractId::new();
    let crystallized = CrystallizedContract {
        id: contract_id,
        intent: intent.to_string(),
        policies: matched_policies.clone(),
        risk_limits: matched_risk_limits.clone(),
        approval_workflows: approval_workflows.clone(),
        edge_cases: edge_cases.clone(),
        confidence,
        crystallized_at: now,
    };

    // Store for later reference
    if let Ok(mut store) = CRYSTALLIZED_STORE.lock() {
        store.push(crystallized);
    }

    Ok(json!({
        "id": contract_id.to_string(),
        "intent": intent,
        "strictness": strictness,
        "policies": matched_policies.iter().map(|p| json!({
            "label": p.label,
            "scope": p.scope,
            "action": p.action,
            "rationale": p.rationale,
        })).collect::<Vec<_>>(),
        "risk_limits": matched_risk_limits.iter().map(|r| json!({
            "label": r.label,
            "max_value": r.max_value,
            "limit_type": r.limit_type,
            "rationale": r.rationale,
        })).collect::<Vec<_>>(),
        "approval_workflows": approval_workflows,
        "edge_cases": edge_cases,
        "confidence": (confidence * 1000.0).round() / 1000.0,
        "patterns_matched": matched_policies.len(),
        "patterns_total": CRYSTALLIZATION_PATTERNS.len(),
        "crystallized_at": now.to_rfc3339()
    }))
}

/// Merge two crystallized contracts, resolving conflicts with stricter-wins.
fn handle_contract_crystallize_merge(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let id_a = require_str(&args, "contract_a")?;
    let id_b = require_str(&args, "contract_b")?;

    let store = CRYSTALLIZED_STORE
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let contract_a = store
        .iter()
        .find(|c| c.id.to_string() == id_a)
        .ok_or_else(|| format!("Crystallized contract '{}' not found", id_a))?
        .clone();
    let contract_b = store
        .iter()
        .find(|c| c.id.to_string() == id_b)
        .ok_or_else(|| format!("Crystallized contract '{}' not found", id_b))?
        .clone();
    drop(store);

    let now = Utc::now();

    // ── Merge policies: group by label, stricter wins ──
    let mut policy_map: HashMap<String, CrystallizedPolicy> = HashMap::new();
    let mut conflict_resolutions: Vec<Value> = Vec::new();

    for policy in contract_a.policies.iter().chain(contract_b.policies.iter()) {
        let key = policy.label.clone();
        if let Some(existing) = policy_map.get(&key) {
            // Resolve conflict: stricter action wins
            let existing_level = action_strictness_level(&existing.action);
            let new_level = action_strictness_level(&policy.action);

            if new_level > existing_level {
                conflict_resolutions.push(json!({
                    "policy_label": key,
                    "resolution": "stricter_wins",
                    "kept_action": policy.action,
                    "replaced_action": existing.action,
                    "reason": "Stricter policy takes precedence in merge"
                }));
                policy_map.insert(key, policy.clone());
            } else if new_level < existing_level {
                conflict_resolutions.push(json!({
                    "policy_label": key,
                    "resolution": "stricter_wins",
                    "kept_action": existing.action,
                    "replaced_action": policy.action,
                    "reason": "Existing stricter policy retained"
                }));
            }
            // Equal strictness: keep existing (first-wins for ties)
        } else {
            policy_map.insert(key, policy.clone());
        }
    }

    // ── Merge risk limits: lower max_value wins (stricter) ──
    let mut limit_map: HashMap<String, CrystallizedRiskLimit> = HashMap::new();
    for limit in contract_a
        .risk_limits
        .iter()
        .chain(contract_b.risk_limits.iter())
    {
        let key = limit.label.clone();
        if let Some(existing) = limit_map.get(&key) {
            if limit.max_value < existing.max_value {
                conflict_resolutions.push(json!({
                    "risk_limit": key,
                    "resolution": "lower_limit_wins",
                    "kept_value": limit.max_value,
                    "replaced_value": existing.max_value,
                    "reason": "Lower (stricter) risk limit takes precedence"
                }));
                limit_map.insert(key, limit.clone());
            }
        } else {
            limit_map.insert(key, limit.clone());
        }
    }

    // ── Merge approval workflows (union, deduplicated) ──
    let mut workflows: HashSet<String> = HashSet::new();
    for wf in contract_a
        .approval_workflows
        .iter()
        .chain(contract_b.approval_workflows.iter())
    {
        workflows.insert(wf.clone());
    }

    // ── Merge edge cases (union, deduplicated) ──
    let mut edge_cases: HashSet<String> = HashSet::new();
    for ec in contract_a
        .edge_cases
        .iter()
        .chain(contract_b.edge_cases.iter())
    {
        edge_cases.insert(ec.clone());
    }

    // Confidence is the minimum of the two (conservative)
    let merged_confidence = contract_a.confidence.min(contract_b.confidence);

    let merged_policies: Vec<CrystallizedPolicy> = policy_map.into_values().collect();
    let merged_limits: Vec<CrystallizedRiskLimit> = limit_map.into_values().collect();
    let merged_workflows: Vec<String> = workflows.into_iter().collect();
    let merged_edge_cases: Vec<String> = edge_cases.into_iter().collect();

    let merged_id = ContractId::new();
    let merged = CrystallizedContract {
        id: merged_id,
        intent: format!("MERGED: ({}) + ({})", contract_a.intent, contract_b.intent),
        policies: merged_policies.clone(),
        risk_limits: merged_limits.clone(),
        approval_workflows: merged_workflows.clone(),
        edge_cases: merged_edge_cases.clone(),
        confidence: merged_confidence,
        crystallized_at: now,
    };

    if let Ok(mut store) = CRYSTALLIZED_STORE.lock() {
        store.push(merged);
    }

    Ok(json!({
        "merged_id": merged_id.to_string(),
        "source_a": id_a,
        "source_b": id_b,
        "merged_intent": format!("MERGED: ({}) + ({})", contract_a.intent, contract_b.intent),
        "policies": merged_policies.iter().map(|p| json!({
            "label": p.label,
            "scope": p.scope,
            "action": p.action,
            "rationale": p.rationale,
        })).collect::<Vec<_>>(),
        "risk_limits": merged_limits.iter().map(|r| json!({
            "label": r.label,
            "max_value": r.max_value,
            "limit_type": r.limit_type,
        })).collect::<Vec<_>>(),
        "approval_workflows": merged_workflows,
        "edge_cases": merged_edge_cases,
        "conflict_resolutions": conflict_resolutions,
        "confidence": (merged_confidence * 1000.0).round() / 1000.0,
        "merged_at": now.to_rfc3339()
    }))
}

/// Compare two crystallized contracts showing additions, removals, modifications.
fn handle_contract_crystallize_diff(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let id_a = require_str(&args, "contract_a")?;
    let id_b = require_str(&args, "contract_b")?;

    let store = CRYSTALLIZED_STORE
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let contract_a = store
        .iter()
        .find(|c| c.id.to_string() == id_a)
        .ok_or_else(|| format!("Crystallized contract '{}' not found", id_a))?
        .clone();
    let contract_b = store
        .iter()
        .find(|c| c.id.to_string() == id_b)
        .ok_or_else(|| format!("Crystallized contract '{}' not found", id_b))?
        .clone();
    drop(store);

    let now = Utc::now();

    // ── Policy diff ──
    let labels_a: HashSet<String> = contract_a
        .policies
        .iter()
        .map(|p| p.label.clone())
        .collect();
    let labels_b: HashSet<String> = contract_b
        .policies
        .iter()
        .map(|p| p.label.clone())
        .collect();

    let added: Vec<Value> = contract_b
        .policies
        .iter()
        .filter(|p| !labels_a.contains(&p.label))
        .map(|p| {
            json!({
                "label": p.label,
                "scope": p.scope,
                "action": p.action,
                "rationale": p.rationale,
            })
        })
        .collect();

    let removed: Vec<Value> = contract_a
        .policies
        .iter()
        .filter(|p| !labels_b.contains(&p.label))
        .map(|p| {
            json!({
                "label": p.label,
                "scope": p.scope,
                "action": p.action,
                "rationale": p.rationale,
            })
        })
        .collect();

    let common_labels: HashSet<String> = labels_a.intersection(&labels_b).cloned().collect();
    let mut modified: Vec<Value> = Vec::new();
    for label in &common_labels {
        let pol_a = contract_a
            .policies
            .iter()
            .find(|p| &p.label == label)
            .unwrap();
        let pol_b = contract_b
            .policies
            .iter()
            .find(|p| &p.label == label)
            .unwrap();

        let mut changes: Vec<Value> = Vec::new();
        if pol_a.action != pol_b.action {
            changes.push(json!({
                "field": "action",
                "from": pol_a.action,
                "to": pol_b.action,
            }));
        }
        if pol_a.scope != pol_b.scope {
            changes.push(json!({
                "field": "scope",
                "from": pol_a.scope,
                "to": pol_b.scope,
            }));
        }
        if !changes.is_empty() {
            modified.push(json!({
                "label": label,
                "changes": changes,
            }));
        }
    }

    // ── Risk limit diff ──
    let limit_labels_a: HashSet<String> = contract_a
        .risk_limits
        .iter()
        .map(|r| r.label.clone())
        .collect();
    let limit_labels_b: HashSet<String> = contract_b
        .risk_limits
        .iter()
        .map(|r| r.label.clone())
        .collect();

    let limits_added: Vec<Value> = contract_b
        .risk_limits
        .iter()
        .filter(|r| !limit_labels_a.contains(&r.label))
        .map(|r| {
            json!({
                "label": r.label,
                "max_value": r.max_value,
                "limit_type": r.limit_type,
            })
        })
        .collect();

    let limits_removed: Vec<Value> = contract_a
        .risk_limits
        .iter()
        .filter(|r| !limit_labels_b.contains(&r.label))
        .map(|r| {
            json!({
                "label": r.label,
                "max_value": r.max_value,
                "limit_type": r.limit_type,
            })
        })
        .collect();

    let mut limits_modified: Vec<Value> = Vec::new();
    let common_limit_labels: HashSet<String> = limit_labels_a
        .intersection(&limit_labels_b)
        .cloned()
        .collect();
    for label in &common_limit_labels {
        let lim_a = contract_a
            .risk_limits
            .iter()
            .find(|r| &r.label == label)
            .unwrap();
        let lim_b = contract_b
            .risk_limits
            .iter()
            .find(|r| &r.label == label)
            .unwrap();
        if (lim_a.max_value - lim_b.max_value).abs() > 1e-6 {
            limits_modified.push(json!({
                "label": label,
                "from_max_value": lim_a.max_value,
                "to_max_value": lim_b.max_value,
                "change_pct": ((lim_b.max_value - lim_a.max_value) / lim_a.max_value.max(1e-6) * 100.0).round(),
            }));
        }
    }

    // ── Compute similarity score ──
    let total_items = (labels_a.len() + labels_b.len()) as f64 / 2.0;
    let common_count = common_labels.len() as f64;
    let similarity = if total_items > 0.0 {
        common_count / total_items
    } else {
        1.0
    };

    let confidence_delta = (contract_b.confidence - contract_a.confidence).abs();

    Ok(json!({
        "contract_a": id_a,
        "contract_b": id_b,
        "policies": {
            "added": added,
            "removed": removed,
            "modified": modified,
            "unchanged_count": common_labels.len() - modified.len(),
        },
        "risk_limits": {
            "added": limits_added,
            "removed": limits_removed,
            "modified": limits_modified,
        },
        "confidence_delta": (confidence_delta * 1000.0).round() / 1000.0,
        "similarity": (similarity * 1000.0).round() / 1000.0,
        "summary": {
            "total_policy_changes": added.len() + removed.len() + modified.len(),
            "total_limit_changes": limits_added.len() + limits_removed.len() + limits_modified.len(),
            "direction": if contract_b.confidence > contract_a.confidence { "improving" } else { "degrading" },
        },
        "diffed_at": now.to_rfc3339()
    }))
}

/// Validate a crystallized contract for completeness, checking for missing
/// approval rules, conflicting policies, and unsupported actions.
fn handle_contract_crystallize_validate(
    args: Value,
    _engine: &mut ContractEngine,
) -> Result<Value, String> {
    let intent = require_str(&args, "intent")?;
    let now = Utc::now();

    // First, crystallize internally to get a contract to validate
    let intent_lower = intent.to_lowercase();
    let intent_words: HashSet<String> = intent_lower
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(String::from)
        .collect();

    if intent_words.is_empty() {
        return Err("Intent must contain meaningful words".to_string());
    }

    let mut matched_patterns: Vec<(usize, f64)> = Vec::new();
    for (idx, pattern) in CRYSTALLIZATION_PATTERNS.iter().enumerate() {
        let keyword_set: HashSet<String> = pattern.keywords.iter().map(|k| k.to_string()).collect();
        let intersection = intent_words.intersection(&keyword_set).count();
        if intersection > 0 {
            let coverage = intersection as f64 / keyword_set.len() as f64;
            matched_patterns.push((idx, coverage));
        }
    }

    let mut warnings: Vec<Value> = Vec::new();
    let mut errors: Vec<Value> = Vec::new();
    let mut suggestions: Vec<Value> = Vec::new();

    // ── Check 1: No patterns matched ──
    if matched_patterns.is_empty() {
        errors.push(json!({
            "code": "NO_PATTERNS_MATCHED",
            "message": "No governance patterns matched the intent description",
            "severity": "error",
            "suggestion": "Use keywords like: budget, deploy, safe, rate, data, audit, delete, admin, external, time"
        }));
    }

    // ── Check 2: Missing approval rules ──
    let has_require_approval = matched_patterns
        .iter()
        .any(|(idx, _)| CRYSTALLIZATION_PATTERNS[*idx].action == PolicyAction::RequireApproval);
    let has_deny = matched_patterns
        .iter()
        .any(|(idx, _)| CRYSTALLIZATION_PATTERNS[*idx].action == PolicyAction::Deny);

    if has_deny && !has_require_approval {
        warnings.push(json!({
            "code": "MISSING_APPROVAL_WORKFLOW",
            "message": "Contract has deny policies but no approval workflows — agents may be completely blocked",
            "severity": "warning",
            "suggestion": "Consider adding approval gates as an alternative to blanket denials"
        }));
    }

    // ── Check 3: Conflicting scope/action combos ──
    let mut scope_action_pairs: Vec<(String, String)> = Vec::new();
    for (idx, _) in &matched_patterns {
        let pattern = &CRYSTALLIZATION_PATTERNS[*idx];
        scope_action_pairs.push((
            format!("{}", pattern.scope),
            format!("{:?}", pattern.action),
        ));
    }

    // Detect if same scope has both Allow and Deny
    let mut scope_actions: HashMap<String, HashSet<String>> = HashMap::new();
    for (scope, action) in &scope_action_pairs {
        scope_actions
            .entry(scope.clone())
            .or_default()
            .insert(action.clone());
    }
    for (scope, actions) in &scope_actions {
        if actions.contains("Allow") && actions.contains("Deny") {
            errors.push(json!({
                "code": "CONFLICTING_POLICIES",
                "message": format!("Scope '{}' has both Allow and Deny policies — undefined behavior", scope),
                "severity": "error",
                "suggestion": "Resolve by choosing either Allow or Deny per scope, or use RequireApproval as middle ground"
            }));
        }
    }

    // ── Check 4: Low coverage patterns ──
    for (idx, coverage) in &matched_patterns {
        if *coverage < 0.3 {
            let pattern = &CRYSTALLIZATION_PATTERNS[*idx];
            warnings.push(json!({
                "code": "LOW_COVERAGE_PATTERN",
                "message": format!("Pattern '{}' has low keyword coverage ({:.0}%) — may be false positive",
                    pattern.label_template, coverage * 100.0),
                "severity": "warning",
                "suggestion": format!("Add more related keywords from: {:?}", pattern.keywords)
            }));
        }
    }

    // ── Check 5: Unsupported action patterns ──
    let unsupported_keywords = ["encrypt", "backup", "replicate", "migrate", "failover"];
    for keyword in &unsupported_keywords {
        if intent_lower.contains(keyword) {
            suggestions.push(json!({
                "code": "UNSUPPORTED_ACTION",
                "message": format!("'{}' is not a supported crystallization pattern yet", keyword),
                "severity": "info",
                "suggestion": "This action may need to be configured manually in the contract"
            }));
        }
    }

    // ── Check 6: Missing risk limits for deny policies ──
    let deny_patterns_without_limits: Vec<_> = matched_patterns
        .iter()
        .filter(|(idx, _)| {
            let p = &CRYSTALLIZATION_PATTERNS[*idx];
            p.action == PolicyAction::Deny && p.risk_limit.is_none()
        })
        .collect();

    if !deny_patterns_without_limits.is_empty() {
        suggestions.push(json!({
            "code": "DENY_WITHOUT_LIMIT",
            "message": format!("{} deny policies have no associated risk limits — consider adding thresholds",
                deny_patterns_without_limits.len()),
            "severity": "info",
            "suggestion": "Adding risk limits provides nuance (allow up to a threshold) instead of blanket denial"
        }));
    }

    // ── Compute validation score ──
    let error_penalty = errors.len() as f64 * 0.3;
    let warning_penalty = warnings.len() as f64 * 0.1;
    let suggestion_penalty = suggestions.len() as f64 * 0.02;
    let validation_score =
        (1.0 - error_penalty - warning_penalty - suggestion_penalty).clamp(0.0, 1.0);

    let is_valid = errors.is_empty();

    Ok(json!({
        "intent": intent,
        "is_valid": is_valid,
        "validation_score": (validation_score * 1000.0).round() / 1000.0,
        "errors": errors,
        "warnings": warnings,
        "suggestions": suggestions,
        "patterns_matched": matched_patterns.len(),
        "patterns_total": CRYSTALLIZATION_PATTERNS.len(),
        "coverage_scores": matched_patterns.iter().map(|(idx, coverage)| json!({
            "pattern": CRYSTALLIZATION_PATTERNS[*idx].label_template,
            "coverage": (coverage * 1000.0).round() / 1000.0,
        })).collect::<Vec<_>>(),
        "validated_at": now.to_rfc3339()
    }))
}

/// Evolve a crystallized contract based on violation history — tighten where
/// violations occur, relax where there is a perfect record.
fn handle_contract_crystallize_evolve(
    args: Value,
    engine: &mut ContractEngine,
) -> Result<Value, String> {
    let intent = require_str(&args, "intent")?;
    let agent_id = require_str(&args, "agent_id")?;
    let window_days = args
        .get("window_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(30);

    let now = Utc::now();
    let cutoff = now - Duration::days(window_days);

    // Collect violations for agent within window
    let violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| v.actor == agent_id && v.detected_at >= cutoff)
        .collect();

    // Crystallize the intent first
    let intent_lower = intent.to_lowercase();
    let intent_words: HashSet<String> = intent_lower
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(String::from)
        .collect();

    let mut evolved_policies: Vec<Value> = Vec::new();
    let mut evolution_log: Vec<Value> = Vec::new();

    for pattern in CRYSTALLIZATION_PATTERNS {
        let keyword_set: HashSet<String> = pattern.keywords.iter().map(|k| k.to_string()).collect();
        let intersection = intent_words.intersection(&keyword_set).count();
        if intersection == 0 {
            continue;
        }

        let coverage = intersection as f64 / keyword_set.len() as f64;

        // Count violations related to this pattern (by word overlap with description)
        let related_violations: Vec<_> = violations
            .iter()
            .filter(|v| word_overlap(&v.description, pattern.label_template) > 0.2)
            .collect();

        let violation_count = related_violations.len();

        // Compute weighted violation score (recent violations count more)
        let weighted_score: f64 = related_violations
            .iter()
            .map(|v| {
                let age_days =
                    now.signed_duration_since(v.detected_at).num_seconds() as f64 / 86400.0;
                severity_weight(&v.severity) * exponential_decay_days(age_days, 15.0)
            })
            .sum();

        // Determine evolution direction
        let (evolved_action, direction) = if weighted_score > 1.5 {
            // Heavy violations — tighten significantly
            let tightened = match pattern.action {
                PolicyAction::Allow => PolicyAction::RequireApproval,
                PolicyAction::AuditOnly => PolicyAction::RequireApproval,
                PolicyAction::RequireApproval => PolicyAction::Deny,
                PolicyAction::Deny => PolicyAction::Deny,
            };
            (tightened, "tightened")
        } else if weighted_score > 0.5 {
            // Moderate violations — tighten slightly
            let tightened = match pattern.action {
                PolicyAction::Allow => PolicyAction::AuditOnly,
                PolicyAction::AuditOnly => PolicyAction::RequireApproval,
                PolicyAction::RequireApproval => PolicyAction::RequireApproval,
                PolicyAction::Deny => PolicyAction::Deny,
            };
            (tightened, "tightened_slightly")
        } else if violation_count == 0 && window_days >= 14 {
            // Perfect record for 2+ weeks — consider relaxing
            let relaxed = match pattern.action {
                PolicyAction::Deny => PolicyAction::RequireApproval,
                PolicyAction::RequireApproval => PolicyAction::AuditOnly,
                PolicyAction::AuditOnly => PolicyAction::AuditOnly,
                PolicyAction::Allow => PolicyAction::Allow,
            };
            (relaxed, "relaxed")
        } else {
            (pattern.action, "unchanged")
        };

        evolved_policies.push(json!({
            "label": pattern.label_template,
            "scope": format!("{}", pattern.scope),
            "original_action": format!("{:?}", pattern.action),
            "evolved_action": format!("{:?}", evolved_action),
            "direction": direction,
            "rationale": pattern.rationale,
        }));

        if direction != "unchanged" {
            evolution_log.push(json!({
                "policy": pattern.label_template,
                "direction": direction,
                "violation_count": violation_count,
                "weighted_score": (weighted_score * 1000.0).round() / 1000.0,
                "coverage": (coverage * 1000.0).round() / 1000.0,
                "reason": if direction.starts_with("tightened") {
                    format!("{} violations with weighted score {:.2} in {} days",
                        violation_count, weighted_score, window_days)
                } else {
                    format!("Zero violations in {} days — safe to relax", window_days)
                }
            }));
        }
    }

    // Compute evolution statistics
    let total_evolved = evolved_policies.len();
    let tightened_count = evolved_policies
        .iter()
        .filter(|p| {
            p["direction"]
                .as_str()
                .is_some_and(|d| d.starts_with("tightened"))
        })
        .count();
    let relaxed_count = evolved_policies
        .iter()
        .filter(|p| p["direction"] == "relaxed")
        .count();
    let unchanged_count = evolved_policies
        .iter()
        .filter(|p| p["direction"] == "unchanged")
        .count();

    // Overall health score: fewer violations and more relaxed policies = healthier
    let health_score = if total_evolved == 0 {
        0.5
    } else {
        let relaxed_ratio = relaxed_count as f64 / total_evolved as f64;
        let tightened_ratio = tightened_count as f64 / total_evolved as f64;
        (0.5 + relaxed_ratio * 0.5 - tightened_ratio * 0.3).clamp(0.0, 1.0)
    };

    Ok(json!({
        "intent": intent,
        "agent_id": agent_id,
        "window_days": window_days,
        "total_violations_in_window": violations.len(),
        "evolved_policies": evolved_policies,
        "evolution_log": evolution_log,
        "statistics": {
            "total_policies": total_evolved,
            "tightened": tightened_count,
            "relaxed": relaxed_count,
            "unchanged": unchanged_count,
        },
        "health_score": (health_score * 1000.0).round() / 1000.0,
        "evolved_at": now.to_rfc3339()
    }))
}

/// Helper: get strictness level of an action string for comparison.
fn action_strictness_level(action: &str) -> u8 {
    if action.contains("Allow") {
        0
    } else if action.contains("Audit") {
        1
    } else if action.contains("Approval") || action.contains("RequireApproval") {
        2
    } else if action.contains("Deny") {
        3
    } else {
        1 // default to audit-level
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INVENTION 7: Policy DNA
// ═══════════════════════════════════════════════════════════════════════════════

/// Extract genetic representation of a policy: scope_breadth, restriction_level,
/// tag_complexity, condition_depth, age_factor. Compute fitness from violations.
fn handle_policy_dna_extract(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let policy_id_str = require_str(&args, "policy_id")?;
    let now = Utc::now();

    // Find the policy
    let policy = engine
        .file
        .policies
        .iter()
        .find(|p| p.id.to_string() == policy_id_str)
        .ok_or_else(|| format!("Policy '{}' not found", policy_id_str))?
        .clone();

    // ── Extract genes ──

    // Gene 1: scope_breadth (Global=1.0, Session=0.5, Agent=0.2)
    let scope_gene = PolicyGene {
        name: "scope_breadth".to_string(),
        value: scope_breadth_value(&policy.scope),
        dominant: policy.scope == PolicyScope::Global,
    };

    // Gene 2: restriction_level (Deny=1.0, RequireApproval=0.7, AuditOnly=0.3, Allow=0.1)
    let restriction_gene = PolicyGene {
        name: "restriction_level".to_string(),
        value: restriction_level_value(&policy.action),
        dominant: policy.action == PolicyAction::Deny
            || policy.action == PolicyAction::RequireApproval,
    };

    // Gene 3: tag_complexity (number of tags normalized, max 1.0)
    let tag_count = policy.tags.len() as f64;
    let tag_complexity = (tag_count / 10.0).min(1.0);
    let tag_gene = PolicyGene {
        name: "tag_complexity".to_string(),
        value: tag_complexity,
        dominant: tag_count >= 5.0,
    };

    // Gene 4: condition_depth (number of conditions normalized, max 1.0)
    let condition_count = policy.conditions.len() as f64;
    let condition_depth = (condition_count / 5.0).min(1.0);
    let condition_gene = PolicyGene {
        name: "condition_depth".to_string(),
        value: condition_depth,
        dominant: condition_count >= 3.0,
    };

    // Gene 5: age_factor (exponential decay: newer policies score higher)
    let age_days = now.signed_duration_since(policy.created_at).num_seconds() as f64 / 86400.0;
    let age_value = exponential_decay_days(age_days, 365.0); // 1-year half-life
    let age_gene = PolicyGene {
        name: "age_factor".to_string(),
        value: age_value,
        dominant: age_days < 30.0,
    };

    let genes = vec![
        scope_gene,
        restriction_gene,
        tag_gene,
        condition_gene,
        age_gene,
    ];

    // ── Compute fitness from violation history ──
    // Fitness = base * decay for each violation related to this policy
    let related_violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| {
            v.policy_id
                .is_some_and(|pid| pid.to_string() == policy_id_str)
        })
        .collect();

    let base_fitness = 1.0;
    let mut fitness = base_fitness;
    for v in &related_violations {
        let v_age_days = now.signed_duration_since(v.detected_at).num_seconds() as f64 / 86400.0;
        // Each violation reduces fitness with exponential decay (30-day half-life)
        let penalty = severity_weight(&v.severity) * exponential_decay_days(v_age_days, 30.0);
        fitness -= penalty * 0.1;
    }
    fitness = fitness.clamp(0.0, 1.0);

    // ── Build DNA record ──
    let dna_id = ContractId::new();
    let dna = PolicyDna {
        id: dna_id,
        policy_id: policy.id,
        genes: genes.clone(),
        fitness,
        generation: 0,
        mutations: vec![],
        extracted_at: now,
    };

    // Store for later reference
    if let Ok(mut store) = DNA_STORE.lock() {
        store.push(dna);
    }

    let gene_vector: Vec<f64> = genes.iter().map(|g| g.value).collect();
    let dominant_traits: Vec<&str> = genes
        .iter()
        .filter(|g| g.dominant)
        .map(|g| g.name.as_str())
        .collect();
    let recessive_traits: Vec<&str> = genes
        .iter()
        .filter(|g| !g.dominant)
        .map(|g| g.name.as_str())
        .collect();

    Ok(json!({
        "dna_id": dna_id.to_string(),
        "policy_id": policy_id_str,
        "policy_label": policy.label,
        "genes": genes.iter().map(|g| json!({
            "name": g.name,
            "value": (g.value * 1000.0).round() / 1000.0,
            "dominant": g.dominant,
        })).collect::<Vec<_>>(),
        "gene_vector": gene_vector.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>(),
        "fitness": (fitness * 1000.0).round() / 1000.0,
        "generation": 0,
        "violation_count": related_violations.len(),
        "dominant_traits": dominant_traits,
        "recessive_traits": recessive_traits,
        "extracted_at": now.to_rfc3339()
    }))
}

/// Compare DNA of two policies using Euclidean distance between gene vectors.
/// Identify dominant/recessive traits.
fn handle_policy_dna_compare(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let policy_a_str = require_str(&args, "policy_a")?;
    let policy_b_str = require_str(&args, "policy_b")?;
    let now = Utc::now();

    // Find both policies
    let policy_a = engine
        .file
        .policies
        .iter()
        .find(|p| p.id.to_string() == policy_a_str)
        .ok_or_else(|| format!("Policy '{}' not found", policy_a_str))?
        .clone();
    let policy_b = engine
        .file
        .policies
        .iter()
        .find(|p| p.id.to_string() == policy_b_str)
        .ok_or_else(|| format!("Policy '{}' not found", policy_b_str))?
        .clone();

    // Extract gene vectors for both
    let genes_a = extract_gene_vector(&policy_a, now);
    let genes_b = extract_gene_vector(&policy_b, now);

    let vec_a: Vec<f64> = genes_a.iter().map(|g| g.value).collect();
    let vec_b: Vec<f64> = genes_b.iter().map(|g| g.value).collect();

    // Euclidean distance
    let distance = euclidean_distance(&vec_a, &vec_b);

    // Similarity = 1 / (1 + distance), normalized to 0.0-1.0
    let similarity = 1.0 / (1.0 + distance);

    // Per-gene comparison
    let gene_comparisons: Vec<Value> = genes_a
        .iter()
        .zip(genes_b.iter())
        .map(|(ga, gb)| {
            let diff = (ga.value - gb.value).abs();
            let dominant_in = if ga.dominant && !gb.dominant {
                "policy_a"
            } else if !ga.dominant && gb.dominant {
                "policy_b"
            } else if ga.dominant && gb.dominant {
                "both"
            } else {
                "neither"
            };
            json!({
                "gene": ga.name,
                "value_a": (ga.value * 1000.0).round() / 1000.0,
                "value_b": (gb.value * 1000.0).round() / 1000.0,
                "difference": (diff * 1000.0).round() / 1000.0,
                "dominant_in": dominant_in,
            })
        })
        .collect();

    // Identify which genes diverge most
    let mut divergence_rank: Vec<(String, f64)> = genes_a
        .iter()
        .zip(genes_b.iter())
        .map(|(ga, gb)| (ga.name.clone(), (ga.value - gb.value).abs()))
        .collect();
    divergence_rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let most_divergent = divergence_rank
        .first()
        .map(|(name, diff)| json!({"gene": name, "difference": (diff * 1000.0).round() / 1000.0}))
        .unwrap_or(json!(null));

    let most_similar = divergence_rank
        .last()
        .map(|(name, diff)| json!({"gene": name, "difference": (diff * 1000.0).round() / 1000.0}))
        .unwrap_or(json!(null));

    // Compute fitness for both
    let fitness_a = compute_policy_fitness(engine, policy_a_str, now);
    let fitness_b = compute_policy_fitness(engine, policy_b_str, now);

    // Relationship classification
    let relationship = if similarity > 0.9 {
        "near_identical"
    } else if similarity > 0.7 {
        "closely_related"
    } else if similarity > 0.4 {
        "distantly_related"
    } else {
        "unrelated"
    };

    Ok(json!({
        "policy_a": {
            "id": policy_a_str,
            "label": policy_a.label,
            "fitness": (fitness_a * 1000.0).round() / 1000.0,
        },
        "policy_b": {
            "id": policy_b_str,
            "label": policy_b.label,
            "fitness": (fitness_b * 1000.0).round() / 1000.0,
        },
        "distance": (distance * 1000.0).round() / 1000.0,
        "similarity": (similarity * 1000.0).round() / 1000.0,
        "relationship": relationship,
        "gene_comparisons": gene_comparisons,
        "most_divergent_gene": most_divergent,
        "most_similar_gene": most_similar,
        "compared_at": now.to_rfc3339()
    }))
}

/// Simulate policy mutations: random gene perturbation, optional crossover,
/// and fitness recalculation.
fn handle_policy_dna_mutate(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let policy_id_str = require_str(&args, "policy_id")?;
    let mutation_rate = args
        .get("mutation_rate")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.3);
    let crossover_id = args.get("crossover_policy_id").and_then(|v| v.as_str());
    let now = Utc::now();

    // Find the primary policy
    let policy = engine
        .file
        .policies
        .iter()
        .find(|p| p.id.to_string() == policy_id_str)
        .ok_or_else(|| format!("Policy '{}' not found", policy_id_str))?
        .clone();

    let original_genes = extract_gene_vector(&policy, now);
    let original_fitness = compute_policy_fitness(engine, policy_id_str, now);

    // Optional crossover parent
    let crossover_genes = if let Some(cross_id) = crossover_id {
        let cross_policy = engine
            .file
            .policies
            .iter()
            .find(|p| p.id.to_string() == cross_id)
            .ok_or_else(|| format!("Crossover policy '{}' not found", cross_id))?
            .clone();
        Some(extract_gene_vector(&cross_policy, now))
    } else {
        None
    };

    // Deterministic seed from policy ID for reproducibility
    let mut seed: u32 = policy_id_str
        .bytes()
        .fold(42u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));

    let mut mutated_genes: Vec<PolicyGene> = Vec::new();
    let mut mutations: Vec<PolicyMutation> = Vec::new();

    for (idx, gene) in original_genes.iter().enumerate() {
        let mut new_value = gene.value;

        // ── Step 1: Crossover (if parent provided) ──
        if let Some(ref cross) = crossover_genes {
            // Single-point crossover: take from crossover parent if past crossover point
            let crossover_point = (pseudo_random(&mut seed) * original_genes.len() as f64) as usize;
            if idx >= crossover_point {
                new_value = cross[idx].value;
            }
        }

        // ── Step 2: Gaussian mutation ──
        let should_mutate = pseudo_random(&mut seed) < mutation_rate;
        if should_mutate {
            let perturbation = gaussian_random(&mut seed) * 0.15; // std dev = 0.15
            let old_value = new_value;
            new_value = (new_value + perturbation).clamp(0.0, 1.0);

            mutations.push(PolicyMutation {
                gene_name: gene.name.clone(),
                old_value,
                new_value,
                beneficial: false, // Will be determined after fitness check
            });
        }

        mutated_genes.push(PolicyGene {
            name: gene.name.clone(),
            value: new_value,
            dominant: new_value > 0.5,
        });
    }

    // ── Compute new fitness ──
    // Simulate fitness: policies with moderate genes tend to be fitter
    // (extreme values are risky — too permissive or too restrictive)
    let gene_values: Vec<f64> = mutated_genes.iter().map(|g| g.value).collect();
    let variance: f64 =
        gene_values.iter().map(|v| (v - 0.5).powi(2)).sum::<f64>() / gene_values.len() as f64;
    let balance_factor = 1.0 - variance; // Higher when genes are moderate
    let mutated_fitness = (original_fitness * 0.6 + balance_factor * 0.4).min(1.0);

    // Mark beneficial mutations
    let fitness_improved = mutated_fitness > original_fitness;
    for m in mutations.iter_mut() {
        m.beneficial = fitness_improved;
    }

    // Store the mutated DNA
    let dna_id = ContractId::new();
    let mutated_dna = PolicyDna {
        id: dna_id,
        policy_id: policy.id,
        genes: mutated_genes.clone(),
        fitness: mutated_fitness,
        generation: 1,
        mutations: mutations.clone(),
        extracted_at: now,
    };

    if let Ok(mut store) = DNA_STORE.lock() {
        store.push(mutated_dna);
    }

    let original_vec: Vec<f64> = original_genes.iter().map(|g| g.value).collect();
    let mutated_vec: Vec<f64> = mutated_genes.iter().map(|g| g.value).collect();
    let mutation_distance = euclidean_distance(&original_vec, &mutated_vec);

    Ok(json!({
        "dna_id": dna_id.to_string(),
        "policy_id": policy_id_str,
        "policy_label": policy.label,
        "mutation_rate": mutation_rate,
        "crossover_applied": crossover_id.is_some(),
        "original_genes": original_genes.iter().map(|g| json!({
            "name": g.name,
            "value": (g.value * 1000.0).round() / 1000.0,
        })).collect::<Vec<_>>(),
        "mutated_genes": mutated_genes.iter().map(|g| json!({
            "name": g.name,
            "value": (g.value * 1000.0).round() / 1000.0,
            "dominant": g.dominant,
        })).collect::<Vec<_>>(),
        "mutations": mutations.iter().map(|m| json!({
            "gene_name": m.gene_name,
            "old_value": (m.old_value * 1000.0).round() / 1000.0,
            "new_value": (m.new_value * 1000.0).round() / 1000.0,
            "beneficial": m.beneficial,
        })).collect::<Vec<_>>(),
        "original_fitness": (original_fitness * 1000.0).round() / 1000.0,
        "mutated_fitness": (mutated_fitness * 1000.0).round() / 1000.0,
        "fitness_change": ((mutated_fitness - original_fitness) * 1000.0).round() / 1000.0,
        "fitness_improved": fitness_improved,
        "mutation_distance": (mutation_distance * 1000.0).round() / 1000.0,
        "generation": 1,
        "mutated_at": now.to_rfc3339()
    }))
}

/// Run genetic algorithm: tournament selection, crossover, mutation over
/// N generations. Track fitness improvement per generation.
fn handle_policy_dna_evolve(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let generations = args
        .get("generations")
        .and_then(|v| v.as_i64())
        .unwrap_or(10) as usize;
    let population_size = args
        .get("population_size")
        .and_then(|v| v.as_i64())
        .unwrap_or(20) as usize;
    let tournament_size = args
        .get("tournament_size")
        .and_then(|v| v.as_i64())
        .unwrap_or(3) as usize;
    let mutation_rate = args
        .get("mutation_rate")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.2);

    let now = Utc::now();

    if engine.file.policies.is_empty() {
        return Err("No policies to evolve — add policies first".to_string());
    }

    if population_size < 2 {
        return Err("Population size must be at least 2".to_string());
    }

    if tournament_size < 1 || tournament_size > population_size {
        return Err(format!(
            "Tournament size must be between 1 and {} (population size)",
            population_size
        ));
    }

    let mut seed: u32 = 12345u32;

    // ── Initialize population ──
    // Create initial population from existing policies (with variation)
    let gene_names = [
        "scope_breadth",
        "restriction_level",
        "tag_complexity",
        "condition_depth",
        "age_factor",
    ];
    let num_genes = gene_names.len();

    struct Individual {
        genes: Vec<f64>,
        fitness: f64,
    }

    // Seed population from existing policies
    let mut population: Vec<Individual> = Vec::with_capacity(population_size);
    for i in 0..population_size {
        let policy_idx = i % engine.file.policies.len();
        let policy = &engine.file.policies[policy_idx];
        let base_genes = extract_gene_vector(policy, now);
        let mut genes: Vec<f64> = base_genes.iter().map(|g| g.value).collect();

        // Add random variation to create diversity
        for gene in genes.iter_mut() {
            let perturbation = gaussian_random(&mut seed) * 0.2;
            *gene = (*gene + perturbation).clamp(0.0, 1.0);
        }

        // Fitness based on gene balance (moderate genes = fitter)
        let variance: f64 = genes.iter().map(|v| (v - 0.5).powi(2)).sum::<f64>() / num_genes as f64;
        let fitness = (1.0 - variance).max(0.0);

        population.push(Individual { genes, fitness });
    }

    // ── Evolution loop ──
    let mut generation_stats: Vec<Value> = Vec::new();
    let initial_best_fitness = population
        .iter()
        .map(|ind| ind.fitness)
        .fold(0.0f64, f64::max);

    for gen in 0..generations {
        let mut new_population: Vec<Individual> = Vec::with_capacity(population_size);

        // Elitism: keep the best individual
        let best_idx = population
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.fitness.partial_cmp(&b.fitness).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        new_population.push(Individual {
            genes: population[best_idx].genes.clone(),
            fitness: population[best_idx].fitness,
        });

        // Extract fitnesses for tournament selection
        let fitnesses: Vec<f64> = population.iter().map(|ind| ind.fitness).collect();

        // Fill the rest with offspring
        while new_population.len() < population_size {
            // ── Tournament selection for parent 1 ──
            let parent1_idx = tournament_select(&fitnesses, tournament_size, &mut seed);

            // ── Tournament selection for parent 2 ──
            let mut parent2_idx = tournament_select(&fitnesses, tournament_size, &mut seed);
            // Ensure different parents
            if parent2_idx == parent1_idx {
                parent2_idx = (parent2_idx + 1) % population.len();
            }

            // ── Single-point crossover ──
            let crossover_point = (pseudo_random(&mut seed) * num_genes as f64) as usize;
            let mut child_genes: Vec<f64> = Vec::with_capacity(num_genes);
            for g in 0..num_genes {
                if g < crossover_point {
                    child_genes.push(population[parent1_idx].genes[g]);
                } else {
                    child_genes.push(population[parent2_idx].genes[g]);
                }
            }

            // ── Gaussian mutation ──
            for gene in child_genes.iter_mut() {
                if pseudo_random(&mut seed) < mutation_rate {
                    let perturbation = gaussian_random(&mut seed) * 0.1;
                    *gene = (*gene + perturbation).clamp(0.0, 1.0);
                }
            }

            // ── Fitness evaluation ──
            // Fitness rewards:
            // 1. Balance (moderate genes)
            // 2. Having restriction > 0.3 (not too permissive)
            // 3. Having scope > 0.3 (not too narrow)
            let variance: f64 =
                child_genes.iter().map(|v| (v - 0.5).powi(2)).sum::<f64>() / num_genes as f64;
            let balance_score = 1.0 - variance;
            let restriction = child_genes.get(1).copied().unwrap_or(0.5);
            let scope = child_genes.first().copied().unwrap_or(0.5);
            let security_bonus = if restriction > 0.3 { 0.1 } else { 0.0 };
            let coverage_bonus = if scope > 0.3 { 0.05 } else { 0.0 };
            let fitness = (balance_score * 0.7
                + restriction * 0.15
                + scope * 0.15
                + security_bonus
                + coverage_bonus)
                .min(1.0);

            new_population.push(Individual {
                genes: child_genes,
                fitness,
            });
        }

        population = new_population;

        // Generation statistics
        let fitnesses: Vec<f64> = population.iter().map(|ind| ind.fitness).collect();
        let gen_best = fitnesses.iter().fold(0.0f64, |a, &b| a.max(b));
        let gen_avg = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        let gen_worst = fitnesses.iter().fold(1.0f64, |a, &b| a.min(b));

        generation_stats.push(json!({
            "generation": gen + 1,
            "best_fitness": (gen_best * 1000.0).round() / 1000.0,
            "avg_fitness": (gen_avg * 1000.0).round() / 1000.0,
            "worst_fitness": (gen_worst * 1000.0).round() / 1000.0,
            "improvement_from_initial": ((gen_best - initial_best_fitness) * 1000.0).round() / 1000.0,
        }));
    }

    // ── Extract best individual ──
    let best = population
        .iter()
        .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap();

    let best_genes: Vec<Value> = gene_names
        .iter()
        .zip(best.genes.iter())
        .map(|(name, value)| {
            json!({
                "name": name,
                "value": (value * 1000.0).round() / 1000.0,
                "dominant": value > &0.5,
            })
        })
        .collect();

    let final_best_fitness = best.fitness;
    let total_improvement = final_best_fitness - initial_best_fitness;

    // ── Recommendations ──
    let mut recommendations: Vec<String> = Vec::new();
    if best.genes[1] > 0.8 {
        recommendations.push(
            "Evolved toward high restriction — consider if this matches operational needs"
                .to_string(),
        );
    }
    if best.genes[0] < 0.3 {
        recommendations
            .push("Evolved toward narrow scope — may miss global-level threats".to_string());
    }
    if total_improvement < 0.01 {
        recommendations.push(
            "Minimal improvement suggests current policies are already near-optimal".to_string(),
        );
    }
    if total_improvement > 0.2 {
        recommendations.push(
            "Significant improvement found — consider applying evolved gene values to policies"
                .to_string(),
        );
    }

    Ok(json!({
        "generations": generations,
        "population_size": population_size,
        "tournament_size": tournament_size,
        "mutation_rate": mutation_rate,
        "initial_best_fitness": (initial_best_fitness * 1000.0).round() / 1000.0,
        "final_best_fitness": (final_best_fitness * 1000.0).round() / 1000.0,
        "total_improvement": (total_improvement * 1000.0).round() / 1000.0,
        "best_genes": best_genes,
        "generation_stats": generation_stats,
        "recommendations": recommendations,
        "converged": total_improvement.abs() < 0.001 && generations > 5,
        "evolved_at": now.to_rfc3339()
    }))
}

/// Trace policy evolution lineage: find related policies by DNA similarity,
/// build a family tree showing genetic relationships.
fn handle_policy_dna_lineage(args: Value, engine: &mut ContractEngine) -> Result<Value, String> {
    let policy_id_str = require_str(&args, "policy_id")?;
    let similarity_threshold = args
        .get("similarity_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.6);
    let now = Utc::now();

    // Verify target policy exists
    let target_policy = engine
        .file
        .policies
        .iter()
        .find(|p| p.id.to_string() == policy_id_str)
        .ok_or_else(|| format!("Policy '{}' not found", policy_id_str))?
        .clone();

    let target_genes = extract_gene_vector(&target_policy, now);
    let target_vec: Vec<f64> = target_genes.iter().map(|g| g.value).collect();

    // ── Compare against all other policies ──
    let mut relatives: Vec<Value> = Vec::new();
    let mut family_tree: Vec<Value> = Vec::new();

    for policy in &engine.file.policies {
        if policy.id.to_string() == policy_id_str {
            continue;
        }

        let genes = extract_gene_vector(policy, now);
        let vec: Vec<f64> = genes.iter().map(|g| g.value).collect();

        let distance = euclidean_distance(&target_vec, &vec);
        let similarity = 1.0 / (1.0 + distance);

        if similarity >= similarity_threshold {
            // Determine relationship type based on similarity and age
            let target_age = now
                .signed_duration_since(target_policy.created_at)
                .num_days();
            let policy_age = now.signed_duration_since(policy.created_at).num_days();

            let relationship = if similarity > 0.95 {
                "clone"
            } else if similarity > 0.85 {
                if policy_age > target_age {
                    "ancestor"
                } else {
                    "descendant"
                }
            } else if similarity > 0.7 {
                "sibling"
            } else {
                "cousin"
            };

            // Find shared and divergent traits
            let shared_dominant: Vec<String> = genes
                .iter()
                .zip(target_genes.iter())
                .filter(|(g, tg)| g.dominant && tg.dominant)
                .map(|(g, _)| g.name.clone())
                .collect();

            let divergent_genes: Vec<Value> = genes
                .iter()
                .zip(target_genes.iter())
                .filter(|(g, tg)| (g.value - tg.value).abs() > 0.2)
                .map(|(g, tg)| {
                    json!({
                        "gene": g.name,
                        "target_value": (tg.value * 1000.0).round() / 1000.0,
                        "relative_value": (g.value * 1000.0).round() / 1000.0,
                    })
                })
                .collect();

            let fitness = compute_policy_fitness(engine, &policy.id.to_string(), now);

            let relative = json!({
                "policy_id": policy.id.to_string(),
                "policy_label": policy.label,
                "relationship": relationship,
                "similarity": (similarity * 1000.0).round() / 1000.0,
                "distance": (distance * 1000.0).round() / 1000.0,
                "fitness": (fitness * 1000.0).round() / 1000.0,
                "shared_dominant_traits": shared_dominant,
                "divergent_genes": divergent_genes,
                "age_days": policy_age,
            });

            relatives.push(relative.clone());

            family_tree.push(json!({
                "node": policy.id.to_string(),
                "label": policy.label,
                "relationship": relationship,
                "similarity": (similarity * 1000.0).round() / 1000.0,
                "depth": match relationship {
                    "clone" => 0,
                    "ancestor" | "descendant" => 1,
                    "sibling" => 1,
                    _ => 2,
                },
            }));
        }
    }

    // Sort by similarity (closest relatives first)
    relatives.sort_by(|a, b| {
        let sim_a = a["similarity"].as_f64().unwrap_or(0.0);
        let sim_b = b["similarity"].as_f64().unwrap_or(0.0);
        sim_b
            .partial_cmp(&sim_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // ── Lineage statistics ──
    let total_policies = engine.file.policies.len();
    let related_count = relatives.len();
    let isolation_score = 1.0 - (related_count as f64 / total_policies.max(1) as f64);

    let avg_similarity = if relatives.is_empty() {
        0.0
    } else {
        relatives
            .iter()
            .map(|r| r["similarity"].as_f64().unwrap_or(0.0))
            .sum::<f64>()
            / relatives.len() as f64
    };

    let target_fitness = compute_policy_fitness(engine, policy_id_str, now);
    let target_age = now
        .signed_duration_since(target_policy.created_at)
        .num_days();

    Ok(json!({
        "policy_id": policy_id_str,
        "policy_label": target_policy.label,
        "target_genes": target_genes.iter().map(|g| json!({
            "name": g.name,
            "value": (g.value * 1000.0).round() / 1000.0,
            "dominant": g.dominant,
        })).collect::<Vec<_>>(),
        "target_fitness": (target_fitness * 1000.0).round() / 1000.0,
        "target_age_days": target_age,
        "similarity_threshold": similarity_threshold,
        "relatives": relatives,
        "family_tree": family_tree,
        "lineage_statistics": {
            "total_policies": total_policies,
            "related_count": related_count,
            "isolation_score": (isolation_score * 1000.0).round() / 1000.0,
            "avg_similarity": (avg_similarity * 1000.0).round() / 1000.0,
        },
        "traced_at": now.to_rfc3339()
    }))
}

// ─── DNA helper functions ────────────────────────────────────────────────────

/// Extract gene vector from a policy (reusable).
fn extract_gene_vector(policy: &agentic_contract::Policy, now: DateTime<Utc>) -> Vec<PolicyGene> {
    let age_days = now.signed_duration_since(policy.created_at).num_seconds() as f64 / 86400.0;

    vec![
        PolicyGene {
            name: "scope_breadth".to_string(),
            value: scope_breadth_value(&policy.scope),
            dominant: policy.scope == PolicyScope::Global,
        },
        PolicyGene {
            name: "restriction_level".to_string(),
            value: restriction_level_value(&policy.action),
            dominant: policy.action == PolicyAction::Deny
                || policy.action == PolicyAction::RequireApproval,
        },
        PolicyGene {
            name: "tag_complexity".to_string(),
            value: (policy.tags.len() as f64 / 10.0).min(1.0),
            dominant: policy.tags.len() >= 5,
        },
        PolicyGene {
            name: "condition_depth".to_string(),
            value: (policy.conditions.len() as f64 / 5.0).min(1.0),
            dominant: policy.conditions.len() >= 3,
        },
        PolicyGene {
            name: "age_factor".to_string(),
            value: exponential_decay_days(age_days, 365.0),
            dominant: age_days < 30.0,
        },
    ]
}

/// Compute fitness of a policy from its violation history.
fn compute_policy_fitness(engine: &ContractEngine, policy_id_str: &str, now: DateTime<Utc>) -> f64 {
    let related_violations: Vec<_> = engine
        .file
        .violations
        .iter()
        .filter(|v| {
            v.policy_id
                .is_some_and(|pid| pid.to_string() == policy_id_str)
        })
        .collect();

    let mut fitness = 1.0;
    for v in &related_violations {
        let v_age_days = now.signed_duration_since(v.detected_at).num_seconds() as f64 / 86400.0;
        let penalty = severity_weight(&v.severity) * exponential_decay_days(v_age_days, 30.0);
        fitness -= penalty * 0.1;
    }
    fitness.clamp(0.0, 1.0)
}

/// Tournament selection: pick the best of k random individuals from a population.
/// Returns the index of the winner.
fn tournament_select(fitnesses: &[f64], tournament_size: usize, seed: &mut u32) -> usize {
    let pop_size = fitnesses.len();
    let mut best_idx = (pseudo_random(seed) * pop_size as f64) as usize % pop_size;
    let mut best_fitness = fitnesses[best_idx];

    for _ in 1..tournament_size {
        let candidate = (pseudo_random(seed) * pop_size as f64) as usize % pop_size;
        if fitnesses[candidate] > best_fitness {
            best_idx = candidate;
            best_fitness = fitnesses[candidate];
        }
    }

    best_idx
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use agentic_contract::policy::{PolicyAction, PolicyScope};
    use agentic_contract::{ContractEngine, Policy};
    use serde_json::json;

    #[test]
    fn test_crystallize_budget_intent() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_crystallize",
            json!({"intent": "keep budget under control with spending limits"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert!(value["patterns_matched"].as_u64().unwrap() > 0);
        assert!(value["confidence"].as_f64().unwrap() > 0.0);
        let policies = value["policies"].as_array().unwrap();
        assert!(!policies.is_empty());
        // Should contain a spending-related policy
        let has_spending = policies
            .iter()
            .any(|p| p["label"].as_str().unwrap().contains("Spending"));
        assert!(has_spending);
    }

    #[test]
    fn test_crystallize_empty_intent_fails() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_crystallize",
            json!({"intent": "a b"}),
            &mut engine,
        );
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_crystallize_validate_no_match() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_crystallize_validate",
            json!({"intent": "completely unrelated nonsense words here"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["is_valid"], false);
        let errors = value["errors"].as_array().unwrap();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_crystallize_evolve_no_violations() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_crystallize_evolve",
            json!({
                "intent": "safe deployment with budget control",
                "agent_id": "agent-1",
                "window_days": 30
            }),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["total_violations_in_window"], 0);
        // With no violations and window >= 14, should see relaxed policies
        let stats = &value["statistics"];
        assert!(
            stats["relaxed"].as_u64().unwrap_or(0) > 0
                || stats["unchanged"].as_u64().unwrap_or(0) > 0
        );
    }

    #[test]
    fn test_dna_extract_basic() {
        let mut engine = ContractEngine::new();
        let policy = Policy::new("Test policy", PolicyScope::Global, PolicyAction::Deny);
        let pid = policy.id.to_string();
        engine.add_policy(policy);

        let result = try_handle("policy_dna_extract", json!({"policy_id": pid}), &mut engine);
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["policy_label"], "Test policy");
        let genes = value["genes"].as_array().unwrap();
        assert_eq!(genes.len(), 5);
        // Global scope should have breadth = 1.0
        let scope_gene = &genes[0];
        assert_eq!(scope_gene["name"], "scope_breadth");
        assert!((scope_gene["value"].as_f64().unwrap() - 1.0).abs() < 0.01);
        // Deny should have restriction = 1.0
        let restriction_gene = &genes[1];
        assert_eq!(restriction_gene["name"], "restriction_level");
        assert!((restriction_gene["value"].as_f64().unwrap() - 1.0).abs() < 0.01);
        // Fitness should be 1.0 (no violations)
        assert!((value["fitness"].as_f64().unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_dna_compare_same_policy_type() {
        let mut engine = ContractEngine::new();
        let p1 = Policy::new("Policy A", PolicyScope::Global, PolicyAction::Deny);
        let p2 = Policy::new("Policy B", PolicyScope::Global, PolicyAction::Deny);
        let id1 = p1.id.to_string();
        let id2 = p2.id.to_string();
        engine.add_policy(p1);
        engine.add_policy(p2);

        let result = try_handle(
            "policy_dna_compare",
            json!({"policy_a": id1, "policy_b": id2}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        // Same scope and action should yield high similarity
        let similarity = value["similarity"].as_f64().unwrap();
        assert!(
            similarity > 0.8,
            "Expected high similarity, got {}",
            similarity
        );
        assert!(
            value["relationship"].as_str().unwrap() == "near_identical"
                || value["relationship"].as_str().unwrap() == "closely_related"
        );
    }

    #[test]
    fn test_dna_compare_different_policies() {
        let mut engine = ContractEngine::new();
        let p1 = Policy::new("Strict Policy", PolicyScope::Global, PolicyAction::Deny);
        let mut p2 = Policy::new("Loose Policy", PolicyScope::Agent, PolicyAction::Allow);
        p2.tags = vec![
            "tag1".into(),
            "tag2".into(),
            "tag3".into(),
            "tag4".into(),
            "tag5".into(),
        ];
        p2.conditions = vec!["cond1".into(), "cond2".into(), "cond3".into()];
        let id1 = p1.id.to_string();
        let id2 = p2.id.to_string();
        engine.add_policy(p1);
        engine.add_policy(p2);

        let result = try_handle(
            "policy_dna_compare",
            json!({"policy_a": id1, "policy_b": id2}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        let similarity = value["similarity"].as_f64().unwrap();
        assert!(
            similarity < 0.8,
            "Expected lower similarity for different policies, got {}",
            similarity
        );
    }

    #[test]
    fn test_dna_mutate_basic() {
        let mut engine = ContractEngine::new();
        let policy = Policy::new(
            "Mutable Policy",
            PolicyScope::Session,
            PolicyAction::RequireApproval,
        );
        let pid = policy.id.to_string();
        engine.add_policy(policy);

        let result = try_handle(
            "policy_dna_mutate",
            json!({"policy_id": pid, "mutation_rate": 0.5}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["generation"], 1);
        assert!(value["mutation_distance"].as_f64().unwrap() >= 0.0);
        let mutations = value["mutations"].as_array().unwrap();
        // With 50% mutation rate and 5 genes, we expect some mutations
        assert!(!mutations.is_empty() || value["mutation_distance"].as_f64().unwrap() == 0.0);
    }

    #[test]
    fn test_dna_evolve_basic() {
        let mut engine = ContractEngine::new();
        engine.add_policy(Policy::new(
            "Base Policy",
            PolicyScope::Global,
            PolicyAction::Deny,
        ));
        engine.add_policy(Policy::new(
            "Alt Policy",
            PolicyScope::Agent,
            PolicyAction::Allow,
        ));

        let result = try_handle(
            "policy_dna_evolve",
            json!({"generations": 5, "population_size": 10, "tournament_size": 3}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["generations"], 5);
        let gen_stats = value["generation_stats"].as_array().unwrap();
        assert_eq!(gen_stats.len(), 5);
        // Best fitness should be non-negative
        assert!(value["final_best_fitness"].as_f64().unwrap() >= 0.0);
        assert!(value["best_genes"].as_array().unwrap().len() == 5);
    }

    #[test]
    fn test_dna_evolve_empty_policies_fails() {
        let mut engine = ContractEngine::new();
        let result = try_handle("policy_dna_evolve", json!({"generations": 3}), &mut engine);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_dna_lineage_basic() {
        let mut engine = ContractEngine::new();
        let p1 = Policy::new("Root Policy", PolicyScope::Global, PolicyAction::Deny);
        let p2 = Policy::new("Similar Policy", PolicyScope::Global, PolicyAction::Deny);
        let p3 = Policy::new("Different Policy", PolicyScope::Agent, PolicyAction::Allow);
        let root_id = p1.id.to_string();
        engine.add_policy(p1);
        engine.add_policy(p2);
        engine.add_policy(p3);

        let result = try_handle(
            "policy_dna_lineage",
            json!({"policy_id": root_id, "similarity_threshold": 0.5}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        assert_eq!(value["policy_label"], "Root Policy");
        let relatives = value["relatives"].as_array().unwrap();
        // The similar policy should appear; the different one may or may not
        assert!(!relatives.is_empty());
        let stats = &value["lineage_statistics"];
        assert!(stats["total_policies"].as_u64().unwrap() == 3);
    }

    #[test]
    fn test_dna_lineage_not_found() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "policy_dna_lineage",
            json!({"policy_id": "nonexistent"}),
            &mut engine,
        );
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_word_overlap_identical() {
        let sim = word_overlap("hello world test", "hello world test");
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_word_overlap_disjoint() {
        let sim = word_overlap("alpha beta gamma", "delta epsilon zeta");
        assert!(sim < 0.01);
    }

    #[test]
    fn test_euclidean_distance_same() {
        let d = euclidean_distance(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]);
        assert!(d.abs() < 1e-10);
    }

    #[test]
    fn test_euclidean_distance_known() {
        let d = euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]);
        assert!((d - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_exponential_decay() {
        // At half-life, decay should be ~0.5
        let decay = exponential_decay_days(30.0, 30.0);
        assert!((decay - 0.5).abs() < 0.01);
        // At t=0, decay should be 1.0
        let decay_zero = exponential_decay_days(0.0, 30.0);
        assert!((decay_zero - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_pseudo_random_deterministic() {
        let mut seed1 = 42u32;
        let mut seed2 = 42u32;
        let v1 = pseudo_random(&mut seed1);
        let v2 = pseudo_random(&mut seed2);
        assert!((v1 - v2).abs() < 1e-10);
    }

    #[test]
    fn test_crystallize_strictness_permissive() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_crystallize",
            json!({"intent": "safe deployment", "strictness": "permissive"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        let policies = value["policies"].as_array().unwrap();
        // Permissive should downgrade Deny to RequireApproval
        for p in policies {
            let action = p["action"].as_str().unwrap();
            assert_ne!(
                action, "Deny",
                "Permissive should not produce Deny policies"
            );
        }
    }

    #[test]
    fn test_crystallize_strictness_restrictive() {
        let mut engine = ContractEngine::new();
        let result = try_handle(
            "contract_crystallize",
            json!({"intent": "audit and monitor everything", "strictness": "restrictive"}),
            &mut engine,
        );
        assert!(result.is_some());
        let value = result.unwrap().unwrap();
        let policies = value["policies"].as_array().unwrap();
        // Restrictive should upgrade AuditOnly to RequireApproval
        for p in policies {
            let action = p["action"].as_str().unwrap();
            assert_ne!(action, "AuditOnly", "Restrictive should upgrade AuditOnly");
        }
    }
}

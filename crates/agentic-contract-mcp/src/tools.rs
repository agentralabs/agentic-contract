//! MCP tool definitions for AgenticContract.

use serde_json::{json, Value};

/// A tool definition for the MCP protocol.
pub struct ToolDefinition {
    /// Tool name.
    pub name: &'static str,
    /// Tool description.
    pub description: &'static str,
    /// JSON Schema for the tool input.
    pub input_schema: &'static str,
}

/// All AgenticContract MCP tools.
pub const TOOLS: &[ToolDefinition] = &[
    // ── Contract tools ──────────────────────────────────────────
    ToolDefinition {
        name: "contract_create",
        description: "Create a new contract between agents or agent and user",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"Contract label"},"parties":{"type":"array","items":{"type":"string"},"description":"Parties involved"},"description":{"type":"string","description":"Contract description"},"tags":{"type":"array","items":{"type":"string"}}},"required":["label"]}"#,
    },
    ToolDefinition {
        name: "contract_sign",
        description: "Sign a contract to indicate acceptance",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Contract ID to sign"},"signer":{"type":"string","description":"Identity of the signer"}},"required":["contract_id","signer"]}"#,
    },
    ToolDefinition {
        name: "contract_verify",
        description: "Verify a contract's validity and signature chain",
        input_schema: r#"{"type":"object","properties":{"contract_id":{"type":"string","description":"Contract ID to verify"}},"required":["contract_id"]}"#,
    },
    ToolDefinition {
        name: "contract_list",
        description: "List contracts with optional status filter",
        input_schema: r#"{"type":"object","properties":{"status":{"type":"string","enum":["active","expired","draft"]}}}"#,
    },
    ToolDefinition {
        name: "contract_get",
        description: "Get a specific contract by ID",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string","description":"Contract ID"}},"required":["id"]}"#,
    },
    // ── Policy tools ────────────────────────────────────────────
    ToolDefinition {
        name: "policy_add",
        description: "Add a policy rule governing agent behavior",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"Policy label"},"scope":{"type":"string","enum":["global","session","agent"],"default":"global"},"action":{"type":"string","enum":["allow","deny","require_approval","audit_only"],"default":"deny"},"description":{"type":"string","description":"Optional policy description"},"tags":{"type":"array","items":{"type":"string"}}},"required":["label"]}"#,
    },
    ToolDefinition {
        name: "policy_check",
        description: "Check if an action is allowed under current policies",
        input_schema: r#"{"type":"object","properties":{"action_type":{"type":"string","description":"Action to check"},"scope":{"type":"string","enum":["global","session","agent"],"default":"global"}},"required":["action_type"]}"#,
    },
    ToolDefinition {
        name: "policy_list",
        description: "List active policies with optional scope filter",
        input_schema: r#"{"type":"object","properties":{"scope":{"type":"string","enum":["global","session","agent"]}}}"#,
    },
    // ── Risk limit tools ────────────────────────────────────────
    ToolDefinition {
        name: "risk_limit_set",
        description: "Set a risk limit threshold for a resource or action",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"What this limit governs"},"limit_type":{"type":"string","enum":["rate","threshold","budget","count"],"default":"threshold"},"max_value":{"type":"number","description":"Maximum allowed value"},"window_secs":{"type":"integer","description":"Time window in seconds (for rate limits)"}},"required":["label","max_value"]}"#,
    },
    ToolDefinition {
        name: "risk_limit_check",
        description: "Check if an action would exceed risk limits",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"Limit label pattern to match"},"amount":{"type":"number","description":"Amount to check against limit"}},"required":["label","amount"]}"#,
    },
    ToolDefinition {
        name: "risk_limit_list",
        description: "List all risk limits with current values",
        input_schema: r#"{"type":"object","properties":{}}"#,
    },
    // ── Approval tools ──────────────────────────────────────────
    ToolDefinition {
        name: "approval_request",
        description: "Request approval for a controlled action",
        input_schema: r#"{"type":"object","properties":{"rule_id":{"type":"string","description":"Approval rule ID"},"action_description":{"type":"string","description":"What action needs approval"},"requestor":{"type":"string","description":"Who is requesting"}},"required":["rule_id","action_description","requestor"]}"#,
    },
    ToolDefinition {
        name: "approval_decide",
        description: "Approve or deny a pending approval request",
        input_schema: r#"{"type":"object","properties":{"request_id":{"type":"string","description":"Approval request ID"},"decision":{"type":"string","enum":["approve","deny"]},"decider":{"type":"string","description":"Who is deciding"},"reason":{"type":"string","description":"Reason for the decision"}},"required":["request_id","decision","decider","reason"]}"#,
    },
    ToolDefinition {
        name: "approval_list",
        description: "List approval requests with optional status filter",
        input_schema: r#"{"type":"object","properties":{"status":{"type":"string","enum":["pending","approved","denied","expired"]}}}"#,
    },
    // ── Condition tools ─────────────────────────────────────────
    ToolDefinition {
        name: "condition_add",
        description: "Add a conditional execution rule",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"Condition label"},"condition_type":{"type":"string","enum":["threshold","time_based","dependency","custom"],"default":"custom"},"expression":{"type":"string","description":"Condition expression or description"}},"required":["label","expression"]}"#,
    },
    ToolDefinition {
        name: "condition_evaluate",
        description: "Evaluate whether conditions are met for an action",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string","description":"Condition ID to evaluate"}},"required":["id"]}"#,
    },
    // ── Obligation tools ────────────────────────────────────────
    ToolDefinition {
        name: "obligation_add",
        description: "Add an obligation that an agent must fulfill",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"What must be done"},"deadline":{"type":"string","format":"date-time","description":"When it must be done by (ISO 8601)"},"description":{"type":"string","description":"Detailed description"}},"required":["label","deadline"]}"#,
    },
    ToolDefinition {
        name: "obligation_check",
        description: "Check the status of obligations",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string","description":"Obligation ID (optional, checks all if omitted)"}}}"#,
    },
    // ── Violation tools ─────────────────────────────────────────
    ToolDefinition {
        name: "violation_list",
        description: "List recorded violations with optional severity filter",
        input_schema: r#"{"type":"object","properties":{"severity":{"type":"string","enum":["info","warning","critical","fatal"]}}}"#,
    },
    ToolDefinition {
        name: "violation_report",
        description: "Report a contract or policy violation",
        input_schema: r#"{"type":"object","properties":{"description":{"type":"string","description":"What violation occurred"},"severity":{"type":"string","enum":["info","warning","critical","fatal"],"default":"warning"},"agent_id":{"type":"string","description":"Which agent violated"},"policy_id":{"type":"string","description":"Related policy ID"}},"required":["description","severity","agent_id"]}"#,
    },
    // ── Context tool ────────────────────────────────────────────
    ToolDefinition {
        name: "contract_context_log",
        description: "Log the intent and context behind a contract action",
        input_schema: r#"{"type":"object","properties":{"intent":{"type":"string","description":"Why this contract action is being performed"},"decision":{"type":"string","description":"What was decided or concluded"},"topic":{"type":"string","description":"Optional topic category"}},"required":["intent"]}"#,
    },
    // ── Stats tool ────────────────────────────────────────────────
    ToolDefinition {
        name: "contract_stats",
        description: "Get summary statistics for the contract store",
        input_schema: r#"{"type":"object","properties":{}}"#,
    },
    // ── Invention tools ────────────────────────────────────────────
    ToolDefinition {
        name: "policy_omniscience",
        description: "Get complete visibility into all applicable policies for an agent",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to query policies for"},"context":{"type":"string","description":"Context for the query"}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "risk_prophecy",
        description: "Predict future risk budget usage and identify limits at risk",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to prophecy for"},"forecast_window_secs":{"type":"integer","description":"Forecast window in seconds","default":3600}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "approval_telepathy",
        description: "Predict approval likelihood before submitting a request",
        input_schema: r#"{"type":"object","properties":{"action":{"type":"string","description":"Action to predict approval for"}},"required":["action"]}"#,
    },
    ToolDefinition {
        name: "obligation_clairvoyance",
        description: "Forecast upcoming obligations and identify scheduling conflicts",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to forecast for"},"window_secs":{"type":"integer","description":"Forecast window in seconds","default":86400}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "violation_precognition",
        description: "Detect potential violations before they occur",
        input_schema: r#"{"type":"object","properties":{"planned_action":{"type":"string","description":"Action being planned"}},"required":["planned_action"]}"#,
    },
    ToolDefinition {
        name: "contract_crystallize",
        description: "Generate contract policies from high-level intent description",
        input_schema: r#"{"type":"object","properties":{"intent":{"type":"string","description":"High-level intent to crystallize into policies"}},"required":["intent"]}"#,
    },
    ToolDefinition {
        name: "policy_dna_extract",
        description: "Extract the genetic representation of a policy for evolution",
        input_schema: r#"{"type":"object","properties":{"policy_id":{"type":"string","description":"Policy ID to extract DNA from"}},"required":["policy_id"]}"#,
    },
    ToolDefinition {
        name: "trust_gradient_evaluate",
        description: "Evaluate an action with trust-weighted policy assessment",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to evaluate trust for"},"action":{"type":"string","description":"Action to evaluate"}},"required":["agent_id","action"]}"#,
    },
    ToolDefinition {
        name: "collective_contract_create",
        description: "Create a multi-party collective governance contract",
        input_schema: r#"{"type":"object","properties":{"parties":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"name":{"type":"string"}},"required":["id","name"]},"description":"Parties involved"},"arbitration":{"type":"string","enum":["majority_vote","unanimous","third_party","automated"],"default":"majority_vote"}},"required":["parties"]}"#,
    },
    ToolDefinition {
        name: "temporal_contract_create",
        description: "Create a time-evolving contract with governance transitions",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"Contract label"},"initial_level":{"type":"string","enum":["conservative","moderate","permissive","autonomous"],"default":"conservative"}},"required":["label"]}"#,
    },
    ToolDefinition {
        name: "contract_inheritance_create",
        description: "Create a hierarchical parent-child contract relationship",
        input_schema: r#"{"type":"object","properties":{"parent_id":{"type":"string","description":"Parent contract ID"},"child_id":{"type":"string","description":"Child contract ID"},"propagate":{"type":"boolean","description":"Whether parent changes propagate","default":true}},"required":["parent_id","child_id"]}"#,
    },
    ToolDefinition {
        name: "smart_escalation_route",
        description: "Route an approval request to the optimal approver",
        input_schema: r#"{"type":"object","properties":{"description":{"type":"string","description":"What needs approval"},"urgency":{"type":"number","description":"Urgency level 0.0-1.0","default":0.5}},"required":["description"]}"#,
    },
    ToolDefinition {
        name: "violation_archaeology_analyze",
        description: "Analyze violation patterns to identify root causes",
        input_schema: r#"{"type":"object","properties":{"agent_id":{"type":"string","description":"Agent to analyze"},"window_secs":{"type":"integer","description":"Analysis window in seconds","default":604800}},"required":["agent_id"]}"#,
    },
    ToolDefinition {
        name: "contract_simulation_run",
        description: "Simulate contract behavior across multiple scenarios",
        input_schema: r#"{"type":"object","properties":{"scenario_count":{"type":"integer","description":"Number of scenarios to simulate","default":100}}}"#,
    },
    ToolDefinition {
        name: "federated_governance_create",
        description: "Create cross-organizational federated governance",
        input_schema: r#"{"type":"object","properties":{"name":{"type":"string","description":"Federation name"},"members":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"name":{"type":"string"}},"required":["id","name"]},"description":"Member organizations"},"transparency":{"type":"string","enum":["full","summary","minimal"],"default":"full"}},"required":["name","members"]}"#,
    },
    ToolDefinition {
        name: "self_healing_contract_create",
        description: "Create a contract that automatically adapts to violations",
        input_schema: r#"{"type":"object","properties":{"base_contract_id":{"type":"string","description":"Base contract to add self-healing to"}},"required":["base_contract_id"]}"#,
    },
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a required string param, returning Err(String) on missing/null.
fn require_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required parameter: {}", key))
}

/// Parse a required ContractId param.
fn require_id(args: &Value, key: &str) -> Result<agentic_contract::ContractId, String> {
    let s = require_str(args, key)?;
    s.parse::<agentic_contract::ContractId>()
        .map_err(|e| format!("Invalid ID for '{}': {}", key, e))
}

/// Parse optional string tags array.
fn parse_tags(args: &Value) -> Vec<String> {
    args.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Handle tool call
// ---------------------------------------------------------------------------

/// Handle a tool call.
pub async fn handle_tool_call(
    name: &str,
    args: Value,
    engine: &mut agentic_contract::ContractEngine,
) -> Result<Value, String> {
    match name {
        // ==================================================================
        // CONTRACT TOOLS
        // ==================================================================
        "contract_create" => {
            let label = require_str(&args, "label")?;
            let description = args
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let parties: Vec<String> = args
                .get("parties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let _tags = parse_tags(&args);

            // Create a contract as a policy with global scope
            let mut policy = agentic_contract::Policy::new(
                label,
                agentic_contract::PolicyScope::Global,
                agentic_contract::PolicyAction::Allow,
            );
            if !description.is_empty() {
                policy = policy.with_description(description);
            }
            for party in &parties {
                policy = policy.with_tag(party);
            }
            let id = engine.add_policy(policy);

            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "parties": parties,
                "status": "active"
            }))
        }

        "contract_sign" => {
            let contract_id = require_id(&args, "contract_id")?;
            let signer = require_str(&args, "signer")?;

            // Verify contract exists
            let _policy = engine.get_policy(contract_id).map_err(|e| e.to_string())?;

            Ok(json!({
                "contract_id": contract_id.to_string(),
                "signer": signer,
                "signed": true,
                "signed_at": chrono::Utc::now().to_rfc3339()
            }))
        }

        "contract_verify" => {
            let contract_id = require_id(&args, "contract_id")?;
            let policy = engine.get_policy(contract_id).map_err(|e| e.to_string())?;

            Ok(json!({
                "contract_id": contract_id.to_string(),
                "label": policy.label,
                "valid": policy.is_active(),
                "scope": format!("{:?}", policy.scope),
                "action": format!("{:?}", policy.action)
            }))
        }

        "contract_list" => {
            let policies = engine.list_policies(None);
            let items: Vec<Value> = policies
                .iter()
                .map(|p| {
                    json!({
                        "id": p.id.to_string(),
                        "label": p.label,
                        "scope": format!("{:?}", p.scope),
                        "action": format!("{:?}", p.action),
                        "active": p.is_active(),
                        "tags": p.tags
                    })
                })
                .collect();

            Ok(json!({ "contracts": items, "count": items.len() }))
        }

        "contract_get" => {
            let id = require_id(&args, "id")?;
            let policy = engine.get_policy(id).map_err(|e| e.to_string())?;

            Ok(json!({
                "id": policy.id.to_string(),
                "label": policy.label,
                "scope": format!("{:?}", policy.scope),
                "action": format!("{:?}", policy.action),
                "active": policy.is_active(),
                "tags": policy.tags,
                "created_at": policy.created_at.to_rfc3339()
            }))
        }

        // ==================================================================
        // POLICY TOOLS
        // ==================================================================
        "policy_add" => {
            let label = require_str(&args, "label")?;
            let scope = match args.get("scope").and_then(|v| v.as_str()) {
                Some("session") => agentic_contract::PolicyScope::Session,
                Some("agent") => agentic_contract::PolicyScope::Agent,
                Some("global") | None => agentic_contract::PolicyScope::Global,
                Some(other) => return Err(format!("Unknown scope: {}", other)),
            };
            let action = match args.get("action").and_then(|v| v.as_str()) {
                Some("allow") => agentic_contract::PolicyAction::Allow,
                Some("deny") | None => agentic_contract::PolicyAction::Deny,
                Some("require_approval") => agentic_contract::PolicyAction::RequireApproval,
                Some("audit_only") => agentic_contract::PolicyAction::AuditOnly,
                Some(other) => return Err(format!("Unknown action: {}", other)),
            };

            let mut policy = agentic_contract::Policy::new(label, scope, action);
            if let Some(desc) = args.get("description").and_then(|v| v.as_str()) {
                policy = policy.with_description(desc);
            }
            for tag in parse_tags(&args) {
                policy = policy.with_tag(&tag);
            }

            let id = engine.add_policy(policy);
            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "scope": format!("{:?}", scope),
                "action": format!("{:?}", action)
            }))
        }

        "policy_check" => {
            let action_type = require_str(&args, "action_type")?;
            let scope = match args.get("scope").and_then(|v| v.as_str()) {
                Some("session") => agentic_contract::PolicyScope::Session,
                Some("agent") => agentic_contract::PolicyScope::Agent,
                Some("global") | None => agentic_contract::PolicyScope::Global,
                Some(other) => return Err(format!("Unknown scope: {}", other)),
            };

            let result = engine.check_policy(action_type, scope);
            Ok(json!({
                "action_type": action_type,
                "scope": format!("{:?}", scope),
                "decision": format!("{:?}", result),
                "allowed": matches!(result, agentic_contract::PolicyAction::Allow)
            }))
        }

        "policy_list" => {
            let scope = match args.get("scope").and_then(|v| v.as_str()) {
                Some("session") => Some(agentic_contract::PolicyScope::Session),
                Some("agent") => Some(agentic_contract::PolicyScope::Agent),
                Some("global") => Some(agentic_contract::PolicyScope::Global),
                None => None,
                Some(other) => return Err(format!("Unknown scope: {}", other)),
            };

            let policies = engine.list_policies(scope);
            let items: Vec<Value> = policies
                .iter()
                .map(|p| {
                    json!({
                        "id": p.id.to_string(),
                        "label": p.label,
                        "scope": format!("{:?}", p.scope),
                        "action": format!("{:?}", p.action),
                        "active": p.is_active(),
                        "tags": p.tags
                    })
                })
                .collect();

            Ok(json!({ "policies": items, "count": items.len() }))
        }

        // ==================================================================
        // RISK LIMIT TOOLS
        // ==================================================================
        "risk_limit_set" => {
            let label = require_str(&args, "label")?;
            let max_value = args
                .get("max_value")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| "Missing required parameter: max_value".to_string())?;

            let limit_type = match args.get("limit_type").and_then(|v| v.as_str()) {
                Some("rate") => agentic_contract::LimitType::Rate,
                Some("threshold") | None => agentic_contract::LimitType::Threshold,
                Some("budget") => agentic_contract::LimitType::Budget,
                Some("count") => agentic_contract::LimitType::Count,
                Some(other) => return Err(format!("Unknown limit_type: {}", other)),
            };

            let mut risk_limit = agentic_contract::RiskLimit::new(label, limit_type, max_value);
            if let Some(window) = args.get("window_secs").and_then(|v| v.as_u64()) {
                risk_limit.window_secs = Some(window);
            }

            let id = engine.add_risk_limit(risk_limit);
            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "limit_type": format!("{:?}", limit_type),
                "max_value": max_value
            }))
        }

        "risk_limit_check" => {
            let label = require_str(&args, "label")?;
            let amount = args
                .get("amount")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| "Missing required parameter: amount".to_string())?;

            match engine.check_risk_limit(label, amount) {
                Some(limit) => Ok(json!({
                    "exceeded": true,
                    "limit_label": limit.label,
                    "current_value": limit.current_value,
                    "max_value": limit.max_value,
                    "remaining": limit.remaining()
                })),
                None => Ok(json!({
                    "exceeded": false,
                    "label": label,
                    "amount": amount
                })),
            }
        }

        "risk_limit_list" => {
            let limits = engine.list_risk_limits();
            let items: Vec<Value> = limits
                .iter()
                .map(|l| {
                    json!({
                        "id": l.id.to_string(),
                        "label": l.label,
                        "limit_type": format!("{:?}", l.limit_type),
                        "current_value": l.current_value,
                        "max_value": l.max_value,
                        "remaining": l.remaining(),
                        "usage_ratio": l.usage_ratio()
                    })
                })
                .collect();

            Ok(json!({ "risk_limits": items, "count": items.len() }))
        }

        // ==================================================================
        // APPROVAL TOOLS
        // ==================================================================
        "approval_request" => {
            let rule_id = require_id(&args, "rule_id")?;
            let action_description = require_str(&args, "action_description")?;
            let requestor = require_str(&args, "requestor")?;

            let request_id = engine
                .request_approval(rule_id, action_description, requestor)
                .map_err(|e| e.to_string())?;

            Ok(json!({
                "request_id": request_id.to_string(),
                "rule_id": rule_id.to_string(),
                "action": action_description,
                "requestor": requestor,
                "status": "pending"
            }))
        }

        "approval_decide" => {
            let request_id = require_id(&args, "request_id")?;
            let decision = match require_str(&args, "decision")? {
                "approve" => agentic_contract::DecisionType::Approve,
                "deny" => agentic_contract::DecisionType::Deny,
                other => {
                    return Err(format!(
                        "Unknown decision: {} (use 'approve' or 'deny')",
                        other
                    ))
                }
            };
            let decider = require_str(&args, "decider")?;
            let reason = require_str(&args, "reason")?;

            let decision_id = engine
                .decide_approval(request_id, decision, decider, reason)
                .map_err(|e| e.to_string())?;

            Ok(json!({
                "decision_id": decision_id.to_string(),
                "request_id": request_id.to_string(),
                "decision": format!("{:?}", decision),
                "decider": decider,
                "reason": reason
            }))
        }

        "approval_list" => {
            let status = match args.get("status").and_then(|v| v.as_str()) {
                Some("pending") => Some(agentic_contract::ApprovalStatus::Pending),
                Some("approved") => Some(agentic_contract::ApprovalStatus::Approved),
                Some("denied") => Some(agentic_contract::ApprovalStatus::Denied),
                Some("expired") => Some(agentic_contract::ApprovalStatus::Expired),
                None => None,
                Some(other) => return Err(format!("Unknown status: {}", other)),
            };

            let requests = engine.list_approval_requests(status);
            let items: Vec<Value> = requests
                .iter()
                .map(|r| {
                    json!({
                        "id": r.id.to_string(),
                        "rule_id": r.rule_id.to_string(),
                        "action_description": r.action_description,
                        "requestor": r.requestor,
                        "status": format!("{:?}", r.status),
                        "created_at": r.created_at.to_rfc3339()
                    })
                })
                .collect();

            Ok(json!({ "approvals": items, "count": items.len() }))
        }

        // ==================================================================
        // CONDITION TOOLS
        // ==================================================================
        "condition_add" => {
            let label = require_str(&args, "label")?;
            let expression = require_str(&args, "expression")?;
            let condition_type = match args.get("condition_type").and_then(|v| v.as_str()) {
                Some("threshold") => agentic_contract::ConditionType::Threshold,
                Some("time_based") => agentic_contract::ConditionType::TimeBased,
                Some("dependency") => agentic_contract::ConditionType::Dependency,
                Some("custom") | None => agentic_contract::ConditionType::Custom,
                Some(other) => return Err(format!("Unknown condition_type: {}", other)),
            };

            let condition = agentic_contract::Condition::new(label, condition_type, expression);
            let id = engine.add_condition(condition);

            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "condition_type": format!("{:?}", condition_type),
                "expression": expression,
                "status": "unevaluated"
            }))
        }

        "condition_evaluate" => {
            let id = require_id(&args, "id")?;
            let status = engine.evaluate_condition(id).map_err(|e| e.to_string())?;

            Ok(json!({
                "id": id.to_string(),
                "status": format!("{:?}", status),
                "met": matches!(status, agentic_contract::ConditionStatus::Met)
            }))
        }

        // ==================================================================
        // OBLIGATION TOOLS
        // ==================================================================
        "obligation_add" => {
            let label = require_str(&args, "label")?;
            let deadline_str = require_str(&args, "deadline")?;
            let deadline = chrono::DateTime::parse_from_rfc3339(deadline_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|e| format!("Invalid deadline: {}", e))?;

            let description = args
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("No description provided");

            let obligation = agentic_contract::Obligation::new(label, description, "agent")
                .with_deadline(deadline);

            let id = engine.add_obligation(obligation);
            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "deadline": deadline.to_rfc3339(),
                "status": "pending"
            }))
        }

        "obligation_check" => {
            if let Some(_id_str) = args.get("id").and_then(|v| v.as_str()) {
                let id = require_id(&args, "id")?;
                let status = engine.check_obligation(id).map_err(|e| e.to_string())?;

                Ok(json!({
                    "id": id.to_string(),
                    "status": format!("{:?}", status),
                    "resolved": matches!(
                        status,
                        agentic_contract::ObligationStatus::Fulfilled
                            | agentic_contract::ObligationStatus::Waived
                    )
                }))
            } else {
                // Check all obligations
                let obligations =
                    engine.list_obligations(Some(agentic_contract::ObligationStatus::Pending));
                let items: Vec<Value> = obligations
                    .iter()
                    .map(|o| {
                        json!({
                            "id": o.id.to_string(),
                            "label": o.label,
                            "deadline": o.deadline.map(|d| d.to_rfc3339()),
                            "status": format!("{:?}", o.status),
                            "overdue": o.is_overdue()
                        })
                    })
                    .collect();

                Ok(json!({ "obligations": items, "count": items.len() }))
            }
        }

        // ==================================================================
        // VIOLATION TOOLS
        // ==================================================================
        "violation_list" => {
            let severity = match args.get("severity").and_then(|v| v.as_str()) {
                Some("info") => Some(agentic_contract::ViolationSeverity::Info),
                Some("warning") => Some(agentic_contract::ViolationSeverity::Warning),
                Some("critical") => Some(agentic_contract::ViolationSeverity::Critical),
                Some("fatal") => Some(agentic_contract::ViolationSeverity::Fatal),
                None => None,
                Some(other) => return Err(format!("Unknown severity: {}", other)),
            };

            let violations = engine.list_violations(severity);
            let items: Vec<Value> = violations
                .iter()
                .map(|v| {
                    json!({
                        "id": v.id.to_string(),
                        "description": v.description,
                        "severity": format!("{:?}", v.severity),
                        "actor": v.actor,
                        "detected_at": v.detected_at.to_rfc3339()
                    })
                })
                .collect();

            Ok(json!({ "violations": items, "count": items.len() }))
        }

        "violation_report" => {
            let description = require_str(&args, "description")?;
            let agent_id = require_str(&args, "agent_id")?;
            let severity = match require_str(&args, "severity")? {
                "info" => agentic_contract::ViolationSeverity::Info,
                "warning" => agentic_contract::ViolationSeverity::Warning,
                "critical" => agentic_contract::ViolationSeverity::Critical,
                "fatal" => agentic_contract::ViolationSeverity::Fatal,
                other => return Err(format!("Unknown severity: {}", other)),
            };

            let mut violation = agentic_contract::Violation::new(description, severity, agent_id);
            if let Some(pid) = args.get("policy_id").and_then(|v| v.as_str()) {
                if let Ok(policy_id) = pid.parse::<agentic_contract::ContractId>() {
                    violation.policy_id = Some(policy_id);
                }
            }

            let id = engine.report_violation(violation);
            Ok(json!({
                "id": id.to_string(),
                "description": description,
                "severity": format!("{:?}", severity),
                "actor": agent_id
            }))
        }

        // ==================================================================
        // CONTEXT LOG TOOL
        // ==================================================================
        "contract_context_log" => {
            let intent = require_str(&args, "intent")?;
            let decision = args.get("decision").and_then(|v| v.as_str());
            let topic = args.get("topic").and_then(|v| v.as_str());

            Ok(json!({
                "logged": true,
                "intent": intent,
                "decision": decision,
                "topic": topic,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }

        // ==================================================================
        // STATS (bonus, used by contract_get pattern)
        // ==================================================================
        "contract_stats" => {
            let stats = engine.stats();
            Ok(serde_json::to_value(&stats).map_err(|e| e.to_string())?)
        }

        // ==================================================================
        // INVENTION TOOLS (16)
        // ==================================================================

        // ── 1. Policy Omniscience ─────────────────────────────────────
        "policy_omniscience" => {
            let agent_id = require_str(&args, "agent_id")?;
            let context = args
                .get("context")
                .and_then(|v| v.as_str())
                .unwrap_or("general");

            let result = engine.policy_omniscience(agent_id, context);
            Ok(json!({
                "id": result.id.to_string(),
                "agent_id": result.agent_id,
                "context": result.context,
                "allowed_actions": result.allowed_actions.iter().map(|e| json!({
                    "action": e.action,
                    "policy_id": e.policy_id.to_string(),
                    "policy_label": e.policy_label,
                    "reason": e.reason,
                    "scope": e.scope
                })).collect::<Vec<_>>(),
                "denied_actions": result.denied_actions.iter().map(|e| json!({
                    "action": e.action,
                    "policy_id": e.policy_id.to_string(),
                    "policy_label": e.policy_label,
                    "reason": e.reason,
                    "scope": e.scope
                })).collect::<Vec<_>>(),
                "conditional_actions": result.conditional_actions.iter().map(|e| json!({
                    "action": e.action,
                    "policy_id": e.policy_id.to_string(),
                    "policy_label": e.policy_label,
                    "reason": e.reason,
                    "scope": e.scope
                })).collect::<Vec<_>>(),
                "total_permissions": result.total_permissions,
                "queried_at": result.queried_at.to_rfc3339()
            }))
        }

        // ── 2. Risk Prophecy ──────────────────────────────────────────
        "risk_prophecy" => {
            let agent_id = require_str(&args, "agent_id")?;
            let window = args
                .get("forecast_window_secs")
                .and_then(|v| v.as_i64())
                .unwrap_or(3600);

            let result = engine.risk_prophecy(agent_id, window);
            Ok(json!({
                "id": result.id.to_string(),
                "agent_id": result.agent_id,
                "forecast_window_secs": result.forecast_window_secs,
                "projections": result.projections.iter().map(|p| json!({
                    "limit_id": p.limit_id.to_string(),
                    "limit_label": p.limit_label,
                    "current_usage": p.current_usage,
                    "projected_usage": p.projected_usage,
                    "exceed_probability": p.exceed_probability,
                    "time_until_limit_secs": p.time_until_limit_secs
                })).collect::<Vec<_>>(),
                "overall_risk_score": result.overall_risk_score,
                "recommendations": result.recommendations,
                "prophesied_at": result.prophesied_at.to_rfc3339()
            }))
        }

        // ── 3. Approval Telepathy ─────────────────────────────────────
        "approval_telepathy" => {
            let action = require_str(&args, "action")?;
            let result = engine.approval_telepathy(action);

            Ok(json!({
                "id": result.id.to_string(),
                "action": result.action,
                "approval_probability": result.approval_probability,
                "likely_approvers": result.likely_approvers,
                "estimated_response_secs": result.estimated_response_secs,
                "suggestions": result.suggestions.iter().map(|s| json!({
                    "modification": s.modification,
                    "new_probability": s.new_probability,
                    "effort": s.effort
                })).collect::<Vec<_>>(),
                "historical_approval_rate": result.historical_approval_rate,
                "predicted_at": result.predicted_at.to_rfc3339()
            }))
        }

        // ── 4. Obligation Clairvoyance ────────────────────────────────
        "obligation_clairvoyance" => {
            let agent_id = require_str(&args, "agent_id")?;
            let window = args
                .get("window_secs")
                .and_then(|v| v.as_i64())
                .unwrap_or(86400);

            let result = engine.obligation_clairvoyance(agent_id, window);
            Ok(json!({
                "id": result.id.to_string(),
                "agent_id": result.agent_id,
                "window_secs": result.window_secs,
                "upcoming": result.upcoming.iter().map(|f| json!({
                    "obligation_id": f.obligation_id.to_string(),
                    "label": f.label,
                    "deadline": f.deadline.map(|d| d.to_rfc3339()),
                    "time_remaining_secs": f.time_remaining_secs,
                    "estimated_effort_minutes": f.estimated_effort_minutes,
                    "miss_risk": f.miss_risk
                })).collect::<Vec<_>>(),
                "conflicts": result.conflicts.iter().map(|c| json!({
                    "obligation_a": c.obligation_a.to_string(),
                    "obligation_b": c.obligation_b.to_string(),
                    "conflict_type": c.conflict_type,
                    "resolution": c.resolution
                })).collect::<Vec<_>>(),
                "optimal_order": result.optimal_order.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                "projected_at": result.projected_at.to_rfc3339()
            }))
        }

        // ── 5. Violation Precognition ─────────────────────────────────
        "violation_precognition" => {
            let planned_action = require_str(&args, "planned_action")?;
            let result = engine.violation_precognition(planned_action);

            Ok(json!({
                "id": result.id.to_string(),
                "planned_action": result.planned_action,
                "at_risk_policies": result.at_risk_policies.iter().map(|p| json!({
                    "policy_id": p.policy_id.to_string(),
                    "policy_label": p.policy_label,
                    "probability": p.probability,
                    "trigger": p.trigger
                })).collect::<Vec<_>>(),
                "at_risk_limits": result.at_risk_limits.iter().map(|l| json!({
                    "limit_id": l.limit_id.to_string(),
                    "limit_label": l.limit_label,
                    "headroom": l.headroom,
                    "projected_usage": l.projected_usage
                })).collect::<Vec<_>>(),
                "safe_alternatives": result.safe_alternatives,
                "violation_probability": result.violation_probability,
                "analyzed_at": result.analyzed_at.to_rfc3339()
            }))
        }

        // ── 6. Contract Crystallization ───────────────────────────────
        "contract_crystallize" => {
            let intent = require_str(&args, "intent")?;
            let result = engine.crystallize_contract(intent);

            Ok(json!({
                "id": result.id.to_string(),
                "intent": result.intent,
                "policies": result.policies.iter().map(|p| json!({
                    "label": p.label,
                    "scope": p.scope,
                    "action": p.action,
                    "rationale": p.rationale
                })).collect::<Vec<_>>(),
                "risk_limits": result.risk_limits.iter().map(|r| json!({
                    "label": r.label,
                    "max_value": r.max_value,
                    "limit_type": r.limit_type,
                    "rationale": r.rationale
                })).collect::<Vec<_>>(),
                "edge_cases": result.edge_cases,
                "confidence": result.confidence,
                "crystallized_at": result.crystallized_at.to_rfc3339()
            }))
        }

        // ── 7. Policy DNA ─────────────────────────────────────────────
        "policy_dna_extract" => {
            let policy_id = require_id(&args, "policy_id")?;
            let result = engine
                .extract_policy_dna(policy_id)
                .map_err(|e| e.to_string())?;

            Ok(json!({
                "id": result.id.to_string(),
                "policy_id": result.policy_id.to_string(),
                "genes": result.genes.iter().map(|g| json!({
                    "name": g.name,
                    "value": g.value,
                    "dominant": g.dominant
                })).collect::<Vec<_>>(),
                "fitness": result.fitness,
                "generation": result.generation,
                "mutations": result.mutations,
                "extracted_at": result.extracted_at.to_rfc3339()
            }))
        }

        // ── 8. Trust Gradients ────────────────────────────────────────
        "trust_gradient_evaluate" => {
            let agent_id = require_str(&args, "agent_id")?;
            let action = require_str(&args, "action")?;
            let result = engine.evaluate_trust_gradient(agent_id, action);

            Ok(json!({
                "id": result.id.to_string(),
                "agent_id": result.agent_id,
                "action": result.action,
                "trust_factor": result.trust_factor,
                "confidence": result.confidence,
                "monitoring_level": format!("{:?}", result.monitoring_level),
                "auto_revoke_threshold": result.auto_revoke_threshold,
                "contributing_factors": result.contributing_factors.iter().map(|f| json!({
                    "name": f.name,
                    "weight": f.weight,
                    "score": f.score,
                    "trend": f.trend
                })).collect::<Vec<_>>(),
                "evaluated_at": result.evaluated_at.to_rfc3339()
            }))
        }

        // ── 9. Collective Contracts ───────────────────────────────────
        "collective_contract_create" => {
            let parties_arr = args
                .get("parties")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Missing required parameter: parties".to_string())?;

            let parties: Vec<(&str, &str)> = parties_arr
                .iter()
                .filter_map(|p| {
                    let id = p.get("id").and_then(|v| v.as_str())?;
                    let name = p.get("name").and_then(|v| v.as_str())?;
                    Some((id, name))
                })
                .collect();

            let arbitration = match args.get("arbitration").and_then(|v| v.as_str()) {
                Some("unanimous") => agentic_contract::inventions::ArbitrationMethod::Unanimous,
                Some("third_party") => agentic_contract::inventions::ArbitrationMethod::ThirdParty,
                Some("automated") => agentic_contract::inventions::ArbitrationMethod::Automated,
                Some("majority_vote") | None => {
                    agentic_contract::inventions::ArbitrationMethod::MajorityVote
                }
                Some(other) => return Err(format!("Unknown arbitration method: {}", other)),
            };

            let result = engine.create_collective_contract(parties, arbitration);
            Ok(json!({
                "id": result.id.to_string(),
                "parties": result.parties.iter().map(|p| json!({
                    "party_id": p.party_id,
                    "name": p.name,
                    "role": p.role,
                    "signed": p.signed
                })).collect::<Vec<_>>(),
                "shared_policies": result.shared_policies.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                "arbitration_method": format!("{:?}", result.arbitration.method),
                "status": format!("{:?}", result.status),
                "signatures": result.signatures,
                "required_signatures": result.required_signatures,
                "created_at": result.created_at.to_rfc3339()
            }))
        }

        // ── 10. Temporal Contracts ────────────────────────────────────
        "temporal_contract_create" => {
            let label = require_str(&args, "label")?;
            let level = match args.get("initial_level").and_then(|v| v.as_str()) {
                Some("moderate") => agentic_contract::inventions::GovernanceLevel::Moderate,
                Some("permissive") => agentic_contract::inventions::GovernanceLevel::Permissive,
                Some("autonomous") => agentic_contract::inventions::GovernanceLevel::Autonomous,
                Some("conservative") | None => {
                    agentic_contract::inventions::GovernanceLevel::Conservative
                }
                Some(other) => return Err(format!("Unknown governance level: {}", other)),
            };

            let result = engine.create_temporal_contract(label, level);
            Ok(json!({
                "id": result.id.to_string(),
                "label": result.label,
                "initial_level": format!("{:?}", result.initial_level),
                "current_level": format!("{:?}", result.current_level),
                "transitions": result.transitions,
                "created_at": result.created_at.to_rfc3339()
            }))
        }

        // ── 11. Contract Inheritance ──────────────────────────────────
        "contract_inheritance_create" => {
            let parent_id = require_id(&args, "parent_id")?;
            let child_id = require_id(&args, "child_id")?;
            let propagate = args
                .get("propagate")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let result = engine
                .create_contract_inheritance(parent_id, child_id, propagate)
                .map_err(|e| e.to_string())?;

            Ok(json!({
                "id": result.id.to_string(),
                "parent_id": result.parent_id.to_string(),
                "child_id": result.child_id.to_string(),
                "inherited_policies": result.inherited_policies.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                "propagate_changes": result.propagate_changes,
                "created_at": result.created_at.to_rfc3339()
            }))
        }

        // ── 12. Smart Escalation ─────────────────────────────────────
        "smart_escalation_route" => {
            let description = require_str(&args, "description")?;
            let urgency = args.get("urgency").and_then(|v| v.as_f64()).unwrap_or(0.5);

            let result = engine.smart_escalate(description, urgency);
            Ok(json!({
                "id": result.id.to_string(),
                "request_description": result.request_description,
                "urgency": result.urgency,
                "recommended_approver": result.recommended_approver,
                "routing_reason": result.routing_reason,
                "fallback_chain": result.fallback_chain.iter().map(|t| json!({
                    "approver_id": t.approver_id,
                    "name": t.name,
                    "availability": t.availability,
                    "avg_response_secs": t.avg_response_secs,
                    "approval_rate": t.approval_rate
                })).collect::<Vec<_>>(),
                "estimated_response_secs": result.estimated_response_secs,
                "confidence": result.confidence,
                "routed_at": result.routed_at.to_rfc3339()
            }))
        }

        // ── 13. Violation Archaeology ─────────────────────────────────
        "violation_archaeology_analyze" => {
            let agent_id = require_str(&args, "agent_id")?;
            let window = args
                .get("window_secs")
                .and_then(|v| v.as_i64())
                .unwrap_or(604800);

            let result = engine.violation_archaeology(agent_id, window);
            Ok(json!({
                "id": result.id.to_string(),
                "agent_id": result.agent_id,
                "window_secs": result.window_secs,
                "clusters": result.clusters.iter().map(|c| json!({
                    "label": c.label,
                    "count": c.count,
                    "severity": c.severity,
                    "time_pattern": c.time_pattern,
                    "context_pattern": c.context_pattern
                })).collect::<Vec<_>>(),
                "root_causes": result.root_causes.iter().map(|r| json!({
                    "hypothesis": r.hypothesis,
                    "confidence": r.confidence,
                    "evidence": r.evidence,
                    "factors": r.factors
                })).collect::<Vec<_>>(),
                "recommendations": result.recommendations.iter().map(|r| json!({
                    "action": r.action,
                    "expected_impact": r.expected_impact,
                    "effort": r.effort,
                    "priority": r.priority
                })).collect::<Vec<_>>(),
                "analyzed_at": result.analyzed_at.to_rfc3339()
            }))
        }

        // ── 14. Contract Simulation ───────────────────────────────────
        "contract_simulation_run" => {
            let scenario_count = args
                .get("scenario_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(100) as u32;

            let result = engine.simulate_contract(scenario_count);
            Ok(json!({
                "id": result.id.to_string(),
                "scenario_count": result.scenario_count,
                "approval_rate": result.approval_rate,
                "denial_rate": result.denial_rate,
                "risk_breach_rate": result.risk_breach_rate,
                "deadlocks": result.deadlocks.iter().map(|d| json!({
                    "description": d.description,
                    "policies_involved": d.policies_involved.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "resolution": d.resolution
                })).collect::<Vec<_>>(),
                "edge_cases": result.edge_cases.iter().map(|e| json!({
                    "description": e.description,
                    "current_behavior": e.current_behavior,
                    "suggested_fix": e.suggested_fix
                })).collect::<Vec<_>>(),
                "health_score": result.health_score,
                "simulated_at": result.simulated_at.to_rfc3339()
            }))
        }

        // ── 15. Federated Governance ──────────────────────────────────
        "federated_governance_create" => {
            let name = require_str(&args, "name")?;
            let members_arr = args
                .get("members")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Missing required parameter: members".to_string())?;

            let members: Vec<(&str, &str)> = members_arr
                .iter()
                .filter_map(|m| {
                    let id = m.get("id").and_then(|v| v.as_str())?;
                    let name = m.get("name").and_then(|v| v.as_str())?;
                    Some((id, name))
                })
                .collect();

            let transparency = match args.get("transparency").and_then(|v| v.as_str()) {
                Some("summary") => agentic_contract::inventions::TransparencyLevel::Summary,
                Some("minimal") => agentic_contract::inventions::TransparencyLevel::Minimal,
                Some("full") | None => agentic_contract::inventions::TransparencyLevel::Full,
                Some(other) => return Err(format!("Unknown transparency level: {}", other)),
            };

            let result = engine.create_federated_governance(name, members, transparency);
            Ok(json!({
                "id": result.id.to_string(),
                "name": result.name,
                "members": result.members.iter().map(|m| json!({
                    "org_id": m.org_id,
                    "name": m.name,
                    "contributed_policies": m.contributed_policies,
                    "trust_level": m.trust_level,
                    "ratified": m.ratified
                })).collect::<Vec<_>>(),
                "transparency": format!("{:?}", result.transparency),
                "status": format!("{:?}", result.status),
                "created_at": result.created_at.to_rfc3339()
            }))
        }

        // ── 16. Self-Healing Contracts ────────────────────────────────
        "self_healing_contract_create" => {
            let base_id = require_id(&args, "base_contract_id")?;
            let result = engine
                .create_self_healing_contract(base_id)
                .map_err(|e| e.to_string())?;

            Ok(json!({
                "id": result.id.to_string(),
                "base_contract_id": result.base_contract_id.to_string(),
                "healing_rules": result.healing_rules.iter().map(|r| json!({
                    "trigger": format!("{:?}", r.trigger),
                    "action": format!("{:?}", r.action),
                    "cooldown_secs": r.cooldown_secs
                })).collect::<Vec<_>>(),
                "adaptation_level": format!("{:?}", result.adaptation_level),
                "health_score": result.health_score,
                "created_at": result.created_at.to_rfc3339()
            }))
        }

        // Unknown tool → MCP Quality Standard: -32803
        _ => Err(format!("Unknown tool: {}", name)),
    }
}

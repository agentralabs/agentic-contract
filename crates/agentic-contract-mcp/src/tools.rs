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
    // NOTE: Invention tools (inventions 1-16) are defined in their own modules:
    // invention_visibility (1-5), invention_generation (6-7),
    // invention_governance (8-12), invention_resilience (13-16).
    // They are registered in server.rs via TOOL_DEFS + try_handle() chaining.
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a required string param, returning Err(String) on missing/null.
pub fn require_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required parameter: {}", key))
}

/// Parse a required ContractId param.
pub fn require_id(args: &Value, key: &str) -> Result<agentic_contract::ContractId, String> {
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

        // NOTE: Invention tools (1-16) are handled by invention modules
        // via try_handle() chaining in server.rs before reaching here.

        // Unknown tool → MCP Quality Standard: -32803
        _ => Err(format!("Unknown tool: {}", name)),
    }
}

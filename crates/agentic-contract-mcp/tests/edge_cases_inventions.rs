//! Edge-case and invention tests for AgenticContract MCP tools.
//!
//! Covers: tool count verification, policy/risk-limit/approval/obligation/
//! violation edge cases, unknown tool handling, unicode labels, full lifecycle
//! workflows, and context logging.

use serde_json::json;

// =========================================================================
// Helper: create a fresh engine
// =========================================================================
fn fresh_engine() -> agentic_contract::ContractEngine {
    agentic_contract::ContractEngine::new()
}

/// A future datetime string guaranteed to be ~1 year ahead.
fn future_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap()
        .to_rfc3339()
}

// =========================================================================
// Section 1: Tool count and definitions
// =========================================================================

#[test]
fn test_tool_count() {
    // Core tools are in tools::TOOLS (34 tools).
    // Invention tools are in separate TOOL_DEFS arrays chained at the server level.
    let core = agentic_contract_mcp::tools::TOOLS.len();
    let inv_vis = agentic_contract_mcp::invention_visibility::TOOL_DEFS.len();
    let inv_gen = agentic_contract_mcp::invention_generation::TOOL_DEFS.len();
    let inv_gov = agentic_contract_mcp::invention_governance::TOOL_DEFS.len();
    let inv_res = agentic_contract_mcp::invention_resilience::TOOL_DEFS.len();
    let total = core + inv_vis + inv_gen + inv_gov + inv_res;
    assert_eq!(core, 34, "Expected 34 core tools, got {}", core);
    assert!(
        total > 34,
        "Expected invention tools to add to core count, total is {}",
        total
    );
}

#[test]
fn test_all_tool_descriptions_are_verb_first() {
    for tool in agentic_contract_mcp::tools::TOOLS {
        let desc = tool.description;
        // First character should be uppercase (verb-first)
        let first_char = desc.chars().next().unwrap();
        assert!(
            first_char.is_uppercase(),
            "Tool '{}' description should start with verb (uppercase): {}",
            tool.name,
            desc
        );
        // No trailing period (MCP Quality Standard)
        assert!(
            !desc.ends_with('.'),
            "Tool '{}' description should not end with period: {}",
            tool.name,
            desc
        );
    }
}

#[test]
fn test_all_tool_schemas_are_valid_json() {
    for tool in agentic_contract_mcp::tools::TOOLS {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(tool.input_schema);
        assert!(
            parsed.is_ok(),
            "Tool '{}' has invalid JSON schema: {}",
            tool.name,
            tool.input_schema
        );
    }
}

#[test]
fn test_resource_count() {
    assert_eq!(agentic_contract_mcp::resources::RESOURCE_COUNT, 12);
    assert_eq!(agentic_contract_mcp::resources::list_resources().len(), 12);
}

#[test]
fn test_prompt_count() {
    assert_eq!(agentic_contract_mcp::prompts::PROMPT_COUNT, 4);
}

// =========================================================================
// Section 2: Policy edge cases
// =========================================================================

#[tokio::test]
async fn test_policy_add_minimal() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "No deploys on Friday"}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert!(val.get("id").is_some());
}

#[tokio::test]
async fn test_policy_add_full() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({
            "label": "Restricted data access",
            "scope": "agent",
            "action": "require_approval",
            "description": "All data access requires approval",
            "tags": ["security", "compliance"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_policy_check_allow() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_check",
        json!({"action_type": "read_file", "scope": "global"}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["allowed"], true);
}

#[tokio::test]
async fn test_policy_check_deny() {
    let mut engine = fresh_engine();
    // Add a deny policy
    agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "deploy", "scope": "global", "action": "deny"}),
        &mut engine,
    )
    .await
    .unwrap();

    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_check",
        json!({"action_type": "deploy", "scope": "global"}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["allowed"], false);
}

#[tokio::test]
async fn test_policy_list_empty() {
    let mut engine = fresh_engine();
    let result =
        agentic_contract_mcp::tools::handle_tool_call("policy_list", json!({}), &mut engine).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["count"], 0);
}

// =========================================================================
// Section 3: Risk limit edge cases
// =========================================================================

#[tokio::test]
async fn test_risk_limit_set_and_check() {
    let mut engine = fresh_engine();
    agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_set",
        json!({"label": "API calls", "max_value": 100, "limit_type": "rate"}),
        &mut engine,
    )
    .await
    .unwrap();

    // Should not be exceeded
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_check",
        json!({"label": "api", "amount": 50}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(result["exceeded"], false);
}

#[tokio::test]
async fn test_risk_limit_list_empty() {
    let mut engine = fresh_engine();
    let result =
        agentic_contract_mcp::tools::handle_tool_call("risk_limit_list", json!({}), &mut engine)
            .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["count"], 0);
}

// =========================================================================
// Section 4: Approval workflow edge cases
// =========================================================================

#[tokio::test]
async fn test_approval_request_invalid_rule() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "approval_request",
        json!({
            "rule_id": "00000000-0000-0000-0000-000000000000",
            "action_description": "Deploy to production",
            "requestor": "agent_1"
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_approval_list_empty() {
    let mut engine = fresh_engine();
    let result =
        agentic_contract_mcp::tools::handle_tool_call("approval_list", json!({}), &mut engine)
            .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["count"], 0);
}

// =========================================================================
// Section 5: Obligation edge cases
// =========================================================================

#[tokio::test]
async fn test_obligation_add() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "obligation_add",
        json!({
            "label": "Submit compliance report",
            "deadline": future_dt(),
            "description": "Monthly compliance report"
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert!(val.get("id").is_some());
    assert_eq!(val["status"], "pending");
}

#[tokio::test]
async fn test_obligation_check_all() {
    let mut engine = fresh_engine();
    let result =
        agentic_contract_mcp::tools::handle_tool_call("obligation_check", json!({}), &mut engine)
            .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["count"], 0);
}

// =========================================================================
// Section 6: Violation edge cases
// =========================================================================

#[tokio::test]
async fn test_violation_report() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "violation_report",
        json!({
            "description": "Rate limit exceeded",
            "severity": "warning",
            "agent_id": "agent_1"
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert!(val.get("id").is_some());
}

#[tokio::test]
async fn test_violation_list_by_severity() {
    let mut engine = fresh_engine();
    // Add violations of different severity
    agentic_contract_mcp::tools::handle_tool_call(
        "violation_report",
        json!({"description": "Info event", "severity": "info", "agent_id": "agent_1"}),
        &mut engine,
    )
    .await
    .unwrap();
    agentic_contract_mcp::tools::handle_tool_call(
        "violation_report",
        json!({"description": "Critical breach", "severity": "critical", "agent_id": "agent_2"}),
        &mut engine,
    )
    .await
    .unwrap();

    // Filter by critical only
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "violation_list",
        json!({"severity": "critical"}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(result["count"], 1);
}

// =========================================================================
// Section 7: Unknown tool and context log
// =========================================================================

#[tokio::test]
async fn test_unknown_tool_returns_error() {
    let mut engine = fresh_engine();
    let result =
        agentic_contract_mcp::tools::handle_tool_call("nonexistent_tool", json!({}), &mut engine)
            .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown tool"));
}

#[tokio::test]
async fn test_context_log() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "contract_context_log",
        json!({
            "intent": "Testing policy compliance",
            "decision": "All clear",
            "topic": "compliance"
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["logged"], true);
}

// =========================================================================
// Section 8: Unicode and special characters
// =========================================================================

#[tokio::test]
async fn test_unicode_policy_label() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "禁止周五部署 🚫", "scope": "global", "action": "deny"}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_emoji_in_violation() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "violation_report",
        json!({
            "description": "⚠️ Rate limit exceeded by 200%",
            "severity": "critical",
            "agent_id": "agent_🤖"
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

// =========================================================================
// Section 9: Full lifecycle workflow
// =========================================================================

#[tokio::test]
async fn test_full_governance_lifecycle() {
    let mut engine = fresh_engine();

    // 1. Create a contract
    let contract = agentic_contract_mcp::tools::handle_tool_call(
        "contract_create",
        json!({
            "label": "Agent Governance Agreement",
            "parties": ["agent_1", "admin"],
            "description": "Governs agent_1 behavior"
        }),
        &mut engine,
    )
    .await
    .unwrap();
    let contract_id = contract["id"].as_str().unwrap();

    // 2. Sign the contract
    let signed = agentic_contract_mcp::tools::handle_tool_call(
        "contract_sign",
        json!({"contract_id": contract_id, "signer": "admin"}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(signed["signed"], true);

    // 3. Add a policy
    agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "deploy", "scope": "global", "action": "require_approval"}),
        &mut engine,
    )
    .await
    .unwrap();

    // 4. Check the policy
    let check = agentic_contract_mcp::tools::handle_tool_call(
        "policy_check",
        json!({"action_type": "deploy"}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(check["allowed"], false); // RequireApproval counts as not immediately allowed

    // 5. Set a risk limit
    agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_set",
        json!({"label": "API calls per hour", "max_value": 1000, "limit_type": "rate"}),
        &mut engine,
    )
    .await
    .unwrap();

    // 6. Add an obligation
    agentic_contract_mcp::tools::handle_tool_call(
        "obligation_add",
        json!({"label": "Weekly report", "deadline": future_dt()}),
        &mut engine,
    )
    .await
    .unwrap();

    // 7. Report a violation
    agentic_contract_mcp::tools::handle_tool_call(
        "violation_report",
        json!({
            "description": "Attempted unauthorized deploy",
            "severity": "warning",
            "agent_id": "agent_1"
        }),
        &mut engine,
    )
    .await
    .unwrap();

    // 8. Verify stats
    let stats =
        agentic_contract_mcp::tools::handle_tool_call("contract_stats", json!({}), &mut engine)
            .await
            .unwrap();
    assert!(stats["policy_count"].as_u64().unwrap() >= 2);
    assert_eq!(stats["risk_limit_count"], 1);
    assert_eq!(stats["violation_count"], 1);
    assert_eq!(stats["obligation_count"], 1);
}

// =========================================================================
// Section 10: Condition tools
// =========================================================================

#[tokio::test]
async fn test_condition_add_and_evaluate() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "condition_add",
        json!({
            "label": "Memory threshold",
            "condition_type": "threshold",
            "expression": "memory_usage < 80%"
        }),
        &mut engine,
    )
    .await
    .unwrap();
    let condition_id = result["id"].as_str().unwrap();

    let eval = agentic_contract_mcp::tools::handle_tool_call(
        "condition_evaluate",
        json!({"id": condition_id}),
        &mut engine,
    )
    .await;
    assert!(eval.is_ok());
}

// =========================================================================
// Section 11: Prompt expansion
// =========================================================================

#[test]
fn test_prompt_expansion_review() {
    let args = std::collections::HashMap::new();
    let result = agentic_contract_mcp::prompts::expand_prompt("contract_review", &args);
    assert!(result.is_some());
    assert!(result.unwrap().contains("policy_list"));
}

#[test]
fn test_prompt_expansion_setup() {
    let mut args = std::collections::HashMap::new();
    args.insert("agent_name".to_string(), "data_analyst".to_string());
    let result = agentic_contract_mcp::prompts::expand_prompt("contract_setup", &args);
    assert!(result.is_some());
    assert!(result.unwrap().contains("data_analyst"));
}

#[test]
fn test_prompt_expansion_unknown() {
    let args = std::collections::HashMap::new();
    let result = agentic_contract_mcp::prompts::expand_prompt("nonexistent", &args);
    assert!(result.is_none());
}

// =========================================================================
// Section 12: Stdio transport edge cases
// =========================================================================

#[test]
fn test_validate_jsonrpc_valid() {
    let valid: serde_json::Value =
        serde_json::from_str(r#"{"jsonrpc":"2.0","method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&valid).is_ok());
}

#[test]
fn test_validate_jsonrpc_invalid_version() {
    let invalid: serde_json::Value =
        serde_json::from_str(r#"{"jsonrpc":"1.0","method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&invalid).is_err());
}

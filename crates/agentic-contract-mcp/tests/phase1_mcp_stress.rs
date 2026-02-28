//! Phase 1: MCP Protocol Stress Tests
//!
//! Covers: all 38 tool smoke tests via macro, JSON-RPC protocol compliance,
//! Content-Length transport edge cases, concurrent tool calls, error handling,
//! and MCP Quality Standard verification.

use serde_json::{json, Value};
use std::io::Cursor;

// =========================================================================
// Helpers
// =========================================================================

fn fresh_engine() -> agentic_contract::ContractEngine {
    agentic_contract::ContractEngine::new()
}

fn future_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap()
        .to_rfc3339()
}

/// Frame a JSON-RPC message with Content-Length header.
fn frame_message(msg: &str) -> Vec<u8> {
    format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).into_bytes()
}

// =========================================================================
// Section 1: Macro-generated smoke tests for all 38 tools
// =========================================================================

/// Macro to generate a smoke test that calls a tool with given args and asserts it returns Ok or Err.
macro_rules! smoke_test_ok {
    ($name:ident, $tool:expr, $args:expr) => {
        #[tokio::test]
        async fn $name() {
            let mut engine = fresh_engine();
            let result =
                agentic_contract_mcp::tools::handle_tool_call($tool, $args, &mut engine).await;
            assert!(
                result.is_ok(),
                "Tool '{}' should succeed but got: {:?}",
                $tool,
                result.err()
            );
        }
    };
}

macro_rules! smoke_test_err {
    ($name:ident, $tool:expr, $args:expr) => {
        #[tokio::test]
        async fn $name() {
            let mut engine = fresh_engine();
            let result =
                agentic_contract_mcp::tools::handle_tool_call($tool, $args, &mut engine).await;
            assert!(
                result.is_err(),
                "Tool '{}' should fail but succeeded with: {:?}",
                $tool,
                result.ok()
            );
        }
    };
}

// ── Core 22 tools ──────────────────────────────────────────────────────

smoke_test_ok!(
    smoke_contract_create,
    "contract_create",
    json!({"label": "Test Contract"})
);
smoke_test_ok!(smoke_contract_list, "contract_list", json!({}));
smoke_test_ok!(
    smoke_policy_add,
    "policy_add",
    json!({"label": "test policy"})
);
smoke_test_ok!(
    smoke_policy_check,
    "policy_check",
    json!({"action_type": "deploy"})
);
smoke_test_ok!(smoke_policy_list, "policy_list", json!({}));
smoke_test_ok!(
    smoke_risk_limit_set,
    "risk_limit_set",
    json!({"label": "api calls", "max_value": 100})
);
smoke_test_ok!(
    smoke_risk_limit_check,
    "risk_limit_check",
    json!({"label": "api", "amount": 10})
);
smoke_test_ok!(smoke_risk_limit_list, "risk_limit_list", json!({}));
smoke_test_ok!(smoke_approval_list, "approval_list", json!({}));
smoke_test_ok!(
    smoke_condition_add,
    "condition_add",
    json!({"label": "mem check", "expression": "mem < 80%"})
);
smoke_test_ok!(
    smoke_obligation_add,
    "obligation_add",
    json!({"label": "weekly report", "deadline": future_dt()})
);
smoke_test_ok!(smoke_obligation_check, "obligation_check", json!({}));
smoke_test_ok!(smoke_violation_list, "violation_list", json!({}));
smoke_test_ok!(
    smoke_violation_report,
    "violation_report",
    json!({"description": "test", "severity": "info", "agent_id": "a1"})
);
smoke_test_ok!(
    smoke_contract_context_log,
    "contract_context_log",
    json!({"intent": "testing"})
);
smoke_test_ok!(smoke_contract_stats, "contract_stats", json!({}));

// ── 16 Invention tools ─────────────────────────────────────────────────

smoke_test_ok!(
    smoke_policy_omniscience,
    "policy_omniscience",
    json!({"agent_id": "agent_1"})
);
smoke_test_ok!(
    smoke_risk_prophecy,
    "risk_prophecy",
    json!({"agent_id": "agent_1"})
);
smoke_test_ok!(
    smoke_approval_telepathy,
    "approval_telepathy",
    json!({"action": "deploy production"})
);
smoke_test_ok!(
    smoke_obligation_clairvoyance,
    "obligation_clairvoyance",
    json!({"agent_id": "agent_1"})
);
smoke_test_ok!(
    smoke_violation_precognition,
    "violation_precognition",
    json!({"planned_action": "delete production database"})
);
smoke_test_ok!(
    smoke_contract_crystallize,
    "contract_crystallize",
    json!({"intent": "Standard agent governance"})
);
smoke_test_ok!(
    smoke_trust_gradient_evaluate,
    "trust_gradient_evaluate",
    json!({"agent_id": "agent_1", "action": "deploy"})
);
smoke_test_ok!(
    smoke_collective_contract_create,
    "collective_contract_create",
    json!({"parties": [{"id": "a1", "name": "Agent1"}, {"id": "a2", "name": "Agent2"}]})
);
smoke_test_ok!(
    smoke_temporal_contract_create,
    "temporal_contract_create",
    json!({"label": "Quarterly review", "duration_secs": 7776000})
);
smoke_test_ok!(
    smoke_smart_escalation_route,
    "smart_escalation_route",
    json!({"description": "Rate limit exceeded", "urgency": 0.8})
);
smoke_test_ok!(
    smoke_violation_archaeology_analyze,
    "violation_archaeology_analyze",
    json!({"agent_id": "agent_1", "window_secs": 604800})
);
smoke_test_ok!(
    smoke_contract_simulation_run,
    "contract_simulation_run",
    json!({"scenario": "high load"})
);
smoke_test_ok!(
    smoke_federated_governance_create,
    "federated_governance_create",
    json!({"name": "Cross-team governance", "members": [{"id": "a1", "name": "Finance"}, {"id": "a2", "name": "Engineering"}]})
);

// Tools that require pre-existing data (need IDs from prior tool calls)
#[tokio::test]
async fn smoke_policy_dna_extract() {
    let mut engine = fresh_engine();
    // First add a policy so we have a valid ID
    let val = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "Test policy", "action": "allow", "scope": "global"}),
        &mut engine,
    )
    .await
    .expect("policy_add should succeed");
    let id = val["id"].as_str().expect("should have id field");
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_dna_extract",
        json!({"policy_id": id}),
        &mut engine,
    )
    .await;
    assert!(
        result.is_ok(),
        "policy_dna_extract should succeed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn smoke_contract_inheritance_create() {
    let mut engine = fresh_engine();
    let v1 = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "Parent policy", "action": "allow", "scope": "global"}),
        &mut engine,
    )
    .await
    .expect("policy_add should succeed");
    let parent_id = v1["id"].as_str().expect("should have id");
    let v2 = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": "Child policy", "action": "deny", "scope": "session"}),
        &mut engine,
    )
    .await
    .expect("policy_add should succeed");
    let child_id = v2["id"].as_str().expect("should have id");
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "contract_inheritance_create",
        json!({"parent_id": parent_id, "child_id": child_id, "propagate": true}),
        &mut engine,
    )
    .await;
    assert!(
        result.is_ok(),
        "contract_inheritance_create should succeed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn smoke_self_healing_contract_create() {
    let mut engine = fresh_engine();
    let val = agentic_contract_mcp::tools::handle_tool_call(
        "contract_create",
        json!({"label": "Base contract"}),
        &mut engine,
    )
    .await
    .expect("contract_create should succeed");
    let base_id = val["id"].as_str().expect("should have id");
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "self_healing_contract_create",
        json!({"base_contract_id": base_id}),
        &mut engine,
    )
    .await;
    assert!(
        result.is_ok(),
        "self_healing_contract_create should succeed: {:?}",
        result.err()
    );
}

// ── Error path smoke tests ─────────────────────────────────────────────

smoke_test_err!(smoke_unknown_tool, "totally_fake_tool", json!({}));
smoke_test_err!(
    smoke_contract_sign_bad_id,
    "contract_sign",
    json!({"contract_id": "nonexistent", "signer": "admin"})
);
smoke_test_err!(
    smoke_contract_verify_bad_id,
    "contract_verify",
    json!({"contract_id": "nonexistent"})
);
smoke_test_err!(
    smoke_contract_get_bad_id,
    "contract_get",
    json!({"id": "nonexistent"})
);
smoke_test_err!(
    smoke_approval_request_bad_rule,
    "approval_request",
    json!({"rule_id": "00000000-0000-0000-0000-000000000000", "action_description": "test", "requestor": "a1"})
);
smoke_test_err!(
    smoke_approval_decide_bad_request,
    "approval_decide",
    json!({"request_id": "nonexistent", "decision": "approve", "decider": "admin", "reason": "ok"})
);
smoke_test_err!(
    smoke_condition_evaluate_bad_id,
    "condition_evaluate",
    json!({"id": "nonexistent"})
);

// =========================================================================
// Section 2: MCP Quality Standard checks
// =========================================================================

#[test]
fn test_all_38_tools_present() {
    assert_eq!(agentic_contract_mcp::tools::TOOLS.len(), 38);
}

#[test]
fn test_tool_names_are_snake_case() {
    for tool in agentic_contract_mcp::tools::TOOLS {
        assert!(
            tool.name.chars().all(|c| c.is_lowercase() || c == '_'),
            "Tool name '{}' should be snake_case",
            tool.name
        );
    }
}

#[test]
fn test_descriptions_verb_first_no_trailing_period() {
    for tool in agentic_contract_mcp::tools::TOOLS {
        let first = tool.description.chars().next().unwrap();
        assert!(
            first.is_uppercase(),
            "Tool '{}' description should be verb-first (uppercase): '{}'",
            tool.name,
            tool.description
        );
        assert!(
            !tool.description.ends_with('.'),
            "Tool '{}' description should not end with period: '{}'",
            tool.name,
            tool.description
        );
    }
}

#[test]
fn test_all_schemas_parse_as_json() {
    for tool in agentic_contract_mcp::tools::TOOLS {
        let parsed: Result<Value, _> = serde_json::from_str(tool.input_schema);
        assert!(
            parsed.is_ok(),
            "Tool '{}' has invalid JSON schema",
            tool.name
        );
        let schema = parsed.unwrap();
        assert_eq!(
            schema["type"], "object",
            "Tool '{}' schema root should be type:object",
            tool.name
        );
    }
}

#[test]
fn test_no_duplicate_tool_names() {
    let mut names: Vec<&str> = agentic_contract_mcp::tools::TOOLS
        .iter()
        .map(|t| t.name)
        .collect();
    names.sort();
    let before = names.len();
    names.dedup();
    assert_eq!(before, names.len(), "Duplicate tool names found");
}

#[test]
fn test_resource_count_matches() {
    let resources = agentic_contract_mcp::resources::list_resources();
    assert_eq!(
        resources.len(),
        agentic_contract_mcp::resources::RESOURCE_COUNT
    );
}

#[test]
fn test_prompt_count_matches() {
    assert_eq!(agentic_contract_mcp::prompts::PROMPT_COUNT, 4);
}

// =========================================================================
// Section 3: Transport stress tests
// =========================================================================

#[test]
fn test_transport_roundtrip() {
    let mut output = Vec::new();
    let msg = r#"{"jsonrpc":"2.0","id":1,"method":"test"}"#;
    let framed = frame_message(msg);

    let mut transport =
        agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(framed), &mut output);
    let read = transport.read_message().unwrap();
    assert_eq!(read, msg);
}

#[test]
fn test_transport_write_framing() {
    let mut output = Vec::new();
    {
        let mut transport =
            agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(Vec::new()), &mut output);
        transport.write_message("hello").unwrap();
    }
    let written = String::from_utf8(output).unwrap();
    assert!(written.starts_with("Content-Length: 5\r\n\r\nhello"));
}

#[test]
fn test_transport_missing_content_length() {
    let bad_input = b"No-Header: here\r\n\r\n{}";
    let mut output = Vec::new();
    let mut transport = agentic_contract_mcp::stdio::StdioTransport::new(
        Cursor::new(bad_input.to_vec()),
        &mut output,
    );
    let result = transport.read_message();
    assert!(result.is_err());
}

#[test]
fn test_transport_max_size_enforcement() {
    let over_limit = agentic_contract_mcp::stdio::MAX_CONTENT_LENGTH_BYTES + 1;
    let header = format!("Content-Length: {}\r\n\r\n", over_limit);
    let mut output = Vec::new();
    let mut transport = agentic_contract_mcp::stdio::StdioTransport::new(
        Cursor::new(header.into_bytes()),
        &mut output,
    );
    let result = transport.read_message();
    assert!(result.is_err());
}

#[test]
fn test_transport_exact_max_size() {
    let max = agentic_contract_mcp::stdio::MAX_CONTENT_LENGTH_BYTES;
    // Just the header claiming max bytes — will fail on read_exact since no body
    let header = format!("Content-Length: {}\r\n\r\n", max);
    let mut output = Vec::new();
    let mut transport = agentic_contract_mcp::stdio::StdioTransport::new(
        Cursor::new(header.into_bytes()),
        &mut output,
    );
    // Should not reject on size (within limit) but fail on body read (not enough data)
    let result = transport.read_message();
    assert!(result.is_err()); // IO error, not MessageTooLarge
}

#[test]
fn test_transport_zero_length() {
    let header = b"Content-Length: 0\r\n\r\n";
    let mut output = Vec::new();
    let mut transport =
        agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(header.to_vec()), &mut output);
    let result = transport.read_message().unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_transport_multiple_messages() {
    let msg1 = r#"{"jsonrpc":"2.0","id":1,"method":"a"}"#;
    let msg2 = r#"{"jsonrpc":"2.0","id":2,"method":"b"}"#;
    let mut data = frame_message(msg1);
    data.extend(frame_message(msg2));

    let mut output = Vec::new();
    let mut transport =
        agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(data), &mut output);
    assert_eq!(transport.read_message().unwrap(), msg1);
    assert_eq!(transport.read_message().unwrap(), msg2);
}

#[test]
fn test_transport_eof_after_messages() {
    let msg = r#"{"test":true}"#;
    let data = frame_message(msg);
    let mut output = Vec::new();
    let mut transport =
        agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(data), &mut output);
    transport.read_message().unwrap(); // Read first
    let result = transport.read_message(); // Should be EOF
    assert!(result.is_err());
}

#[test]
fn test_transport_case_insensitive_header() {
    let input = b"content-length: 4\r\n\r\ntest";
    let mut output = Vec::new();
    let mut transport =
        agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(input.to_vec()), &mut output);
    assert_eq!(transport.read_message().unwrap(), "test");
}

// =========================================================================
// Section 4: JSON-RPC validation
// =========================================================================

#[test]
fn test_jsonrpc_valid_20() {
    let v: Value = serde_json::from_str(r#"{"jsonrpc":"2.0","method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&v).is_ok());
}

#[test]
fn test_jsonrpc_invalid_10() {
    let v: Value = serde_json::from_str(r#"{"jsonrpc":"1.0","method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&v).is_err());
}

#[test]
fn test_jsonrpc_missing_version() {
    let v: Value = serde_json::from_str(r#"{"method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&v).is_err());
}

#[test]
fn test_jsonrpc_null_version() {
    let v: Value = serde_json::from_str(r#"{"jsonrpc":null,"method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&v).is_err());
}

#[test]
fn test_jsonrpc_numeric_version() {
    let v: Value = serde_json::from_str(r#"{"jsonrpc":2.0,"method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&v).is_err());
}

#[test]
fn test_jsonrpc_empty_string_version() {
    let v: Value = serde_json::from_str(r#"{"jsonrpc":"","method":"test"}"#).unwrap();
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&v).is_err());
}

// =========================================================================
// Section 5: Concurrent tool execution
// =========================================================================

#[tokio::test]
async fn test_concurrent_policy_adds() {
    let engine = std::sync::Arc::new(tokio::sync::Mutex::new(fresh_engine()));
    let mut handles = Vec::new();

    for i in 0..50 {
        let engine = engine.clone();
        handles.push(tokio::spawn(async move {
            let mut eng = engine.lock().await;
            agentic_contract_mcp::tools::handle_tool_call(
                "policy_add",
                json!({"label": format!("Policy #{}", i)}),
                &mut eng,
            )
            .await
        }));
    }

    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }
    assert_eq!(success_count, 50);

    let eng = engine.lock().await;
    let stats = eng.stats();
    assert!(stats.policy_count >= 50);
}

#[tokio::test]
async fn test_concurrent_violation_reports() {
    let engine = std::sync::Arc::new(tokio::sync::Mutex::new(fresh_engine()));
    let mut handles = Vec::new();

    for i in 0..100 {
        let engine = engine.clone();
        handles.push(tokio::spawn(async move {
            let mut eng = engine.lock().await;
            agentic_contract_mcp::tools::handle_tool_call(
                "violation_report",
                json!({
                    "description": format!("Violation #{}", i),
                    "severity": if i % 4 == 0 { "critical" } else if i % 3 == 0 { "warning" } else { "info" },
                    "agent_id": format!("agent_{}", i % 5)
                }),
                &mut eng,
            )
            .await
        }));
    }

    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }

    let eng = engine.lock().await;
    let stats = eng.stats();
    assert_eq!(stats.violation_count, 100);
}

#[tokio::test]
async fn test_concurrent_mixed_operations() {
    let engine = std::sync::Arc::new(tokio::sync::Mutex::new(fresh_engine()));
    let mut handles = Vec::new();

    // Mix policy adds, risk limits, violations, and obligations
    for i in 0..40 {
        let engine = engine.clone();
        handles.push(tokio::spawn(async move {
            let mut eng = engine.lock().await;
            match i % 4 {
                0 => {
                    agentic_contract_mcp::tools::handle_tool_call(
                        "policy_add",
                        json!({"label": format!("P{}", i)}),
                        &mut eng,
                    )
                    .await
                }
                1 => {
                    agentic_contract_mcp::tools::handle_tool_call(
                        "risk_limit_set",
                        json!({"label": format!("L{}", i), "max_value": 100}),
                        &mut eng,
                    )
                    .await
                }
                2 => agentic_contract_mcp::tools::handle_tool_call(
                    "violation_report",
                    json!({"description": format!("V{}", i), "severity": "info", "agent_id": "a1"}),
                    &mut eng,
                )
                .await,
                _ => {
                    agentic_contract_mcp::tools::handle_tool_call(
                        "obligation_add",
                        json!({"label": format!("O{}", i), "deadline": future_dt()}),
                        &mut eng,
                    )
                    .await
                }
            }
        }));
    }

    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }

    let eng = engine.lock().await;
    let stats = eng.stats();
    let total = stats.policy_count
        + stats.risk_limit_count
        + stats.violation_count
        + stats.obligation_count;
    assert_eq!(total, 40);
}

// =========================================================================
// Section 6: Tool boundary and error path tests
// =========================================================================

#[tokio::test]
async fn test_empty_label_policy() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": ""}),
        &mut engine,
    )
    .await;
    // Should still work — no minimum length enforced
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_very_long_label() {
    let mut engine = fresh_engine();
    let long_label = "A".repeat(10_000);
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"label": long_label}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_special_chars_in_labels() {
    let mut engine = fresh_engine();
    let specials = [
        "label with\nnewlines",
        "label\twith\ttabs",
        r#"label "with" quotes"#,
        "label\\backslash",
        "label<html>tags</html>",
        "label & ampersand",
        "null\0byte",
    ];

    for label in &specials {
        let result = agentic_contract_mcp::tools::handle_tool_call(
            "policy_add",
            json!({"label": label}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed for label: {:?}", label);
    }
}

#[tokio::test]
async fn test_missing_required_fields() {
    let mut engine = fresh_engine();

    // policy_add missing label
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "policy_add",
        json!({"scope": "global"}),
        &mut engine,
    )
    .await;
    // Should handle gracefully — either error or default
    // The important thing is it doesn't panic
    let _ = result;
}

#[tokio::test]
async fn test_wrong_types_in_args() {
    let mut engine = fresh_engine();

    // max_value should be number, pass string
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_set",
        json!({"label": "test", "max_value": "not a number"}),
        &mut engine,
    )
    .await;
    // Should handle gracefully
    let _ = result;
}

#[tokio::test]
async fn test_negative_risk_limit() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_set",
        json!({"label": "negative", "max_value": -1}),
        &mut engine,
    )
    .await;
    // Should handle gracefully
    let _ = result;
}

#[tokio::test]
async fn test_zero_risk_limit() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_set",
        json!({"label": "zero", "max_value": 0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_massive_risk_limit() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "risk_limit_set",
        json!({"label": "huge", "max_value": f64::MAX}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_past_deadline_obligation() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "obligation_add",
        json!({"label": "overdue", "deadline": "2020-01-01T00:00:00Z"}),
        &mut engine,
    )
    .await;
    // Should still create — deadline enforcement at check time
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_invalid_datetime_obligation() {
    let mut engine = fresh_engine();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "obligation_add",
        json!({"label": "bad date", "deadline": "not-a-date"}),
        &mut engine,
    )
    .await;
    // Should handle gracefully
    let _ = result;
}

#[tokio::test]
async fn test_all_severity_levels() {
    let mut engine = fresh_engine();
    let severities = ["info", "warning", "critical", "fatal"];
    for sev in &severities {
        let result = agentic_contract_mcp::tools::handle_tool_call(
            "violation_report",
            json!({"description": format!("test {}", sev), "severity": sev, "agent_id": "a1"}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed for severity: {}", sev);
    }
}

#[tokio::test]
async fn test_all_policy_scopes() {
    let mut engine = fresh_engine();
    for scope in &["global", "session", "agent"] {
        let result = agentic_contract_mcp::tools::handle_tool_call(
            "policy_add",
            json!({"label": format!("{} policy", scope), "scope": scope}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed for scope: {}", scope);
    }
}

#[tokio::test]
async fn test_all_policy_actions() {
    let mut engine = fresh_engine();
    for action in &["allow", "deny", "require_approval", "audit_only"] {
        let result = agentic_contract_mcp::tools::handle_tool_call(
            "policy_add",
            json!({"label": format!("{} action", action), "action": action}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed for action: {}", action);
    }
}

#[tokio::test]
async fn test_all_limit_types() {
    let mut engine = fresh_engine();
    for lt in &["rate", "threshold", "budget", "count"] {
        let result = agentic_contract_mcp::tools::handle_tool_call(
            "risk_limit_set",
            json!({"label": format!("{} limit", lt), "max_value": 100, "limit_type": lt}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed for limit_type: {}", lt);
    }
}

// =========================================================================
// Section 7: Stats accuracy after bulk operations
// =========================================================================

#[tokio::test]
async fn test_stats_after_bulk_load() {
    let mut engine = fresh_engine();

    // Add various entities
    for i in 0..20 {
        agentic_contract_mcp::tools::handle_tool_call(
            "policy_add",
            json!({"label": format!("P{}", i)}),
            &mut engine,
        )
        .await
        .unwrap();
    }
    for i in 0..15 {
        agentic_contract_mcp::tools::handle_tool_call(
            "risk_limit_set",
            json!({"label": format!("L{}", i), "max_value": i * 10 + 1}),
            &mut engine,
        )
        .await
        .unwrap();
    }
    for i in 0..10 {
        agentic_contract_mcp::tools::handle_tool_call(
            "obligation_add",
            json!({"label": format!("O{}", i), "deadline": future_dt()}),
            &mut engine,
        )
        .await
        .unwrap();
    }
    for i in 0..25 {
        agentic_contract_mcp::tools::handle_tool_call(
            "violation_report",
            json!({"description": format!("V{}", i), "severity": "info", "agent_id": "a1"}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    let stats =
        agentic_contract_mcp::tools::handle_tool_call("contract_stats", json!({}), &mut engine)
            .await
            .unwrap();

    assert!(stats["policy_count"].as_u64().unwrap() >= 20);
    assert!(stats["risk_limit_count"].as_u64().unwrap() >= 15);
    assert!(stats["obligation_count"].as_u64().unwrap() >= 10);
    assert!(stats["violation_count"].as_u64().unwrap() >= 25);
}

// =========================================================================
// Section 8: Contract lifecycle through MCP
// =========================================================================

#[tokio::test]
async fn test_contract_create_sign_verify_list_get() {
    let mut engine = fresh_engine();

    // Create
    let created = agentic_contract_mcp::tools::handle_tool_call(
        "contract_create",
        json!({"label": "Service Agreement", "parties": ["agent_a", "agent_b"], "description": "Mutual trust"}),
        &mut engine,
    )
    .await
    .unwrap();
    let cid = created["id"].as_str().unwrap().to_string();

    // Sign
    let signed = agentic_contract_mcp::tools::handle_tool_call(
        "contract_sign",
        json!({"contract_id": &cid, "signer": "agent_a"}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(signed["signed"], true);

    // Verify
    let verified = agentic_contract_mcp::tools::handle_tool_call(
        "contract_verify",
        json!({"contract_id": &cid}),
        &mut engine,
    )
    .await
    .unwrap();
    assert!(verified.get("valid").is_some());

    // List
    let list =
        agentic_contract_mcp::tools::handle_tool_call("contract_list", json!({}), &mut engine)
            .await
            .unwrap();
    assert!(list["count"].as_u64().unwrap() >= 1);

    // Get
    let got = agentic_contract_mcp::tools::handle_tool_call(
        "contract_get",
        json!({"id": &cid}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(got["label"], "Service Agreement");
}

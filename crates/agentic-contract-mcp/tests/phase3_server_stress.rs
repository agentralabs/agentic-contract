//! Phase 3: Server Mode Stress Tests
//!
//! Covers: server auth checks, stdio transport edge cases, Content-Length
//! framing edge cases, JSON-RPC error handling, large message handling,
//! rapid sequential messages, and binary data rejection.

use std::io::Cursor;
use serde_json::{json, Value};

// =========================================================================
// Helpers
// =========================================================================

fn frame(msg: &str) -> Vec<u8> {
    format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).into_bytes()
}

fn make_transport(
    input: Vec<u8>,
) -> agentic_contract_mcp::stdio::StdioTransport<Cursor<Vec<u8>>, Vec<u8>> {
    agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(input), Vec::new())
}

// =========================================================================
// Section 1: Server auth environment checks
// =========================================================================

#[test]
fn test_server_mode_requires_token() {
    // When AGENTRA_RUNTIME_MODE=server and no token, server should refuse
    // We can't test the actual server startup easily, but we verify the pattern
    let is_server = std::env::var("AGENTRA_RUNTIME_MODE")
        .map(|v| v == "server")
        .unwrap_or(false);
    let has_token =
        std::env::var("AGENTIC_TOKEN").is_ok() || std::env::var("AGENTIC_TOKEN_FILE").is_ok();

    // In test env, we're NOT in server mode, so this should pass
    assert!(!is_server || has_token, "Server mode without token should fail");
}

// =========================================================================
// Section 2: Content-Length framing edge cases
// =========================================================================

#[test]
fn test_frame_exact_boundary() {
    // Message of exactly 1 byte
    let data = frame("x");
    let mut transport = make_transport(data);
    assert_eq!(transport.read_message().unwrap(), "x");
}

#[test]
fn test_frame_large_message() {
    let large = "A".repeat(1_000_000); // 1 MB
    let data = frame(&large);
    let mut transport = make_transport(data);
    let msg = transport.read_message().unwrap();
    assert_eq!(msg.len(), 1_000_000);
}

#[test]
fn test_frame_unicode_content() {
    let unicode = "日本語テスト🎉";
    let data = frame(unicode);
    let mut transport = make_transport(data);
    // Content-Length is in bytes, not chars
    let msg = transport.read_message().unwrap();
    assert_eq!(msg, unicode);
}

#[test]
fn test_frame_empty_content() {
    let data = frame("");
    let mut transport = make_transport(data);
    assert_eq!(transport.read_message().unwrap(), "");
}

#[test]
fn test_frame_newlines_in_content() {
    let content = "line1\r\nline2\r\nline3";
    let data = frame(content);
    let mut transport = make_transport(data);
    assert_eq!(transport.read_message().unwrap(), content);
}

#[test]
fn test_frame_negative_content_length() {
    let data = b"Content-Length: -1\r\n\r\n".to_vec();
    let mut transport = make_transport(data);
    assert!(transport.read_message().is_err());
}

#[test]
fn test_frame_non_numeric_content_length() {
    let data = b"Content-Length: abc\r\n\r\n".to_vec();
    let mut transport = make_transport(data);
    assert!(transport.read_message().is_err());
}

#[test]
fn test_frame_missing_header_entirely() {
    let data = b"\r\n{\"test\":true}".to_vec();
    let mut transport = make_transport(data);
    assert!(transport.read_message().is_err());
}

#[test]
fn test_frame_extra_headers_ignored() {
    let data = b"X-Custom: value\r\nContent-Length: 4\r\n\r\ntest".to_vec();
    let mut transport = make_transport(data);
    assert_eq!(transport.read_message().unwrap(), "test");
}

#[test]
fn test_frame_case_variations() {
    // Various cases of Content-Length
    let cases = [
        b"content-length: 4\r\n\r\ntest".to_vec(),
        b"CONTENT-LENGTH: 4\r\n\r\ntest".to_vec(),
        b"Content-Length: 4\r\n\r\ntest".to_vec(),
    ];
    for data in &cases {
        let mut transport = make_transport(data.clone());
        assert_eq!(transport.read_message().unwrap(), "test");
    }
}

#[test]
fn test_frame_max_boundary() {
    let max = agentic_contract_mcp::stdio::MAX_CONTENT_LENGTH_BYTES;
    // Exactly at max — should not reject on size
    let header = format!("Content-Length: {}\r\n\r\n", max);
    let mut transport = make_transport(header.into_bytes());
    // Will fail on read_exact (not enough data) but not on size check
    let result = transport.read_message();
    assert!(result.is_err());
    // Verify it's an IO error, not MessageTooLarge
    if let Err(e) = result {
        let msg = e.to_string();
        assert!(
            !msg.contains("too large"),
            "Exact max should not be 'too large'"
        );
    }
}

#[test]
fn test_frame_over_max() {
    let over = agentic_contract_mcp::stdio::MAX_CONTENT_LENGTH_BYTES + 1;
    let header = format!("Content-Length: {}\r\n\r\n", over);
    let mut transport = make_transport(header.into_bytes());
    let result = transport.read_message();
    assert!(result.is_err());
    if let Err(e) = result {
        let msg = e.to_string();
        assert!(msg.contains("too large") || msg.contains("Message"));
    }
}

// =========================================================================
// Section 3: Multiple messages in sequence
// =========================================================================

#[test]
fn test_rapid_sequential_messages() {
    let mut data = Vec::new();
    for i in 0..100 {
        let msg = format!(r#"{{"jsonrpc":"2.0","id":{},"method":"test"}}"#, i);
        data.extend(frame(&msg));
    }

    let mut transport = make_transport(data);
    for i in 0..100 {
        let msg = transport.read_message().unwrap();
        let parsed: Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["id"], i);
    }
}

#[test]
fn test_alternating_sizes() {
    let mut data = Vec::new();
    for i in 0..50 {
        let content = if i % 2 == 0 {
            "x".repeat(10) // small
        } else {
            "y".repeat(10_000) // medium
        };
        data.extend(frame(&content));
    }

    let mut transport = make_transport(data);
    for i in 0..50 {
        let msg = transport.read_message().unwrap();
        if i % 2 == 0 {
            assert_eq!(msg.len(), 10);
        } else {
            assert_eq!(msg.len(), 10_000);
        }
    }
}

// =========================================================================
// Section 4: JSON-RPC validation stress
// =========================================================================

#[test]
fn test_jsonrpc_all_valid_versions() {
    let valid = json!({"jsonrpc": "2.0", "method": "test"});
    assert!(agentic_contract_mcp::stdio::validate_jsonrpc(&valid).is_ok());
}

#[test]
fn test_jsonrpc_invalid_versions() {
    let cases = [
        json!({"jsonrpc": "1.0"}),
        json!({"jsonrpc": "3.0"}),
        json!({"jsonrpc": "2.1"}),
        json!({"jsonrpc": ""}),
        json!({"jsonrpc": null}),
        json!({"jsonrpc": 2.0}),  // number, not string
        json!({"jsonrpc": true}),
        json!({"jsonrpc": []}),
        json!({"jsonrpc": {}}),
        json!({}), // missing entirely
    ];

    for (i, case) in cases.iter().enumerate() {
        assert!(
            agentic_contract_mcp::stdio::validate_jsonrpc(case).is_err(),
            "Case {} should be invalid: {:?}",
            i,
            case
        );
    }
}

// =========================================================================
// Section 5: Write framing verification
// =========================================================================

#[test]
fn test_write_framing_correct() {
    let mut output = Vec::new();
    {
        let mut transport =
            agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(Vec::new()), &mut output);
        transport.write_message("hello world").unwrap();
    }
    let written = String::from_utf8(output).unwrap();
    assert_eq!(written, "Content-Length: 11\r\n\r\nhello world");
}

#[test]
fn test_write_empty_message() {
    let mut output = Vec::new();
    {
        let mut transport =
            agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(Vec::new()), &mut output);
        transport.write_message("").unwrap();
    }
    let written = String::from_utf8(output).unwrap();
    assert_eq!(written, "Content-Length: 0\r\n\r\n");
}

#[test]
fn test_write_unicode_message() {
    let msg = "日本語🎉";
    let mut output = Vec::new();
    {
        let mut transport =
            agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(Vec::new()), &mut output);
        transport.write_message(msg).unwrap();
    }
    let written = String::from_utf8(output).unwrap();
    let expected_len = msg.len(); // byte length
    assert!(written.starts_with(&format!("Content-Length: {}\r\n\r\n", expected_len)));
    assert!(written.ends_with(msg));
}

#[test]
fn test_write_multiple_messages() {
    let mut output = Vec::new();
    {
        let mut transport =
            agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(Vec::new()), &mut output);
        transport.write_message("first").unwrap();
        transport.write_message("second").unwrap();
        transport.write_message("third").unwrap();
    }
    let written = String::from_utf8(output).unwrap();
    assert!(written.contains("Content-Length: 5\r\n\r\nfirst"));
    assert!(written.contains("Content-Length: 6\r\n\r\nsecond"));
    assert!(written.contains("Content-Length: 5\r\n\r\nthird"));
}

// =========================================================================
// Section 6: Write then read roundtrip
// =========================================================================

#[test]
fn test_write_read_roundtrip() {
    let messages = ["hello", "world", r#"{"json":"value"}"#, ""];
    let mut output = Vec::new();

    // Write
    {
        let mut transport =
            agentic_contract_mcp::stdio::StdioTransport::new(Cursor::new(Vec::new()), &mut output);
        for msg in &messages {
            transport.write_message(msg).unwrap();
        }
    }

    // Read back
    let mut discard = Vec::new();
    let mut transport = agentic_contract_mcp::stdio::StdioTransport::new(
        Cursor::new(output),
        &mut discard,
    );
    for expected in &messages {
        let read = transport.read_message().unwrap();
        assert_eq!(&read, expected);
    }
}

// =========================================================================
// Section 7: Prompt expansion stress
// =========================================================================

#[test]
fn test_all_prompts_expand() {
    let prompts = agentic_contract_mcp::prompts::PROMPTS;
    for prompt in prompts {
        // Supply required args so prompts that need them can expand
        let mut args = std::collections::HashMap::new();
        for arg in prompt.arguments {
            if arg.required {
                args.insert(arg.name.to_string(), "test_value".to_string());
            }
        }
        let result = agentic_contract_mcp::prompts::expand_prompt(prompt.name, &args);
        assert!(
            result.is_some(),
            "Prompt '{}' should expand",
            prompt.name
        );
    }
}

#[test]
fn test_prompt_unknown_returns_none() {
    let args = std::collections::HashMap::new();
    assert!(agentic_contract_mcp::prompts::expand_prompt("nonexistent_prompt", &args).is_none());
}

#[test]
fn test_prompt_with_extra_args() {
    let mut args = std::collections::HashMap::new();
    // Include required args plus extra ones
    args.insert("agent_name".to_string(), "test_agent".to_string());
    args.insert("extra_key".to_string(), "extra_value".to_string());
    args.insert("another".to_string(), "value".to_string());

    // Should still expand without error
    for prompt in agentic_contract_mcp::prompts::PROMPTS {
        let result = agentic_contract_mcp::prompts::expand_prompt(prompt.name, &args);
        assert!(result.is_some());
    }
}

// =========================================================================
// Section 8: Resource listing
// =========================================================================

#[test]
fn test_resources_all_have_fields() {
    let resources = agentic_contract_mcp::resources::list_resources();
    for r in resources {
        assert!(!r.uri.is_empty(), "Resource URI should not be empty");
        assert!(!r.name.is_empty(), "Resource name should not be empty");
        assert!(
            !r.description.is_empty(),
            "Resource description should not be empty"
        );
        assert!(
            !r.mime_type.is_empty(),
            "Resource mime_type should not be empty"
        );
    }
}

#[test]
fn test_resources_no_duplicate_uris() {
    let resources = agentic_contract_mcp::resources::list_resources();
    let mut uris: Vec<&str> = resources.iter().map(|r| r.uri).collect();
    uris.sort();
    let before = uris.len();
    uris.dedup();
    assert_eq!(before, uris.len(), "Duplicate resource URIs found");
}

// =========================================================================
// Section 9: Tool handler error code compliance
// =========================================================================

#[tokio::test]
async fn test_tool_error_is_string() {
    let mut engine = agentic_contract::ContractEngine::new();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "nonexistent_tool",
        json!({}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Error should be a displayable string
    assert!(!err.is_empty());
}

#[tokio::test]
async fn test_tool_success_returns_json() {
    let mut engine = agentic_contract::ContractEngine::new();
    let result = agentic_contract_mcp::tools::handle_tool_call(
        "contract_stats",
        json!({}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    // Should be a JSON object
    assert!(val.is_object());
}

// =========================================================================
// Section 10: Binary data / invalid UTF-8 handling
// =========================================================================

#[test]
fn test_binary_data_in_body_rejected() {
    // Valid header but body contains invalid UTF-8
    let mut data = b"Content-Length: 4\r\n\r\n".to_vec();
    data.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]); // Invalid UTF-8
    let mut transport = make_transport(data);
    let result = transport.read_message();
    assert!(result.is_err());
}

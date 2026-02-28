//! MCP server — handles JSON-RPC messages over stdio.

use std::path::PathBuf;

use serde_json::{json, Value};

use crate::greeting;
use crate::invention_generation;
use crate::invention_governance;
use crate::invention_resilience;
use crate::invention_visibility;
use crate::prompts;
use crate::stdio::{validate_jsonrpc, StdioTransport, TransportError};
use crate::tools;

/// Check AGENTIC_TOKEN for server-mode authentication.
fn check_server_auth() -> Result<(), Box<dyn std::error::Error>> {
    let is_server_mode = std::env::var("AGENTRA_RUNTIME_MODE")
        .map(|v| v == "server")
        .unwrap_or(false)
        || std::env::var("AGENTRA_SERVER")
            .map(|v| v == "1")
            .unwrap_or(false);

    if is_server_mode {
        let has_token =
            std::env::var("AGENTIC_TOKEN").is_ok() || std::env::var("AGENTIC_TOKEN_FILE").is_ok();
        if !has_token {
            return Err("Server mode requires AGENTIC_TOKEN or AGENTIC_TOKEN_FILE".into());
        }
    }
    Ok(())
}

/// Run the MCP server on stdio.
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    check_server_auth()?;
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    let mut transport = StdioTransport::new(stdin, stdout);

    // Determine .acon file path
    let acon_path = std::env::var("ACON_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs_home().join(".agentic").join("contract.acon"));

    // Ensure parent dir exists
    if let Some(parent) = acon_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Print startup greeting to stderr (MCP uses stdout for JSON-RPC)
    greeting::print_greeting();

    // Open or create the engine
    let mut engine = if acon_path.exists() {
        agentic_contract::ContractEngine::open(&acon_path).map_err(|e| e.to_string())?
    } else {
        agentic_contract::ContractEngine::new()
    };

    tracing::info!("AgenticContract MCP server started");

    loop {
        let msg = match transport.read_message() {
            Ok(m) => m,
            Err(TransportError::Io(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                tracing::info!("Client disconnected");
                break;
            }
            Err(e) => {
                tracing::error!("Transport error: {}", e);
                let err_response = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": e.to_string() }
                });
                let _ = transport.write_message(&err_response.to_string());
                continue;
            }
        };

        let request: Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(e) => {
                let err_response = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) }
                });
                let _ = transport.write_message(&err_response.to_string());
                continue;
            }
        };

        if let Err(e) = validate_jsonrpc(&request) {
            let err_response = json!({
                "jsonrpc": "2.0",
                "id": request.get("id").cloned().unwrap_or(Value::Null),
                "error": { "code": -32600, "message": e.to_string() }
            });
            let _ = transport.write_message(&err_response.to_string());
            continue;
        }

        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let response = match method {
            "initialize" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": { "listChanged": false },
                            "resources": { "subscribe": false, "listChanged": false },
                            "prompts": { "listChanged": false }
                        },
                        "serverInfo": {
                            "name": "agentic-contract",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    }
                })
            }
            "notifications/initialized" => continue, // No response needed
            "tools/list" => {
                // Concatenate core tools with all invention module tools
                let all_tools: Vec<&tools::ToolDefinition> = tools::TOOLS
                    .iter()
                    .chain(invention_visibility::TOOL_DEFS.iter())
                    .chain(invention_generation::TOOL_DEFS.iter())
                    .chain(invention_governance::TOOL_DEFS.iter())
                    .chain(invention_resilience::TOOL_DEFS.iter())
                    .collect();

                let tool_list: Vec<Value> = all_tools
                    .iter()
                    .map(|t| {
                        json!({
                            "name": t.name,
                            "description": t.description,
                            "inputSchema": serde_json::from_str::<Value>(t.input_schema).unwrap_or(json!({}))
                        })
                    })
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "tools": tool_list }
                })
            }
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(json!({}));
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                // Check if tool exists in core tools or any invention module
                let tool_exists = tools::TOOLS.iter().any(|t| t.name == tool_name)
                    || invention_visibility::TOOL_DEFS.iter().any(|t| t.name == tool_name)
                    || invention_generation::TOOL_DEFS.iter().any(|t| t.name == tool_name)
                    || invention_governance::TOOL_DEFS.iter().any(|t| t.name == tool_name)
                    || invention_resilience::TOOL_DEFS.iter().any(|t| t.name == tool_name);

                if !tool_exists {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32803,
                            "message": format!("Tool not found: {}", tool_name)
                        }
                    })
                } else {
                    // Try invention modules first, then fall back to core tools
                    let result = invention_visibility::try_handle(tool_name, args.clone(), &mut engine)
                        .or_else(|| invention_generation::try_handle(tool_name, args.clone(), &mut engine))
                        .or_else(|| invention_governance::try_handle(tool_name, args.clone(), &mut engine))
                        .or_else(|| invention_resilience::try_handle(tool_name, args.clone(), &mut engine));

                    let tool_result = match result {
                        Some(r) => r,
                        None => tools::handle_tool_call(tool_name, args, &mut engine).await,
                    };

                    match tool_result {
                        Ok(result) => json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                                }]
                            }
                        }),
                        Err(e) => json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [{
                                    "type": "text",
                                    "text": format!("Error: {}", e)
                                }],
                                "isError": true
                            }
                        }),
                    }
                }
            }
            "resources/list" => {
                let resources: Vec<Value> = crate::resources::list_resources()
                    .iter()
                    .map(|r| {
                        json!({
                            "uri": r.uri,
                            "name": r.name,
                            "description": r.description,
                            "mimeType": r.mime_type
                        })
                    })
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "resources": resources }
                })
            }
            "prompts/list" => {
                let prompt_list: Vec<Value> = crate::prompts::PROMPTS
                    .iter()
                    .map(|p| {
                        let args: Vec<Value> = p
                            .arguments
                            .iter()
                            .map(|a| {
                                json!({
                                    "name": a.name,
                                    "description": a.description,
                                    "required": a.required
                                })
                            })
                            .collect();
                        json!({
                            "name": p.name,
                            "description": p.description,
                            "arguments": args
                        })
                    })
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "prompts": prompt_list }
                })
            }
            "prompts/get" => {
                let params = request.get("params").cloned().unwrap_or(json!({}));
                let prompt_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let arguments = params
                    .get("arguments")
                    .and_then(|a| {
                        serde_json::from_value::<std::collections::HashMap<String, String>>(
                            a.clone(),
                        )
                        .ok()
                    })
                    .unwrap_or_default();

                match prompts::expand_prompt(prompt_name, &arguments) {
                    Some(content) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "messages": [{
                                "role": "user",
                                "content": { "type": "text", "text": content }
                            }]
                        }
                    }),
                    None => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32602, "message": format!("Prompt not found: {}", prompt_name) }
                    }),
                }
            }
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": format!("Method not found: {}", method) }
                })
            }
        };

        transport.write_message(&response.to_string())?;
    }

    Ok(())
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

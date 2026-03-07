//! MCP server — handles JSON-RPC messages over stdio.

use std::path::PathBuf;

use serde_json::{json, Value};

use crate::ghost_bridge;
use crate::greeting;
use crate::invention_generation;
use crate::invention_governance;
use crate::invention_resilience;
use crate::invention_visibility;
use crate::prompts;
use crate::stdio::{validate_jsonrpc, StdioTransport, TransportError};
use crate::tools;

#[derive(Debug, Clone)]
struct RuntimeConfig {
    auto_capture_mode: String,
    auto_capture_redact: bool,
    auto_capture_max_chars: usize,
    storage_budget_mode: String,
    storage_budget_bytes: u64,
    storage_budget_horizon_years: u32,
    storage_budget_target_fraction: f64,
}

impl RuntimeConfig {
    fn from_env() -> Self {
        Self {
            auto_capture_mode: env_with_fallback("ACON_AUTO_CAPTURE_MODE", "AUTO_CAPTURE_MODE")
                .unwrap_or_else(|| "summary".to_string()),
            auto_capture_redact: env_with_fallback(
                "ACON_AUTO_CAPTURE_REDACT",
                "AUTO_CAPTURE_REDACT",
            )
            .map(|v| parse_bool(&v))
            .unwrap_or(true),
            auto_capture_max_chars: env_with_fallback(
                "ACON_AUTO_CAPTURE_MAX_CHARS",
                "AUTO_CAPTURE_MAX_CHARS",
            )
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(768),
            storage_budget_mode: env_with_fallback(
                "ACON_STORAGE_BUDGET_MODE",
                "STORAGE_BUDGET_MODE",
            )
            .unwrap_or_else(|| "auto-rollup".to_string()),
            storage_budget_bytes: env_with_fallback(
                "ACON_STORAGE_BUDGET_BYTES",
                "STORAGE_BUDGET_BYTES",
            )
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(536_870_912),
            storage_budget_horizon_years: env_with_fallback(
                "ACON_STORAGE_BUDGET_HORIZON_YEARS",
                "STORAGE_BUDGET_HORIZON_YEARS",
            )
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(5),
            storage_budget_target_fraction: env_with_fallback(
                "ACON_STORAGE_BUDGET_TARGET_FRACTION",
                "STORAGE_BUDGET_TARGET_FRACTION",
            )
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.85),
        }
    }

    fn as_json(&self) -> Value {
        json!({
            "auto_capture_mode": self.auto_capture_mode,
            "auto_capture_redact": self.auto_capture_redact,
            "auto_capture_max_chars": self.auto_capture_max_chars,
            "storage_budget_mode": self.storage_budget_mode,
            "storage_budget_bytes": self.storage_budget_bytes,
            "storage_budget_horizon_years": self.storage_budget_horizon_years,
            "storage_budget_target_fraction": self.storage_budget_target_fraction
        })
    }

    fn enforce_storage_budget(&self, acon_path: &PathBuf) -> Result<(), String> {
        if self.storage_budget_mode.eq_ignore_ascii_case("off") {
            return Ok(());
        }
        let metadata = std::fs::metadata(acon_path).map_err(|e| e.to_string())?;
        let used = metadata.len();
        let threshold =
            (self.storage_budget_bytes as f64 * self.storage_budget_target_fraction).round() as u64;
        if used > threshold {
            tracing::warn!(
                "storage budget near/exceeded: used={} threshold={} mode={} horizon_years={}",
                used,
                threshold,
                self.storage_budget_mode,
                self.storage_budget_horizon_years
            );
        }
        Ok(())
    }
}

fn env_with_fallback(primary: &str, fallback: &str) -> Option<String> {
    std::env::var(primary)
        .ok()
        .or_else(|| std::env::var(fallback).ok())
}

fn parse_bool(v: &str) -> bool {
    matches!(
        v.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn mcp_tool_surface_is_compact() -> bool {
    env_with_fallback("ACON_MCP_TOOL_SURFACE", "MCP_TOOL_SURFACE")
        .map(|v| v.eq_ignore_ascii_case("compact"))
        .unwrap_or(false)
}

fn compact_input_schema(ops: &[String], description: &str) -> Value {
    json!({
        "type": "object",
        "required": ["operation"],
        "properties": {
            "operation": {
                "type": "string",
                "enum": ops,
                "description": description
            },
            "params": {
                "type": "object",
                "description": "Arguments for the selected operation"
            }
        }
    })
}

fn ops_from_defs(defs: &[tools::ToolDefinition]) -> Vec<String> {
    defs.iter().map(|d| d.name.to_string()).collect()
}

fn core_ops_filtered<F>(filter: F) -> Vec<String>
where
    F: Fn(&str) -> bool,
{
    tools::TOOLS
        .iter()
        .filter_map(|t| {
            if filter(t.name) {
                Some(t.name.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn ops_for_group(group: &str) -> Option<Vec<String>> {
    match group {
        "contract_main" => Some(core_ops_filtered(|name| {
            name.starts_with("contract_")
                && !name.starts_with("contract_workspace_")
                && name != "contract_session_resume"
        })),
        "contract_policy" => Some(core_ops_filtered(|name| name.starts_with("policy_"))),
        "contract_risk" => Some(core_ops_filtered(|name| name.starts_with("risk_limit_"))),
        "contract_approval" => Some(core_ops_filtered(|name| name.starts_with("approval_"))),
        "contract_enforcement" => Some(core_ops_filtered(|name| {
            name.starts_with("condition_")
                || name.starts_with("obligation_")
                || name.starts_with("violation_")
        })),
        "contract_workspace" => Some(core_ops_filtered(|name| {
            name.starts_with("contract_workspace_")
                || name == "session_start"
                || name == "session_end"
                || name == "contract_session_resume"
        })),
        "contract_visibility" => Some(ops_from_defs(invention_visibility::TOOL_DEFS)),
        "contract_generation" => Some(ops_from_defs(invention_generation::TOOL_DEFS)),
        "contract_governance" => Some(ops_from_defs(invention_governance::TOOL_DEFS)),
        "contract_resilience" => Some(ops_from_defs(invention_resilience::TOOL_DEFS)),
        _ => None,
    }
}

fn compact_tool_list() -> Vec<Value> {
    vec![
        json!({
            "name": "contract_main",
            "description": "Compact contract core facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_main").unwrap_or_default(), "Contract core operation")
        }),
        json!({
            "name": "contract_policy",
            "description": "Compact policy facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_policy").unwrap_or_default(), "Policy operation")
        }),
        json!({
            "name": "contract_risk",
            "description": "Compact risk-limit facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_risk").unwrap_or_default(), "Risk-limit operation")
        }),
        json!({
            "name": "contract_approval",
            "description": "Compact approval facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_approval").unwrap_or_default(), "Approval operation")
        }),
        json!({
            "name": "contract_enforcement",
            "description": "Compact condition/obligation/violation facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_enforcement").unwrap_or_default(), "Enforcement operation")
        }),
        json!({
            "name": "contract_workspace",
            "description": "Compact workspace/session facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_workspace").unwrap_or_default(), "Workspace/session operation")
        }),
        json!({
            "name": "contract_visibility",
            "description": "Compact visibility inventions facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_visibility").unwrap_or_default(), "Visibility invention operation")
        }),
        json!({
            "name": "contract_generation",
            "description": "Compact generation inventions facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_generation").unwrap_or_default(), "Generation invention operation")
        }),
        json!({
            "name": "contract_governance",
            "description": "Compact governance inventions facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_governance").unwrap_or_default(), "Governance invention operation")
        }),
        json!({
            "name": "contract_resilience",
            "description": "Compact resilience inventions facade",
            "inputSchema": compact_input_schema(&ops_for_group("contract_resilience").unwrap_or_default(), "Resilience invention operation")
        }),
    ]
}

fn decode_compact_operation(args: Value) -> Result<(String, Value), String> {
    let obj = args
        .as_object()
        .ok_or_else(|| "arguments must be an object".to_string())?;
    let operation = obj
        .get("operation")
        .and_then(Value::as_str)
        .ok_or_else(|| "'operation' is required".to_string())?
        .to_string();

    if let Some(params) = obj.get("params") {
        return Ok((operation, params.clone()));
    }

    let mut passthrough = obj.clone();
    passthrough.remove("operation");
    Ok((operation, Value::Object(passthrough)))
}

fn normalize_compact_tool_call(tool_name: &str, args: Value) -> Result<(String, Value), String> {
    if !matches!(
        tool_name,
        "contract_main"
            | "contract_policy"
            | "contract_risk"
            | "contract_approval"
            | "contract_enforcement"
            | "contract_workspace"
            | "contract_visibility"
            | "contract_generation"
            | "contract_governance"
            | "contract_resilience"
    ) {
        return Ok((tool_name.to_string(), args));
    }

    let (operation, params) = decode_compact_operation(args)?;
    let ops = ops_for_group(tool_name).unwrap_or_default();
    if ops.iter().any(|op| op == &operation) {
        Ok((operation, params))
    } else {
        Err(format!("Unknown {tool_name} operation: {operation}"))
    }
}

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
pub fn run_server() -> Result<(), Box<dyn std::error::Error>> {
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

    let runtime = RuntimeConfig::from_env();

    // Print startup greeting to stderr (MCP uses stdout for JSON-RPC)
    greeting::print_greeting();

    // Open or create the engine
    let mut engine = if acon_path.exists() {
        agentic_contract::ContractEngine::open(&acon_path).map_err(|e| e.to_string())?
    } else {
        let mut e = agentic_contract::ContractEngine::new();
        e.file.path = Some(acon_path.clone());
        e
    };

    tracing::info!("AgenticContract MCP server started");

    // Ghost bridge — sync context to AI coding assistants after each request
    let mut ghost = ghost_bridge::GhostBridge::new();

    loop {
        if let Err(e) = runtime.enforce_storage_budget(&acon_path) {
            tracing::warn!("storage budget check failed: {}", e);
        }

        let msg = match transport.read_message() {
            Ok(m) => m,
            Err(TransportError::Io(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                tracing::info!("Client disconnected, saving before exit");
                if let Some(ref mut g) = ghost {
                    g.sync(&engine);
                }
                if let Err(e) = engine.save() {
                    tracing::error!("Failed to save on disconnect: {}", e);
                }
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
                        },
                        "runtimeConfig": runtime.as_json()
                    }
                })
            }
            "notifications/initialized" => continue, // No response needed
            "tools/list" => {
                let tool_list: Vec<Value> = if mcp_tool_surface_is_compact() {
                    compact_tool_list()
                } else {
                    // Concatenate core tools with all invention module tools
                    let all_tools: Vec<&tools::ToolDefinition> = tools::TOOLS
                        .iter()
                        .chain(invention_visibility::TOOL_DEFS.iter())
                        .chain(invention_generation::TOOL_DEFS.iter())
                        .chain(invention_governance::TOOL_DEFS.iter())
                        .chain(invention_resilience::TOOL_DEFS.iter())
                        .collect();

                    all_tools
                        .iter()
                        .map(|t| {
                            json!({
                                "name": t.name,
                                "description": t.description,
                                "inputSchema": serde_json::from_str::<Value>(t.input_schema).unwrap_or(json!({}))
                            })
                        })
                        .collect()
                };

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "tools": tool_list }
                })
            }
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(json!({}));
                let requested_tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let raw_args = params.get("arguments").cloned().unwrap_or(json!({}));
                let (tool_name, args) =
                    match normalize_compact_tool_call(requested_tool_name, raw_args) {
                        Ok(mapped) => mapped,
                        Err(e) => {
                            let response = json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "content": [{
                                        "type": "text",
                                        "text": format!("Error: {}", e)
                                    }],
                                    "isError": true
                                }
                            });
                            transport.write_message(&response.to_string())?;
                            continue;
                        }
                    };

                // Check if tool exists in core tools or any invention module
                let tool_exists = tools::TOOLS.iter().any(|t| t.name == tool_name)
                    || invention_visibility::TOOL_DEFS
                        .iter()
                        .any(|t| t.name == tool_name)
                    || invention_generation::TOOL_DEFS
                        .iter()
                        .any(|t| t.name == tool_name)
                    || invention_governance::TOOL_DEFS
                        .iter()
                        .any(|t| t.name == tool_name)
                    || invention_resilience::TOOL_DEFS
                        .iter()
                        .any(|t| t.name == tool_name);

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
                    let result =
                        invention_visibility::try_handle(&tool_name, args.clone(), &mut engine)
                            .or_else(|| {
                                invention_generation::try_handle(
                                    &tool_name,
                                    args.clone(),
                                    &mut engine,
                                )
                            })
                            .or_else(|| {
                                invention_governance::try_handle(
                                    &tool_name,
                                    args.clone(),
                                    &mut engine,
                                )
                            })
                            .or_else(|| {
                                invention_resilience::try_handle(
                                    &tool_name,
                                    args.clone(),
                                    &mut engine,
                                )
                            });

                    let tool_result = match result {
                        Some(r) => r,
                        None => tools::handle_tool_call(&tool_name, args, &mut engine),
                    };

                    match tool_result {
                        Ok(result) => {
                            // Persist after every successful tool call
                            if let Err(save_err) = engine.save() {
                                tracing::error!("Failed to save after tool call: {}", save_err);
                            }
                            json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "content": [{
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                                    }]
                                }
                            })
                        }
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

        // Sync ghost bridge after each request
        if let Some(ref mut g) = ghost {
            g.sync(&engine);
        }
    }

    Ok(())
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn compact_tool_list_has_expected_surface() {
        let defs = compact_tool_list();
        let names: Vec<&str> = defs
            .iter()
            .filter_map(|d| d.get("name").and_then(|n| n.as_str()))
            .collect();
        assert_eq!(defs.len(), 10);
        assert!(names.contains(&"contract_main"));
        assert!(names.contains(&"contract_policy"));
        assert!(names.contains(&"contract_risk"));
        assert!(names.contains(&"contract_approval"));
        assert!(names.contains(&"contract_enforcement"));
        assert!(names.contains(&"contract_workspace"));
        assert!(names.contains(&"contract_visibility"));
        assert!(names.contains(&"contract_generation"));
        assert!(names.contains(&"contract_governance"));
        assert!(names.contains(&"contract_resilience"));
    }

    #[test]
    fn compact_main_operation_routes_to_legacy_name() {
        let (name, args) = normalize_compact_tool_call(
            "contract_main",
            json!({
                "operation": "contract_create",
                "params": {
                    "label": "Ops Agreement"
                }
            }),
        )
        .unwrap_or_else(|_| Default::default());
        assert_eq!(name, "contract_create");
        assert_eq!(args["label"], "Ops Agreement");
    }

    #[test]
    fn compact_workspace_operation_routes_to_legacy_name() {
        let (name, args) = normalize_compact_tool_call(
            "contract_workspace",
            json!({
                "operation": "contract_session_resume",
                "params": { "limit": 2 }
            }),
        )
        .unwrap_or_else(|_| Default::default());
        assert_eq!(name, "contract_session_resume");
        assert_eq!(args["limit"], 2);
    }

    #[test]
    fn compact_unknown_operation_errors() {
        let err = normalize_compact_tool_call(
            "contract_policy",
            json!({
                "operation": "does_not_exist"
            }),
        )
        .expect_err("unknown operation should fail");
        assert!(err.contains("Unknown contract_policy operation"));
    }
}

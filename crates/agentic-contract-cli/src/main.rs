//! AgenticContract CLI — `acon` command-line interface.

mod commands;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkspaceContext {
    path: String,
    role: String,
    label: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct WorkspaceState {
    workspaces: HashMap<String, Vec<WorkspaceContext>>,
}

#[derive(Debug, Clone, Serialize)]
struct EntityRecord {
    id: String,
    entity_type: String,
    label: String,
    status: Option<String>,
    text: String,
    raw: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct QueryResult {
    id: String,
    entity_type: String,
    label: String,
    status: Option<String>,
    score: f32,
}

#[derive(Parser)]
#[command(name = "acon", version, about = "Policy engine for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the .acon file
    #[arg(long, global = true, env = "ACON_PATH")]
    path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new .acon file if it does not exist
    Init {
        /// Optional explicit .acon path
        path: Option<String>,
    },
    /// Manage policies
    Policy(commands::policy::PolicyArgs),
    /// Manage risk limits
    Limit(commands::limit::LimitArgs),
    /// Manage approval workflow
    Approval(commands::approval::ApprovalArgs),
    /// Manage obligations
    Obligation(commands::obligation::ObligationArgs),
    /// Manage violations
    Violation(commands::violation::ViolationArgs),
    /// Query contract entities
    Query {
        /// Entity type filter: policy, risk_limit, approval_request, condition, obligation, violation
        #[arg(long)]
        entity_type: Option<String>,
        /// Free-text query
        #[arg(long)]
        text: Option<String>,
        /// Optional status filter
        #[arg(long)]
        status: Option<String>,
        /// Max rows
        #[arg(long, default_value_t = 25)]
        limit: usize,
    },
    /// Export contract data as JSON
    Export {
        /// Optional output path; stdout if omitted
        #[arg(long)]
        output: Option<String>,
        /// Entity type filter
        #[arg(long)]
        entity_type: Option<String>,
        /// Pretty-print JSON
        #[arg(long)]
        pretty: bool,
    },
    /// Verify a claim has contract backing
    Ground {
        claim: String,
        #[arg(long, default_value_t = 0.35)]
        threshold: f32,
    },
    /// Get evidence rows for a query
    Evidence {
        query: String,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Get suggestion rows for a query
    Suggest {
        query: String,
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
    /// Workspace operations across multiple .acon files
    Workspace {
        #[command(subcommand)]
        subcommand: WorkspaceCommands,
    },
    /// Runtime sync workflow for scanning .acon files
    RuntimeSync {
        /// Workspace root to scan
        #[arg(long, default_value = ".")]
        workspace: String,
        /// Maximum directory depth
        #[arg(long, default_value_t = 4)]
        max_depth: u32,
    },
    /// Show contract statistics
    Stats,
    /// Show file info
    Info {
        /// File path
        path: Option<String>,
    },
    /// Install MCP configuration
    Install {
        /// Install profile
        #[arg(long, default_value = "desktop")]
        profile: String,
    },
    /// Start MCP server
    Serve,
}

#[derive(Subcommand)]
enum WorkspaceCommands {
    /// Create a workspace label
    Create { workspace: String },
    /// Add an .acon file to a workspace
    Add {
        workspace: String,
        path: String,
        #[arg(long, default_value = "primary")]
        role: String,
        #[arg(long)]
        label: Option<String>,
    },
    /// List contexts in a workspace
    List { workspace: String },
    /// Query across workspace contexts
    Query {
        workspace: String,
        query: String,
        #[arg(long, default_value_t = 5)]
        max_per_context: usize,
    },
    /// Compare an item across workspace contexts
    Compare {
        workspace: String,
        item: String,
        #[arg(long, default_value_t = 5)]
        max_per_context: usize,
    },
    /// Cross-reference item presence across contexts
    Xref { workspace: String, item: String },
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let acon_path = cli.path.unwrap_or_else(default_acon_path);

    match cli.command {
        Commands::Init { path } => {
            let target = path.unwrap_or(acon_path);
            let p = PathBuf::from(&target);
            if let Some(parent) = p.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    fail(&format!("Failed to create parent directory: {}", e));
                }
            }
            if p.exists() {
                println!("Already initialized: {}", target);
                return;
            }
            let mut engine = agentic_contract::ContractEngine::new();
            engine.file.path = Some(p);
            if let Err(e) = engine.save() {
                fail(&format!("Init failed: {}", e));
            }
            println!("Initialized: {}", target);
        }
        Commands::Query {
            entity_type,
            text,
            status,
            limit,
        } => {
            let engine = open_or_create(&acon_path);
            let rows = find_records(
                &engine,
                entity_type.as_deref(),
                text.as_deref(),
                status.as_deref(),
                limit,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "count": rows.len(),
                    "results": rows,
                }))
                .unwrap_or_default()
            );
        }
        Commands::Export {
            output,
            entity_type,
            pretty,
        } => {
            let engine = open_or_create(&acon_path);
            let rows = find_records(&engine, entity_type.as_deref(), None, None, usize::MAX);
            let payload = json!({
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "file": acon_path,
                "count": rows.len(),
                "entities": rows,
            });
            let encoded = if pretty {
                serde_json::to_string_pretty(&payload)
            } else {
                serde_json::to_string(&payload)
            }
            .unwrap_or_else(|_| "{}".to_string());

            if let Some(path) = output {
                if let Err(e) = std::fs::write(&path, encoded) {
                    fail(&format!("Failed to write export file: {}", e));
                }
                println!("Exported to {}", path);
            } else {
                println!("{}", encoded);
            }
        }
        Commands::Ground { claim, threshold } => {
            let engine = open_or_create(&acon_path);
            let evidence = ranked_matches(&engine, &claim, 5);
            let best_score = evidence.first().map(|r| r.score).unwrap_or(0.0);
            let status = if best_score >= threshold {
                "grounded"
            } else if !evidence.is_empty() {
                "partial"
            } else {
                "ungrounded"
            };

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "status": status,
                    "threshold": threshold,
                    "best_score": best_score,
                    "claim": claim,
                    "evidence": evidence,
                }))
                .unwrap_or_default()
            );
        }
        Commands::Evidence { query, limit } => {
            let engine = open_or_create(&acon_path);
            let evidence = ranked_matches(&engine, &query, limit);
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "query": query,
                    "count": evidence.len(),
                    "evidence": evidence,
                }))
                .unwrap_or_default()
            );
        }
        Commands::Suggest { query, limit } => {
            let engine = open_or_create(&acon_path);
            let suggestions = ranked_matches(&engine, &query, limit);
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "query": query,
                    "count": suggestions.len(),
                    "suggestions": suggestions,
                }))
                .unwrap_or_default()
            );
        }
        Commands::Workspace { subcommand } => handle_workspace(subcommand),
        Commands::RuntimeSync {
            workspace,
            max_depth,
        } => {
            let root = PathBuf::from(&workspace);
            if !root.exists() {
                fail(&format!("Workspace root does not exist: {}", workspace));
            }

            let mut files = Vec::new();
            scan_acon_files(&root, max_depth, &mut files);

            let mut total_entities = 0usize;
            let mut per_file = Vec::new();
            for path in &files {
                match agentic_contract::ContractEngine::open(path) {
                    Ok(engine) => {
                        let count = engine.file.total_entities();
                        total_entities += count;
                        per_file.push(json!({"path": path.to_string_lossy(), "entities": count}));
                    }
                    Err(e) => {
                        per_file
                            .push(json!({"path": path.to_string_lossy(), "error": e.to_string()}));
                    }
                }
            }

            let report = json!({
                "mode": "runtime-sync",
                "workspace": workspace,
                "scanned_files": files.len(),
                "total_entities": total_entities,
                "files": per_file,
                "synced_at": chrono::Utc::now().to_rfc3339(),
            });

            if let Err(e) = write_runtime_sync_snapshot(&report) {
                fail(&format!("Failed to write runtime-sync snapshot: {}", e));
            }

            println!(
                "{}",
                serde_json::to_string_pretty(&report).unwrap_or_default()
            );
        }
        Commands::Stats => {
            let engine = open_or_create(&acon_path);
            let stats = engine.stats();
            println!("{}", serde_json::to_string_pretty(&stats).unwrap());
        }
        Commands::Info { path } => {
            let p = path.as_deref().unwrap_or(&acon_path);
            match agentic_contract::ContractEngine::open(p) {
                Ok(engine) => {
                    println!("File: {}", p);
                    println!("Total entities: {}", engine.file.total_entities());
                    println!("Policies: {}", engine.file.policies.len());
                    println!("Risk limits: {}", engine.file.risk_limits.len());
                    println!("Obligations: {}", engine.file.obligations.len());
                    println!("Violations: {}", engine.file.violations.len());
                }
                Err(e) => fail(&format!("Error opening {}: {}", p, e)),
            }
        }
        Commands::Install { profile } => {
            println!("Installing AgenticContract for profile: {}", profile);
            println!("TODO: implement install command");
        }
        Commands::Serve => {
            println!("Starting MCP server... Use agentic-contract-mcp serve instead.");
        }
        Commands::Policy(args) => commands::policy::run(args, &acon_path),
        Commands::Limit(args) => commands::limit::run(args, &acon_path),
        Commands::Approval(args) => commands::approval::run(args, &acon_path),
        Commands::Obligation(args) => commands::obligation::run(args, &acon_path),
        Commands::Violation(args) => commands::violation::run(args, &acon_path),
    }
}

fn handle_workspace(subcommand: WorkspaceCommands) {
    let mut state = load_workspace_state().unwrap_or_default();

    match subcommand {
        WorkspaceCommands::Create { workspace } => {
            state.workspaces.entry(workspace.clone()).or_default();
            save_workspace_state(&state).unwrap_or_else(|e| fail(&e.to_string()));
            println!("Workspace created: {}", workspace);
        }
        WorkspaceCommands::Add {
            workspace,
            path,
            role,
            label,
        } => {
            let file_path = PathBuf::from(&path);
            if !file_path.exists() {
                fail(&format!("File does not exist: {}", path));
            }
            let contexts = state.workspaces.entry(workspace.clone()).or_default();
            let already = contexts.iter().any(|c| c.path == path);
            if !already {
                contexts.push(WorkspaceContext { path, role, label });
                save_workspace_state(&state).unwrap_or_else(|e| fail(&e.to_string()));
            }
            println!("Workspace '{}' updated", workspace);
        }
        WorkspaceCommands::List { workspace } => {
            let contexts = state
                .workspaces
                .get(&workspace)
                .cloned()
                .unwrap_or_default();
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "workspace": workspace,
                    "count": contexts.len(),
                    "contexts": contexts,
                }))
                .unwrap_or_default()
            );
        }
        WorkspaceCommands::Query {
            workspace,
            query,
            max_per_context,
        } => {
            let contexts = state
                .workspaces
                .get(&workspace)
                .cloned()
                .unwrap_or_default();
            let mut out = Vec::new();
            for ctx in contexts {
                let path = PathBuf::from(&ctx.path);
                let rows = agentic_contract::ContractEngine::open(&path)
                    .map(|engine| ranked_matches(&engine, &query, max_per_context))
                    .unwrap_or_default();
                out.push(json!({
                    "path": ctx.path,
                    "role": ctx.role,
                    "label": ctx.label,
                    "matches": rows,
                }));
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "workspace": workspace,
                    "query": query,
                    "results": out,
                }))
                .unwrap_or_default()
            );
        }
        WorkspaceCommands::Compare {
            workspace,
            item,
            max_per_context,
        } => {
            let contexts = state
                .workspaces
                .get(&workspace)
                .cloned()
                .unwrap_or_default();
            let mut out = Vec::new();
            for ctx in contexts {
                let matches = agentic_contract::ContractEngine::open(&ctx.path)
                    .map(|engine| ranked_matches(&engine, &item, max_per_context))
                    .unwrap_or_default();
                out.push(json!({
                    "path": ctx.path,
                    "match_count": matches.len(),
                    "top_matches": matches,
                }));
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "workspace": workspace,
                    "item": item,
                    "comparison": out,
                }))
                .unwrap_or_default()
            );
        }
        WorkspaceCommands::Xref { workspace, item } => {
            let contexts = state
                .workspaces
                .get(&workspace)
                .cloned()
                .unwrap_or_default();
            let mut present = Vec::new();
            let mut absent = Vec::new();
            for ctx in contexts {
                let has = agentic_contract::ContractEngine::open(&ctx.path)
                    .map(|engine| !ranked_matches(&engine, &item, 1).is_empty())
                    .unwrap_or(false);
                if has {
                    present.push(ctx.path);
                } else {
                    absent.push(ctx.path);
                }
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "workspace": workspace,
                    "item": item,
                    "present_in": present,
                    "absent_from": absent,
                }))
                .unwrap_or_default()
            );
        }
    }
}

fn default_acon_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.agentic/contract.acon", home)
}

fn fail(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn workspace_state_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".agentic")
        .join("contract")
        .join("workspaces.json")
}

fn load_workspace_state() -> Result<WorkspaceState, String> {
    let path = workspace_state_path();
    if !path.exists() {
        return Ok(WorkspaceState::default());
    }
    let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

fn save_workspace_state(state: &WorkspaceState) -> Result<(), String> {
    let path = workspace_state_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let raw = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    std::fs::write(path, raw).map_err(|e| e.to_string())
}

fn write_runtime_sync_snapshot(report: &serde_json::Value) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path = PathBuf::from(home)
        .join(".agentic")
        .join("contract")
        .join("runtime-sync.json");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let raw = serde_json::to_string_pretty(report).map_err(|e| e.to_string())?;
    std::fs::write(path, raw).map_err(|e| e.to_string())
}

fn scan_acon_files(root: &Path, max_depth: u32, out: &mut Vec<PathBuf>) {
    if max_depth == 0 {
        return;
    }
    let Ok(read_dir) = std::fs::read_dir(root) else {
        return;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_acon_files(&path, max_depth - 1, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("acon") {
            out.push(path);
        }
    }
}

fn normalize(s: &str) -> String {
    s.trim().to_ascii_lowercase()
}

fn token_score(haystack: &str, query: &str) -> f32 {
    let hs = normalize(haystack);
    let words: Vec<String> = query
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(normalize)
        .collect();
    if words.is_empty() {
        return 0.0;
    }
    let hits = words.iter().filter(|w| hs.contains(w.as_str())).count();
    hits as f32 / words.len() as f32
}

fn matches_type(record: &EntityRecord, entity_type: Option<&str>) -> bool {
    entity_type
        .map(normalize)
        .map(|t| record.entity_type == t)
        .unwrap_or(true)
}

fn matches_status(record: &EntityRecord, status: Option<&str>) -> bool {
    match (record.status.as_ref(), status) {
        (_, None) => true,
        (Some(rs), Some(expected)) => normalize(rs) == normalize(expected),
        (None, Some(_)) => false,
    }
}

fn find_records(
    engine: &agentic_contract::ContractEngine,
    entity_type: Option<&str>,
    text: Option<&str>,
    status: Option<&str>,
    limit: usize,
) -> Vec<EntityRecord> {
    let text_norm = text.map(normalize);
    let mut rows: Vec<EntityRecord> = collect_records(engine)
        .into_iter()
        .filter(|r| matches_type(r, entity_type))
        .filter(|r| matches_status(r, status))
        .filter(|r| {
            text_norm
                .as_ref()
                .map(|q| r.text.contains(q))
                .unwrap_or(true)
        })
        .collect();
    rows.truncate(limit);
    rows
}

fn ranked_matches(
    engine: &agentic_contract::ContractEngine,
    query: &str,
    limit: usize,
) -> Vec<QueryResult> {
    let mut scored: Vec<QueryResult> = collect_records(engine)
        .into_iter()
        .filter_map(|r| {
            let score = token_score(&r.text, query);
            if score > 0.0 {
                Some(QueryResult {
                    id: r.id,
                    entity_type: r.entity_type,
                    label: r.label,
                    status: r.status,
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    scored.truncate(limit);
    scored
}

fn collect_records(engine: &agentic_contract::ContractEngine) -> Vec<EntityRecord> {
    let mut out = Vec::new();

    for p in &engine.file.policies {
        let raw = serde_json::to_value(p).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: p.id.to_string(),
            entity_type: "policy".to_string(),
            label: p.label.clone(),
            status: Some(format!("{:?}", p.status).to_ascii_lowercase()),
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for l in &engine.file.risk_limits {
        let raw = serde_json::to_value(l).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: l.id.to_string(),
            entity_type: "risk_limit".to_string(),
            label: l.label.clone(),
            status: None,
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for r in &engine.file.approval_rules {
        let raw = serde_json::to_value(r).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: r.id.to_string(),
            entity_type: "approval_rule".to_string(),
            label: r.label.clone(),
            status: None,
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for r in &engine.file.approval_requests {
        let raw = serde_json::to_value(r).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: r.id.to_string(),
            entity_type: "approval_request".to_string(),
            label: r.action_description.clone(),
            status: Some(format!("{:?}", r.status).to_ascii_lowercase()),
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for d in &engine.file.approval_decisions {
        let raw = serde_json::to_value(d).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: d.id.to_string(),
            entity_type: "approval_decision".to_string(),
            label: d.reason.clone(),
            status: Some(format!("{:?}", d.decision).to_ascii_lowercase()),
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for c in &engine.file.conditions {
        let raw = serde_json::to_value(c).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: c.id.to_string(),
            entity_type: "condition".to_string(),
            label: c.label.clone(),
            status: Some(format!("{:?}", c.status).to_ascii_lowercase()),
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for o in &engine.file.obligations {
        let raw = serde_json::to_value(o).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: o.id.to_string(),
            entity_type: "obligation".to_string(),
            label: o.label.clone(),
            status: Some(format!("{:?}", o.status).to_ascii_lowercase()),
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    for v in &engine.file.violations {
        let raw = serde_json::to_value(v).unwrap_or_else(|_| json!({}));
        out.push(EntityRecord {
            id: v.id.to_string(),
            entity_type: "violation".to_string(),
            label: v.description.clone(),
            status: Some(format!("{:?}", v.severity).to_ascii_lowercase()),
            text: raw.to_string().to_ascii_lowercase(),
            raw,
        });
    }

    out
}

/// Open an existing .acon file or create a new one, always setting the file path.
pub fn open_or_create(path: &str) -> agentic_contract::ContractEngine {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::path::Path::new(path).exists() {
        agentic_contract::ContractEngine::open(path).expect("Failed to open .acon file")
    } else {
        let mut engine = agentic_contract::ContractEngine::new();
        engine.file.path = Some(std::path::PathBuf::from(path));
        engine
    }
}

/// Save the engine to disk. Called after mutating operations.
pub fn save_engine(engine: &agentic_contract::ContractEngine) {
    if let Err(e) = engine.file.save() {
        eprintln!("Error saving .acon file: {}", e);
    }
}

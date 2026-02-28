//! AgenticContract CLI — `acon` command-line interface.

mod commands;

use clap::{Parser, Subcommand};

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

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let acon_path = cli.path.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.agentic/contract.acon", home)
    });

    match cli.command {
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
                Err(e) => eprintln!("Error opening {}: {}", p, e),
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

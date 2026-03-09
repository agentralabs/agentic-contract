//! AgenticContract MCP Server — policy engine for AI agents.

mod ghost_bridge;
mod greeting;
mod invention_generation;
mod invention_governance;
mod invention_resilience;
mod invention_visibility;
mod prompts;
mod resources;
mod server;
mod stdio;
mod tools;

use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();

    // Extract --contract <path> flag and set ACON_PATH env var for server.rs.
    // The launcher script may inject `--contract <path>` before the subcommand.
    let mut command: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--contract" | "-c" => {
                // Next arg is the contract file path
                if let Some(path) = args.get(i + 1) {
                    std::env::set_var("ACON_PATH", path);
                }
                i += 2;
            }
            arg if arg.starts_with("--contract=") => {
                if let Some(path) = arg.strip_prefix("--contract=") {
                    std::env::set_var("ACON_PATH", path);
                }
                i += 1;
            }
            "serve" | "--stdio" | "info" => {
                command = Some(args[i].as_str());
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    match command.unwrap_or("serve") {
        "serve" | "--stdio" => {
            if let Err(e) = server::run_server() {
                tracing::error!("Server error: {}", e);
                std::process::exit(1);
            }
        }
        "info" => {
            println!("AgenticContract MCP Server v{}", env!("CARGO_PKG_VERSION"));
            println!("Tool count: {}", tools::TOOLS.len());
            println!("Resource count: {}", resources::RESOURCE_COUNT);
            println!("Prompt count: {}", prompts::PROMPT_COUNT);
        }
        _ => {
            eprintln!("Usage: agentic-contract-mcp [serve|info] [--contract <path>]");
            std::process::exit(1);
        }
    }
}

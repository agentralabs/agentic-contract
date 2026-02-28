//! Startup greeting for the MCP server.

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn print_greeting() {
    eprintln!("╔═══════════════════════════════════════════╗");
    eprintln!("║  AgenticContract MCP Server v{:<12}║", VERSION);
    eprintln!("║  Policy engine for AI agents              ║");
    eprintln!("║  📋 Policies · Limits · Approvals          ║");
    eprintln!("╚═══════════════════════════════════════════╝");
}

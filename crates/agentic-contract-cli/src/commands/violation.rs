//! Violation CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ViolationArgs {
    #[command(subcommand)]
    pub command: ViolationCommand,
}

#[derive(Subcommand)]
pub enum ViolationCommand {
    /// Report a violation
    Report {
        /// Violation description
        description: String,
        /// Severity: info, warning, critical, fatal
        #[arg(long, default_value = "warning")]
        severity: String,
        /// Actor who caused the violation
        #[arg(long)]
        actor: String,
    },
    /// List violations
    List {
        /// Filter by severity: info, warning, critical, fatal
        #[arg(long)]
        severity: Option<String>,
    },
}

pub fn run(args: ViolationArgs, acon_path: &str) {
    let mut engine = crate::open_or_create(acon_path);

    match args.command {
        ViolationCommand::Report {
            description,
            severity,
            actor,
        } => {
            let severity = match severity.as_str() {
                "info" => agentic_contract::ViolationSeverity::Info,
                "warning" => agentic_contract::ViolationSeverity::Warning,
                "critical" => agentic_contract::ViolationSeverity::Critical,
                "fatal" => agentic_contract::ViolationSeverity::Fatal,
                other => {
                    eprintln!("Unknown severity: {}", other);
                    return;
                }
            };
            let violation = agentic_contract::Violation::new(&description, severity, &actor);
            let id = engine.report_violation(violation);
            crate::save_engine(&engine);
            println!("Violation reported: {}", id);
        }
        ViolationCommand::List { severity } => {
            let severity = severity.as_deref().and_then(|s| match s {
                "info" => Some(agentic_contract::ViolationSeverity::Info),
                "warning" => Some(agentic_contract::ViolationSeverity::Warning),
                "critical" => Some(agentic_contract::ViolationSeverity::Critical),
                "fatal" => Some(agentic_contract::ViolationSeverity::Fatal),
                _ => None,
            });
            let violations = engine.list_violations(severity);
            for v in &violations {
                println!(
                    "  {} | {:?} | {} | {}",
                    v.id, v.severity, v.actor, v.description
                );
            }
            println!("Total: {}", violations.len());
        }
    }
}

//! Risk limit CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct LimitArgs {
    #[command(subcommand)]
    pub command: LimitCommand,
}

#[derive(Subcommand)]
pub enum LimitCommand {
    /// Set a risk limit
    Set {
        /// Limit label
        label: String,
        /// Maximum value
        #[arg(long)]
        max: f64,
        /// Limit type: rate, threshold, budget, count
        #[arg(long, default_value = "threshold")]
        r#type: String,
    },
    /// Check if an amount exceeds limits
    Check {
        /// Label pattern to match
        label: String,
        /// Amount to check
        #[arg(long)]
        amount: f64,
    },
    /// List all risk limits
    List,
}

pub fn run(args: LimitArgs, acon_path: &str) {
    let mut engine = crate::open_or_create(acon_path);

    match args.command {
        LimitCommand::Set { label, max, r#type } => {
            let limit_type = match r#type.as_str() {
                "rate" => agentic_contract::LimitType::Rate,
                "threshold" => agentic_contract::LimitType::Threshold,
                "budget" => agentic_contract::LimitType::Budget,
                "count" => agentic_contract::LimitType::Count,
                other => {
                    eprintln!("Unknown limit type: {}", other);
                    return;
                }
            };
            let limit = agentic_contract::RiskLimit::new(&label, limit_type, max);
            let id = engine.add_risk_limit(limit);
            crate::save_engine(&engine);
            println!("Created risk limit: {}", id);
        }
        LimitCommand::Check { label, amount } => match engine.check_risk_limit(&label, amount) {
            Some(limit) => {
                println!(
                    "EXCEEDED: {} (current: {}, max: {})",
                    limit.label, limit.current_value, limit.max_value
                );
            }
            None => {
                println!("OK: amount {} is within limits", amount);
            }
        },
        LimitCommand::List => {
            let limits = engine.list_risk_limits();
            for l in limits {
                println!(
                    "  {} | {:?} | {:.1}/{:.1} ({:.0}%)",
                    l.id,
                    l.limit_type,
                    l.current_value,
                    l.max_value,
                    l.usage_ratio() * 100.0
                );
            }
            println!("Total: {}", limits.len());
        }
    }
}

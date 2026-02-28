//! Obligation CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ObligationArgs {
    #[command(subcommand)]
    pub command: ObligationCommand,
}

#[derive(Subcommand)]
pub enum ObligationCommand {
    /// Add an obligation
    Add {
        /// Obligation label
        label: String,
        /// Description of what must be done
        #[arg(long)]
        description: String,
        /// Deadline (ISO 8601)
        #[arg(long)]
        deadline: Option<String>,
        /// Assignee
        #[arg(long, default_value = "agent")]
        assignee: String,
    },
    /// Check obligation status
    Check {
        /// Obligation ID (optional, checks all if omitted)
        id: Option<String>,
    },
    /// Fulfill an obligation
    Fulfill {
        /// Obligation ID
        id: String,
    },
    /// List obligations
    List {
        /// Filter by status: pending, fulfilled, overdue, waived
        #[arg(long)]
        status: Option<String>,
    },
}

pub fn run(args: ObligationArgs, acon_path: &str) {
    let mut engine = crate::open_or_create(acon_path);

    match args.command {
        ObligationCommand::Add {
            label,
            description,
            deadline,
            assignee,
        } => {
            let mut obligation = agentic_contract::Obligation::new(&label, &description, &assignee);
            if let Some(dl) = deadline {
                match chrono::DateTime::parse_from_rfc3339(&dl) {
                    Ok(dt) => {
                        obligation = obligation.with_deadline(dt.with_timezone(&chrono::Utc));
                    }
                    Err(e) => {
                        eprintln!("Invalid deadline: {}", e);
                        return;
                    }
                }
            }
            let id = engine.add_obligation(obligation);
            crate::save_engine(&engine);
            println!("Created obligation: {}", id);
        }
        ObligationCommand::Check { id } => {
            if let Some(id_str) = id {
                match id_str.parse::<agentic_contract::ContractId>() {
                    Ok(oid) => match engine.check_obligation(oid) {
                        Ok(status) => println!("Status: {:?}", status),
                        Err(e) => eprintln!("Error: {}", e),
                    },
                    Err(e) => eprintln!("Invalid ID: {}", e),
                }
            } else {
                let obligations =
                    engine.list_obligations(Some(agentic_contract::ObligationStatus::Pending));
                for o in &obligations {
                    println!(
                        "  {} | {:?} | {} | overdue: {}",
                        o.id,
                        o.status,
                        o.label,
                        o.is_overdue()
                    );
                }
                println!("Pending: {}", obligations.len());
            }
        }
        ObligationCommand::Fulfill { id } => match id.parse::<agentic_contract::ContractId>() {
            Ok(oid) => match engine.fulfill_obligation(oid) {
                Ok(_) => {
                    crate::save_engine(&engine);
                    println!("Obligation fulfilled");
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Invalid ID: {}", e),
        },
        ObligationCommand::List { status } => {
            let status = status.as_deref().and_then(|s| match s {
                "pending" => Some(agentic_contract::ObligationStatus::Pending),
                "fulfilled" => Some(agentic_contract::ObligationStatus::Fulfilled),
                "overdue" => Some(agentic_contract::ObligationStatus::Overdue),
                "waived" => Some(agentic_contract::ObligationStatus::Waived),
                _ => None,
            });
            let obligations = engine.list_obligations(status);
            for o in &obligations {
                println!("  {} | {:?} | {}", o.id, o.status, o.label);
            }
            println!("Total: {}", obligations.len());
        }
    }
}

//! Approval CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ApprovalArgs {
    #[command(subcommand)]
    pub command: ApprovalCommand,
}

#[derive(Subcommand)]
pub enum ApprovalCommand {
    /// Add an approval rule
    Rule {
        /// Rule label
        label: String,
        /// Action pattern that triggers this rule
        #[arg(long)]
        pattern: String,
    },
    /// Create an approval request
    Request {
        /// Rule ID
        #[arg(long)]
        rule_id: String,
        /// Action description
        description: String,
        /// Requestor identity
        #[arg(long)]
        requestor: String,
    },
    /// Decide on a request
    Decide {
        /// Request ID
        request_id: String,
        /// Decision: approve or deny
        #[arg(long)]
        decision: String,
        /// Decider identity
        #[arg(long)]
        decider: String,
        /// Reason for decision
        #[arg(long)]
        reason: String,
    },
    /// List approval requests
    List {
        /// Filter by status: pending, approved, denied, expired
        #[arg(long)]
        status: Option<String>,
    },
}

pub fn run(args: ApprovalArgs, acon_path: &str) {
    let mut engine = crate::open_or_create(acon_path);

    match args.command {
        ApprovalCommand::Rule { label, pattern } => {
            let rule = agentic_contract::ApprovalRule::new(&label, &pattern);
            let id = engine.add_approval_rule(rule);
            crate::save_engine(&engine);
            println!("Created approval rule: {}", id);
        }
        ApprovalCommand::Request {
            rule_id,
            description,
            requestor,
        } => match rule_id.parse::<agentic_contract::ContractId>() {
            Ok(rid) => match engine.request_approval(rid, &description, &requestor) {
                Ok(id) => {
                    crate::save_engine(&engine);
                    println!("Created approval request: {}", id);
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Invalid rule ID: {}", e),
        },
        ApprovalCommand::Decide {
            request_id,
            decision,
            decider,
            reason,
        } => {
            let decision_type = match decision.as_str() {
                "approve" => agentic_contract::DecisionType::Approve,
                "deny" => agentic_contract::DecisionType::Deny,
                other => {
                    eprintln!("Unknown decision: {} (use 'approve' or 'deny')", other);
                    return;
                }
            };
            match request_id.parse::<agentic_contract::ContractId>() {
                Ok(rid) => match engine.decide_approval(rid, decision_type, &decider, &reason) {
                    Ok(id) => {
                        crate::save_engine(&engine);
                        println!("Decision recorded: {}", id);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                },
                Err(e) => eprintln!("Invalid request ID: {}", e),
            }
        }
        ApprovalCommand::List { status } => {
            let status = status.as_deref().and_then(|s| match s {
                "pending" => Some(agentic_contract::ApprovalStatus::Pending),
                "approved" => Some(agentic_contract::ApprovalStatus::Approved),
                "denied" => Some(agentic_contract::ApprovalStatus::Denied),
                "expired" => Some(agentic_contract::ApprovalStatus::Expired),
                _ => None,
            });
            let requests = engine.list_approval_requests(status);
            for r in &requests {
                println!(
                    "  {} | {:?} | {} | {}",
                    r.id, r.status, r.requestor, r.action_description
                );
            }
            println!("Total: {}", requests.len());
        }
    }
}

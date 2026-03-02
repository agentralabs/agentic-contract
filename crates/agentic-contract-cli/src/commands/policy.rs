//! Policy CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct PolicyArgs {
    #[command(subcommand)]
    pub command: PolicyCommand,
}

#[derive(Subcommand)]
pub enum PolicyCommand {
    /// Add a new policy
    Add {
        /// Policy label
        label: String,
        /// Scope: global, session, agent
        #[arg(long, default_value = "global")]
        scope: String,
        /// Action: allow, deny, require_approval, audit_only
        #[arg(long, default_value = "deny")]
        action: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    /// Check if an action is allowed
    Check {
        /// Action type to check
        action_type: String,
        /// Scope: global, session, agent
        #[arg(long, default_value = "global")]
        scope: String,
    },
    /// List policies
    List {
        /// Filter by scope
        #[arg(long)]
        scope: Option<String>,
    },
}

pub fn run(args: PolicyArgs, acon_path: &str) {
    let mut engine = crate::open_or_create(acon_path);

    match args.command {
        PolicyCommand::Add {
            label,
            scope,
            action,
            description,
        } => {
            let scope = match scope.as_str() {
                "global" => agentic_contract::PolicyScope::Global,
                "session" => agentic_contract::PolicyScope::Session,
                "agent" => agentic_contract::PolicyScope::Agent,
                other => {
                    crate::fail(&format!("Unknown scope: {}", other));
                }
            };
            let action = match action.as_str() {
                "allow" => agentic_contract::PolicyAction::Allow,
                "deny" => agentic_contract::PolicyAction::Deny,
                "require_approval" => agentic_contract::PolicyAction::RequireApproval,
                "audit_only" => agentic_contract::PolicyAction::AuditOnly,
                other => {
                    crate::fail(&format!("Unknown action: {}", other));
                }
            };

            let mut policy = agentic_contract::Policy::new(&label, scope, action);
            if let Some(desc) = description {
                policy = policy.with_description(&desc);
            }
            let id = engine.add_policy(policy);
            crate::save_engine(&engine);
            println!("Created policy: {}", id);
        }
        PolicyCommand::Check { action_type, scope } => {
            let scope = match scope.as_str() {
                "global" => agentic_contract::PolicyScope::Global,
                "session" => agentic_contract::PolicyScope::Session,
                "agent" => agentic_contract::PolicyScope::Agent,
                other => {
                    crate::fail(&format!("Unknown scope: {}", other));
                }
            };
            let result = engine.check_policy(&action_type, scope);
            println!("Decision: {:?}", result);
        }
        PolicyCommand::List { scope } => {
            let scope = scope.as_deref().and_then(|s| match s {
                "global" => Some(agentic_contract::PolicyScope::Global),
                "session" => Some(agentic_contract::PolicyScope::Session),
                "agent" => Some(agentic_contract::PolicyScope::Agent),
                _ => None,
            });
            let policies = engine.list_policies(scope);
            for p in &policies {
                println!("  {} | {:?} | {:?} | {}", p.id, p.scope, p.action, p.label);
            }
            println!("Total: {}", policies.len());
        }
    }
}

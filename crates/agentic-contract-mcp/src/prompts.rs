//! MCP prompt definitions for AgenticContract.

use std::collections::HashMap;

/// Number of prompts.
pub const PROMPT_COUNT: usize = 4;

/// A prompt definition.
pub struct PromptDefinition {
    /// Prompt name.
    pub name: &'static str,
    /// Prompt description.
    pub description: &'static str,
    /// Prompt arguments.
    pub arguments: &'static [PromptArgument],
}

/// A prompt argument.
pub struct PromptArgument {
    /// Argument name.
    pub name: &'static str,
    /// Argument description.
    pub description: &'static str,
    /// Whether this argument is required.
    pub required: bool,
}

/// All AgenticContract MCP prompts.
pub const PROMPTS: &[PromptDefinition] = &[
    PromptDefinition {
        name: "contract_review",
        description: "Review current contract state: policies, limits, approvals",
        arguments: &[],
    },
    PromptDefinition {
        name: "contract_setup",
        description: "Set up a contract with policies and risk limits for an agent",
        arguments: &[
            PromptArgument {
                name: "agent_name",
                description: "Name of the agent to govern",
                required: true,
            },
            PromptArgument {
                name: "risk_level",
                description: "Risk tolerance: low, medium, high",
                required: false,
            },
        ],
    },
    PromptDefinition {
        name: "contract_audit",
        description: "Audit contract compliance: check violations and obligation status",
        arguments: &[PromptArgument {
            name: "severity",
            description: "Minimum severity to report: info, warning, critical, fatal",
            required: false,
        }],
    },
    PromptDefinition {
        name: "contract_risk_assessment",
        description: "Assess current risk exposure across all limits",
        arguments: &[PromptArgument {
            name: "action",
            description: "Specific action to assess risk for",
            required: false,
        }],
    },
];

/// Expand a prompt with arguments.
pub fn expand_prompt(name: &str, args: &HashMap<String, String>) -> Option<String> {
    match name {
        "contract_review" => Some(
            "Please review the current contract state:\n\n\
             1. List all active policies (use policy_list)\n\
             2. Check risk limits (use risk_limit_list)\n\
             3. Show pending approvals (use approval_list with status=pending)\n\
             4. Check pending obligations (use obligation_check)\n\
             5. List recent violations (use violation_list)\n\
             6. Show overall statistics (use contract_stats)\n\
             7. Highlight anything requiring immediate attention"
                .to_string(),
        ),
        "contract_setup" => {
            let agent = args.get("agent_name")?;
            let risk_text = args
                .get("risk_level")
                .map(|r| format!(" with {} risk tolerance", r))
                .unwrap_or_default();
            Some(format!(
                "Set up a governance contract for agent: {}{}\n\n\
                 Please:\n\
                 1. Create appropriate policies for the agent's scope\n\
                 2. Set risk limits based on the risk tolerance level\n\
                 3. Define approval rules for high-impact actions\n\
                 4. Add obligations the agent must fulfill\n\
                 5. Verify the contract is complete and consistent",
                agent, risk_text
            ))
        }
        "contract_audit" => {
            let severity_text = args
                .get("severity")
                .map(|s| format!(" at {} level or above", s))
                .unwrap_or_else(|| " at all severity levels".to_string());
            Some(format!(
                "Audit contract compliance{}:\n\n\
                 1. List all violations (use violation_list)\n\
                 2. Check for overdue obligations (use obligation_check)\n\
                 3. Review risk limit usage (use risk_limit_list)\n\
                 4. Verify all pending approvals are addressed (use approval_list)\n\
                 5. Summarize compliance status and recommend actions",
                severity_text
            ))
        }
        "contract_risk_assessment" => {
            let action_text = args
                .get("action")
                .map(|a| format!(" for action: {}", a))
                .unwrap_or_else(|| " across all domains".to_string());
            Some(format!(
                "Assess current risk exposure{}:\n\n\
                 1. Check all risk limits and their current usage (use risk_limit_list)\n\
                 2. Identify limits approaching threshold (use risk_limit_check)\n\
                 3. Review recent violations for risk patterns (use violation_list)\n\
                 4. Check policy constraints (use policy_check)\n\
                 5. Provide risk score and recommendations",
                action_text
            ))
        }
        _ => None,
    }
}

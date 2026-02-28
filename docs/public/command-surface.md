---
status: stable
---

# Command Surface

## MCP Tools (50)

| Tool | Parameters | Description |
|------|-----------|-------------|
| `contract_create` | `title`, `parties` | Create a new contract between agents or agent and user |
| `contract_sign` | `contract_id` | Sign a contract to indicate acceptance |
| `contract_verify` | `contract_id` | Verify a contract's validity and signature chain |
| `contract_list` | `status?` | List contracts with optional status filter |
| `contract_get` | `contract_id` | Get a specific contract by ID |
| `policy_add` | `label`, `scope?`, `action?`, `tags?` | Add a policy rule governing agent behavior |
| `policy_check` | `action` | Check if an action is allowed under current policies |
| `policy_list` | `scope?` | List active policies with optional scope filter |
| `risk_limit_set` | `label`, `max_value`, `limit_type?` | Set a risk limit threshold for a resource or action |
| `risk_limit_check` | `label`, `proposed_usage` | Check if an action would exceed risk limits |
| `risk_limit_list` | — | List all risk limits with current values |
| `approval_request` | `rule_id`, `action_description`, `requestor` | Request approval for a controlled action |
| `approval_decide` | `request_id`, `approved`, `reason?` | Approve or deny a pending approval request |
| `approval_list` | `status?` | List approval requests with optional status filter |
| `condition_add` | `label`, `condition_type`, `expression` | Add a conditional execution rule |
| `condition_evaluate` | `condition_id` | Evaluate whether conditions are met for an action |
| `obligation_add` | `label`, `description?`, `deadline?`, `assignee?` | Add an obligation that an agent must fulfill |
| `obligation_check` | `obligation_id?` | Check the status of obligations |
| `violation_list` | `severity?` | List recorded violations with optional severity filter |
| `violation_report` | `description`, `severity?`, `agent_id?` | Report a contract or policy violation |
| `contract_context_log` | `intent`, `topic?` | Log the intent and context behind a contract action |
| `contract_stats` | — | Get summary statistics for the contract store |
| `contract_ground` | `claim`, `threshold?` | Verify a claim has contract backing |
| `contract_evidence` | `query`, `max_results?` | Get detailed evidence for a contract query |
| `contract_suggest` | `query`, `limit?` | Suggest similar contract records |
| `contract_workspace_create` | `workspace` | Create a multi-contract workspace |
| `contract_workspace_add` | `workspace`, `path`, `role?`, `label?` | Add an `.acon` context file to workspace |
| `contract_workspace_list` | `workspace` | List contexts in a workspace |
| `contract_workspace_query` | `workspace`, `query`, `max_per_context?` | Query across workspace contexts |
| `contract_workspace_compare` | `workspace`, `item`, `max_per_context?` | Compare an item across contexts |
| `contract_workspace_xref` | `workspace`, `item` | Cross-reference item presence across contexts |
| `session_start` | `session_id?`, `metadata?` | Start a contract session |
| `session_end` | `session_id?` | End a contract session |
| `contract_session_resume` | `limit?` | Resume recent session context |

### Advanced Tools (16)

| Tool | Description |
|------|-------------|
| `policy_omniscience` | Get complete visibility into all applicable policies for an agent |
| `risk_prophecy` | Predict future risk budget usage and identify limits at risk |
| `approval_telepathy` | Predict approval likelihood before submitting a request |
| `obligation_clairvoyance` | Forecast upcoming obligations and identify scheduling conflicts |
| `violation_precognition` | Detect potential violations before they occur |
| `contract_crystallize` | Generate contract policies from high-level intent description |
| `policy_dna_extract` | Extract the genetic representation of a policy for evolution |
| `trust_gradient_evaluate` | Evaluate an action with trust-weighted policy assessment |
| `collective_contract_create` | Create a multi-party collective governance contract |
| `temporal_contract_create` | Create a time-evolving contract with governance transitions |
| `contract_inheritance_create` | Create a hierarchical parent-child contract relationship |
| `smart_escalation_route` | Route an approval request to the optimal approver |
| `violation_archaeology_analyze` | Analyze violation patterns to identify root causes |
| `contract_simulation_run` | Simulate contract behavior across multiple scenarios |
| `federated_governance_create` | Create cross-organizational federated governance |
| `self_healing_contract_create` | Create a contract that automatically adapts to violations |

### Compact Facade Tools (v0.3+)

Use these to keep MCP tool surfaces small while preserving backward compatibility:

| Tool | Description |
|------|-------------|
| `contract_main` | Unified core contract operations via `operation` |
| `contract_policy` | Unified policy operations via `operation` |
| `contract_risk` | Unified risk-limit operations via `operation` |
| `contract_approval` | Unified approval operations via `operation` |
| `contract_enforcement` | Unified condition/obligation/violation operations via `operation` |
| `contract_workspace` | Unified workspace/session operations via `operation` |
| `contract_visibility` | Unified visibility invention operations via `operation` |
| `contract_generation` | Unified generation invention operations via `operation` |
| `contract_governance` | Unified governance invention operations via `operation` |
| `contract_resilience` | Unified resilience invention operations via `operation` |

Compact list mode:

```bash
export ACON_MCP_TOOL_SURFACE=compact
```

In compact mode, `tools/list` returns only the 10 facade tools above, while all legacy tool names remain callable.

## CLI Commands

| Command | Description |
|---------|-------------|
| `acon init` | Initialize a new `.acon` file |
| `acon query` | Query contract records by type/text/status |
| `acon export` | Export contract records as JSON |
| `acon ground` | Verify claim grounding in contract state |
| `acon evidence` | Return supporting evidence rows |
| `acon suggest` | Return similar/suggested rows |
| `acon workspace create` | Create workspace label |
| `acon workspace add` | Add `.acon` file to workspace |
| `acon workspace list` | List workspace contexts |
| `acon workspace query` | Query across workspace contexts |
| `acon workspace compare` | Compare item across contexts |
| `acon workspace xref` | Cross-reference item presence |
| `acon runtime-sync` | Scan `.acon` files and emit sync snapshot |
| `acon policy add` | Add a policy |
| `acon policy check` | Check action against policies |
| `acon policy list` | List policies |
| `acon limit set` | Set a risk limit |
| `acon limit check` | Check risk limit |
| `acon limit list` | List risk limits |
| `acon approval rule` | Add approval rule |
| `acon approval request` | Request approval |
| `acon approval decide` | Decide on approval |
| `acon approval list` | List approvals |
| `acon obligation add` | Add obligation |
| `acon obligation check` | Check obligations |
| `acon obligation fulfill` | Fulfill obligation |
| `acon obligation list` | List obligations |
| `acon violation report` | Report violation |
| `acon violation list` | List violations |
| `acon stats` | Show statistics |
| `acon info` | Show build info |
| `acon install` | Configure MCP |
| `acon serve` | Start MCP server |

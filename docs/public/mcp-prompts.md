---
status: stable
---

# MCP Prompts

AgenticContract provides 4 prompts via the MCP protocol for guided contract operations.

## Available Prompts

### `contract_review`

Review an existing contract for completeness and potential issues.

**Arguments:**
| Name | Required | Description |
|------|----------|-------------|
| `contract_id` | Yes | ID of the contract to review |

**Behavior:** Analyzes the contract's policies, risk limits, approvals, and obligations. Identifies gaps in coverage and suggests improvements.

### `contract_setup`

Guide through setting up a new contract with policies and limits.

**Arguments:**
| Name | Required | Description |
|------|----------|-------------|
| `context` | No | Description of the agent's intended use case |

**Behavior:** Walks through creating a comprehensive governance contract including policies, risk limits, approval rules, and obligations appropriate for the described context.

### `contract_audit`

Perform an audit of contract compliance and violations.

**Arguments:**
| Name | Required | Description |
|------|----------|-------------|
| `severity` | No | Minimum severity to include: `info`, `warning`, `critical`, `fatal` |

**Behavior:** Reviews all recorded violations, checks obligation status, and evaluates risk limit usage. Produces a compliance summary.

### `contract_risk_assessment`

Assess the risk profile of the current contract configuration.

**Arguments:**
| Name | Required | Description |
|------|----------|-------------|
| `scope` | No | Assessment scope: `global`, `session`, `agent` |

**Behavior:** Evaluates risk limits, identifies uncovered risk categories, and assesses the strength of policy coverage across the specified scope.

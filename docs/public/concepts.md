---
status: stable
title: Concepts
description: Core concepts and domain model of AgenticContract
---

# Concepts

AgenticContract provides six governance primitives that work together to constrain, monitor, and audit AI agent behavior. Each primitive is a first-class entity stored in the `.acon` binary file.

## Why Governance Primitives?

Traditional access control is binary: allow or deny. AI agents need richer governance:

- **Graduated responses**: Some actions should be allowed, some denied, some gated behind approval, some merely audited
- **Quantitative limits**: "Don't spend more than $100" is not a boolean rule — it requires tracking state
- **Temporal obligations**: "Submit a report by Friday" requires deadline tracking and overdue detection
- **Audit requirements**: Even allowed actions may need to be recorded for compliance review

AgenticContract models governance as a graph of interconnected primitives, not a flat list of permissions.

## Policy

A policy is a named rule governing agent behavior. Policies are the primary enforcement mechanism.

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier (UUID) |
| `label` | String | Human-readable rule description |
| `description` | String | Optional detailed explanation |
| `scope` | PolicyScope | Where this policy applies |
| `action` | PolicyAction | What happens when the policy matches |
| `conditions` | Vec | Optional prerequisite conditions |
| `status` | PolicyStatus | Active, Disabled, or Expired |
| `tags` | Vec | Categorization labels |
| `created_at` | DateTime | When the policy was created |
| `updated_at` | DateTime | Last modification time |
| `expires_at` | Option | Optional expiration time |

### Scope

Policies apply at one of three scopes:

- **Global**: Applies to all agents and sessions. Use for organization-wide rules.
- **Session**: Applies within the current session only. Use for task-specific constraints.
- **Agent**: Applies to a specific agent. Use for per-agent capability boundaries.

### Action

When a policy matches an action, it produces one of four outcomes:

- **Allow**: The action may proceed. This is the default when no policy matches.
- **Deny**: The action is blocked. A violation is typically recorded.
- **RequireApproval**: The action is gated behind an approval workflow.
- **AuditOnly**: The action proceeds but is logged for review.

### Precedence

When multiple policies match, the most restrictive action wins:

```
Deny > RequireApproval > AuditOnly > Allow
```

### Example

```python
engine.add_policy(
    "No production deploys without approval",
    scope="global",
    action="require_approval"
)

result = engine.check_policy("deploy to production")
# Returns: RequireApproval
```

## Risk Limit

A risk limit sets quantitative boundaries on agent operations. Unlike policies (which are boolean), risk limits track continuous values against thresholds.

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier |
| `label` | String | Human-readable limit name |
| `limit_type` | LimitType | Rate, Threshold, Budget, or Count |
| `current_value` | f64 | Current usage level |
| `max_value` | f64 | Maximum allowed value |
| `window_secs` | Option | Optional time window for rate limits |
| `window_start` | Option | Start of current window |

### Limit Types

- **Rate**: Operations per time window (e.g., 100 API calls per hour)
- **Threshold**: Maximum value that must not be exceeded (e.g., error rate below 5%)
- **Budget**: Cumulative spending cap (e.g., $100 total API spend)
- **Count**: Simple counter with a maximum (e.g., no more than 10 retries)

### Usage Tracking

```python
# Set a budget limit
engine.set_risk_limit("API spend", max=100.0, type="budget")

# Before each API call, check and increment
limit = engine.check_risk_limit("API spend", amount=0.05)
if limit.would_exceed:
    engine.report_violation("Budget exceeded", severity="critical")
else:
    engine.increment_risk_limit(limit.id, 0.05)
```

### Window Expiration

Rate limits with a `window_secs` value automatically expire. When `window_expired()` returns true, the current value resets to zero at the next check.

## Approval

The approval system gates high-stakes actions behind human or automated review. It has three components.

### Approval Rule

Defines when approval is required.

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier |
| `label` | String | Rule description |
| `action_pattern` | String | Pattern that triggers this rule |
| `approvers` | Vec | Authorized approver identifiers |
| `timeout_secs` | Option | Auto-expire after this duration |

### Approval Request

A pending approval submitted by an agent.

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier |
| `rule_id` | ContractId | Which rule triggered this request |
| `action_description` | String | What the agent wants to do |
| `requestor` | String | Who submitted the request |
| `status` | ApprovalStatus | Pending, Approved, Denied, or Expired |

### Approval Decision

The outcome of an approval request.

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier |
| `request_id` | ContractId | Which request this decides |
| `decision` | DecisionType | Approve or Deny |
| `decider` | String | Who made the decision |
| `reason` | String | Justification for the decision |

### Workflow

```
Agent action triggers policy -> RequireApproval
  |
approval_request created (status: Pending)
  |
Human or automated approver reviews
  |
approval_decide: Approve or Deny
  |
If Approved -> action proceeds
If Denied -> violation recorded, action blocked
```

## Condition

A condition is a prerequisite that must be satisfied before an action can proceed. Conditions decouple "when can this happen?" from "is this allowed?"

### Condition Types

- **Threshold**: A numeric value must be above or below a target
- **TimeBased**: An action can only happen during certain time windows
- **Dependency**: Another action or state must exist first
- **Custom**: Arbitrary expression evaluated at runtime

### Status Lifecycle

```
Unevaluated -> Met      (condition satisfied)
            -> NotMet   (condition not satisfied)
            -> Unknown  (evaluation failed or inconclusive)
```

Conditions are re-evaluated on demand via `condition_evaluate`. They do not auto-update.

### Example

```python
engine.add_condition(
    "Business hours only",
    type="time_based",
    expression="09:00-17:00 UTC weekdays"
)

result = engine.evaluate_condition(condition_id)
if not result.is_met:
    print("Action blocked: outside business hours")
```

## Obligation

An obligation represents a commitment that an agent must fulfill, optionally with a deadline.

### Status Lifecycle

```
Pending -> Fulfilled  (completed on time)
        -> Overdue    (deadline passed, not fulfilled)
        -> Waived     (explicitly excused)
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier |
| `label` | String | Short obligation name |
| `description` | String | Detailed description |
| `assignee` | String | Responsible agent or user |
| `deadline` | Option | When the obligation is due |
| `status` | ObligationStatus | Current state |

### Overdue Detection

`is_overdue()` returns true when the deadline has passed and the status is still `Pending`. The engine does not automatically transition to `Overdue` — call `obligation_check` to trigger detection.

### Example

```python
engine.add_obligation(
    "Submit weekly compliance report",
    description="Generate and submit the weekly compliance summary",
    assignee="compliance-agent",
    deadline="2026-03-07T17:00:00Z"
)

# Periodic check
overdue = engine.check_obligations()
for ob in overdue:
    engine.report_violation(
        f"Missed obligation: {ob.label}",
        severity="warning"
    )
```

## Violation

A violation records a breach of a policy or contract term. Violations are the audit trail.

### Severity Levels

| Level | When to Use |
|-------|-------------|
| **Info** | Minor deviation, informational logging only |
| **Warning** | Significant deviation requiring attention |
| **Critical** | Serious breach requiring immediate response |
| **Fatal** | System integrity compromised, halt operations |

Severity implements `Ord`, so violations can be sorted and filtered by severity.

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | ContractId | Unique identifier |
| `policy_id` | Option | Which policy was violated (if applicable) |
| `description` | String | Human-readable breach description |
| `severity` | ViolationSeverity | Info, Warning, Critical, or Fatal |
| `actor` | String | Who committed the violation |
| `detected_at` | DateTime | When the breach was detected |
| `context` | Value | Arbitrary JSON context |

### Example

```python
engine.report_violation(
    "Agent exceeded API budget by $12.50",
    severity="critical",
    actor="data-agent-3"
)
```

## Contract Engine

The `ContractEngine` is the central runtime that ties all six primitives together. It wraps the `.acon` file format and provides:

- CRUD operations for all entity types
- Policy evaluation with text-matching and precedence
- Risk limit tracking with window expiration
- Approval workflow management
- Obligation deadline monitoring
- Violation recording and querying
- Statistics and summary reporting

The engine is the only entry point for modifying the contract store. All MCP tools, CLI commands, FFI functions, and Python SDK methods delegate to it.

## Entity Relationships

```
Policies -------> RequireApproval --> Approval Rules
    |                                    |
    |                                    v
    |                              Approval Requests
    |                                    |
    |                                    v
    |                              Approval Decisions
    |
    +-------> Conditions (prerequisites)
    |
    +-------> Violations (breaches)

Risk Limits --> Violations (on exceed)

Obligations --> Violations (on overdue)
```

All entities are identified by `ContractId` (UUID-based) and timestamped with `created_at`. This enables temporal queries and audit trails across the full governance lifecycle.

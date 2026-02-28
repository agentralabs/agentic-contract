---
status: stable
title: API Reference
description: Complete API reference for AgenticContract
---

# API Reference

Complete reference for the AgenticContract Rust and Python APIs. Version 0.1.

## ContractEngine

The main entry point for the policy engine. All operations go through this struct.

### Constructor

```rust
let engine = ContractEngine::new();
```

Creates an empty contract engine with no entities.

### `open(path: &str) -> ContractResult<ContractEngine>`

Opens an existing `.acon` file from disk. Returns an error if the file doesn't exist or has an invalid format.

```rust
let engine = ContractEngine::open("~/.agentic/contract.acon")?;
```

### `save() -> ContractResult<()>`

Writes the current state to the file path set during `open()`. Computes BLAKE3 checksum and updates the header.

```rust
engine.save()?;
```

### Policy Methods

#### `add_policy(policy: Policy) -> ContractId`

Adds a policy to the engine. Returns the policy's unique ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `policy` | `Policy` | The policy to add |

```rust
use agentic_contract::{Policy, PolicyScope, PolicyAction};

let policy = Policy::new("No deploys on Friday", PolicyScope::Global, PolicyAction::Deny);
let id = engine.add_policy(policy);
```

#### `check_policy(action_type: &str, scope: Option<PolicyScope>) -> PolicyAction`

Evaluates all active policies against the given action description. Returns the most restrictive matching action.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action_type` | `&str` | Yes | The action being attempted |
| `scope` | `Option<PolicyScope>` | No | Filter to specific scope |

```rust
let result = engine.check_policy("deploy to production", None);
match result {
    PolicyAction::Deny => println!("Blocked"),
    PolicyAction::RequireApproval => println!("Needs approval"),
    _ => println!("Allowed"),
}
```

#### `list_policies(scope: Option<PolicyScope>) -> Vec<&Policy>`

Returns all policies, optionally filtered by scope.

#### `get_policy(id: ContractId) -> ContractResult<&Policy>`

Returns a specific policy by ID. Returns `ContractError::NotFound` if the ID doesn't exist.

### Risk Limit Methods

#### `add_risk_limit(limit: RiskLimit) -> ContractId`

Adds a risk limit to the engine.

```rust
use agentic_contract::{RiskLimit, LimitType};

let limit = RiskLimit::new("API calls per hour", LimitType::Rate, 100.0)
    .with_window(3600);
let id = engine.add_risk_limit(limit);
```

#### `check_risk_limit(label_pattern: &str, amount: f64) -> Option<&RiskLimit>`

Checks whether adding `amount` to the matching risk limit would exceed the maximum. Returns the first limit that would be exceeded, or `None` if all limits are within bounds.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label_pattern` | `&str` | Yes | Text pattern to match limit labels |
| `amount` | `f64` | Yes | Proposed increment amount |

#### `increment_risk_limit(id: ContractId, amount: f64) -> ContractResult<()>`

Increments a risk limit's current value. Call this after the guarded action succeeds.

#### `list_risk_limits() -> &[RiskLimit]`

Returns all risk limits.

### Approval Methods

#### `add_approval_rule(rule: ApprovalRule) -> ContractId`

Adds an approval rule.

```rust
use agentic_contract::ApprovalRule;

let rule = ApprovalRule::new("Production deploy approval", "deploy.*production")
    .with_approver("ops-lead")
    .with_timeout(3600);
let id = engine.add_approval_rule(rule);
```

#### `request_approval(rule_id: ContractId, action_description: &str, requestor: &str) -> ContractResult<ContractId>`

Creates a pending approval request linked to a rule.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `rule_id` | `ContractId` | Yes | The rule triggering this request |
| `action_description` | `&str` | Yes | What the agent wants to do |
| `requestor` | `&str` | Yes | Who is requesting |

#### `decide_approval(request_id: ContractId, decision: DecisionType, decider: &str, reason: &str) -> ContractResult<ContractId>`

Approves or denies a pending request. Returns the decision ID.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `request_id` | `ContractId` | Yes | Which request to decide |
| `decision` | `DecisionType` | Yes | `Approve` or `Deny` |
| `decider` | `&str` | Yes | Who is deciding |
| `reason` | `&str` | Yes | Justification |

#### `list_approval_requests(status: Option<ApprovalStatus>) -> Vec<&ApprovalRequest>`

Returns approval requests, optionally filtered by status.

### Condition Methods

#### `add_condition(condition: Condition) -> ContractId`

Adds a conditional execution rule.

```rust
use agentic_contract::{Condition, ConditionType};

let cond = Condition::new("Business hours", ConditionType::TimeBased, "09:00-17:00 UTC");
let id = engine.add_condition(cond);
```

#### `evaluate_condition(id: ContractId) -> ContractResult<bool>`

Evaluates a condition and updates its status. Returns whether the condition is met.

### Obligation Methods

#### `add_obligation(obligation: Obligation) -> ContractId`

Adds an obligation with optional deadline.

```rust
use agentic_contract::Obligation;

let ob = Obligation::new(
    "Weekly report",
    "Submit compliance summary",
    "compliance-agent"
).with_deadline(deadline_time);
let id = engine.add_obligation(ob);
```

#### `check_obligations() -> Vec<&Obligation>`

Returns obligations that are overdue (deadline passed, status still Pending).

#### `fulfill_obligation(id: ContractId) -> ContractResult<()>`

Marks an obligation as fulfilled.

#### `waive_obligation(id: ContractId) -> ContractResult<()>`

Marks an obligation as waived (explicitly excused).

### Violation Methods

#### `report_violation(description: &str, severity: ViolationSeverity, actor: &str) -> ContractId`

Records a violation.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `description` | `&str` | Yes | What happened |
| `severity` | `ViolationSeverity` | Yes | Info, Warning, Critical, or Fatal |
| `actor` | `&str` | Yes | Who committed the violation |

```rust
use agentic_contract::ViolationSeverity;

let id = engine.report_violation(
    "Budget exceeded by $12.50",
    ViolationSeverity::Critical,
    "data-agent-3"
);
```

#### `list_violations(severity: Option<ViolationSeverity>) -> Vec<&Violation>`

Returns violations, optionally filtered by minimum severity.

### Statistics

#### `stats() -> ContractStats`

Returns summary statistics for all entity types.

```rust
let s = engine.stats();
println!("Policies: {}, Limits: {}, Violations: {}", s.policies, s.risk_limits, s.violations);
```

## Core Types

### Policy

```rust
pub struct Policy {
    pub id: ContractId,
    pub label: String,
    pub description: String,
    pub scope: PolicyScope,
    pub action: PolicyAction,
    pub conditions: Vec<String>,
    pub status: PolicyStatus,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
```

Builder methods: `with_description()`, `with_condition()`, `with_tag()`, `expires_at()`.

### RiskLimit

```rust
pub struct RiskLimit {
    pub id: ContractId,
    pub label: String,
    pub limit_type: LimitType,
    pub current_value: f64,
    pub max_value: f64,
    pub window_secs: Option<u64>,
    pub window_start: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

Utility methods: `would_exceed(amount)`, `remaining()`, `usage_ratio()`, `window_expired()`, `reset()`.

### ApprovalRule

```rust
pub struct ApprovalRule {
    pub id: ContractId,
    pub label: String,
    pub action_pattern: String,
    pub approvers: Vec<String>,
    pub timeout_secs: Option<u64>,
    pub created_at: DateTime<Utc>,
}
```

### ApprovalRequest

```rust
pub struct ApprovalRequest {
    pub id: ContractId,
    pub rule_id: ContractId,
    pub action_description: String,
    pub requestor: String,
    pub status: ApprovalStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
```

### ApprovalDecision

```rust
pub struct ApprovalDecision {
    pub id: ContractId,
    pub request_id: ContractId,
    pub decision: DecisionType,
    pub decider: String,
    pub reason: String,
    pub decided_at: DateTime<Utc>,
}
```

### Condition

```rust
pub struct Condition {
    pub id: ContractId,
    pub label: String,
    pub condition_type: ConditionType,
    pub expression: String,
    pub status: ConditionStatus,
    pub last_result: Option<String>,
    pub created_at: DateTime<Utc>,
    pub evaluated_at: Option<DateTime<Utc>>,
}
```

### Obligation

```rust
pub struct Obligation {
    pub id: ContractId,
    pub label: String,
    pub description: String,
    pub assignee: String,
    pub deadline: Option<DateTime<Utc>>,
    pub status: ObligationStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}
```

### Violation

```rust
pub struct Violation {
    pub id: ContractId,
    pub policy_id: Option<ContractId>,
    pub description: String,
    pub severity: ViolationSeverity,
    pub actor: String,
    pub detected_at: DateTime<Utc>,
    pub context: serde_json::Value,
}
```

## Enums

### PolicyScope

```rust
pub enum PolicyScope { Global, Session, Agent }
```

### PolicyAction

```rust
pub enum PolicyAction { Allow, Deny, RequireApproval, AuditOnly }
```

### PolicyStatus

```rust
pub enum PolicyStatus { Active, Disabled, Expired }
```

### LimitType

```rust
pub enum LimitType { Rate, Threshold, Budget, Count }
```

### ApprovalStatus

```rust
pub enum ApprovalStatus { Pending, Approved, Denied, Expired }
```

### DecisionType

```rust
pub enum DecisionType { Approve, Deny }
```

### ConditionType

```rust
pub enum ConditionType { Threshold, TimeBased, Dependency, Custom }
```

### ConditionStatus

```rust
pub enum ConditionStatus { Unevaluated, Met, NotMet, Unknown }
```

### ObligationStatus

```rust
pub enum ObligationStatus { Pending, Fulfilled, Overdue, Waived }
```

### ViolationSeverity

```rust
pub enum ViolationSeverity { Info, Warning, Critical, Fatal }
```

Implements `Ord` for severity comparison. `Fatal > Critical > Warning > Info`.

## Error Types

```rust
pub enum ContractError {
    NotFound(String),
    PolicyViolation(String),
    RiskLimitExceeded(String),
    ApprovalRequired(String),
    ApprovalDenied(String),
    ConditionNotMet(String),
    ObligationUnfulfilled(String),
    ContractExpired(String),
    InvalidContract(String),
    FileFormat(String),
    Io(std::io::Error),
    Serialization(String),
}
```

All variants carry a human-readable message. `Io` wraps `std::io::Error` for file operations. `Serialization` wraps serde errors.

### ContractResult

```rust
pub type ContractResult<T> = Result<T, ContractError>;
```

## Python SDK

### ContractEngine (Python)

```python
from agentic_contract import ContractEngine

engine = ContractEngine()               # New empty engine
engine = ContractEngine(path="my.acon") # Open existing file
```

### Methods

| Method | Parameters | Returns | Description |
|--------|-----------|---------|-------------|
| `add_policy(label, scope, action)` | str, str, str | str (id) | Add a policy |
| `check_policy(action)` | str | str | Check action against policies |
| `list_policies()` | — | list | List all policies |
| `set_risk_limit(label, max, type)` | str, float, str | str (id) | Set a risk limit |
| `check_risk_limit(label, amount)` | str, float | dict | Check risk limit |
| `request_approval(rule_id, desc, who)` | str, str, str | str (id) | Request approval |
| `decide_approval(req_id, decision, who, reason)` | str, str, str, str | str (id) | Decide approval |
| `add_obligation(label, desc, assignee)` | str, str, str | str (id) | Add obligation |
| `report_violation(desc, severity, actor)` | str, str, str | str (id) | Report violation |
| `stats()` | — | dict | Summary statistics |

The Python SDK wraps the `acon` CLI binary via subprocess. It requires the CLI to be installed on the system.

## FFI Reference

See the [FFI Reference](ffi-reference.md) for C/Python interop via the shared library.

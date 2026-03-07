# AGENTIC-CONTRACT SPECIFICATIONS

> Complete engineering specifications for AgenticContract v0.2.0
> The governance layer for AI agents: policies, boundaries, and accountability.

---

# TABLE OF CONTENTS

1. [SPEC-01: Overview](#spec-01-overview)
2. [SPEC-02: Core Concepts](#spec-02-core-concepts)
3. [SPEC-03: Data Structures](#spec-03-data-structures)
4. [SPEC-04: File Format](#spec-04-file-format)
5. [SPEC-05: Policy Engine](#spec-05-policy-engine)
6. [SPEC-06: Query Engine](#spec-06-query-engine)
7. [SPEC-07: Index Structures](#spec-07-index-structures)
8. [SPEC-08: Validation Rules](#spec-08-validation-rules)
9. [SPEC-09: CLI Reference](#spec-09-cli-reference)
10. [SPEC-10: MCP Server](#spec-10-mcp-server)

---

# SPEC-01: OVERVIEW

> What is AgenticContract and what does it need to become?

## What Is AgenticContract?

**AgenticContract** is the 6th sister in the Agentra ecosystem. It provides:

| Primitive | Purpose |
|-----------|---------|
| **Policies** | What an agent CAN and CANNOT do |
| **Risk Limits** | Thresholds that trigger escalation |
| **Approvals** | Who must authorize actions |
| **Conditions** | "If X then allow Y" rules |
| **Obligations** | What an agent MUST do |
| **Violations** | What happens when rules are broken |

**The Problem It Solves:**

AI agents today operate without formal boundaries. They either:
- Do everything (dangerous)
- Do nothing without permission (useless)

AgenticContract provides the middle ground: **governed autonomy**.

---

## What The Scaffold Provides

| Component | Status | Details |
|-----------|--------|---------|
| Core Engine | ✅ Built | 6 governance primitives, basic CRUD |
| .acon Format | ✅ Built | Binary format with BLAKE3 checksums |
| MCP Server | ✅ Built | 22 tools over JSON-RPC stdio |
| CLI (acon) | ✅ Built | Basic commands |
| SDK Traits | ✅ Built | All 8 traits implemented |
| FFI/Python | ✅ Built | C header, Python SDK, WASM |
| Tests | ✅ Built | 73 tests passing |

---

## What's Missing (The Real Work)

### Gap 1: Deep Policy Semantics

**Current:** Policies are simple allow/deny rules.

**Needed:**
- Hierarchical policies (org → team → agent)
- Policy inheritance and override
- Conflict resolution (when policies contradict)
- Policy composition (combining multiple policies)
- Temporal policies (valid from X to Y)
- Contextual policies (applies only in context Z)

### Gap 2: Risk Calculation Engine

**Current:** Risk limits are static thresholds.

**Needed:**
- Dynamic risk scoring
- Cumulative risk tracking (per session, per day)
- Risk decay over time
- Risk categories (financial, reputational, security, privacy)
- Risk aggregation across actions
- Risk budget management

### Gap 3: Approval Workflows

**Current:** Simple approve/deny.

**Needed:**
- Multi-party approval (2-of-3 required)
- Escalation chains (if A doesn't respond, go to B)
- Delegation (A can approve on behalf of B)
- Conditional approval ("approved if amount < $1000")
- Approval expiration
- Approval audit trail

### Gap 4: Obligation Tracking

**Current:** Obligations are recorded but not enforced.

**Needed:**
- Deadline tracking
- Reminder generation
- Obligation dependencies (B must happen after A)
- Obligation fulfillment verification
- Penalty calculation for missed obligations

### Gap 5: Violation Detection

**Current:** Violations are logged.

**Needed:**
- Real-time violation detection
- Violation severity classification
- Automatic response triggers
- Violation patterns (repeated violations → escalate)
- Remediation workflows

### Gap 6: Sister Integration

**Current:** Placeholders only.

**Needed:**
- **Hydra**: Policy evaluation API for execution_gate
- **Identity**: Cryptographic policy signatures
- **Time**: Temporal condition evaluation
- **Memory**: Contract-related event storage

---

## Success Criteria

### Functional Requirements

- [ ] Agent can query "Am I allowed to do X?" in < 10ms
- [ ] Policy conflicts are detected and resolved deterministically
- [ ] Risk budget is tracked accurately across sessions
- [ ] Multi-party approvals work with timeout/escalation
- [ ] Obligations generate reminders before deadlines
- [ ] Violations trigger configurable responses

### Integration Requirements

- [ ] Hydra can evaluate policies via execution_gate
- [ ] Identity can sign contracts and verify signatures
- [ ] Time can evaluate temporal conditions
- [ ] Memory can store contract-related events

### Performance Requirements

- [ ] Policy evaluation: < 1ms for single policy
- [ ] Risk calculation: < 5ms for session total
- [ ] 10,000+ active contracts per project
- [ ] File size: < 1KB per simple contract

---

## Sister Dependencies

```
AgenticContract
      │
      ├──▶ AgenticIdentity (signatures, trust levels)
      │
      ├──▶ AgenticTime (temporal conditions, deadlines)
      │
      ├──▶ AgenticMemory (event storage, audit trail)
      │
      └──▶ Hydra (policy enforcement in execution_gate)
```

---

## What Contract Unlocks

Once complete, Contract enables:

1. **AgenticComm** — Governed agent-to-agent communication
2. **AgenticPlanning** — Bounded goals with constraints
3. **Hydra** — Automatic policy enforcement
4. **Multi-agent systems** — Formal inter-agent agreements

---

# SPEC-02: CORE CONCEPTS

> Deep semantics of governance primitives.

## The Six Primitives

### 2.1 Policy

A **Policy** defines what an agent CAN or CANNOT do.

```
POLICY = {
    id: PolicyId,
    name: String,
    effect: Allow | Deny | RequireApproval,
    scope: Scope,
    conditions: Vec<Condition>,
    priority: u32,
    valid_from: Option<Timestamp>,
    valid_until: Option<Timestamp>,
    metadata: HashMap<String, Value>,
}
```

**Policy Effects:**

| Effect | Meaning |
|--------|---------|
| `Allow` | Action is permitted |
| `Deny` | Action is forbidden |
| `RequireApproval` | Action needs explicit approval |

**Policy Scope:**

```rust
enum Scope {
    Global,                          // Applies to all agents
    Organization(OrgId),             // Applies to org members
    Team(TeamId),                    // Applies to team members
    Agent(AgentId),                  // Applies to specific agent
    Action(ActionPattern),           // Applies to matching actions
    Resource(ResourcePattern),       // Applies to matching resources
}
```

**Policy Hierarchy:**

```
Organization Policy
       │
       ▼ (can override)
   Team Policy
       │
       ▼ (can override)
   Agent Policy
       │
       ▼ (most specific wins)
   Action Policy
```

**Conflict Resolution Rules:**

1. More specific scope wins over general scope
2. Explicit Deny wins over Allow (safety first)
3. Higher priority number wins
4. RequireApproval is middle ground

---

### 2.2 Risk Limit

A **Risk Limit** defines thresholds that trigger actions.

```
RISK_LIMIT = {
    id: RiskLimitId,
    name: String,
    category: RiskCategory,
    threshold: f64,
    window: Duration,
    action: RiskAction,
    cumulative: bool,
}
```

**Risk Categories:**

| Category | Examples |
|----------|----------|
| `Financial` | Money spent, transactions |
| `Security` | Access attempts, privilege escalation |
| `Privacy` | Data access, PII handling |
| `Reputational` | External communications |
| `Operational` | Resource usage, API calls |
| `Compliance` | Regulatory actions |

**Risk Actions:**

```rust
enum RiskAction {
    Warn,                    // Log warning, continue
    RequireApproval,         // Pause and get approval
    Block,                   // Prevent action
    Alert(Vec<Recipient>),   // Notify stakeholders
    Escalate(EscalationChain),
}
```

**Risk Budget Model:**

```
RISK_BUDGET = {
    category: RiskCategory,
    total_budget: f64,
    used: f64,
    window_start: Timestamp,
    window_duration: Duration,
    decay_rate: f64,         // Budget recovery per hour
}
```

---

### 2.3 Approval

An **Approval** is authorization for a gated action.

```
APPROVAL = {
    id: ApprovalId,
    request: ApprovalRequest,
    status: ApprovalStatus,
    approvers: Vec<Approver>,
    quorum: QuorumRule,
    deadline: Option<Timestamp>,
    escalation: Option<EscalationChain>,
    decisions: Vec<ApprovalDecision>,
}
```

**Approval Status:**

```rust
enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
    Expired,
    Escalated,
    Withdrawn,
}
```

**Quorum Rules:**

```rust
enum QuorumRule {
    Any,                     // Any one approver
    All,                     // All approvers must approve
    Majority,                // > 50% must approve
    Threshold(u32),          // Specific count required
    Weighted(f64),           // Weighted votes must exceed
}
```

**Escalation Chain:**

```rust
struct EscalationChain {
    levels: Vec<EscalationLevel>,
}

struct EscalationLevel {
    approvers: Vec<ApproverId>,
    timeout: Duration,
    auto_action: Option<AutoAction>,  // Auto-approve/deny on timeout
}
```

---

### 2.4 Condition

A **Condition** is a rule that must be satisfied.

```
CONDITION = {
    id: ConditionId,
    expression: Expression,
    context_requirements: Vec<ContextKey>,
}
```

**Expression Language:**

```
// Comparison
amount <= 100
path.starts_with("/project")
time.hour >= 9 AND time.hour <= 17

// Logical
condition_a AND condition_b
condition_a OR condition_b
NOT condition_a

// Context
requester.trust_level >= 0.8
action.type == "file_write"
resource.sensitivity <= "internal"

// Temporal
now >= "2024-01-01" AND now <= "2024-12-31"
elapsed_since(last_action) >= "1 hour"

// Aggregates
sum(actions.amount, window="1d") <= 1000
count(actions, type="api_call", window="1h") <= 100
```

**Context Variables:**

| Variable | Type | Description |
|----------|------|-------------|
| `requester` | Agent | The agent making the request |
| `action` | Action | The action being evaluated |
| `resource` | Resource | The target resource |
| `time` | Timestamp | Current time |
| `session` | Session | Current session context |
| `history` | History | Past actions/events |

---

### 2.5 Obligation

An **Obligation** is something an agent MUST do.

```
OBLIGATION = {
    id: ObligationId,
    contract_id: ContractId,
    description: String,
    deadline: Option<Timestamp>,
    recurrence: Option<Recurrence>,
    status: ObligationStatus,
    dependencies: Vec<ObligationId>,
    verification: VerificationMethod,
    penalty: Option<Penalty>,
}
```

**Obligation Status:**

```rust
enum ObligationStatus {
    Pending,
    InProgress,
    Fulfilled,
    Failed,
    Waived,
    Expired,
}
```

**Verification Methods:**

```rust
enum VerificationMethod {
    Manual(ApproverId),           // Human confirms
    Automatic(AutoVerifyRule),    // System checks
    Evidence(EvidenceRequirement),// Proof required
    Attestation(AttesterId),      // Third party confirms
}
```

**Recurrence:**

```rust
enum Recurrence {
    Once,
    Daily,
    Weekly,
    Monthly,
    Custom(CronExpression),
}
```

---

### 2.6 Violation

A **Violation** is a breach of policy or contract.

```
VIOLATION = {
    id: ViolationId,
    contract_id: ContractId,
    policy_id: Option<PolicyId>,
    severity: Severity,
    action: ActionRecord,
    detected_at: Timestamp,
    status: ViolationStatus,
    response: Option<ViolationResponse>,
    remediation: Option<Remediation>,
}
```

**Severity Levels:**

| Level | Description | Typical Response |
|-------|-------------|------------------|
| `Info` | Minor deviation | Log only |
| `Warning` | Notable breach | Alert + log |
| `Serious` | Significant breach | Block + alert + review |
| `Critical` | Severe breach | Block + alert + immediate escalation |
| `Emergency` | System-threatening | Shutdown + full audit |

**Violation Responses:**

```rust
enum ViolationResponse {
    Logged,                     // Recorded only
    Alerted(Vec<Recipient>),    // Notifications sent
    Blocked,                    // Action prevented
    Reverted,                   // Action undone
    Escalated(EscalationId),    // Sent to higher authority
    Suspended(AgentId),         // Agent suspended
}
```

---

## Contract Composition

A **Contract** bundles multiple primitives:

```
CONTRACT = {
    id: ContractId,
    name: String,
    version: Version,
    parties: Vec<Party>,
    policies: Vec<Policy>,
    risk_limits: Vec<RiskLimit>,
    obligations: Vec<Obligation>,
    conditions: Vec<Condition>,
    effective_from: Timestamp,
    effective_until: Option<Timestamp>,
    status: ContractStatus,
    signatures: Vec<Signature>,
}
```

**Contract Status:**

```rust
enum ContractStatus {
    Draft,          // Being composed
    Pending,        // Awaiting signatures
    Active,         // In effect
    Suspended,      // Temporarily paused
    Terminated,     // Ended early
    Expired,        // Reached end date
    Violated,       // Breach detected
}
```

---

# SPEC-03: DATA STRUCTURES

> Enhanced Rust types for full implementation.

## 3.1 Scaffold Types (Already Exist)

```rust
// These exist in the scaffold
pub struct Contract {
    pub id: ContractId,
    pub name: String,
    pub status: ContractStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

pub struct Policy {
    pub id: PolicyId,
    pub name: String,
    pub effect: Effect,
}

pub struct RiskLimit {
    pub id: RiskLimitId,
    pub name: String,
    pub threshold: f64,
}

pub struct Approval {
    pub id: ApprovalId,
    pub status: ApprovalStatus,
}

pub struct Obligation {
    pub id: ObligationId,
    pub description: String,
}

pub struct Violation {
    pub id: ViolationId,
    pub severity: Severity,
}
```

---

## 3.2 Enhanced Types (To Implement)

### PolicyV2

```rust
/// Enhanced policy with full semantics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyV2 {
    // Core identity
    pub id: PolicyId,
    pub name: String,
    pub description: Option<String>,
    
    // Effect
    pub effect: Effect,
    pub priority: u32,
    
    // Scope
    pub scope: PolicyScope,
    pub target: PolicyTarget,
    
    // Conditions
    pub conditions: Vec<Condition>,
    pub condition_logic: ConditionLogic,
    
    // Temporal
    pub valid_from: Option<Timestamp>,
    pub valid_until: Option<Timestamp>,
    pub schedule: Option<Schedule>,
    
    // Hierarchy
    pub parent_id: Option<PolicyId>,
    pub override_behavior: OverrideBehavior,
    
    // Metadata
    pub tags: Vec<String>,
    pub metadata: HashMap<String, Value>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub created_by: Option<AgentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyScope {
    Global,
    Organization { org_id: OrgId },
    Team { team_id: TeamId },
    Agent { agent_id: AgentId },
    Session { session_id: SessionId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyTarget {
    AllActions,
    ActionType { pattern: String },
    Resource { pattern: String },
    Custom { expression: Expression },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionLogic {
    All,    // AND - all conditions must pass
    Any,    // OR - any condition passes
    Custom { expression: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverrideBehavior {
    Inherit,    // Add to parent policies
    Replace,    // Replace parent policies
    Merge,      // Merge with conflict resolution
}
```

### RiskEngineTypes

```rust
/// Risk calculation and budget management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProfile {
    pub agent_id: AgentId,
    pub session_id: Option<SessionId>,
    pub budgets: HashMap<RiskCategory, RiskBudget>,
    pub history: Vec<RiskEvent>,
    pub current_score: f64,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskBudget {
    pub category: RiskCategory,
    pub total: f64,
    pub used: f64,
    pub reserved: f64,          // Pending approvals
    pub window_start: Timestamp,
    pub window_duration: Duration,
    pub decay_rate: f64,        // Recovery per hour
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvent {
    pub id: RiskEventId,
    pub category: RiskCategory,
    pub amount: f64,
    pub action_id: ActionId,
    pub timestamp: Timestamp,
    pub approved: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RiskCategory {
    Financial,
    Security,
    Privacy,
    Reputational,
    Operational,
    Compliance,
    Custom(u32),
}

/// Risk limit with full configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitV2 {
    pub id: RiskLimitId,
    pub name: String,
    pub description: Option<String>,
    pub category: RiskCategory,
    pub threshold: f64,
    pub window: Duration,
    pub cumulative: bool,
    pub action: RiskAction,
    pub escalation: Option<EscalationChain>,
    pub enabled: bool,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskAction {
    Warn { message: String },
    RequireApproval { approvers: Vec<ApproverId> },
    Block { message: String },
    Alert { recipients: Vec<Recipient> },
    Escalate { chain: EscalationChain },
    Custom { handler: String },
}
```

### ApprovalWorkflowTypes

```rust
/// Full approval workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: ApprovalRequestId,
    pub contract_id: ContractId,
    pub action: ActionDescription,
    pub requester: AgentId,
    pub reason: Option<String>,
    pub context: HashMap<String, Value>,
    pub created_at: Timestamp,
    pub deadline: Option<Timestamp>,
    pub status: ApprovalStatus,
    pub workflow: ApprovalWorkflow,
    pub decisions: Vec<ApprovalDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    pub approvers: Vec<ApproverSpec>,
    pub quorum: QuorumRule,
    pub escalation: Option<EscalationChain>,
    pub delegation_allowed: bool,
    pub conditional_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApproverSpec {
    Agent { id: AgentId },
    Role { role: String },
    Team { team_id: TeamId },
    Any { from: Vec<ApproverId> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuorumRule {
    Any,
    All,
    Majority,
    Count { required: u32 },
    Weighted { threshold: f64, weights: HashMap<ApproverId, f64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    pub approver: ApproverId,
    pub decision: Decision,
    pub conditions: Vec<ApprovalCondition>,
    pub comment: Option<String>,
    pub timestamp: Timestamp,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    Approve,
    Deny { reason: String },
    Delegate { to: ApproverId },
    Abstain,
    ConditionalApprove { conditions: Vec<Condition> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationChain {
    pub levels: Vec<EscalationLevel>,
    pub max_escalations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLevel {
    pub approvers: Vec<ApproverId>,
    pub timeout: Duration,
    pub auto_action: Option<AutoAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoAction {
    Approve,
    Deny,
    EscalateNext,
    Alert { recipients: Vec<Recipient> },
}
```

### ObligationTypes

```rust
/// Obligation with full tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationV2 {
    pub id: ObligationId,
    pub contract_id: ContractId,
    pub obligor: AgentId,           // Who must fulfill
    pub beneficiary: Option<AgentId>, // Who benefits
    pub description: String,
    pub details: ObligationDetails,
    pub deadline: Option<Timestamp>,
    pub recurrence: Option<Recurrence>,
    pub status: ObligationStatus,
    pub dependencies: Vec<ObligationId>,
    pub verification: VerificationMethod,
    pub penalty: Option<Penalty>,
    pub reminders: Vec<Reminder>,
    pub history: Vec<ObligationEvent>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationDetails {
    pub action_required: ActionDescription,
    pub acceptance_criteria: Vec<Criterion>,
    pub evidence_required: Vec<EvidenceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    Manual { verifier: ApproverId },
    Automatic { rule: VerificationRule },
    Evidence { types: Vec<EvidenceType>, verifier: Option<ApproverId> },
    Attestation { attester: AgentId },
    SelfReport { require_evidence: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    pub severity: Severity,
    pub actions: Vec<PenaltyAction>,
    pub grace_period: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenaltyAction {
    Warning,
    RiskIncrease { category: RiskCategory, amount: f64 },
    TrustDecrease { amount: f64 },
    Suspension { duration: Duration },
    ContractTermination,
    Escalation { chain: EscalationChain },
    Custom { action: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub offset: Duration,           // Before deadline
    pub recipient: Recipient,
    pub message: String,
    pub sent: bool,
    pub sent_at: Option<Timestamp>,
}
```

### ViolationTypes

```rust
/// Violation with response workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationV2 {
    pub id: ViolationId,
    pub contract_id: ContractId,
    pub policy_id: Option<PolicyId>,
    pub obligation_id: Option<ObligationId>,
    pub violator: AgentId,
    pub severity: Severity,
    pub category: ViolationCategory,
    pub action: ActionRecord,
    pub detected_at: Timestamp,
    pub detection_method: DetectionMethod,
    pub status: ViolationStatus,
    pub response: ViolationResponse,
    pub remediation: Option<Remediation>,
    pub impact_assessment: Option<ImpactAssessment>,
    pub related_violations: Vec<ViolationId>,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationCategory {
    PolicyBreach,
    RiskExceeded,
    ObligationMissed,
    UnauthorizedAction,
    DataBreach,
    ComplianceFailure,
    ContractBreach,
    Custom { category: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionMethod {
    RealTime,           // Caught before action completed
    PostAction,         // Detected after action
    Audit,              // Found in periodic review
    Report,             // Someone reported it
    Pattern,            // Detected from pattern analysis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationResponse {
    pub actions_taken: Vec<ResponseAction>,
    pub notifications_sent: Vec<NotificationRecord>,
    pub escalations: Vec<EscalationRecord>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseAction {
    Logged,
    ActionBlocked,
    ActionReverted,
    AgentSuspended { until: Option<Timestamp> },
    ContractSuspended,
    AlertSent { recipients: Vec<Recipient> },
    EscalationTriggered { chain_id: EscalationChainId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remediation {
    pub id: RemediationId,
    pub plan: RemediationPlan,
    pub status: RemediationStatus,
    pub assigned_to: Option<AgentId>,
    pub deadline: Option<Timestamp>,
    pub actions_completed: Vec<RemediationAction>,
}
```

### Expression Engine Types

```rust
/// Expression evaluation for conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    // Literals
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    
    // Variables
    Variable(String),
    Path(Vec<String>),
    
    // Comparisons
    Eq(Box<Expression>, Box<Expression>),
    Ne(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Le(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Ge(Box<Expression>, Box<Expression>),
    
    // Logical
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Not(Box<Expression>),
    
    // String operations
    StartsWith(Box<Expression>, Box<Expression>),
    EndsWith(Box<Expression>, Box<Expression>),
    Contains(Box<Expression>, Box<Expression>),
    Matches(Box<Expression>, String),  // Regex
    
    // Numeric operations
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    
    // Aggregates
    Sum { field: String, filter: Option<Box<Expression>>, window: Option<Duration> },
    Count { filter: Option<Box<Expression>>, window: Option<Duration> },
    Avg { field: String, filter: Option<Box<Expression>>, window: Option<Duration> },
    Min { field: String, filter: Option<Box<Expression>>, window: Option<Duration> },
    Max { field: String, filter: Option<Box<Expression>>, window: Option<Duration> },
    
    // Temporal
    Now,
    TimeSince(Box<Expression>),
    TimeUntil(Box<Expression>),
    InTimeRange { start: Box<Expression>, end: Box<Expression> },
    
    // Existence
    Exists(String),
    IsNull(Box<Expression>),
    
    // Collections
    In(Box<Expression>, Vec<Expression>),
    All(Box<Expression>, Box<Expression>),
    Any(Box<Expression>, Box<Expression>),
}

/// Context for expression evaluation
#[derive(Debug, Clone)]
pub struct EvaluationContext {
    pub agent: AgentContext,
    pub action: ActionContext,
    pub resource: Option<ResourceContext>,
    pub session: SessionContext,
    pub time: Timestamp,
    pub history: HistoryContext,
    pub custom: HashMap<String, Value>,
}
```

---

# SPEC-04: FILE FORMAT

> The .acon binary format specification.

## 4.1 Current Format (Scaffold)

```
┌─────────────────────────────────────────┐
│ HEADER (64 bytes)                       │
├─────────────────────────────────────────┤
│ Magic: "ACON" (4 bytes)                 │
│ Version: u16                            │
│ Flags: u16                              │
│ Contract Count: u32                     │
│ Total Size: u64                         │
│ Checksum: BLAKE3 (32 bytes)             │
│ Reserved (16 bytes)                     │
├─────────────────────────────────────────┤
│ BODY (variable)                         │
├─────────────────────────────────────────┤
│ Contracts Section                       │
│ Policies Section                        │
│ Risk Limits Section                     │
│ Approvals Section                       │
│ Obligations Section                     │
│ Violations Section                      │
└─────────────────────────────────────────┘
```

## 4.2 Enhanced Format (V2)

```
┌─────────────────────────────────────────┐
│ HEADER (128 bytes)                      │
├─────────────────────────────────────────┤
│ Magic: "ACON" (4 bytes)                 │
│ Format Version: u16 (2)                 │
│ Feature Flags: u32                      │
│ Compression: u8                         │
│ Encryption: u8                          │
│ Reserved: u16                           │
│                                         │
│ Section Count: u32                      │
│ Total Size: u64                         │
│ Uncompressed Size: u64                  │
│                                         │
│ Created At: i64 (timestamp)             │
│ Modified At: i64 (timestamp)            │
│ Schema Version: u32                     │
│                                         │
│ Content Checksum: BLAKE3 (32 bytes)     │
│ Header Checksum: CRC32 (4 bytes)        │
│                                         │
│ Index Offset: u64                       │
│ Signature Offset: u64                   │
│                                         │
│ Reserved (24 bytes)                     │
├─────────────────────────────────────────┤
│ SECTION DIRECTORY                       │
├─────────────────────────────────────────┤
│ For each section:                       │
│   Type: u32                             │
│   Offset: u64                           │
│   Size: u64                             │
│   Checksum: u32 (CRC32)                 │
│   Flags: u32                            │
├─────────────────────────────────────────┤
│ SECTIONS                                │
├─────────────────────────────────────────┤
│ Section 1: Contracts                    │
│ Section 2: Policies                     │
│ Section 3: Risk Limits                  │
│ Section 4: Risk Profiles                │
│ Section 5: Approvals                    │
│ Section 6: Approval Workflows           │
│ Section 7: Obligations                  │
│ Section 8: Violations                   │
│ Section 9: Expressions                  │
│ Section 10: History                     │
├─────────────────────────────────────────┤
│ INDEX SECTION                           │
├─────────────────────────────────────────┤
│ Contract Index (by ID)                  │
│ Policy Index (by scope)                 │
│ Temporal Index (by date)                │
│ Status Index (by status)                │
│ Agent Index (by agent)                  │
├─────────────────────────────────────────┤
│ SIGNATURE SECTION (optional)            │
├─────────────────────────────────────────┤
│ Signature Count: u32                    │
│ For each signature:                     │
│   Signer ID: 32 bytes                   │
│   Algorithm: u8                         │
│   Signature: variable                   │
│   Timestamp: i64                        │
└─────────────────────────────────────────┘
```

### Feature Flags

| Bit | Flag | Description |
|-----|------|-------------|
| 0 | `HAS_INDEX` | Index section present |
| 1 | `HAS_SIGNATURES` | Signature section present |
| 2 | `HAS_HISTORY` | History section present |
| 3 | `COMPRESSED` | Body is compressed |
| 4 | `ENCRYPTED` | Body is encrypted |
| 5 | `STREAMING` | Supports streaming reads |
| 6-31 | Reserved | Future use |

### Section Types

| Type | Name | Description |
|------|------|-------------|
| 0x01 | `CONTRACTS` | Contract definitions |
| 0x02 | `POLICIES` | Policy rules |
| 0x03 | `RISK_LIMITS` | Risk thresholds |
| 0x04 | `RISK_PROFILES` | Agent risk state |
| 0x05 | `APPROVALS` | Approval requests |
| 0x06 | `WORKFLOWS` | Approval workflows |
| 0x07 | `OBLIGATIONS` | Obligation records |
| 0x08 | `VIOLATIONS` | Violation records |
| 0x09 | `EXPRESSIONS` | Compiled expressions |
| 0x0A | `HISTORY` | Event history |
| 0x0B | `INDEX` | Lookup indexes |
| 0x0C | `SIGNATURES` | Cryptographic signatures |

### Compression

| Value | Algorithm |
|-------|-----------|
| 0x00 | None |
| 0x01 | LZ4 |
| 0x02 | Zstd |

### Encryption

| Value | Algorithm |
|-------|-----------|
| 0x00 | None |
| 0x01 | AES-256-GCM |
| 0x02 | ChaCha20-Poly1305 |

---

# SPEC-05: POLICY ENGINE

> How policies are evaluated and enforced.

## 5.1 Policy Evaluation Pipeline

```
Action Request
      │
      ▼
┌─────────────────┐
│ 1. Collect      │  ← Gather all applicable policies
│    Policies     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ 2. Sort by      │  ← Most specific first, then priority
│    Precedence   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ 3. Evaluate     │  ← Check conditions against context
│    Conditions   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ 4. Resolve      │  ← Handle conflicting effects
│    Conflicts    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ 5. Check        │  ← Evaluate risk budget
│    Risk         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ 6. Return       │  ← Allow, Deny, or RequireApproval
│    Decision     │
└─────────────────┘
```

## 5.2 Policy Collection

```rust
impl PolicyEngine {
    /// Collect all policies that apply to this action
    pub fn collect_applicable_policies(
        &self,
        action: &Action,
        context: &EvaluationContext,
    ) -> Vec<&PolicyV2> {
        let mut policies = Vec::new();
        
        // 1. Global policies
        policies.extend(self.get_global_policies());
        
        // 2. Organization policies (if agent belongs to org)
        if let Some(org_id) = context.agent.organization_id {
            policies.extend(self.get_org_policies(org_id));
        }
        
        // 3. Team policies
        for team_id in &context.agent.team_ids {
            policies.extend(self.get_team_policies(*team_id));
        }
        
        // 4. Agent-specific policies
        policies.extend(self.get_agent_policies(context.agent.id));
        
        // 5. Action-type policies
        policies.extend(self.get_action_policies(&action.action_type));
        
        // 6. Resource policies
        if let Some(resource) = &action.resource {
            policies.extend(self.get_resource_policies(resource));
        }
        
        // 7. Session policies
        if let Some(session_id) = context.session.id {
            policies.extend(self.get_session_policies(session_id));
        }
        
        // Filter to only active policies
        policies.retain(|p| self.is_policy_active(p, context.time));
        
        policies
    }
}
```

## 5.3 Precedence Sorting

```rust
impl PolicyEngine {
    /// Sort policies by precedence (most specific wins)
    pub fn sort_by_precedence(&self, policies: &mut [&PolicyV2]) {
        policies.sort_by(|a, b| {
            // 1. More specific scope wins
            let scope_order_a = self.scope_specificity(&a.scope);
            let scope_order_b = self.scope_specificity(&b.scope);
            
            if scope_order_a != scope_order_b {
                return scope_order_b.cmp(&scope_order_a); // Higher = more specific
            }
            
            // 2. Higher priority wins
            if a.priority != b.priority {
                return b.priority.cmp(&a.priority);
            }
            
            // 3. More conditions = more specific
            let cond_count_a = a.conditions.len();
            let cond_count_b = b.conditions.len();
            
            if cond_count_a != cond_count_b {
                return cond_count_b.cmp(&cond_count_a);
            }
            
            // 4. Deny wins over Allow (safety)
            match (&a.effect, &b.effect) {
                (Effect::Deny, Effect::Allow) => std::cmp::Ordering::Less,
                (Effect::Allow, Effect::Deny) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });
    }
    
    fn scope_specificity(&self, scope: &PolicyScope) -> u32 {
        match scope {
            PolicyScope::Global => 0,
            PolicyScope::Organization { .. } => 1,
            PolicyScope::Team { .. } => 2,
            PolicyScope::Agent { .. } => 3,
            PolicyScope::Session { .. } => 4,
        }
    }
}
```

## 5.4 Condition Evaluation

```rust
impl PolicyEngine {
    /// Evaluate policy conditions against context
    pub fn evaluate_conditions(
        &self,
        policy: &PolicyV2,
        context: &EvaluationContext,
    ) -> Result<bool, PolicyError> {
        if policy.conditions.is_empty() {
            return Ok(true); // No conditions = always applies
        }
        
        let results: Vec<bool> = policy.conditions
            .iter()
            .map(|c| self.evaluate_condition(c, context))
            .collect::<Result<Vec<_>, _>>()?;
        
        match policy.condition_logic {
            ConditionLogic::All => Ok(results.iter().all(|&r| r)),
            ConditionLogic::Any => Ok(results.iter().any(|&r| r)),
            ConditionLogic::Custom { ref expression } => {
                self.evaluate_custom_logic(expression, &results)
            }
        }
    }
    
    fn evaluate_condition(
        &self,
        condition: &Condition,
        context: &EvaluationContext,
    ) -> Result<bool, PolicyError> {
        self.expression_engine.evaluate(&condition.expression, context)
    }
}
```

## 5.5 Conflict Resolution

```rust
impl PolicyEngine {
    /// Resolve conflicts between applicable policies
    pub fn resolve_conflicts(
        &self,
        policies: &[(&PolicyV2, bool)], // (policy, conditions_met)
    ) -> PolicyDecision {
        // Filter to policies where conditions are met
        let applicable: Vec<_> = policies
            .iter()
            .filter(|(_, met)| *met)
            .map(|(p, _)| *p)
            .collect();
        
        if applicable.is_empty() {
            return PolicyDecision::Allow { reason: "No applicable policies".into() };
        }
        
        // First applicable policy wins (already sorted by precedence)
        let winning_policy = applicable[0];
        
        // Check for explicit deny anywhere (safety override)
        let has_explicit_deny = applicable.iter()
            .any(|p| matches!(p.effect, Effect::Deny) && p.priority >= 1000);
        
        if has_explicit_deny {
            return PolicyDecision::Deny {
                policy_id: winning_policy.id,
                reason: "Explicit deny policy".into(),
            };
        }
        
        match &winning_policy.effect {
            Effect::Allow => PolicyDecision::Allow {
                reason: format!("Allowed by policy: {}", winning_policy.name),
            },
            Effect::Deny => PolicyDecision::Deny {
                policy_id: winning_policy.id,
                reason: format!("Denied by policy: {}", winning_policy.name),
            },
            Effect::RequireApproval => PolicyDecision::RequireApproval {
                policy_id: winning_policy.id,
                workflow: self.get_approval_workflow(winning_policy),
            },
        }
    }
}
```

## 5.6 Risk Evaluation

```rust
impl PolicyEngine {
    /// Check if action exceeds risk limits
    pub fn evaluate_risk(
        &self,
        action: &Action,
        context: &EvaluationContext,
    ) -> RiskEvaluation {
        let mut evaluations = Vec::new();
        
        // Get agent's risk profile
        let profile = self.get_risk_profile(context.agent.id);
        
        // Calculate risk for each category
        for (category, risk_amount) in self.calculate_action_risk(action) {
            let budget = profile.budgets.get(&category);
            let limit = self.get_risk_limit(context.agent.id, category);
            
            if let (Some(budget), Some(limit)) = (budget, limit) {
                let new_total = budget.used + budget.reserved + risk_amount;
                
                if new_total > limit.threshold {
                    evaluations.push(RiskEvaluation {
                        category,
                        current: budget.used,
                        requested: risk_amount,
                        limit: limit.threshold,
                        action: limit.action.clone(),
                        exceeded: true,
                    });
                }
            }
        }
        
        // Return worst case
        evaluations.into_iter()
            .filter(|e| e.exceeded)
            .max_by(|a, b| a.action.severity().cmp(&b.action.severity()))
            .unwrap_or(RiskEvaluation::ok())
    }
}
```

## 5.7 Main Evaluation Entry Point

```rust
impl PolicyEngine {
    /// Main entry point: Can this action be performed?
    pub fn evaluate(
        &self,
        action: &Action,
        context: &EvaluationContext,
    ) -> EvaluationResult {
        // 1. Collect applicable policies
        let mut policies = self.collect_applicable_policies(action, context);
        
        // 2. Sort by precedence
        self.sort_by_precedence(&mut policies);
        
        // 3. Evaluate conditions for each policy
        let evaluated: Vec<_> = policies
            .into_iter()
            .map(|p| {
                let met = self.evaluate_conditions(p, context).unwrap_or(false);
                (p, met)
            })
            .collect();
        
        // 4. Resolve conflicts
        let policy_decision = self.resolve_conflicts(&evaluated);
        
        // 5. Check risk (even if policy allows)
        let risk_evaluation = self.evaluate_risk(action, context);
        
        // 6. Combine decisions
        match (&policy_decision, &risk_evaluation.exceeded) {
            (PolicyDecision::Deny { .. }, _) => EvaluationResult {
                allowed: false,
                decision: policy_decision,
                risk: risk_evaluation,
                requires_approval: false,
            },
            (_, true) => EvaluationResult {
                allowed: false,
                decision: PolicyDecision::RequireApproval {
                    policy_id: PolicyId::default(),
                    workflow: self.get_risk_approval_workflow(&risk_evaluation),
                },
                risk: risk_evaluation,
                requires_approval: true,
            },
            (PolicyDecision::RequireApproval { .. }, _) => EvaluationResult {
                allowed: false,
                decision: policy_decision,
                risk: risk_evaluation,
                requires_approval: true,
            },
            (PolicyDecision::Allow { .. }, false) => EvaluationResult {
                allowed: true,
                decision: policy_decision,
                risk: risk_evaluation,
                requires_approval: false,
            },
        }
    }
}
```

---

# SPEC-06: QUERY ENGINE

> Searching and filtering contracts, policies, and state.

## 6.1 Query Types

```rust
pub enum ContractQuery {
    // By identity
    ById(ContractId),
    ByIds(Vec<ContractId>),
    
    // By status
    ByStatus(ContractStatus),
    Active,
    Pending,
    Expired,
    
    // By party
    ByParty(AgentId),
    ByParties(Vec<AgentId>),
    
    // By time
    EffectiveBefore(Timestamp),
    EffectiveAfter(Timestamp),
    ExpiringWithin(Duration),
    
    // By content
    ContainingPolicy(PolicyId),
    ContainingObligation(ObligationId),
    
    // By tags
    WithTag(String),
    WithTags(Vec<String>),
    
    // Full text
    Search(String),
    
    // Compound
    And(Vec<ContractQuery>),
    Or(Vec<ContractQuery>),
    Not(Box<ContractQuery>),
}

pub enum PolicyQuery {
    // By identity
    ById(PolicyId),
    
    // By scope
    ForAgent(AgentId),
    ForTeam(TeamId),
    ForOrganization(OrgId),
    Global,
    
    // By effect
    WithEffect(Effect),
    Denying,
    Allowing,
    RequiringApproval,
    
    // By target
    TargetingAction(String),
    TargetingResource(String),
    
    // By validity
    ValidAt(Timestamp),
    ValidNow,
    
    // By priority
    PriorityAbove(u32),
    PriorityBelow(u32),
}

pub enum ApprovalQuery {
    // By identity
    ById(ApprovalRequestId),
    
    // By status
    Pending,
    ByStatus(ApprovalStatus),
    
    // By participant
    RequestedBy(AgentId),
    AssignedTo(ApproverId),
    
    // By deadline
    ExpiringWithin(Duration),
    Overdue,
    
    // By urgency
    HighPriority,
}
```

## 6.2 Query Execution

```rust
impl ContractEngine {
    pub fn query(&self, query: &ContractQuery) -> Vec<Contract> {
        match query {
            ContractQuery::ById(id) => {
                self.contracts.get(id).cloned().into_iter().collect()
            }
            
            ContractQuery::Active => {
                self.contracts.values()
                    .filter(|c| c.status == ContractStatus::Active)
                    .cloned()
                    .collect()
            }
            
            ContractQuery::ByParty(agent_id) => {
                self.index.by_party.get(agent_id)
                    .map(|ids| ids.iter()
                        .filter_map(|id| self.contracts.get(id))
                        .cloned()
                        .collect())
                    .unwrap_or_default()
            }
            
            ContractQuery::ExpiringWithin(duration) => {
                let deadline = Timestamp::now() + *duration;
                self.contracts.values()
                    .filter(|c| {
                        c.effective_until
                            .map(|t| t <= deadline)
                            .unwrap_or(false)
                    })
                    .cloned()
                    .collect()
            }
            
            ContractQuery::Search(text) => {
                self.search_index.search(text)
                    .into_iter()
                    .filter_map(|id| self.contracts.get(&id))
                    .cloned()
                    .collect()
            }
            
            ContractQuery::And(queries) => {
                let results: Vec<HashSet<ContractId>> = queries
                    .iter()
                    .map(|q| self.query(q).into_iter().map(|c| c.id).collect())
                    .collect();
                
                let intersection = results.into_iter()
                    .reduce(|a, b| a.intersection(&b).cloned().collect())
                    .unwrap_or_default();
                
                intersection.into_iter()
                    .filter_map(|id| self.contracts.get(&id))
                    .cloned()
                    .collect()
            }
            
            // ... other variants
        }
    }
}
```

## 6.3 Semantic Queries

```rust
impl ContractEngine {
    /// "What can agent X do?"
    pub fn query_agent_permissions(&self, agent_id: AgentId) -> AgentPermissions {
        let context = self.build_context_for_agent(agent_id);
        let policies = self.policy_engine.collect_applicable_policies_for_agent(agent_id);
        
        AgentPermissions {
            allowed_actions: self.extract_allowed_actions(&policies, &context),
            denied_actions: self.extract_denied_actions(&policies, &context),
            conditional_actions: self.extract_conditional_actions(&policies, &context),
            risk_budgets: self.get_risk_profile(agent_id).budgets,
            active_obligations: self.get_agent_obligations(agent_id),
        }
    }
    
    /// "Can agent X do action Y?"
    pub fn query_can_do(
        &self,
        agent_id: AgentId,
        action: &ActionDescription,
    ) -> CanDoResult {
        let context = self.build_context_for_agent(agent_id);
        let action = Action::from_description(action);
        
        let result = self.policy_engine.evaluate(&action, &context);
        
        CanDoResult {
            allowed: result.allowed,
            requires_approval: result.requires_approval,
            applicable_policies: result.applicable_policies(),
            risk_impact: result.risk,
            conditions: result.unfulfilled_conditions(),
        }
    }
    
    /// "What would happen if agent X did action Y?"
    pub fn query_impact(
        &self,
        agent_id: AgentId,
        action: &ActionDescription,
    ) -> ImpactAnalysis {
        let context = self.build_context_for_agent(agent_id);
        
        ImpactAnalysis {
            policy_result: self.policy_engine.evaluate(&action.into(), &context),
            risk_change: self.calculate_risk_change(agent_id, action),
            obligations_triggered: self.find_triggered_obligations(agent_id, action),
            potential_violations: self.predict_violations(agent_id, action),
            approval_required: self.check_approval_required(agent_id, action),
        }
    }
    
    /// "Who can approve action X?"
    pub fn query_approvers(
        &self,
        agent_id: AgentId,
        action: &ActionDescription,
    ) -> Vec<ApproverInfo> {
        let context = self.build_context_for_agent(agent_id);
        let action = Action::from_description(action);
        
        let result = self.policy_engine.evaluate(&action, &context);
        
        if let PolicyDecision::RequireApproval { workflow, .. } = result.decision {
            workflow.approvers.iter()
                .flat_map(|spec| self.resolve_approver_spec(spec))
                .collect()
        } else {
            Vec::new()
        }
    }
}
```

## 6.4 Obligation Queries

```rust
impl ContractEngine {
    /// "What obligations are due soon?"
    pub fn query_upcoming_obligations(
        &self,
        agent_id: Option<AgentId>,
        window: Duration,
    ) -> Vec<ObligationV2> {
        let deadline = Timestamp::now() + window;
        
        self.obligations.values()
            .filter(|o| {
                // Match agent if specified
                agent_id.map(|id| o.obligor == id).unwrap_or(true)
            })
            .filter(|o| {
                // Not yet fulfilled
                matches!(o.status, ObligationStatus::Pending | ObligationStatus::InProgress)
            })
            .filter(|o| {
                // Due within window
                o.deadline.map(|d| d <= deadline).unwrap_or(false)
            })
            .cloned()
            .collect()
    }
    
    /// "What obligations are overdue?"
    pub fn query_overdue_obligations(
        &self,
        agent_id: Option<AgentId>,
    ) -> Vec<ObligationV2> {
        let now = Timestamp::now();
        
        self.obligations.values()
            .filter(|o| {
                agent_id.map(|id| o.obligor == id).unwrap_or(true)
            })
            .filter(|o| {
                matches!(o.status, ObligationStatus::Pending | ObligationStatus::InProgress)
            })
            .filter(|o| {
                o.deadline.map(|d| d < now).unwrap_or(false)
            })
            .cloned()
            .collect()
    }
}
```

---

# SPEC-07: INDEX STRUCTURES

> Fast lookup structures for contract queries.

## 7.1 Index Types

```rust
pub struct ContractIndexes {
    // Primary index
    pub by_id: HashMap<ContractId, Contract>,
    
    // Secondary indexes
    pub by_status: HashMap<ContractStatus, HashSet<ContractId>>,
    pub by_party: HashMap<AgentId, HashSet<ContractId>>,
    pub by_tag: HashMap<String, HashSet<ContractId>>,
    
    // Temporal indexes
    pub by_effective_date: BTreeMap<Timestamp, HashSet<ContractId>>,
    pub by_expiry_date: BTreeMap<Timestamp, HashSet<ContractId>>,
    
    // Full-text search
    pub search: SearchIndex,
}

pub struct PolicyIndexes {
    // Primary
    pub by_id: HashMap<PolicyId, PolicyV2>,
    
    // By scope
    pub global: HashSet<PolicyId>,
    pub by_organization: HashMap<OrgId, HashSet<PolicyId>>,
    pub by_team: HashMap<TeamId, HashSet<PolicyId>>,
    pub by_agent: HashMap<AgentId, HashSet<PolicyId>>,
    
    // By target
    pub by_action_type: HashMap<String, HashSet<PolicyId>>,
    pub by_resource_pattern: HashMap<String, HashSet<PolicyId>>,
    
    // By effect
    pub by_effect: HashMap<Effect, HashSet<PolicyId>>,
    
    // Hierarchy
    pub children: HashMap<PolicyId, HashSet<PolicyId>>,
    pub parent: HashMap<PolicyId, PolicyId>,
}

pub struct ApprovalIndexes {
    pub by_id: HashMap<ApprovalRequestId, ApprovalRequest>,
    pub by_status: HashMap<ApprovalStatus, HashSet<ApprovalRequestId>>,
    pub by_requester: HashMap<AgentId, HashSet<ApprovalRequestId>>,
    pub by_approver: HashMap<ApproverId, HashSet<ApprovalRequestId>>,
    pub by_deadline: BTreeMap<Timestamp, HashSet<ApprovalRequestId>>,
}

pub struct ObligationIndexes {
    pub by_id: HashMap<ObligationId, ObligationV2>,
    pub by_status: HashMap<ObligationStatus, HashSet<ObligationId>>,
    pub by_obligor: HashMap<AgentId, HashSet<ObligationId>>,
    pub by_deadline: BTreeMap<Timestamp, HashSet<ObligationId>>,
    pub by_contract: HashMap<ContractId, HashSet<ObligationId>>,
}

pub struct ViolationIndexes {
    pub by_id: HashMap<ViolationId, ViolationV2>,
    pub by_severity: HashMap<Severity, HashSet<ViolationId>>,
    pub by_violator: HashMap<AgentId, HashSet<ViolationId>>,
    pub by_status: HashMap<ViolationStatus, HashSet<ViolationId>>,
    pub by_contract: HashMap<ContractId, HashSet<ViolationId>>,
    pub by_timestamp: BTreeMap<Timestamp, HashSet<ViolationId>>,
}
```

## 7.2 Index Maintenance

```rust
impl ContractIndexes {
    pub fn insert(&mut self, contract: Contract) {
        let id = contract.id;
        
        // Update status index
        self.by_status
            .entry(contract.status)
            .or_default()
            .insert(id);
        
        // Update party indexes
        for party in &contract.parties {
            self.by_party
                .entry(party.agent_id)
                .or_default()
                .insert(id);
        }
        
        // Update tag indexes
        for tag in &contract.tags {
            self.by_tag
                .entry(tag.clone())
                .or_default()
                .insert(id);
        }
        
        // Update temporal indexes
        self.by_effective_date
            .entry(contract.effective_from)
            .or_default()
            .insert(id);
        
        if let Some(expiry) = contract.effective_until {
            self.by_expiry_date
                .entry(expiry)
                .or_default()
                .insert(id);
        }
        
        // Update search index
        self.search.index_contract(&contract);
        
        // Store contract
        self.by_id.insert(id, contract);
    }
    
    pub fn remove(&mut self, id: ContractId) {
        if let Some(contract) = self.by_id.remove(&id) {
            // Remove from all secondary indexes
            self.by_status.get_mut(&contract.status).map(|s| s.remove(&id));
            
            for party in &contract.parties {
                self.by_party.get_mut(&party.agent_id).map(|s| s.remove(&id));
            }
            
            for tag in &contract.tags {
                self.by_tag.get_mut(tag).map(|s| s.remove(&id));
            }
            
            self.by_effective_date.get_mut(&contract.effective_from).map(|s| s.remove(&id));
            
            if let Some(expiry) = contract.effective_until {
                self.by_expiry_date.get_mut(&expiry).map(|s| s.remove(&id));
            }
            
            self.search.remove_contract(&id);
        }
    }
    
    pub fn update(&mut self, contract: Contract) {
        self.remove(contract.id);
        self.insert(contract);
    }
}
```

## 7.3 Search Index

```rust
pub struct SearchIndex {
    // Term frequency index
    terms: HashMap<String, HashSet<ContractId>>,
    
    // Document lengths for BM25
    doc_lengths: HashMap<ContractId, usize>,
    avg_doc_length: f64,
    
    // Field-specific indexes
    name_index: HashMap<String, HashSet<ContractId>>,
    description_index: HashMap<String, HashSet<ContractId>>,
}

impl SearchIndex {
    pub fn index_contract(&mut self, contract: &Contract) {
        let id = contract.id;
        let mut terms = Vec::new();
        
        // Index name
        for term in tokenize(&contract.name) {
            self.name_index.entry(term.clone()).or_default().insert(id);
            terms.push(term);
        }
        
        // Index description
        if let Some(desc) = &contract.description {
            for term in tokenize(desc) {
                self.description_index.entry(term.clone()).or_default().insert(id);
                terms.push(term);
            }
        }
        
        // Index policy names
        for policy in &contract.policies {
            for term in tokenize(&policy.name) {
                terms.push(term);
            }
        }
        
        // Update term index
        for term in &terms {
            self.terms.entry(term.clone()).or_default().insert(id);
        }
        
        // Update doc length
        self.doc_lengths.insert(id, terms.len());
        self.update_avg_length();
    }
    
    pub fn search(&self, query: &str) -> Vec<ContractId> {
        let query_terms = tokenize(query);
        
        if query_terms.is_empty() {
            return Vec::new();
        }
        
        // Score documents using BM25
        let mut scores: HashMap<ContractId, f64> = HashMap::new();
        
        for term in &query_terms {
            if let Some(docs) = self.terms.get(term) {
                let idf = self.calculate_idf(docs.len());
                
                for doc_id in docs {
                    let tf = self.calculate_tf(*doc_id, term);
                    let doc_len = self.doc_lengths.get(doc_id).copied().unwrap_or(1);
                    let bm25_score = self.bm25_score(tf, idf, doc_len);
                    
                    *scores.entry(*doc_id).or_default() += bm25_score;
                }
            }
        }
        
        // Sort by score
        let mut results: Vec<_> = scores.into_iter().collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        results.into_iter().map(|(id, _)| id).collect()
    }
}
```

---

# SPEC-08: VALIDATION RULES

> Contract validation and integrity checks.

## 8.1 Validation Categories

```rust
pub enum ValidationCategory {
    Structural,     // Schema correctness
    Semantic,       // Business logic
    Temporal,       // Time consistency
    Referential,    // References valid
    Security,       // Safety checks
    Compliance,     // Policy compliance
}

pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

pub struct ValidationError {
    pub category: ValidationCategory,
    pub code: String,
    pub message: String,
    pub location: Option<String>,
    pub severity: Severity,
}
```

## 8.2 Structural Validation

```rust
impl ContractValidator {
    pub fn validate_structure(&self, contract: &Contract) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Required fields
        if contract.name.is_empty() {
            errors.push(ValidationError {
                category: ValidationCategory::Structural,
                code: "MISSING_NAME".into(),
                message: "Contract name is required".into(),
                location: Some("name".into()),
                severity: Severity::Serious,
            });
        }
        
        // At least one party
        if contract.parties.is_empty() {
            errors.push(ValidationError {
                category: ValidationCategory::Structural,
                code: "NO_PARTIES".into(),
                message: "Contract must have at least one party".into(),
                location: Some("parties".into()),
                severity: Severity::Serious,
            });
        }
        
        // Validate each policy
        for (i, policy) in contract.policies.iter().enumerate() {
            errors.extend(self.validate_policy_structure(policy, i));
        }
        
        // Validate each risk limit
        for (i, limit) in contract.risk_limits.iter().enumerate() {
            errors.extend(self.validate_risk_limit_structure(limit, i));
        }
        
        // Validate each obligation
        for (i, obligation) in contract.obligations.iter().enumerate() {
            errors.extend(self.validate_obligation_structure(obligation, i));
        }
        
        errors
    }
    
    fn validate_policy_structure(&self, policy: &PolicyV2, index: usize) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let location = format!("policies[{}]", index);
        
        if policy.name.is_empty() {
            errors.push(ValidationError {
                category: ValidationCategory::Structural,
                code: "POLICY_MISSING_NAME".into(),
                message: "Policy name is required".into(),
                location: Some(format!("{}.name", location)),
                severity: Severity::Serious,
            });
        }
        
        // Validate conditions
        for (ci, condition) in policy.conditions.iter().enumerate() {
            if let Err(e) = self.expression_parser.validate(&condition.expression) {
                errors.push(ValidationError {
                    category: ValidationCategory::Structural,
                    code: "INVALID_CONDITION".into(),
                    message: format!("Invalid condition expression: {}", e),
                    location: Some(format!("{}.conditions[{}]", location, ci)),
                    severity: Severity::Serious,
                });
            }
        }
        
        errors
    }
}
```

## 8.3 Semantic Validation

```rust
impl ContractValidator {
    pub fn validate_semantics(&self, contract: &Contract) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Check for policy conflicts
        errors.extend(self.check_policy_conflicts(&contract.policies));
        
        // Check obligation feasibility
        errors.extend(self.check_obligation_feasibility(&contract.obligations));
        
        // Check risk limit consistency
        errors.extend(self.check_risk_limit_consistency(&contract.risk_limits));
        
        // Check approval workflow validity
        for policy in &contract.policies {
            if let Effect::RequireApproval = policy.effect {
                errors.extend(self.validate_approval_workflow(policy));
            }
        }
        
        errors
    }
    
    fn check_policy_conflicts(&self, policies: &[PolicyV2]) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        for (i, p1) in policies.iter().enumerate() {
            for (j, p2) in policies.iter().enumerate() {
                if i >= j {
                    continue;
                }
                
                if self.policies_conflict(p1, p2) {
                    // Check if conflict is resolvable by priority
                    if p1.priority == p2.priority {
                        errors.push(ValidationError {
                            category: ValidationCategory::Semantic,
                            code: "POLICY_CONFLICT".into(),
                            message: format!(
                                "Policies '{}' and '{}' conflict and have same priority",
                                p1.name, p2.name
                            ),
                            location: Some(format!("policies[{}] vs policies[{}]", i, j)),
                            severity: Severity::Warning,
                        });
                    }
                }
            }
        }
        
        errors
    }
    
    fn policies_conflict(&self, p1: &PolicyV2, p2: &PolicyV2) -> bool {
        // Same scope
        if p1.scope != p2.scope {
            return false;
        }
        
        // Same target
        if p1.target != p2.target {
            return false;
        }
        
        // Different effects
        p1.effect != p2.effect
    }
}
```

## 8.4 Temporal Validation

```rust
impl ContractValidator {
    pub fn validate_temporal(&self, contract: &Contract) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let now = Timestamp::now();
        
        // Effective dates must be logical
        if let Some(until) = contract.effective_until {
            if until <= contract.effective_from {
                errors.push(ValidationError {
                    category: ValidationCategory::Temporal,
                    code: "INVALID_DATE_RANGE".into(),
                    message: "effective_until must be after effective_from".into(),
                    location: Some("effective_until".into()),
                    severity: Severity::Serious,
                });
            }
        }
        
        // Warn if already expired
        if contract.effective_until.map(|t| t < now).unwrap_or(false) {
            errors.push(ValidationError {
                category: ValidationCategory::Temporal,
                code: "ALREADY_EXPIRED".into(),
                message: "Contract is already expired".into(),
                location: Some("effective_until".into()),
                severity: Severity::Warning,
            });
        }
        
        // Check obligation deadlines
        for (i, obligation) in contract.obligations.iter().enumerate() {
            if let Some(deadline) = obligation.deadline {
                // Deadline before effective date
                if deadline < contract.effective_from {
                    errors.push(ValidationError {
                        category: ValidationCategory::Temporal,
                        code: "DEADLINE_BEFORE_START".into(),
                        message: "Obligation deadline is before contract effective date".into(),
                        location: Some(format!("obligations[{}].deadline", i)),
                        severity: Severity::Serious,
                    });
                }
                
                // Deadline after contract expiry
                if let Some(until) = contract.effective_until {
                    if deadline > until {
                        errors.push(ValidationError {
                            category: ValidationCategory::Temporal,
                            code: "DEADLINE_AFTER_EXPIRY".into(),
                            message: "Obligation deadline is after contract expiry".into(),
                            location: Some(format!("obligations[{}].deadline", i)),
                            severity: Severity::Warning,
                        });
                    }
                }
            }
        }
        
        // Check policy validity windows
        for (i, policy) in contract.policies.iter().enumerate() {
            if let (Some(from), Some(until)) = (policy.valid_from, policy.valid_until) {
                if until <= from {
                    errors.push(ValidationError {
                        category: ValidationCategory::Temporal,
                        code: "POLICY_INVALID_DATES".into(),
                        message: "Policy valid_until must be after valid_from".into(),
                        location: Some(format!("policies[{}]", i)),
                        severity: Severity::Serious,
                    });
                }
            }
        }
        
        errors
    }
}
```

## 8.5 Security Validation

```rust
impl ContractValidator {
    pub fn validate_security(&self, contract: &Contract) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Check for overly permissive policies
        for (i, policy) in contract.policies.iter().enumerate() {
            if matches!(policy.effect, Effect::Allow) 
                && matches!(policy.scope, PolicyScope::Global)
                && policy.conditions.is_empty() 
            {
                errors.push(ValidationError {
                    category: ValidationCategory::Security,
                    code: "OVERLY_PERMISSIVE".into(),
                    message: "Global allow without conditions is overly permissive".into(),
                    location: Some(format!("policies[{}]", i)),
                    severity: Severity::Warning,
                });
            }
        }
        
        // Check risk limits aren't too high
        for (i, limit) in contract.risk_limits.iter().enumerate() {
            if limit.threshold > self.max_safe_thresholds.get(&limit.category).copied().unwrap_or(f64::MAX) {
                errors.push(ValidationError {
                    category: ValidationCategory::Security,
                    code: "RISK_THRESHOLD_HIGH".into(),
                    message: format!("Risk threshold for {:?} exceeds recommended maximum", limit.category),
                    location: Some(format!("risk_limits[{}]", i)),
                    severity: Severity::Warning,
                });
            }
        }
        
        // Check for approval bypass
        for (i, policy) in contract.policies.iter().enumerate() {
            if matches!(policy.effect, Effect::Allow) && policy.priority > 1000 {
                errors.push(ValidationError {
                    category: ValidationCategory::Security,
                    code: "HIGH_PRIORITY_ALLOW".into(),
                    message: "High-priority allow policy could bypass safety checks".into(),
                    location: Some(format!("policies[{}]", i)),
                    severity: Severity::Warning,
                });
            }
        }
        
        errors
    }
}
```

---

# SPEC-09: CLI REFERENCE

> Enhanced CLI commands for full functionality.

## 9.1 Current Commands (Scaffold)

```
acon
├── contract
│   ├── create <name>
│   ├── get <id>
│   ├── list
│   ├── update <id>
│   └── delete <id>
├── policy
│   ├── add <contract-id>
│   ├── remove <policy-id>
│   └── list <contract-id>
├── risk
│   ├── set-limit <contract-id>
│   └── check <agent-id>
├── approval
│   ├── request <action>
│   ├── grant <request-id>
│   ├── deny <request-id>
│   └── pending
└── violation
    ├── report <details>
    └── list
```

## 9.2 Enhanced Commands

```
acon
├── contract
│   ├── create <name> [--parties <agents>] [--effective <date>] [--expires <date>]
│   ├── get <id> [--format json|yaml|table]
│   ├── list [--status <status>] [--party <agent>] [--expiring-within <duration>]
│   ├── update <id> [--name <name>] [--status <status>]
│   ├── delete <id> [--force]
│   ├── activate <id>
│   ├── suspend <id> [--reason <reason>]
│   ├── terminate <id> [--reason <reason>]
│   ├── validate <id> [--strict]
│   ├── export <id> [--format json|yaml|binary]
│   ├── import <file>
│   ├── diff <id1> <id2>
│   └── history <id>
│
├── policy
│   ├── add <contract-id> --name <name> --effect <allow|deny|require-approval>
│   │       [--scope <global|org|team|agent>] [--priority <n>]
│   │       [--condition <expr>] [--valid-from <date>] [--valid-until <date>]
│   ├── remove <policy-id>
│   ├── list [--contract <id>] [--agent <id>] [--effect <effect>]
│   ├── get <policy-id>
│   ├── evaluate <action> --agent <id> [--dry-run]
│   ├── conflicts [--contract <id>]
│   ├── simulate <action> --agent <id>
│   └── explain <policy-id>
│
├── risk
│   ├── limit
│   │   ├── set <contract-id> --category <cat> --threshold <n> [--window <duration>]
│   │   ├── get <limit-id>
│   │   ├── list [--contract <id>] [--category <cat>]
│   │   └── remove <limit-id>
│   ├── budget
│   │   ├── show <agent-id> [--category <cat>]
│   │   ├── reset <agent-id> [--category <cat>]
│   │   └── adjust <agent-id> --category <cat> --amount <n>
│   ├── check <agent-id> --action <action>
│   ├── forecast <agent-id> [--window <duration>]
│   └── history <agent-id> [--since <date>]
│
├── approval
│   ├── request <action> --agent <id> [--reason <reason>] [--deadline <date>]
│   ├── grant <request-id> [--condition <expr>] [--comment <text>]
│   ├── deny <request-id> --reason <reason>
│   ├── delegate <request-id> --to <approver-id>
│   ├── escalate <request-id>
│   ├── withdraw <request-id>
│   ├── pending [--approver <id>] [--agent <id>]
│   ├── get <request-id>
│   ├── history [--agent <id>] [--approver <id>] [--since <date>]
│   └── stats [--since <date>]
│
├── obligation
│   ├── create <contract-id> --description <text> [--deadline <date>]
│   │       [--obligor <agent>] [--recurrence <pattern>]
│   ├── get <obligation-id>
│   ├── list [--contract <id>] [--agent <id>] [--status <status>]
│   ├── upcoming [--agent <id>] [--within <duration>]
│   ├── overdue [--agent <id>]
│   ├── fulfill <obligation-id> [--evidence <file>]
│   ├── waive <obligation-id> --reason <reason>
│   └── remind <obligation-id>
│
├── violation
│   ├── report --contract <id> --action <action> --severity <level>
│   ├── get <violation-id>
│   ├── list [--contract <id>] [--agent <id>] [--severity <level>] [--status <status>]
│   ├── resolve <violation-id> --action <taken>
│   ├── escalate <violation-id>
│   ├── remediate <violation-id> --plan <file>
│   └── stats [--since <date>]
│
├── agent
│   ├── permissions <agent-id>
│   ├── can-do <agent-id> <action>
│   ├── contracts <agent-id>
│   ├── obligations <agent-id>
│   ├── violations <agent-id>
│   └── risk-profile <agent-id>
│
├── query
│   ├── contracts <expression>
│   ├── policies <expression>
│   ├── search <text>
│   └── impact <action> --agent <id>
│
├── file
│   ├── info <file.acon>
│   ├── validate <file.acon>
│   ├── repair <file.acon>
│   └── convert <input> --to <format>
│
└── server
    ├── start [--port <n>] [--stdio]
    ├── status
    └── stop
```

## 9.3 Example Usage

```bash
# Create a contract with spending limits
acon contract create "Engineering Budget Q1" \
    --parties agent:eng-bot,agent:finance-bot \
    --effective 2024-01-01 \
    --expires 2024-03-31

# Add a spending policy
acon policy add CONTRACT_ID \
    --name "Small Purchase" \
    --effect allow \
    --condition "action.type == 'purchase' AND amount <= 500" \
    --priority 10

# Add approval requirement for large purchases
acon policy add CONTRACT_ID \
    --name "Large Purchase Approval" \
    --effect require-approval \
    --condition "action.type == 'purchase' AND amount > 500" \
    --priority 20

# Set risk limits
acon risk limit set CONTRACT_ID \
    --category financial \
    --threshold 10000 \
    --window 30d

# Check if agent can perform action
acon agent can-do eng-bot "purchase:amount=750"

# Request approval
acon approval request "purchase:amount=750:vendor=acme" \
    --agent eng-bot \
    --reason "Server hardware upgrade"

# View pending approvals
acon approval pending --approver finance-lead

# Grant with condition
acon approval grant REQ_123 \
    --condition "vendor.verified == true" \
    --comment "Approved with vendor verification requirement"

# Check upcoming obligations
acon obligation upcoming --within 7d

# View agent's full permissions
acon agent permissions eng-bot

# Simulate action impact
acon query impact "purchase:amount=2000" --agent eng-bot
```

---

# SPEC-10: MCP SERVER

> Enhanced MCP tools for full contract functionality.

## 10.1 Current Tools (Scaffold - 22)

```
Contract Management:
- contract_create, contract_get, contract_list
- contract_update, contract_delete
- contract_activate, contract_suspend, contract_revoke

Policy Operations:
- policy_add, policy_remove, policy_evaluate
- policy_list, policy_conflicts

Risk Management:
- risk_limit_set, risk_limit_check, risk_budget_query

Approvals:
- approval_request, approval_grant, approval_deny
- approval_pending, approval_history

Violations:
- violation_report, violation_list, violation_resolve
```

## 10.2 Enhanced Tools (To Add)

### Policy Tools

```json
{
  "name": "policy_simulate",
  "description": "Simulate policy evaluation without executing",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "action": { "type": "object", "required": true },
    "context": { "type": "object", "required": false }
  },
  "returns": {
    "allowed": "boolean",
    "applicable_policies": "array",
    "risk_impact": "object",
    "approval_required": "boolean",
    "explanation": "string"
  }
}

{
  "name": "policy_explain",
  "description": "Get human-readable explanation of a policy",
  "parameters": {
    "policy_id": { "type": "string", "required": true }
  },
  "returns": {
    "summary": "string",
    "conditions_explained": "array",
    "examples": "array"
  }
}

{
  "name": "policy_compare",
  "description": "Compare two policies for conflicts or overlaps",
  "parameters": {
    "policy_id_1": { "type": "string", "required": true },
    "policy_id_2": { "type": "string", "required": true }
  },
  "returns": {
    "conflicts": "array",
    "overlaps": "array",
    "resolution": "string"
  }
}
```

### Risk Tools

```json
{
  "name": "risk_forecast",
  "description": "Forecast risk budget usage",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "category": { "type": "string", "required": false },
    "window": { "type": "string", "required": true, "description": "e.g., '7d'" }
  },
  "returns": {
    "current": "number",
    "projected": "number",
    "limit": "number",
    "will_exceed": "boolean",
    "exceed_at": "string or null"
  }
}

{
  "name": "risk_history",
  "description": "Get risk event history for an agent",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "category": { "type": "string", "required": false },
    "since": { "type": "string", "required": false }
  },
  "returns": {
    "events": "array",
    "total": "number"
  }
}
```

### Obligation Tools

```json
{
  "name": "obligation_create",
  "description": "Create a new obligation",
  "parameters": {
    "contract_id": { "type": "string", "required": true },
    "description": { "type": "string", "required": true },
    "obligor": { "type": "string", "required": true },
    "deadline": { "type": "string", "required": false },
    "recurrence": { "type": "string", "required": false }
  }
}

{
  "name": "obligation_upcoming",
  "description": "Get obligations due within a window",
  "parameters": {
    "agent_id": { "type": "string", "required": false },
    "window": { "type": "string", "required": true }
  },
  "returns": {
    "obligations": "array"
  }
}

{
  "name": "obligation_fulfill",
  "description": "Mark an obligation as fulfilled",
  "parameters": {
    "obligation_id": { "type": "string", "required": true },
    "evidence": { "type": "string", "required": false }
  }
}

{
  "name": "obligation_dependencies",
  "description": "Get obligation dependency graph",
  "parameters": {
    "obligation_id": { "type": "string", "required": true }
  },
  "returns": {
    "depends_on": "array",
    "blocks": "array"
  }
}
```

### Agent Tools

```json
{
  "name": "agent_permissions",
  "description": "Get comprehensive permissions for an agent",
  "parameters": {
    "agent_id": { "type": "string", "required": true }
  },
  "returns": {
    "allowed_actions": "array",
    "denied_actions": "array",
    "conditional_actions": "array",
    "risk_budgets": "object",
    "active_contracts": "array",
    "pending_obligations": "array"
  }
}

{
  "name": "agent_can_do",
  "description": "Check if agent can perform specific action",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "action": { "type": "object", "required": true }
  },
  "returns": {
    "allowed": "boolean",
    "requires_approval": "boolean",
    "reason": "string",
    "blocking_policies": "array",
    "risk_impact": "object"
  }
}

{
  "name": "agent_impact",
  "description": "Analyze impact of an action before execution",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "action": { "type": "object", "required": true }
  },
  "returns": {
    "policy_result": "object",
    "risk_change": "object",
    "obligations_triggered": "array",
    "potential_violations": "array",
    "recommendations": "array"
  }
}
```

### Query Tools

```json
{
  "name": "query_contracts",
  "description": "Search contracts with complex criteria",
  "parameters": {
    "status": { "type": "string", "required": false },
    "party": { "type": "string", "required": false },
    "expiring_within": { "type": "string", "required": false },
    "search": { "type": "string", "required": false },
    "limit": { "type": "number", "required": false }
  },
  "returns": {
    "contracts": "array",
    "total": "number"
  }
}

{
  "name": "query_violations",
  "description": "Search violations with filtering",
  "parameters": {
    "agent_id": { "type": "string", "required": false },
    "severity": { "type": "string", "required": false },
    "status": { "type": "string", "required": false },
    "since": { "type": "string", "required": false }
  },
  "returns": {
    "violations": "array",
    "by_severity": "object",
    "by_category": "object"
  }
}
```

### Hydra Integration Tools

```json
{
  "name": "hydra_evaluate",
  "description": "Evaluate action for Hydra execution gate",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "action": { "type": "object", "required": true },
    "context": { "type": "object", "required": false }
  },
  "returns": {
    "gate_decision": "string",
    "policies_applied": "array",
    "risk_assessment": "object",
    "approval_status": "object",
    "obligations_checked": "array",
    "proceed": "boolean",
    "conditions": "array"
  }
}

{
  "name": "hydra_register_action",
  "description": "Register completed action for tracking",
  "parameters": {
    "agent_id": { "type": "string", "required": true },
    "action": { "type": "object", "required": true },
    "result": { "type": "object", "required": true }
  },
  "returns": {
    "risk_updated": "boolean",
    "violations_detected": "array",
    "obligations_fulfilled": "array"
  }
}
```

## 10.3 MCP Resources

```json
{
  "resources": [
    {
      "uri": "contract://contracts",
      "name": "All Contracts",
      "description": "List all contracts in the system"
    },
    {
      "uri": "contract://contracts/{id}",
      "name": "Contract Details",
      "description": "Get specific contract by ID"
    },
    {
      "uri": "contract://contracts/active",
      "name": "Active Contracts",
      "description": "List all active contracts"
    },
    {
      "uri": "contract://policies/{contract_id}",
      "name": "Contract Policies",
      "description": "List policies for a contract"
    },
    {
      "uri": "contract://agent/{agent_id}/permissions",
      "name": "Agent Permissions",
      "description": "Get agent's current permissions"
    },
    {
      "uri": "contract://agent/{agent_id}/risk",
      "name": "Agent Risk Profile",
      "description": "Get agent's risk budgets and history"
    },
    {
      "uri": "contract://approvals/pending",
      "name": "Pending Approvals",
      "description": "List pending approval requests"
    },
    {
      "uri": "contract://obligations/upcoming",
      "name": "Upcoming Obligations",
      "description": "List obligations due soon"
    },
    {
      "uri": "contract://violations/recent",
      "name": "Recent Violations",
      "description": "List recent violations"
    }
  ]
}
```

## 10.4 MCP Prompts

```json
{
  "prompts": [
    {
      "name": "create_contract",
      "description": "Help user create a new contract with appropriate policies",
      "arguments": [
        { "name": "purpose", "required": true },
        { "name": "parties", "required": true },
        { "name": "constraints", "required": false }
      ]
    },
    {
      "name": "review_permissions",
      "description": "Review and explain an agent's current permissions",
      "arguments": [
        { "name": "agent_id", "required": true }
      ]
    },
    {
      "name": "troubleshoot_denied",
      "description": "Help understand why an action was denied",
      "arguments": [
        { "name": "agent_id", "required": true },
        { "name": "action", "required": true }
      ]
    },
    {
      "name": "risk_analysis",
      "description": "Analyze risk profile and provide recommendations",
      "arguments": [
        { "name": "agent_id", "required": true }
      ]
    }
  ]
}
```

---

# SUMMARY

## Scaffold vs Full Implementation

| Component | Scaffold | Full v0.2.0 |
|-----------|----------|-------------|
| **Policy** | Simple allow/deny | Hierarchical, temporal, contextual, conflict resolution |
| **Risk** | Static thresholds | Dynamic scoring, budgets, decay, forecasting |
| **Approval** | Single approver | Multi-party, escalation, delegation, conditional |
| **Obligation** | Record only | Deadlines, dependencies, verification, penalties |
| **Violation** | Log only | Detection, classification, response, remediation |
| **Query** | Basic CRUD | Semantic queries, impact analysis, search |
| **Integration** | Placeholders | Deep Hydra/Identity/Time/Memory |

## Implementation Priority

1. **Policy Engine** (SPEC-05) — Core of everything
2. **Expression Engine** (SPEC-03) — Required by policies
3. **Risk Engine** (SPEC-03, SPEC-05) — Required for risk limits
4. **Enhanced Indexes** (SPEC-07) — Required for queries
5. **Approval Workflows** (SPEC-03) — Required for RequireApproval
6. **Obligation Tracking** (SPEC-03) — Required for obligations
7. **Violation Detection** (SPEC-03) — Required for enforcement
8. **Hydra Integration** — Required for execution gate
9. **Enhanced CLI** (SPEC-09) — Developer experience
10. **Enhanced MCP** (SPEC-10) — Agent integration

## Estimated Effort

| Phase | Specs | Effort |
|-------|-------|--------|
| Data Structures | SPEC-03 | 8-12 hours |
| File Format | SPEC-04 | 4-6 hours |
| Policy Engine | SPEC-05 | 12-16 hours |
| Query Engine | SPEC-06 | 8-12 hours |
| Indexes | SPEC-07 | 6-8 hours |
| Validation | SPEC-08 | 4-6 hours |
| CLI Enhancement | SPEC-09 | 6-8 hours |
| MCP Enhancement | SPEC-10 | 8-12 hours |
| **Total** | | **56-80 hours** |

---

**AgenticContract: Governed autonomy for AI agents.**

*Build the policies. Enforce the boundaries. Trust the system.*

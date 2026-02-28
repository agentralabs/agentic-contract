---
status: stable
title: Playbooks - Agent Integration
description: Step-by-step playbooks for integrating AgenticContract with AI agents
---

# Playbooks: Agent Integration

Practical step-by-step workflows for common governance scenarios. Each playbook includes the tools to call, expected responses, and error handling.

## Playbook 1: Basic Policy Enforcement

Set up a policy that prevents agents from performing dangerous actions.

### Setup

```
1. policy_add(
     label: "No production deploys without approval",
     scope: "global",
     action: "require_approval",
     tags: ["deployment", "production"]
   )
   -> Returns: policy_id
```

### Runtime Check

```
2. Before any deploy action:
   policy_check(action: "deploy to production")
   -> Returns: { action: "require_approval", matching_policy: "..." }

3. If require_approval:
   approval_request(
     rule_id: <rule_id>,
     action_description: "deploy v2.1.0 to production",
     requestor: "deploy-agent"
   )
   -> Returns: { request_id: "...", status: "pending" }

4. Wait for human operator:
   approval_decide(
     request_id: <request_id>,
     decision: "approve",
     decider: "ops-lead",
     reason: "Reviewed changes, ready for production"
   )
   -> Returns: { decision_id: "...", status: "approved" }

5. On approval: proceed with deploy

6. On deny:
   violation_report(
     description: "Deploy denied by ops-lead: not ready",
     severity: "info",
     actor: "deploy-agent"
   )
```

### Error Handling

- If `policy_check` returns `deny`: Stop immediately and report violation
- If approval times out: Log as expired and re-request if needed
- If no matching policy: Default action is `allow`

## Playbook 2: Budget-Controlled Agent

Enforce spending limits on an agent that makes API calls.

### Setup

```
1. risk_limit_set(
     label: "API spend",
     max_value: 100.0,
     limit_type: "budget"
   )
   -> Returns: limit_id

2. risk_limit_set(
     label: "API calls per hour",
     max_value: 1000,
     limit_type: "rate",
     window_secs: 3600
   )
   -> Returns: limit_id
```

### Runtime Check

```
3. Before each API call:
   risk_limit_check(label: "API spend", amount: 0.05)
   -> Returns: { would_exceed: false, remaining: 57.50, usage_ratio: 0.425 }

4. If within budget: proceed with API call

5. After successful call: update limit
   (The check automatically tracks usage)

6. If would_exceed:
   violation_report(
     description: "API budget would be exceeded: $100.00 limit, $0.05 requested",
     severity: "critical",
     actor: "data-agent"
   )
```

### Monitoring

```
7. Periodically check budget health:
   risk_limit_list()
   -> Returns all limits with current values and usage ratios

8. Alert when usage_ratio > 0.8 (80% consumed):
   contract_context_log(
     intent: "Budget approaching limit",
     topic: "risk-monitoring"
   )
```

## Playbook 3: Compliance Obligations

Track mandatory reporting obligations with deadlines.

### Setup

```
1. obligation_add(
     label: "Submit weekly compliance report",
     description: "Generate and submit the weekly compliance summary to the audit team",
     assignee: "compliance-agent",
     deadline: "2026-03-07T17:00:00Z"
   )
   -> Returns: obligation_id

2. obligation_add(
     label: "Review access logs",
     description: "Review and flag anomalous access patterns",
     assignee: "security-agent",
     deadline: "2026-03-01T09:00:00Z"
   )
   -> Returns: obligation_id
```

### Monitoring

```
3. Periodically (e.g., daily):
   obligation_check()
   -> Returns: [{ id: "...", label: "Review access logs", status: "overdue" }]

4. For each overdue obligation:
   violation_report(
     description: "Missed deadline: Review access logs (due 2026-03-01)",
     severity: "warning",
     actor: "security-agent"
   )
```

### Completion

```
5. When obligation is fulfilled:
   obligation_check(assignee: "compliance-agent", status: "pending")
   (Mark fulfilled through the engine)

6. For exceptions:
   (Waive obligation with documented reason)
```

## Playbook 4: Multi-Agent Governance

Set up governance for multiple agents working together on a project.

### Agent-Specific Policies

```
1. Per-agent capability boundaries:
   policy_add(label: "data-agent: read-only database", scope: "agent", action: "deny")
   policy_add(label: "deploy-agent: staging only", scope: "agent", action: "deny")
   policy_add(label: "review-agent: no code modifications", scope: "agent", action: "audit_only")
```

### Shared Risk Limits

```
2. Global resource constraints:
   risk_limit_set(label: "Total API budget", max_value: 500.0, limit_type: "budget")
   risk_limit_set(label: "Database queries per hour", max_value: 10000, limit_type: "rate", window_secs: 3600)
```

### Cross-Agent Approval

```
3. High-stakes actions require approval from a different agent or human:
   approval_request(
     rule_id: <production-deploy-rule>,
     action_description: "deploy-agent wants to deploy to staging",
     requestor: "deploy-agent"
   )

4. Review agent or human decides:
   approval_decide(
     request_id: <request_id>,
     decision: "approve",
     decider: "review-agent",
     reason: "Code review passed, tests green"
   )
```

### Audit

```
5. Regular compliance review:
   violation_list(severity: "warning")
   -> Returns all warnings and above across all agents

6. Per-agent violation breakdown:
   violation_list(actor: "data-agent")
   violation_list(actor: "deploy-agent")
```

## Playbook 5: Agentra Ecosystem Integration

Combine AgenticContract with other Agentra sisters for comprehensive governance.

### With AgenticIdentity

```
1. Link approval decisions to identity trust:
   - approval_decide creates a receipt
   - Receipt is signed by the approver's identity
   - Trust grant verification confirms authority

2. Pattern:
   identity: action_sign(action: "Approved production deploy")
   contract: approval_decide(decision: "approve", decider: <identity_id>)
```

### With AgenticMemory

```
3. Store governance decisions in memory:
   - Each violation_report creates a memory node (type: "decision")
   - Agents can query past decisions: "Why was this action denied last time?"

4. Pattern:
   contract: violation_report(description: "Budget exceeded")
   memory: memory_add(content: "Budget exceeded: $112.50 vs $100 limit", event_type: "decision")
```

### With AgenticTime

```
5. Temporal governance analysis:
   - All contract events carry timestamps
   - Query patterns: "violations in the last 24 hours", "approval response times"

6. Pattern:
   contract: violation_list()
   time: Filter by temporal range for trend analysis
```

### With AgenticCodebase

```
7. Code-aware conditions:
   - condition_add with type "dependency"
   - Expression references code analysis results
   - Deploy only if code review score > threshold

8. Pattern:
   codebase: analysis_log(intent: "Pre-deploy code review")
   contract: condition_evaluate(condition_id: <code-review-condition>)
```

### With AgenticVision

```
9. Visual compliance monitoring:
   - Screenshot shows sensitive data in UI
   - Triggers policy check for data exposure

10. Pattern:
    vision: observation_log(intent: "Checking UI for PII exposure")
    contract: policy_check(action: "display user PII in dashboard")
```

## Common Patterns

### Retry with Escalation

When an action is denied, retry with escalation:

```
1. policy_check(action: "delete user data") -> "deny"
2. approval_request(action_description: "Emergency data deletion required")
3. Wait for senior approval
4. On approval: proceed
5. Log: contract_context_log(intent: "Emergency data deletion approved by CTO")
```

### Graduated Enforcement

Start with audit, escalate to enforcement:

```
Week 1: policy_add(action: "audit_only")    # Monitor only
Week 2: Review violation_list()              # Analyze patterns
Week 3: policy_add(action: "require_approval") # Gate high-risk
Week 4: policy_add(action: "deny")           # Full enforcement
```

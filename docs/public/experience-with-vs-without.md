---
status: stable
title: Experience With vs Without
description: What changes when you add AgenticContract to your AI agent workflow
---

# Experience: With vs Without AgenticContract

## Without AgenticContract

When AI agents operate without structured governance:

- **No policy enforcement**: Agents can take any action without constraints. A coding agent might delete production data, or a research agent might make unlimited API calls.

- **No risk visibility**: There is no way to track how close an agent is to exceeding budgets, rate limits, or other quantitative boundaries. You only discover the problem after the damage.

- **Ad-hoc approvals**: High-stakes actions are gated by chat messages ("Hey, can I deploy?"). There is no audit trail, no timeout handling, and no structured workflow.

- **Forgotten obligations**: Recurring tasks (weekly reports, compliance checks) rely on agent memory or external reminders. Deadlines are missed without detection.

- **Silent violations**: Policy breaches go undetected until a human notices the consequences. There is no systematic recording, severity classification, or alerting.

- **Scattered audit trail**: Governance events are spread across chat logs, terminal output, and application logs. Reconstructing what happened and why requires manual investigation.

## With AgenticContract

When AI agents are governed by AgenticContract:

- **Automated policy enforcement**: Every action can be checked against named policies before execution. Deny, require_approval, and audit_only responses are handled consistently.

- **Real-time risk tracking**: Quantitative limits (budgets, rates, thresholds) are tracked continuously. Agents know exactly how much headroom remains before hitting a limit.

- **Structured approval workflows**: High-stakes actions go through a formal request/decide cycle. Every approval has a decider, a reason, and a timestamp. Timeouts prevent stuck requests.

- **Deadline-aware obligations**: Obligations are tracked with assignees and deadlines. The engine detects overdue items and can trigger violation reports automatically.

- **Classified violation records**: Every breach is recorded with severity (info/warning/critical/fatal), actor, timestamp, and optional context. The violation log is the primary audit artifact.

- **Single-file audit trail**: All governance data lives in one `.acon` file. The entire history of policies, approvals, obligations, and violations is portable and inspectable.

## Key Differences

| Aspect | Without | With |
|--------|---------|------|
| Policy enforcement | Manual, inconsistent | Automated, precedence-based |
| Risk awareness | None until failure | Real-time tracking with usage ratios |
| Approval process | Ad-hoc chat messages | Structured rule/request/decision workflow |
| Obligation tracking | Spreadsheets or memory | Engine-managed with deadline detection |
| Violation detection | Post-incident investigation | Real-time reporting with severity classification |
| Audit trail | Scattered across logs | Single `.acon` binary file |
| Cross-agent governance | Per-agent custom logic | Unified engine with scoped policies |
| Cost tracking | Manual spreadsheet | Automatic budget/rate limits |

## Migration Path

Moving from ungoverned agents to AgenticContract can be done gradually:

1. **Week 1 — Observe**: Add policies with `audit_only` action. This logs all matched actions without blocking anything.

2. **Week 2 — Analyze**: Review `violation_list()` to understand which actions agents take and how often. Identify high-risk patterns.

3. **Week 3 — Gate**: Change high-risk policies to `require_approval`. This forces human review without fully blocking agents.

4. **Week 4 — Enforce**: Change validated policies to `deny` for actions that should never happen. Keep `require_approval` for actions that need case-by-case judgment.

5. **Ongoing**: Add risk limits for quantitative constraints, obligations for recurring tasks, and conditions for prerequisite checks.

## Real-World Impact

### Before: Uncontrolled API Spending

An agent making LLM API calls with no budget tracking spent $847 in one session. The user only discovered this when reviewing their monthly bill.

### After: Budget-Limited Agent

```
risk_limit_set("API spend per session", max=50.0, type="budget")
```

The agent is blocked at $50 with a clear violation report. The user can review and approve additional budget if needed.

### Before: Accidental Production Deploy

A coding agent deployed untested code to production because no policy distinguished staging from production environments.

### After: Approval-Gated Deploys

```
policy_add("Production deploy requires approval", scope="global", action="require_approval")
```

Every production deploy goes through a human-reviewed approval workflow. The deploy agent cannot bypass the check.

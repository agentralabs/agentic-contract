---
status: stable
title: AgenticContract Overview
description: Policy engine for AI agents — enforceable rules, risk limits, approval gates, obligation tracking, and violation detection in a single .acon file
---

# AgenticContract Overview

AgenticContract is a policy engine for AI agents. It models policies, risk limits, approvals, obligations, and violations in a single binary `.acon` file.

## Why AgenticContract

Every AI agent today is ungoverned. Claude can exceed your API budget. GPT can bypass approval gates. Your copilot can violate compliance rules without anyone noticing. Every action happens without oversight.

AgenticContract gives your agent a governance engine — behavior constraints, budget enforcement, approval gates, and breach tracking — all structured, all queryable in microseconds.

## Core Capabilities

| Capability | What It Does |
|------------|-------------|
| **Policies** | Define allow/deny rules scoped to global, session, or task levels |
| **Risk Limits** | Enforce rate, budget, and threshold limits with real-time tracking |
| **Approval Gates** | Route sensitive actions through configurable approval workflows |
| **Obligations** | Track time-bound compliance requirements with deadline enforcement |
| **Violation Detection** | Record and query policy breaches with severity and actor attribution |
| **Conditions** | Evaluate boolean conditions that gate policy activation |

## How It Works

1. **Create** a `.acon` contract artifact
2. **Add** policies, limits, approval rules, and obligations
3. **Check** every agent action against the contract in microseconds
4. **Enforce** limits, route approvals, and track violations automatically
5. **Query** the full governance state at any time

## Install

```bash
curl -fsSL https://agentralabs.tech/install/contract | bash
```

Or via package managers:

```bash
cargo install agentic-contract-cli agentic-contract-mcp
pip install agentic-contract
npm install @agenticamem/contract
```

## File Format

AgenticContract uses the `.acon` binary format — portable, self-contained, and designed for 20-year durability. One file holds your entire governance state: policies, limits, approvals, obligations, violations, and audit history.

## Works With

- **AgenticMemory** — memory-aware policy decisions
- **AgenticVision** — visual observation governance
- **AgenticCodebase** — code change approval gates
- **AgenticIdentity** — identity-bound policy enforcement
- **AgenticTime** — temporal obligation tracking

## See Also

- [Quickstart](quickstart.md) — get started in 5 minutes
- [Concepts](concepts.md) — core domain model
- [CLI Reference](cli-reference.md) — full command documentation
- [MCP Tools](mcp-tools.md) — tool reference for MCP integration
- [Integration Guide](integration-guide.md) — embedding in your agent

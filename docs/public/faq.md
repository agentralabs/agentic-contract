---
status: stable
title: FAQ
description: Frequently asked questions about AgenticContract
---

# FAQ

## General

**What is AgenticContract?**

AgenticContract is a policy engine for AI agents. It provides six governance primitives — policies, risk limits, approvals, conditions, obligations, and violations — stored in a single `.acon` binary file. It is part of the Agentra ecosystem.

**Do I need the other Agentra sisters to use AgenticContract?**

No. AgenticContract is fully standalone. It works independently via MCP, CLI, FFI, or the Python SDK. Integration with other sisters (memory, identity, time, vision, codebase) is optional and adds cross-referencing capabilities.

**What file format does AgenticContract use?**

The `.acon` binary format with `ACON` magic bytes (0x41434F4E) and BLAKE3 checksums for integrity verification. See the [file format documentation](file-format.md) for the complete specification.

**How does AgenticContract compare to OPA or Cedar?**

AgenticContract is a local-first, single-file policy engine designed for AI agent governance. OPA uses the Rego language over HTTP; Cedar uses a custom policy language. AgenticContract uses text-matching with precedence ordering — simpler and 10-100x faster (49 ns vs 0.5-5 ms) because there is no network round-trip or language interpretation. The trade-off is that AgenticContract does not support complex policy logic.

**Is AgenticContract suitable for production use?**

AgenticContract is designed for production agent governance. The `.acon` format includes BLAKE3 checksums, the engine has 288 tests (including stress tests), and all core operations complete in under 1 microsecond. However, it is a local-first tool — it does not provide distributed consensus or multi-node coordination.

## Installation

**What are the system requirements?**

| Component | Requirement |
|-----------|------------|
| macOS | 12.0+ (Monterey), x86_64 or aarch64 |
| Linux | glibc 2.31+, x86_64 or aarch64 |
| Python | 3.8+ (for Python SDK) |
| Rust | 1.75+ (for building from source) |
| Node.js | 18+ (for npm package) |

No runtime dependencies are required for the pre-built binary.

**How do I install AgenticContract?**

The quickest method:

```bash
curl -fsSL https://agentralabs.tech/install/contract | bash
```

Or via package managers:

```bash
pip install agentic-contract        # Python SDK
cargo install agentic-contract-cli  # Rust CLI
npm install @agenticamem/contract   # npm package
```

**What install profiles are available?**

Three profiles: `desktop` (configures Claude Desktop, Cursor, Windsurf), `terminal` (configures Claude Code CLI), and `server` (for remote deployment with `AGENTIC_TOKEN` auth).

**The binary isn't found after installation. What do I do?**

Ensure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Add this to `~/.zshrc` or `~/.bashrc` for persistence.

## Usage

**How are policies evaluated?**

Policies are matched against action descriptions using case-insensitive text matching. When multiple policies match, the most restrictive action wins: `deny` > `require_approval` > `audit_only` > `allow`.

**Can I have multiple contract files?**

Yes. Use the `--path` flag or `ACON_PATH` environment variable to specify which contract file to use. Each file is an independent governance unit with no shared state.

**How do risk limits work?**

Risk limits track a current value against a maximum. Four limit types are supported: `rate` (per time window), `threshold` (absolute maximum), `budget` (cumulative spending), and `count` (simple counter). When a check would exceed the limit, the action should be blocked.

**What happens when an approval times out?**

If an approval rule has a `timeout_secs` value and the request remains pending beyond that duration, it transitions to `expired` status. The requesting agent should re-request or escalate.

**How do I handle overdue obligations?**

Call `obligation_check()` periodically. It returns obligations where the deadline has passed and status is still `pending`. For each overdue item, you can report a violation and optionally waive the obligation.

**Can policies have expiration dates?**

Yes. Use the `expires_at` field when creating a policy. Expired policies are not evaluated during `policy_check`. The engine does not automatically remove expired policies — they remain in the store for audit purposes.

**What is the difference between audit_only and allow?**

Both permit the action to proceed. `audit_only` explicitly records that the action matched a policy and was logged. `allow` is the default when no policy matches — it means "no policy had an opinion about this action."

## MCP

**What MCP protocol version is supported?**

Protocol version `2024-11-05`.

**How many MCP tools are available?**

22 core tools covering all six entity types plus context logging and statistics. Additionally, 16 advanced tools provide advanced capabilities (risk prophecy, violation precognition, contract simulation, etc.).

**How are errors reported in MCP?**

Tool execution errors are returned with `isError: true` in the MCP response (not as JSON-RPC errors). Unknown tools receive JSON-RPC error code `-32803` (TOOL_NOT_FOUND). Protocol errors use standard JSON-RPC codes.

**Can I use AgenticContract with Claude Desktop?**

Yes. Install via the auto-installer or manually add the MCP configuration:

```json
{
  "mcpServers": {
    "agentic-contract": {
      "command": "agentic-contract-mcp",
      "args": ["serve"]
    }
  }
}
```

**Does the MCP server require authentication?**

Only in server profile. Set the `AGENTIC_TOKEN` environment variable. In desktop and terminal profiles, the server runs locally without authentication.

## Data and Security

**Is my contract data encrypted?**

Not currently. The `.acon` file stores data in a binary format with BLAKE3 checksums for integrity but without encryption. Keep contract files in secure locations and use file-system permissions for access control.

**How large can a contract file get?**

File size scales linearly: ~50 KB for 100 entities, ~500 KB for 1,000, ~5 MB for 10,000. For typical agent governance (10-100 policies, a few dozen limits and obligations), the file is under 100 KB.

**Can I back up and restore contract files?**

Yes. The `.acon` file is a single binary file. Copy it to back up, restore by copying it back. The BLAKE3 checksum verifies integrity on load.

## Cross-Sister Integration

**How does AgenticContract work with AgenticIdentity?**

Approval decisions link to identity trust grants via receipts. When an approval is granted, the identity system can verify who approved it and under what authority.

**How does AgenticContract work with AgenticMemory?**

Contract decisions (violations, approvals) can be stored as memory nodes. This enables agents to recall past governance decisions: "Why was this action denied last time?"

**Can I use AgenticContract without the Agentra SDK?**

The MCP server and CLI work independently. The core Rust library depends on `agentic-sdk` for trait implementations, but the Python SDK and FFI bindings have no SDK dependency.

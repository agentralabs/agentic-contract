---
status: stable
title: Architecture
description: AgenticContract system architecture
---

# Architecture

AgenticContract is a policy engine for AI agents, built as a 4-crate Rust workspace with Python SDK, FFI bindings, and MCP server.

## Workspace Structure

```
agentic-contract/
├── crates/
│   ├── agentic-contract/           # Core library
│   │   ├── src/
│   │   │   ├── lib.rs              # Module declarations + ContractId type
│   │   │   ├── policy.rs           # Policy, PolicyScope, PolicyAction, PolicyStatus
│   │   │   ├── risk_limit.rs       # RiskLimit, LimitType
│   │   │   ├── approval.rs         # ApprovalRule, ApprovalRequest, ApprovalDecision
│   │   │   ├── condition.rs        # Condition, ConditionType, ConditionStatus
│   │   │   ├── obligation.rs       # Obligation, ObligationStatus
│   │   │   ├── violation.rs        # Violation, ViolationSeverity
│   │   │   ├── contract_engine.rs  # ContractEngine (facade)
│   │   │   ├── contracts.rs        # SDK trait implementations (8 traits)
│   │   │   ├── file_format.rs      # .acon binary format, FileHeader, ContractFile
│   │   │   └── error.rs            # ContractError, ContractResult
│   │   ├── benches/
│   │   │   └── contract_bench.rs   # Criterion benchmarks
│   │   └── tests/
│   │       └── stress_tests.rs     # 288 stress tests
│   ├── agentic-contract-mcp/       # MCP server
│   │   ├── src/
│   │   │   ├── main.rs             # Entry point (serve/info)
│   │   │   ├── server.rs           # JSON-RPC loop
│   │   │   ├── stdio.rs            # Content-Length framing transport
│   │   │   ├── tools.rs            # 22 core + 16 invention tools
│   │   │   ├── resources.rs        # 12 resources (acon:// URIs)
│   │   │   ├── prompts.rs          # 4 prompts
│   │   │   └── greeting.rs         # Startup banner
│   │   └── tests/
│   │       └── edge_cases_inventions.rs
│   ├── agentic-contract-cli/       # CLI binary (acon)
│   │   └── src/
│   │       ├── main.rs             # Clap parser, 9 subcommands
│   │       └── commands/           # Subcommand implementations
│   └── agentic-contract-ffi/       # FFI bindings
│       ├── src/lib.rs              # C-compatible API
│       └── include/
│           └── agentic_contract.h  # C header
├── python/                         # Python SDK
│   └── src/agentic_contract/
│       └── __init__.py             # ContractEngine class
├── scripts/                        # Install + CI scripts
├── paper/                          # Research paper
└── assets/                         # SVG assets
```

## Crate Responsibilities

### `agentic-contract` (Core)

The core library defines all domain types and the `ContractEngine` facade. It has no network dependencies and no async runtime.

**Key modules**:
- 6 domain modules (policy, risk_limit, approval, condition, obligation, violation)
- `contract_engine.rs`: Single entry point wrapping all entity operations
- `file_format.rs`: Binary `.acon` format (ACON magic, BLAKE3 checksum)
- `contracts.rs`: SDK trait implementations
- `error.rs`: Typed error enum with 12 variants

**Approximate size**: 4,552 lines (excluding tests)

### `agentic-contract-mcp` (MCP Server)

JSON-RPC server communicating over stdio using Content-Length framing. Implements MCP protocol version `2024-11-05`.

**Capabilities**: 22 core tools + 16 invention tools, 12 resources, 4 prompts

**Key behaviors**:
- Unknown method: JSON-RPC error `-32601`
- Unknown tool: JSON-RPC error `-32803` (MCP Quality Standard)
- Tool execution errors: `isError: true` (not JSON-RPC errors)
- Server auth: `AGENTIC_TOKEN` required in server mode

**Approximate size**: 1,979 lines

### `agentic-contract-cli` (CLI)

Command-line interface installed as `acon`. Uses Clap for argument parsing.

**Subcommands**: policy, limit, approval, obligation, violation, stats, info, install, serve

**Approximate size**: 1,625 lines

### `agentic-contract-ffi` (FFI)

C-compatible shared library for embedding in non-Rust applications. Exposes `#[no_mangle] pub unsafe extern "C"` functions.

**Functions**: `acon_open`, `acon_create`, `acon_close`, `acon_save`, `acon_stats`, `acon_policy_add`, `acon_policy_check`

**Approximate size**: 226 lines

## Data Model

The contract engine manages 8 entity types stored in a single `.acon` file:

```
┌─────────────────────────────────────────────────┐
│                 ContractEngine                   │
│                                                  │
│  ┌──────────┐  ┌───────────┐  ┌──────────────┐  │
│  │ Policies │  │Risk Limits│  │Approval Rules│  │
│  └──────────┘  └───────────┘  └──────────────┘  │
│                                                  │
│  ┌──────────────────┐  ┌───────────────────┐     │
│  │Approval Requests │  │Approval Decisions │     │
│  └──────────────────┘  └───────────────────┘     │
│                                                  │
│  ┌───────────┐  ┌────────────┐  ┌───────────┐   │
│  │Conditions │  │Obligations │  │Violations  │   │
│  └───────────┘  └────────────┘  └───────────┘   │
│                                                  │
│  ┌─────────────────────────────────────────┐     │
│  │          .acon Binary File              │     │
│  │  ACON magic | header | JSON entities    │     │
│  └─────────────────────────────────────────┘     │
└─────────────────────────────────────────────────┘
```

## Data Flow

```
User/Agent Request
       |
       v
┌──────────────┐
│  MCP Server  │<---- JSON-RPC over stdio
│  or CLI      │
│  or Python   │
│  or FFI      │
└──────┬───────┘
       |
       v
┌──────────────┐
│ContractEngine│<---- In-memory entity store
└──────┬───────┘
       |
       v
┌──────────────┐
│  .acon File  │<---- Binary persistence
│  (BLAKE3)    │
└──────────────┘
```

All four interfaces (MCP, CLI, Python, FFI) delegate to the same `ContractEngine`, which manages the in-memory entity store and persists to the `.acon` file.

## SDK Integration

AgenticContract implements all 8 agentic-sdk traits:

| Trait | Purpose | Implementation |
|-------|---------|----------------|
| `Sister` | Type identity | `SisterType::Contract`, extension `.acon` |
| `SessionManagement` | Session lifecycle | Contract scoped to sessions |
| `Grounding` | Claim verification | BM25 word-overlap search over labels |
| `Queryable` | Query interface | List, search, recent, get |
| `FileFormatReader` | Binary read | `.acon` deserialization + checksum |
| `FileFormatWriter` | Binary write | `.acon` serialization + BLAKE3 |
| `EventEmitter` | Event system | Using SDK's EventManager |
| `ReceiptIntegration` | Audit receipts | Receipts for approvals + violations |

## Cross-Sister Integration

AgenticContract integrates with the 5 other Agentra sisters:

- **AgenticIdentity**: Approval decisions link to identity trust grants via receipts
- **AgenticMemory**: Contract decisions stored as memory nodes for recall
- **AgenticTime**: All events timestamped for temporal analysis
- **AgenticVision**: Visual observations trigger policy evaluations
- **AgenticCodebase**: Code analysis feeds condition evaluations

## Runtime Isolation

Each `.acon` file is a self-contained governance unit. Multiple contract files can coexist:

- Different agents can use different contract files
- Different projects can maintain separate governance policies
- The `ACON_PATH` environment variable or `--path` flag selects which file to use
- No shared state between contract files

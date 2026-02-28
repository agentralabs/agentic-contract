# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-27

### Added

#### Core Engine
- Policy engine with typed policies: allow, deny, require_approval, audit_only
- Risk limit tracking with 4 types: rate, threshold, budget, count
- Approval workflow: rules, requests, decisions with full audit trail
- Condition evaluation for prerequisite checks
- Obligation tracking with deadline-aware overdue detection
- Violation recording with 4 severity levels: info, warning, critical, fatal
- ContractStats for real-time governance state summary

#### File Format
- Binary `.acon` format with `b"ACON"` magic bytes
- Fixed-size records for O(1) entity access
- BLAKE3 checksum integrity verification
- Header with version, flags, entity counts, timestamps

#### MCP Server (38 tools)
- 22 core governance tools (contract, policy, risk, approval, condition, obligation, violation)
- 16 invention tools: omniscience, prophecy, telepathy, clairvoyance, precognition, crystallize, DNA extraction, trust gradients, collective contracts, temporal contracts, inheritance, smart escalation, violation archaeology, simulation, federated governance, self-healing contracts
- 4 MCP resources with acon:// URI scheme
- 4 MCP prompts: contract_review, contract_setup, contract_audit, contract_risk_assessment
- JSON-RPC 2.0 over stdio transport
- Content-Length framing with 8 MiB limit
- Server mode authentication via AGENTIC_TOKEN

#### CLI (acon)
- `acon policy` -- add, check, list policies with scope and action filters
- `acon limit` -- set, check, list risk limits with type support
- `acon approval` -- rule, request, decide, list approval workflow
- `acon obligation` -- add, check, fulfill, list with deadline and status filters
- `acon violation` -- report, list with severity filters
- `acon stats` -- full governance state summary as JSON
- `acon info` -- file metadata inspection
- Global `--path` flag and `ACON_PATH` environment variable

#### SDK Integration
- 8 agentic-sdk trait implementations: Sister, SessionManagement, Grounding, Queryable, FileFormatReader, FileFormatWriter, EventEmitter, ReceiptIntegration
- SisterType::Contract (byte 0x06, extension .acon, MCP prefix "contract")
- MockContract in agentic-sdk test suite

#### FFI
- C-compatible shared library (cdylib + staticlib)
- Opaque AconHandle with lifecycle functions
- C header (include/agentic_contract.h) with AconError enum
- Functions: acon_open, acon_create, acon_close, acon_save, acon_stats, acon_policy_add, acon_policy_check, acon_free_string

#### Python SDK
- ContractEngine class wrapping `acon` CLI via subprocess
- Methods: stats, policy_add, policy_check, risk_limit_set, violation_report

#### Testing
- 288 tests across 8 test suites (all passing)
- MCP stress tests: all 38 tools, protocol compliance, transport edge cases, concurrency
- CLI integration tests: all subcommands, persistence, workflow chaining
- Engine invention tests: all 16 inventions, large datasets, error paths
- Server stress tests: prompts, resources, transport, authentication
- Edge case tests: Unicode, special characters, boundary values

#### Documentation
- 21+ public documentation pages
- Canonical Sister Kit (byte-identical shared helpers)
- Sister manifest with complete page index
- Research Paper I: Policy Engine for AI Agents

#### CI/CD
- ci.yml: fmt, clippy, test, build, guardrails
- release.yml: cross-platform builds (linux/macos x x86_64/aarch64), crate publishing
- canonical-sister-guardrails.yml
- install-command-guardrails.yml

#### Scripts
- install.sh: 3 profiles (desktop, terminal, server), 7 canonical strings
- check-canonical-sister.sh: byte-identical shared helpers
- check-install-commands.sh, check-runtime-hardening.sh, test-primary-problems.sh

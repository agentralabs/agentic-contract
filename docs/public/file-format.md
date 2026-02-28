---
status: stable
title: File Format
description: The .acon binary file format specification
---

# .acon File Format

AgenticContract stores all governance data in a single `.acon` binary file. The format is designed for fast random access, integrity verification, and forward compatibility.

## Design Goals

- **Single-file portability**: One `.acon` file contains all policies, limits, approvals, conditions, obligations, and violations
- **Integrity**: BLAKE3 checksum covers all serialized data
- **Forward compatibility**: Version field in header allows readers to reject unsupported formats gracefully
- **Compactness**: Binary encoding minimizes disk and network overhead
- **Simplicity**: No external dependencies; the file can be read with any language that supports basic I/O

## Magic Bytes

Files begin with the ASCII bytes `ACON` (hex `0x41 0x43 0x4F 0x4E`). This magic sequence allows file-type detection without relying on file extensions.

```
Offset  Hex            ASCII
0x0000  41 43 4F 4E    ACON
```

## Overall Structure

```
┌─────────────────────────────────────┐
│  Header (fixed size)                │
│  ┌─────────────────────────────┐    │
│  │ Magic: ACON (4 bytes)       │    │
│  │ Version: u32 (4 bytes)      │    │
│  │ Flags: u32 (4 bytes)        │    │
│  │ Entity counts (8 × u32)     │    │
│  │ Timestamps (2 × i64)        │    │
│  │ Checksum: BLAKE3 (32 bytes) │    │
│  └─────────────────────────────┘    │
├─────────────────────────────────────┤
│  Entity sections (variable size)    │
│  ┌─────────────────────────────┐    │
│  │ Policies (JSON array)       │    │
│  │ Risk Limits (JSON array)    │    │
│  │ Approval Rules (JSON array) │    │
│  │ Approval Requests           │    │
│  │ Approval Decisions          │    │
│  │ Conditions (JSON array)     │    │
│  │ Obligations (JSON array)    │    │
│  │ Violations (JSON array)     │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```

## Header Layout

The header is a fixed-size binary block at the start of every `.acon` file.

| Offset | Size | Type | Field | Description |
|--------|------|------|-------|-------------|
| 0 | 4 | `[u8; 4]` | magic | `b"ACON"` — file type identifier |
| 4 | 4 | `u32` | version | Format version (currently `1`) |
| 8 | 4 | `u32` | flags | Reserved for future use (currently `0`) |
| 12 | 4 | `u32` | policy_count | Number of policies |
| 16 | 4 | `u32` | risk_limit_count | Number of risk limits |
| 20 | 4 | `u32` | approval_rule_count | Number of approval rules |
| 24 | 4 | `u32` | approval_request_count | Number of approval requests |
| 28 | 4 | `u32` | condition_count | Number of conditions |
| 32 | 4 | `u32` | obligation_count | Number of obligations |
| 36 | 4 | `u32` | violation_count | Number of violations |
| 40 | 4 | `u32` | _reserved | Reserved (approval decision count) |
| 44 | 8 | `i64` | created_at | File creation timestamp (Unix seconds) |
| 52 | 8 | `i64` | modified_at | Last modification timestamp |
| 60 | 32 | `[u8; 32]` | checksum | BLAKE3 hash of all entity data |

Total header size: 92 bytes.

## Entity Type Discriminator

Each entity type has a `u8` discriminator used internally:

| Value | Entity Type |
|-------|-------------|
| `1` | Policy |
| `2` | RiskLimit |
| `3` | ApprovalRule |
| `4` | ApprovalRequest |
| `5` | ApprovalDecision |
| `6` | Condition |
| `7` | Obligation |
| `8` | Violation |

## Entity Serialization

After the header, entities are serialized as JSON arrays, one per entity type, written sequentially. Each array is length-prefixed with a `u64` byte count followed by the JSON payload.

```
┌──────────────────────────────────┐
│ length: u64 (8 bytes)            │
│ json_data: [u8; length]          │
└──────────────────────────────────┘
```

This approach balances human debuggability (entities are JSON) with structure (the header provides O(1) metadata access and the checksum covers integrity).

## Checksum Verification

The BLAKE3 checksum in the header covers all bytes after the header. On load, the reader:

1. Reads the header (92 bytes)
2. Reads all remaining bytes
3. Computes BLAKE3 over the remaining bytes
4. Compares against `header.checksum`
5. Returns `ContractError::FileFormat` on mismatch

This catches corruption from partial writes, disk errors, and accidental edits.

## Version Compatibility

**Current version**: 1

Readers must:

- Reject files with unrecognized magic bytes
- Reject files with `version > 1` (forward-incompatible)
- Accept files with `version == 1` regardless of flags (flags are advisory)

Future versions may:

- Add new entity sections after existing ones
- Add new fields to existing entity JSON schemas
- Change the serialization format (would require version bump)

**Byte order**: All multi-byte integers use little-endian encoding.

## Size Estimates

| Content | Approximate Size |
|---------|-----------------|
| Empty file (header only) | 92 bytes |
| 1 policy | ~512 bytes |
| 100 entities (mixed types) | ~50 KB |
| 1,000 entities (mixed types) | ~500 KB |
| 10,000 entities (mixed types) | ~5 MB |

File sizes scale linearly with entity count. JSON serialization adds overhead compared to pure binary but keeps the format inspectable with standard tools.

## Inspecting Files

The `acon info` CLI command shows file metadata:

```bash
$ acon info
Format:   ACON v1
Created:  2026-02-27T10:30:00Z
Modified: 2026-02-27T14:15:00Z
Entities: 47 total
  Policies:          12
  Risk Limits:        5
  Approval Rules:     3
  Approval Requests:  8
  Conditions:         4
  Obligations:        6
  Violations:         9
Checksum: ok (BLAKE3)
```

Raw hex inspection shows the magic bytes:

```bash
$ xxd -l 16 ~/.agentic/contract.acon
00000000: 4143 4f4e 0100 0000 0000 0000 0100 0000  ACON............
```

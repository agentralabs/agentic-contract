---
status: stable
title: Benchmarks
description: Performance benchmarks for AgenticContract
---

# Benchmarks

All benchmarks measured with Criterion on Apple M-series (ARM64) hardware. Results represent median values from 100+ iterations with outlier detection enabled.

## Test Environment

| Property | Value |
|----------|-------|
| Hardware | Apple M-series (ARM64) |
| Rust | 1.75+ |
| Framework | Criterion 0.5 |
| OS | macOS 14+ |
| Profile | release (optimized) |

## Summary Results

At typical workloads, all core governance operations complete in under 1 microsecond. File I/O for 100 entities takes under 200 microseconds.

| Category | Representative | Latency |
|----------|---------------|---------|
| Policy evaluation | Single policy check | 49.3 ns |
| Risk limit check | Single limit | 43.9 ns |
| Violation reporting | With severity | 1.01 us |
| Engine statistics | Full summary | 60.4 ns |
| File save | 100 entities | 154.5 us |
| File load | 100 entities | 96.7 us |

## Detailed Results: Core Operations

Single-operation benchmarks measure the cost of one governance primitive call.

| Operation | Median | Notes |
|-----------|--------|-------|
| `policy_evaluate_single` | 49.3 ns | Evaluate one policy against one action |
| `risk_limit_check` | 43.9 ns | Check one limit against proposed amount |
| `violation_report` | 1.01 us | Create and store a violation record |
| `engine_stats` | 60.4 ns | Compute summary statistics |

## Detailed Results: Scale Operations

Policy evaluation scales linearly with policy count.

| Operation | Median | Notes |
|-----------|--------|-------|
| `policy_evaluate_scale/10` | 684.5 ns | Evaluate action against 10 policies |
| `policy_evaluate_scale/100` | 7.04 us | Evaluate action against 100 policies |
| `policy_evaluate_scale/1000` | 72.8 us | Evaluate action against 1,000 policies |

### Scaling Analysis

| Policy Count | Latency | Per-Policy Cost |
|-------------|---------|-----------------|
| 1 | 49.3 ns | 49.3 ns |
| 10 | 684.5 ns | 68.5 ns |
| 100 | 7.04 us | 70.4 ns |
| 1,000 | 72.8 us | 72.8 ns |

The per-policy cost is essentially constant at ~70 ns, confirming O(n) linear scaling with minimal overhead.

## Detailed Results: File I/O

Binary serialization benchmarks measure `.acon` file write and read performance.

| Operation | Median | Notes |
|-----------|--------|-------|
| `file_save_100_entities` | 154.5 us | Write 100 mixed entities to disk |
| `file_load_100_entities` | 96.7 us | Read + BLAKE3 verify from disk |

### I/O Scaling Estimates

| Entity Count | Save | Load | File Size |
|-------------|------|------|-----------|
| 1 | ~5 us | ~3 us | ~512 B |
| 100 | 154.5 us | 96.7 us | ~50 KB |
| 1,000 | ~1.5 ms | ~1 ms | ~500 KB |
| 10,000 | ~15 ms | ~10 ms | ~5 MB |

Load includes BLAKE3 checksum verification. Save includes checksum computation and header update.

## Comparison Context

AgenticContract is a local-first policy engine optimized for single-agent governance. It is not a distributed policy server.

| System | Architecture | Typical Latency | Storage |
|--------|-------------|-----------------|---------|
| **AgenticContract** | Single-file binary, in-process | 49 ns (eval) | `.acon` file |
| OPA (Open Policy Agent) | HTTP server, Rego evaluation | 0.5-5 ms | In-memory |
| Cedar (AWS) | Library, Cedar language | 1-10 us | In-memory |

AgenticContract is 10-100x faster than OPA for policy evaluation because there is no network round-trip or language interpretation. Compared to Cedar, AgenticContract trades policy language expressiveness for simpler text-matching semantics.

## Performance Tiers

| Tier | Latency Range | Operations |
|------|---------------|------------|
| **Interactive** | < 1 us | Policy eval, risk check, stats |
| **Batch** | 1-100 us | Scale evaluation, violation report |
| **I/O** | 100 us - 15 ms | File save/load (scales with size) |

All interactive operations are suitable for hot-path enforcement (e.g., checking every API call against policies).

## Memory Usage

The engine holds all entities in memory. Approximate memory footprint:

| Entity Count | Heap Usage |
|-------------|------------|
| 100 | ~50 KB |
| 1,000 | ~500 KB |
| 10,000 | ~5 MB |

For typical agent governance (10-100 active policies, a few dozen limits and obligations), memory usage is negligible.

## Reproducing Benchmarks

### Prerequisites

- Rust 1.75+
- Criterion 0.5 (included in dev-dependencies)

### Running All Benchmarks

```bash
cd crates/agentic-contract
cargo bench
```

### Running Specific Benchmarks

```bash
cargo bench -- policy_evaluate
cargo bench -- file_save
cargo bench -- risk_limit
```

### Viewing Reports

Criterion generates HTML reports in `target/criterion/`. Open `report/index.html` for interactive charts.

```bash
open target/criterion/report/index.html
```

### Custom Configurations

Set environment variables to adjust benchmark parameters:

```bash
BENCH_POLICY_COUNT=5000 cargo bench -- policy_evaluate_scale
```

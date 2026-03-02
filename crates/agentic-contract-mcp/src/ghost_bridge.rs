//! Ghost Writer Bridge — Syncs contract context to AI coding assistants.
//!
//! Detects Claude Code, Cursor, Windsurf, and Cody, then writes a
//! contract context summary to each client's memory directory.
//!
//! Called from the stdio loop after each request (synchronous — no background thread).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use agentic_contract::ContractEngine;

/// Cached client directories (detected once, reused on each sync).
pub struct GhostBridge {
    clients: Vec<ClientDir>,
    last_content_hash: u64,
}

struct ClientDir {
    name: &'static str,
    dir: PathBuf,
    filename: String,
}

impl GhostBridge {
    /// Create and detect all AI clients. Returns None if none found.
    pub fn new() -> Option<Self> {
        let clients = detect_all_memory_dirs();
        if clients.is_empty() {
            return None;
        }
        for c in &clients {
            eprintln!("[ghost_bridge] Contract context: {} at {:?}", c.name, c.dir);
        }
        Some(Self {
            clients,
            last_content_hash: 0,
        })
    }

    /// Sync current engine state to all detected clients.
    /// Only writes if content has changed (dedup via FNV-1a hash).
    pub fn sync(&mut self, engine: &ContractEngine) {
        let markdown = build_contract_context(engine);

        let hash = fnv1a_hash(&markdown);
        if hash == self.last_content_hash {
            return;
        }
        self.last_content_hash = hash;

        for client in &self.clients {
            let target = client.dir.join(&client.filename);
            if let Err(e) = atomic_write(&target, markdown.as_bytes()) {
                eprintln!("[ghost_bridge] Failed to sync to {:?}: {e}", target);
            }
        }
    }
}

fn build_contract_context(engine: &ContractEngine) -> String {
    let f = &engine.file;
    let now = now_utc_string();

    let mut md = String::new();
    md.push_str("# AgenticContract Context\n\n");
    md.push_str(&format!("> Auto-synced by Ghost Writer at {now}\n\n"));

    // Overview stats
    let total = f.policies.len()
        + f.risk_limits.len()
        + f.approval_rules.len()
        + f.conditions.len()
        + f.obligations.len()
        + f.violations.len();

    md.push_str("## Overview\n\n");
    md.push_str(&format!(
        "| Metric | Count |\n|--------|-------|\n\
         | Policies | {} |\n\
         | Risk Limits | {} |\n\
         | Approval Rules | {} |\n\
         | Pending Requests | {} |\n\
         | Decisions | {} |\n\
         | Conditions | {} |\n\
         | Obligations | {} |\n\
         | Violations | {} |\n\
         | **Total Entities** | **{total}** |\n\n",
        f.policies.len(),
        f.risk_limits.len(),
        f.approval_rules.len(),
        f.approval_requests.len(),
        f.approval_decisions.len(),
        f.conditions.len(),
        f.obligations.len(),
        f.violations.len(),
    ));

    // Active policies
    let active_policies: Vec<_> = f.policies.iter().filter(|p| p.is_active()).collect();
    if !active_policies.is_empty() {
        md.push_str("## Active Policies\n\n");
        md.push_str("| Label | Scope | Action |\n|-------|-------|--------|\n");
        for p in active_policies.iter().take(15) {
            md.push_str(&format!(
                "| {} | {:?} | {:?} |\n",
                truncate(&p.label, 40),
                p.scope,
                p.action,
            ));
        }
        if active_policies.len() > 15 {
            md.push_str(&format!(
                "| _...and {} more_ | | |\n",
                active_policies.len() - 15
            ));
        }
        md.push('\n');
    }

    // Pending approval requests
    let pending: Vec<_> = f
        .approval_requests
        .iter()
        .filter(|r| matches!(r.status, agentic_contract::approval::ApprovalStatus::Pending))
        .collect();
    if !pending.is_empty() {
        md.push_str("## Pending Approvals\n\n");
        for r in pending.iter().take(10) {
            md.push_str(&format!(
                "- **{}**: {} (requested by {})\n",
                r.id,
                truncate(&r.action_description, 60),
                r.requestor,
            ));
        }
        if pending.len() > 10 {
            md.push_str(&format!("- _...and {} more_\n", pending.len() - 10));
        }
        md.push('\n');
    }

    // Recent violations
    if !f.violations.is_empty() {
        md.push_str("## Recent Violations\n\n");
        for v in f.violations.iter().rev().take(10) {
            md.push_str(&format!(
                "- [{:?}] {} (policy {:?})\n",
                v.severity,
                truncate(&v.description, 60),
                v.policy_id,
            ));
        }
        if f.violations.len() > 10 {
            md.push_str(&format!(
                "- _...and {} more_\n",
                f.violations.len() - 10
            ));
        }
        md.push('\n');
    }

    // Unfulfilled obligations
    let unfulfilled: Vec<_> = f
        .obligations
        .iter()
        .filter(|o| {
            matches!(
                o.status,
                agentic_contract::obligation::ObligationStatus::Pending
                    | agentic_contract::obligation::ObligationStatus::Overdue
            )
        })
        .collect();
    if !unfulfilled.is_empty() {
        md.push_str("## Active Obligations\n\n");
        for o in unfulfilled.iter().take(10) {
            let status_tag = match o.status {
                agentic_contract::obligation::ObligationStatus::Overdue => " **OVERDUE**",
                _ => "",
            };
            md.push_str(&format!(
                "- {}{}: {}\n",
                truncate(&o.label, 40),
                status_tag,
                truncate(&o.description, 50),
            ));
        }
        md.push('\n');
    }

    md.push_str("---\n");
    md.push_str("_Auto-generated by AgenticContract. Do not edit manually._\n");
    md
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max])
    } else {
        s.to_string()
    }
}

fn fnv1a_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn now_utc_string() -> String {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let s = secs % 60;
    let min = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let z = (secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };
    format!("{y:04}-{mo:02}-{d:02} {h:02}:{min:02}:{s:02} UTC")
}

// ═══════════════════════════════════════════════════════════════════
// Multi-client detection
// ═══════════════════════════════════════════════════════════════════

fn detect_all_memory_dirs() -> Vec<ClientDir> {
    let home = match std::env::var("HOME").ok().map(PathBuf::from) {
        Some(h) => h,
        None => return vec![],
    };

    let candidates = [
        (
            "Claude Code",
            home.join(".claude").join("memory"),
            "CONTRACT_CONTEXT.md",
        ),
        (
            "Cursor",
            home.join(".cursor").join("memory"),
            "agentic-contract.md",
        ),
        (
            "Windsurf",
            home.join(".windsurf").join("memory"),
            "agentic-contract.md",
        ),
        (
            "Cody",
            home.join(".sourcegraph").join("cody").join("memory"),
            "agentic-contract.md",
        ),
    ];

    let mut dirs = Vec::new();
    for (name, memory_dir, filename) in &candidates {
        if create_if_parent_exists(memory_dir) {
            dirs.push(ClientDir {
                name,
                dir: memory_dir.clone(),
                filename: filename.to_string(),
            });
        }
    }
    dirs
}

fn create_if_parent_exists(memory_dir: &Path) -> bool {
    if memory_dir.exists() {
        return true;
    }
    if let Some(parent) = memory_dir.parent() {
        if parent.exists() {
            return std::fs::create_dir_all(memory_dir).is_ok();
        }
    }
    false
}

fn atomic_write(target: &Path, content: &[u8]) -> Result<(), std::io::Error> {
    let tmp = target.with_extension("tmp");
    let mut f = std::fs::File::create(&tmp)?;
    f.write_all(content)?;
    f.sync_all()?;
    std::fs::rename(&tmp, target)?;
    Ok(())
}

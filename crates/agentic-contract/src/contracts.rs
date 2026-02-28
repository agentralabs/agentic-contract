//! Agentic-sdk trait implementations for AgenticContract.
//!
//! Implements: Sister, SessionManagement, Grounding, Queryable,
//!             FileFormatReader, FileFormatWriter, EventEmitter,
//!             ReceiptIntegration
//!
//! Contract is the most trait-rich sister — governance requires
//! sessions (scoped policies), grounding (verify claims against
//! policies), receipts (audit trail), and events (real-time notifications).

use std::path::{Path, PathBuf};
use std::time::Instant;

use agentic_sdk::prelude::*;
use chrono::Utc;

use crate::contract_engine::ContractEngine;
use crate::error::ContractError;
use crate::file_format::{ContractFile, MAGIC, VERSION};

// ═══════════════════════════════════════════════════════════════════
// ERROR BRIDGE
// ═══════════════════════════════════════════════════════════════════

impl From<ContractError> for SisterError {
    fn from(e: ContractError) -> Self {
        match &e {
            ContractError::NotFound(entity) => {
                SisterError::not_found(format!("contract entity not found: {entity}"))
            }
            ContractError::PolicyViolation(msg) => {
                SisterError::new(ErrorCode::InvalidState, format!("Policy violation: {msg}"))
            }
            ContractError::RiskLimitExceeded {
                limit,
                current,
                max,
            } => SisterError::new(
                ErrorCode::InvalidState,
                format!("Risk limit exceeded: {limit} (current: {current}, max: {max})"),
            ),
            ContractError::ApprovalRequired(msg) => {
                SisterError::new(ErrorCode::InvalidState, format!("Approval required: {msg}"))
            }
            ContractError::ApprovalDenied(msg) => {
                SisterError::new(ErrorCode::InvalidState, format!("Approval denied: {msg}"))
            }
            ContractError::ConditionNotMet(msg) => {
                SisterError::new(ErrorCode::InvalidState, format!("Condition not met: {msg}"))
            }
            ContractError::ObligationUnfulfilled(msg) => SisterError::new(
                ErrorCode::InvalidState,
                format!("Obligation unfulfilled: {msg}"),
            ),
            ContractError::ContractExpired(msg) => {
                SisterError::new(ErrorCode::InvalidState, format!("Contract expired: {msg}"))
            }
            ContractError::InvalidContract(msg) => {
                SisterError::invalid_input(format!("Invalid contract: {msg}"))
            }
            ContractError::FileFormat(msg) => SisterError::new(
                ErrorCode::VersionMismatch,
                format!("File format error: {msg}"),
            ),
            ContractError::Io(err) => {
                SisterError::new(ErrorCode::StorageError, format!("IO error: {err}"))
            }
            ContractError::Serialization(err) => SisterError::new(
                ErrorCode::StorageError,
                format!("Serialization error: {err}"),
            ),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// FACADE
// ═══════════════════════════════════════════════════════════════════

/// Contract facade wrapping the ContractEngine with SDK trait implementations.
///
/// Contract is session-scoped: policies and governance rules are loaded
/// into a session context and can be queried, grounded, and audited.
pub struct ContractSister {
    /// Core engine (owns the ContractFile).
    engine: ContractEngine,

    /// Path to the .acon file.
    #[allow(dead_code)]
    file_path: PathBuf,

    /// Startup time for uptime tracking.
    started_at: Instant,

    /// Current session ID.
    session_id: Option<ContextId>,

    /// Event manager for real-time notifications.
    events: EventManager,
}

// ═══════════════════════════════════════════════════════════════════
// SISTER TRAIT
// ═══════════════════════════════════════════════════════════════════

impl Sister for ContractSister {
    const SISTER_TYPE: SisterType = SisterType::Contract;
    const FILE_EXTENSION: &'static str = "acon";

    fn init(config: SisterConfig) -> SisterResult<Self>
    where
        Self: Sized,
    {
        let path = config
            .data_path
            .unwrap_or_else(|| PathBuf::from("contract.acon"));

        let file = if path.exists() {
            ContractFile::load(&path).map_err(SisterError::from)?
        } else if config.create_if_missing {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        SisterError::new(
                            ErrorCode::StorageError,
                            format!("Failed to create parent dir: {e}"),
                        )
                    })?;
                }
            }
            let mut f = ContractFile::new();
            f.path = Some(path.clone());
            f
        } else {
            return Err(SisterError::not_found(format!(
                "Contract file not found: {}",
                path.display()
            )));
        };

        let engine = ContractEngine::from_file(file);

        Ok(Self {
            engine,
            file_path: path,
            started_at: Instant::now(),
            session_id: None,
            events: EventManager::new(256),
        })
    }

    fn health(&self) -> HealthStatus {
        let uptime = self.started_at.elapsed();
        let stats = self.engine.stats();

        HealthStatus {
            healthy: true,
            status: Status::Ready,
            uptime,
            resources: ResourceUsage {
                memory_bytes: 0,
                disk_bytes: 0,
                open_handles: stats.total_entities,
            },
            warnings: vec![],
            last_error: None,
        }
    }

    fn version(&self) -> Version {
        Version::new(0, 1, 0)
    }

    fn shutdown(&mut self) -> SisterResult<()> {
        self.events
            .emit(SisterEvent::shutting_down(SisterType::Contract));
        self.engine.save().map_err(SisterError::from)?;
        Ok(())
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![
            Capability::new("policy_add", "Add a policy rule governing agent behavior"),
            Capability::new(
                "policy_check",
                "Check if an action is allowed under policies",
            ),
            Capability::new(
                "policy_list",
                "List active policies with optional scope filter",
            ),
            Capability::new("risk_limit_set", "Set a risk limit threshold"),
            Capability::new("risk_limit_check", "Check if an action would exceed limits"),
            Capability::new(
                "approval_request",
                "Request approval for a controlled action",
            ),
            Capability::new("approval_decide", "Approve or deny a pending request"),
            Capability::new("obligation_add", "Add an obligation that must be fulfilled"),
            Capability::new("obligation_check", "Check the status of obligations"),
            Capability::new("violation_report", "Report a contract or policy violation"),
            Capability::new("violation_list", "List recorded violations"),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════
// SESSION MANAGEMENT
// ═══════════════════════════════════════════════════════════════════

impl SessionManagement for ContractSister {
    fn start_session(&mut self, name: &str) -> SisterResult<ContextId> {
        let id = ContextId::new();
        self.session_id = Some(id);

        self.events.emit(SisterEvent::context_created(
            SisterType::Contract,
            id,
            name.to_string(),
        ));

        Ok(id)
    }

    fn end_session(&mut self) -> SisterResult<()> {
        self.session_id = None;
        Ok(())
    }

    fn current_session(&self) -> Option<ContextId> {
        self.session_id
    }

    fn current_session_info(&self) -> SisterResult<ContextInfo> {
        let id = self
            .session_id
            .ok_or_else(|| SisterError::new(ErrorCode::InvalidState, "No active session"))?;

        let stats = self.engine.stats();
        Ok(ContextInfo {
            id,
            name: "contract_session".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            item_count: stats.total_entities,
            size_bytes: 0,
            metadata: Metadata::new(),
        })
    }

    fn list_sessions(&self) -> SisterResult<Vec<ContextSummary>> {
        // Contract sessions are ephemeral — only current session tracked
        Ok(vec![])
    }

    fn export_session(&self, _id: ContextId) -> SisterResult<ContextSnapshot> {
        let info = self.current_session_info()?;
        let data = serde_json::to_vec(&self.engine.file)
            .map_err(|e| SisterError::new(ErrorCode::Internal, e.to_string()))?;
        let checksum = *blake3::hash(&data).as_bytes();

        Ok(ContextSnapshot {
            sister_type: SisterType::Contract,
            version: Version::new(0, 1, 0),
            context_info: info,
            data,
            checksum,
            snapshot_at: Utc::now(),
        })
    }

    fn import_session(&mut self, snapshot: ContextSnapshot) -> SisterResult<ContextId> {
        if !snapshot.verify() {
            return Err(SisterError::new(
                ErrorCode::ChecksumMismatch,
                "Snapshot checksum verification failed",
            ));
        }
        self.start_session(&snapshot.context_info.name)
    }
}

// ═══════════════════════════════════════════════════════════════════
// GROUNDING
// ═══════════════════════════════════════════════════════════════════

impl Grounding for ContractSister {
    fn ground(&self, claim: &str) -> SisterResult<GroundingResult> {
        // Search policies
        let mut best_score = 0.0f64;
        let mut evidence = Vec::new();

        for policy in &self.engine.file.policies {
            let score = word_overlap_score(claim, &policy.label);
            let desc_score = word_overlap_score(claim, &policy.description);
            let combined = score.max(desc_score);

            if combined > 0.0 {
                best_score = best_score.max(combined);
                evidence.push(GroundingEvidence::new(
                    "policy",
                    policy.id.to_string(),
                    combined,
                    format!("{} [{}]", policy.label, policy.scope),
                ));
            }
        }

        // Search obligations
        for obligation in &self.engine.file.obligations {
            let score = word_overlap_score(claim, &obligation.label);
            if score > 0.0 {
                best_score = best_score.max(score);
                evidence.push(GroundingEvidence::new(
                    "obligation",
                    obligation.id.to_string(),
                    score,
                    format!("{} [{}]", obligation.label, obligation.assignee),
                ));
            }
        }

        // Search violations
        for violation in &self.engine.file.violations {
            let score = word_overlap_score(claim, &violation.description);
            if score > 0.0 {
                best_score = best_score.max(score);
                evidence.push(GroundingEvidence::new(
                    "violation",
                    violation.id.to_string(),
                    score,
                    format!("{} [{}]", violation.description, violation.severity),
                ));
            }
        }

        if evidence.is_empty() {
            // Build suggestions from policies
            let suggestions: Vec<String> = self
                .engine
                .file
                .policies
                .iter()
                .take(3)
                .map(|p| p.label.clone())
                .collect();

            Ok(GroundingResult::ungrounded(
                claim,
                "No matching policies, obligations, or violations found",
            )
            .with_suggestions(suggestions))
        } else if best_score > 0.5 {
            Ok(GroundingResult::verified(claim, best_score)
                .with_evidence(evidence)
                .with_reason("Found matching contract entities"))
        } else {
            Ok(GroundingResult::partial(claim, best_score)
                .with_evidence(evidence)
                .with_reason("Some evidence found in contract entities"))
        }
    }

    fn evidence(&self, query: &str, max_results: usize) -> SisterResult<Vec<EvidenceDetail>> {
        let mut results = Vec::new();

        for policy in &self.engine.file.policies {
            let score = word_overlap_score(query, &policy.label);
            if score > 0.0 {
                results.push(EvidenceDetail {
                    evidence_type: "policy".to_string(),
                    id: policy.id.to_string(),
                    score,
                    created_at: policy.created_at,
                    source_sister: SisterType::Contract,
                    content: format!("{} [{}]", policy.label, policy.scope),
                    data: Metadata::new(),
                });
            }
        }

        for obligation in &self.engine.file.obligations {
            let score = word_overlap_score(query, &obligation.label);
            if score > 0.0 {
                results.push(EvidenceDetail {
                    evidence_type: "obligation".to_string(),
                    id: obligation.id.to_string(),
                    score,
                    created_at: obligation.created_at,
                    source_sister: SisterType::Contract,
                    content: obligation.label.to_string(),
                    data: Metadata::new(),
                });
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(max_results);
        Ok(results)
    }

    fn suggest(&self, _query: &str, limit: usize) -> SisterResult<Vec<GroundingSuggestion>> {
        let mut suggestions = Vec::new();

        for policy in self.engine.file.policies.iter().take(limit) {
            suggestions.push(GroundingSuggestion {
                item_type: "policy".to_string(),
                id: policy.id.to_string(),
                relevance_score: 0.5,
                description: format!("{} [{}]", policy.label, policy.scope),
                data: Metadata::new(),
            });
        }

        Ok(suggestions)
    }
}

// ═══════════════════════════════════════════════════════════════════
// QUERYABLE
// ═══════════════════════════════════════════════════════════════════

impl Queryable for ContractSister {
    fn query(&self, query: Query) -> SisterResult<QueryResult> {
        let start = Instant::now();

        match query.query_type.as_str() {
            "list" => {
                let limit = query.limit.unwrap_or(50);
                let offset = query.offset.unwrap_or(0);
                let entity_filter = query.get_string("entity_type");

                let mut results: Vec<serde_json::Value> = Vec::new();

                match entity_filter.as_deref() {
                    Some("policy") | None => {
                        for p in &self.engine.file.policies {
                            results.push(serde_json::json!({
                                "id": p.id.to_string(),
                                "type": "policy",
                                "label": p.label,
                                "scope": format!("{}", p.scope),
                                "action": format!("{:?}", p.action),
                                "status": format!("{:?}", p.status),
                            }));
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("risk_limit") | None => {
                        for r in &self.engine.file.risk_limits {
                            results.push(serde_json::json!({
                                "id": r.id.to_string(),
                                "type": "risk_limit",
                                "label": r.label,
                                "current": r.current_value,
                                "max": r.max_value,
                                "usage": format!("{:.1}%", r.usage_ratio() * 100.0),
                            }));
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("obligation") | None => {
                        for o in &self.engine.file.obligations {
                            results.push(serde_json::json!({
                                "id": o.id.to_string(),
                                "type": "obligation",
                                "label": o.label,
                                "assignee": o.assignee,
                                "status": format!("{:?}", o.status),
                            }));
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("violation") | None => {
                        for v in &self.engine.file.violations {
                            results.push(serde_json::json!({
                                "id": v.id.to_string(),
                                "type": "violation",
                                "description": v.description,
                                "severity": format!("{}", v.severity),
                                "actor": v.actor,
                            }));
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("condition") | None => {
                        for c in &self.engine.file.conditions {
                            results.push(serde_json::json!({
                                "id": c.id.to_string(),
                                "type": "condition",
                                "label": c.label,
                                "status": format!("{:?}", c.status),
                            }));
                        }
                    }
                    _ => {}
                }

                let total = results.len();
                let paged: Vec<serde_json::Value> =
                    results.into_iter().skip(offset).take(limit).collect();

                Ok(QueryResult::new(query, paged, start.elapsed())
                    .with_pagination(total, offset + limit < total))
            }

            "search" => {
                let query_text = query.get_string("text").unwrap_or_default();
                let limit = query.limit.unwrap_or(20);

                let mut scored: Vec<(f64, serde_json::Value)> = Vec::new();

                // Search policies
                for p in &self.engine.file.policies {
                    let score = word_overlap_score(&query_text, &p.label)
                        .max(word_overlap_score(&query_text, &p.description));
                    if score > 0.0 {
                        scored.push((
                            score,
                            serde_json::json!({
                                "id": p.id.to_string(),
                                "type": "policy",
                                "label": p.label,
                                "scope": format!("{}", p.scope),
                                "score": score,
                            }),
                        ));
                    }
                }

                // Search obligations
                for o in &self.engine.file.obligations {
                    let score = word_overlap_score(&query_text, &o.label)
                        .max(word_overlap_score(&query_text, &o.description));
                    if score > 0.0 {
                        scored.push((
                            score,
                            serde_json::json!({
                                "id": o.id.to_string(),
                                "type": "obligation",
                                "label": o.label,
                                "score": score,
                            }),
                        ));
                    }
                }

                // Search violations
                for v in &self.engine.file.violations {
                    let score = word_overlap_score(&query_text, &v.description);
                    if score > 0.0 {
                        scored.push((
                            score,
                            serde_json::json!({
                                "id": v.id.to_string(),
                                "type": "violation",
                                "description": v.description,
                                "severity": format!("{}", v.severity),
                                "score": score,
                            }),
                        ));
                    }
                }

                // Sort by score descending
                scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                let results: Vec<serde_json::Value> =
                    scored.into_iter().take(limit).map(|(_, v)| v).collect();

                Ok(QueryResult::new(query, results, start.elapsed()))
            }

            "recent" => {
                let limit = query.limit.unwrap_or(10);

                // Combine entities with timestamps, take most recent
                let mut items: Vec<(chrono::DateTime<Utc>, serde_json::Value)> = Vec::new();

                for p in &self.engine.file.policies {
                    items.push((
                        p.created_at,
                        serde_json::json!({
                            "id": p.id.to_string(),
                            "type": "policy",
                            "label": p.label,
                            "created_at": p.created_at.to_rfc3339(),
                        }),
                    ));
                }

                for v in &self.engine.file.violations {
                    items.push((
                        v.detected_at,
                        serde_json::json!({
                            "id": v.id.to_string(),
                            "type": "violation",
                            "description": v.description,
                            "detected_at": v.detected_at.to_rfc3339(),
                        }),
                    ));
                }

                for o in &self.engine.file.obligations {
                    items.push((
                        o.created_at,
                        serde_json::json!({
                            "id": o.id.to_string(),
                            "type": "obligation",
                            "label": o.label,
                            "created_at": o.created_at.to_rfc3339(),
                        }),
                    ));
                }

                // Sort by time descending (most recent first)
                items.sort_by(|a, b| b.0.cmp(&a.0));
                let results: Vec<serde_json::Value> =
                    items.into_iter().take(limit).map(|(_, v)| v).collect();

                Ok(QueryResult::new(query, results, start.elapsed()))
            }

            "get" => {
                let id_str = query
                    .get_string("id")
                    .ok_or_else(|| SisterError::invalid_input("Missing required field: id"))?;

                let id: crate::ContractId = id_str
                    .parse()
                    .map_err(|_| SisterError::invalid_input(format!("Invalid UUID: {id_str}")))?;

                // Search all entity types for this ID
                if let Some(p) = self.engine.file.find_policy(id) {
                    let result = serde_json::json!({
                        "id": p.id.to_string(),
                        "type": "policy",
                        "label": p.label,
                        "description": p.description,
                        "scope": format!("{}", p.scope),
                        "action": format!("{:?}", p.action),
                        "status": format!("{:?}", p.status),
                        "tags": p.tags,
                        "created_at": p.created_at.to_rfc3339(),
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed()));
                }

                if let Some(o) = self.engine.file.find_obligation(id) {
                    let result = serde_json::json!({
                        "id": o.id.to_string(),
                        "type": "obligation",
                        "label": o.label,
                        "description": o.description,
                        "assignee": o.assignee,
                        "status": format!("{:?}", o.status),
                        "created_at": o.created_at.to_rfc3339(),
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed()));
                }

                Err(SisterError::not_found(format!("Entity {id_str}")))
            }

            "stats" => {
                let stats = self.engine.stats();
                let result = serde_json::to_value(stats)
                    .map_err(|e| SisterError::new(ErrorCode::Internal, e.to_string()))?;
                Ok(QueryResult::new(query, vec![result], start.elapsed()))
            }

            _ => Ok(QueryResult::new(query, vec![], start.elapsed())),
        }
    }

    fn supports_query(&self, query_type: &str) -> bool {
        matches!(query_type, "list" | "search" | "recent" | "get" | "stats")
    }

    fn query_types(&self) -> Vec<QueryTypeInfo> {
        vec![
            QueryTypeInfo::new("list", "List all contract entities"),
            QueryTypeInfo::new("search", "Search entities by text").required(vec!["text"]),
            QueryTypeInfo::new("recent", "Get most recently created entities"),
            QueryTypeInfo::new("get", "Get entity by ID").required(vec!["id"]),
            QueryTypeInfo::new("stats", "Get contract statistics"),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════
// FILE FORMAT (Reader + Writer)
// ═══════════════════════════════════════════════════════════════════

impl FileFormatReader for ContractSister {
    fn read_file(path: &Path) -> SisterResult<Self>
    where
        Self: Sized,
    {
        let config = SisterConfig::new(path).create_if_missing(false);
        Self::init(config)
    }

    fn can_read(path: &Path) -> SisterResult<FileInfo> {
        // Peek at magic bytes
        use std::io::Read;
        let mut f = std::fs::File::open(path)?;
        let mut magic = [0u8; 4];
        f.read_exact(&mut magic).map_err(|e| {
            SisterError::new(ErrorCode::StorageError, format!("Cannot read file: {e}"))
        })?;

        if magic != MAGIC {
            return Err(SisterError::new(
                ErrorCode::VersionMismatch,
                format!("Not an .acon file (magic: {:?})", magic),
            ));
        }

        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        Ok(FileInfo {
            sister_type: SisterType::Contract,
            version: Version::new(0, VERSION as u8, 0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            content_length: file_size,
            needs_migration: false,
            format_id: "ACON".to_string(),
        })
    }

    fn file_version(path: &Path) -> SisterResult<Version> {
        let info = Self::can_read(path)?;
        Ok(info.version)
    }

    fn migrate(_data: &[u8], _from_version: Version) -> SisterResult<Vec<u8>> {
        // v0.1.0 is the only version — no migration needed
        Err(SisterError::new(
            ErrorCode::NotImplemented,
            "Migration not needed for v0.1.0",
        ))
    }
}

impl FileFormatWriter for ContractSister {
    fn write_file(&self, path: &Path) -> SisterResult<()> {
        let mut file = self.engine.file.clone();
        file.path = Some(path.to_path_buf());
        file.save().map_err(SisterError::from)
    }

    fn to_bytes(&self) -> SisterResult<Vec<u8>> {
        serde_json::to_vec(&self.engine.file)
            .map_err(|e| SisterError::new(ErrorCode::Internal, e.to_string()))
    }
}

// ═══════════════════════════════════════════════════════════════════
// EVENT EMITTER
// ═══════════════════════════════════════════════════════════════════

impl EventEmitter for ContractSister {
    fn subscribe(&self, _filter: EventFilter) -> EventReceiver {
        self.events.subscribe()
    }

    fn recent_events(&self, limit: usize) -> Vec<SisterEvent> {
        self.events.recent(limit)
    }

    fn emit(&self, event: SisterEvent) {
        self.events.emit(event);
    }
}

// ═══════════════════════════════════════════════════════════════════
// RECEIPT INTEGRATION
// ═══════════════════════════════════════════════════════════════════

impl ReceiptIntegration for ContractSister {
    fn create_receipt(&self, action: ActionRecord) -> SisterResult<ReceiptId> {
        // Contract creates receipts for policy decisions, approvals, and violations.
        // In a full implementation this would delegate to Identity sister.
        // Here we generate a receipt ID and log the event.
        let receipt_id = ReceiptId::new();

        self.events.emit(SisterEvent::operation_started(
            SisterType::Contract,
            receipt_id.to_string(),
            format!("receipt:{}", action.action_type),
        ));

        Ok(receipt_id)
    }

    fn get_receipt(&self, id: ReceiptId) -> SisterResult<Receipt> {
        // In production, receipts live in Identity. Contract only creates them.
        Err(SisterError::new(
            ErrorCode::NotImplemented,
            format!(
                "Receipt {} should be retrieved from Identity sister. Contract creates receipts but delegates storage to Identity.",
                id
            ),
        ))
    }

    fn list_receipts(&self, _filter: ReceiptFilter) -> SisterResult<Vec<Receipt>> {
        // Delegate to Identity for receipt storage
        Ok(vec![])
    }
}

// ═══════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════

/// BM25-inspired word overlap score between query and text.
fn word_overlap_score(query: &str, text: &str) -> f64 {
    let query_lower = query.to_lowercase();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();
    let text_lower = text.to_lowercase();

    if query_words.is_empty() {
        return 0.0;
    }

    let matched = query_words
        .iter()
        .filter(|w| text_lower.contains(**w))
        .count();

    matched as f64 / query_words.len() as f64
}

// ═══════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{Policy, PolicyAction, PolicyScope};
    use crate::violation::{Violation, ViolationSeverity};

    fn make_sister() -> ContractSister {
        let config = SisterConfig::stateless().create_if_missing(true);
        ContractSister::init(config).unwrap()
    }

    #[test]
    fn test_sister_basics() {
        let sister = make_sister();
        assert_eq!(sister.sister_type(), SisterType::Contract);
        assert_eq!(sister.file_extension(), "acon");
        assert_eq!(sister.mcp_prefix(), "contract");
        assert!(sister.is_healthy());
    }

    #[test]
    fn test_session_lifecycle() {
        let mut sister = make_sister();
        assert!(sister.current_session().is_none());

        let id = sister.start_session("test").unwrap();
        assert_eq!(sister.current_session().unwrap(), id);

        let info = sister.current_session_info().unwrap();
        assert_eq!(info.id, id);

        sister.end_session().unwrap();
        assert!(sister.current_session().is_none());
    }

    #[test]
    fn test_grounding() {
        let mut sister = make_sister();
        sister.engine.add_policy(Policy::new(
            "Require approval for deploys",
            PolicyScope::Global,
            PolicyAction::RequireApproval,
        ));
        sister.engine.add_policy(Policy::new(
            "Rate limit API calls",
            PolicyScope::Session,
            PolicyAction::AuditOnly,
        ));

        let result = sister.ground("approval for deploys").unwrap();
        assert_eq!(result.status, GroundingStatus::Verified);
        assert!(!result.evidence.is_empty());

        let result = sister.ground("cats can teleport").unwrap();
        assert_eq!(result.status, GroundingStatus::Ungrounded);
    }

    #[test]
    fn test_queryable() {
        let mut sister = make_sister();
        sister.engine.add_policy(Policy::new(
            "Policy A",
            PolicyScope::Global,
            PolicyAction::Allow,
        ));
        sister.engine.report_violation(Violation::new(
            "Limit exceeded",
            ViolationSeverity::Warning,
            "agent_1",
        ));

        // List
        let result = sister.query(Query::list()).unwrap();
        assert!(result.len() >= 2);

        // Search
        let result = sister.search("limit").unwrap();
        assert!(!result.is_empty());

        // Stats
        let result = sister.query(Query::new("stats")).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_events() {
        let mut sister = make_sister();
        sister.start_session("test").unwrap();

        let events = sister.recent_events(10);
        assert!(!events.is_empty());
    }

    #[test]
    fn test_receipt_creation() {
        let sister = make_sister();
        let action = ActionRecord::new(
            SisterType::Contract,
            "policy_check",
            ActionOutcome::success(),
        );
        let receipt_id = sister.create_receipt(action).unwrap();
        assert!(!receipt_id.to_string().is_empty());
    }

    #[test]
    fn test_word_overlap() {
        assert!(word_overlap_score("deploy approval", "Require approval for deploys") > 0.5);
        assert_eq!(
            word_overlap_score("cats", "Require approval for deploys"),
            0.0
        );
        assert_eq!(word_overlap_score("", "anything"), 0.0);
    }

    #[test]
    fn test_error_bridge() {
        let err = ContractError::NotFound("policy_42".to_string());
        let sister_err: SisterError = err.into();
        assert_eq!(sister_err.code, ErrorCode::NotFound);

        let err = ContractError::PolicyViolation("no deploys on friday".to_string());
        let sister_err: SisterError = err.into();
        assert_eq!(sister_err.code, ErrorCode::InvalidState);
    }
}

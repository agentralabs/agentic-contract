//! Phase 2: Core Engine Stress Tests — Inventions, Large Datasets, Error Paths
//!
//! Covers: all 16 inventions exercised directly through the engine,
//! large dataset handling, file format roundtrip under load,
//! concurrent engine access, and error recovery paths.

use agentic_contract::*;
use agentic_contract::inventions::{ArbitrationMethod, GovernanceLevel, TransparencyLevel};

// =========================================================================
// Helpers
// =========================================================================

fn fresh() -> ContractEngine {
    ContractEngine::new()
}

fn future_dt() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap()
}

/// Populate an engine with a realistic governance scenario.
fn populated_engine() -> ContractEngine {
    let mut engine = fresh();

    // Policies
    engine.add_policy(Policy::new("deploy", PolicyScope::Global, PolicyAction::RequireApproval));
    engine.add_policy(Policy::new("read_data", PolicyScope::Global, PolicyAction::Allow));
    engine.add_policy(Policy::new("write_data", PolicyScope::Session, PolicyAction::Deny));
    engine.add_policy(Policy::new("admin_action", PolicyScope::Agent, PolicyAction::RequireApproval));
    engine.add_policy(Policy::new("logging", PolicyScope::Global, PolicyAction::AuditOnly));

    // Risk limits
    engine.add_risk_limit(RiskLimit::new("API calls/hr", LimitType::Rate, 1000.0));
    engine.add_risk_limit(RiskLimit::new("Memory MB", LimitType::Threshold, 512.0));
    engine.add_risk_limit(RiskLimit::new("Budget $", LimitType::Budget, 100.0));

    // Obligations
    engine.add_obligation(Obligation::new("Weekly report", "Submit weekly compliance report", "agent_1").with_deadline(future_dt()));
    engine.add_obligation(Obligation::new("Security scan", "Run vulnerability scan", "agent_2"));

    // Violations
    engine.report_violation(Violation::new("Rate spike", ViolationSeverity::Warning, "agent_1"));
    engine.report_violation(Violation::new("Memory overflow", ViolationSeverity::Critical, "agent_2"));

    // Conditions
    engine.add_condition(Condition::new("mem_check", ConditionType::Threshold, "memory < 80%"));
    engine.add_condition(Condition::new("time_check", ConditionType::TimeBased, "weekday"));

    // Approval rules
    engine.add_approval_rule(ApprovalRule::new("deploy gate", "deploy:*"));

    engine
}

// =========================================================================
// Section 1: Invention — Policy Omniscience
// =========================================================================

#[test]
fn test_invention_policy_omniscience() {
    let engine = populated_engine();
    let result = engine.policy_omniscience("agent_1", "deploy");
    assert!(result.total_permissions > 0);
    assert!(!result.allowed_actions.is_empty() || !result.denied_actions.is_empty() || !result.conditional_actions.is_empty());
}

#[test]
fn test_invention_policy_omniscience_empty() {
    let engine = fresh();
    let result = engine.policy_omniscience("agent_1", "anything");
    assert_eq!(result.total_permissions, 0);
}

// =========================================================================
// Section 2: Invention — Risk Prophecy
// =========================================================================

#[test]
fn test_invention_risk_prophecy() {
    let engine = populated_engine();
    let result = engine.risk_prophecy("agent_1", 3600);
    assert!(result.overall_risk_score >= 0.0 && result.overall_risk_score <= 1.0);
    assert!(!result.projections.is_empty());
}

#[test]
fn test_invention_risk_prophecy_no_limits() {
    let engine = fresh();
    let result = engine.risk_prophecy("agent_1", 3600);
    assert_eq!(result.projections.len(), 0);
}

// =========================================================================
// Section 3: Invention — Approval Telepathy
// =========================================================================

#[test]
fn test_invention_approval_telepathy() {
    let engine = populated_engine();
    let result = engine.approval_telepathy("deploy production");
    assert!(result.approval_probability >= 0.0 && result.approval_probability <= 1.0);
}

// =========================================================================
// Section 4: Invention — Obligation Clairvoyance
// =========================================================================

#[test]
fn test_invention_obligation_clairvoyance() {
    let engine = populated_engine();
    let result = engine.obligation_clairvoyance("agent_1", 86400 * 30);
    assert!(!result.upcoming.is_empty() || result.conflicts.is_empty());
}

// =========================================================================
// Section 5: Invention — Violation Precognition
// =========================================================================

#[test]
fn test_invention_violation_precognition() {
    let engine = populated_engine();
    let result = engine.violation_precognition("deploy to production");
    assert!(result.violation_probability >= 0.0 && result.violation_probability <= 1.0);
}

#[test]
fn test_invention_violation_precognition_safe_action() {
    let engine = populated_engine();
    let result = engine.violation_precognition("read only data");
    // Even safe actions should return valid data
    assert!(result.violation_probability >= 0.0);
}

// =========================================================================
// Section 6: Invention — Contract Crystallization
// =========================================================================

#[test]
fn test_invention_contract_crystallize() {
    let engine = populated_engine();
    let result = engine.crystallize_contract("Standard agent governance agreement");
    assert!(!result.policies.is_empty() || !result.risk_limits.is_empty());
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
}

#[test]
fn test_invention_contract_crystallize_empty_engine() {
    let engine = fresh();
    let result = engine.crystallize_contract("New governance");
    // Should still produce something from intent alone
    assert!(!result.intent.is_empty());
}

// =========================================================================
// Section 7: Invention — Policy DNA
// =========================================================================

#[test]
fn test_invention_policy_dna() {
    let engine = populated_engine();
    let policies = engine.list_policies(None);
    let policy_id = policies[0].id;
    let result = engine.extract_policy_dna(policy_id).unwrap();
    assert!(!result.genes.is_empty());
    assert!(result.fitness >= 0.0 && result.fitness <= 1.0);
}

#[test]
fn test_invention_policy_dna_nonexistent() {
    let engine = fresh();
    let fake_id = ContractId::new();
    let result = engine.extract_policy_dna(fake_id);
    assert!(result.is_err());
}

// =========================================================================
// Section 8: Invention — Trust Gradients
// =========================================================================

#[test]
fn test_invention_trust_gradient() {
    let engine = populated_engine();
    let result = engine.evaluate_trust_gradient("agent_1", "deploy");
    assert!(result.trust_factor >= 0.0 && result.trust_factor <= 1.0);
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
}

#[test]
fn test_invention_trust_gradient_unknown_agent() {
    let engine = populated_engine();
    let result = engine.evaluate_trust_gradient("unknown_agent", "read");
    // Unknown agent should still return valid data
    assert!(result.trust_factor >= 0.0);
}

// =========================================================================
// Section 9: Invention — Collective Contracts
// =========================================================================

#[test]
fn test_invention_collective_contract() {
    let engine = populated_engine();
    let result = engine.create_collective_contract(
        vec![("agent_1", "Engineer"), ("agent_2", "Reviewer"), ("agent_3", "Observer")],
        ArbitrationMethod::MajorityVote,
    );
    assert!(!result.id.to_string().is_empty());
    assert_eq!(result.parties.len(), 3);
    assert_eq!(result.required_signatures, 3);
}

#[test]
fn test_invention_collective_contract_unanimous() {
    let engine = fresh();
    let result = engine.create_collective_contract(
        vec![("a", "Alice"), ("b", "Bob")],
        ArbitrationMethod::Unanimous,
    );
    assert_eq!(result.parties.len(), 2);
}

// =========================================================================
// Section 10: Invention — Temporal Contracts
// =========================================================================

#[test]
fn test_invention_temporal_contract() {
    let engine = populated_engine();
    let result = engine.create_temporal_contract("Quarterly review", GovernanceLevel::Moderate);
    assert!(!result.id.to_string().is_empty());
    assert_eq!(result.label, "Quarterly review");
    assert_eq!(result.initial_level, GovernanceLevel::Moderate);
    assert_eq!(result.current_level, GovernanceLevel::Moderate);
}

#[test]
fn test_invention_temporal_contract_all_levels() {
    let engine = fresh();
    for level in [GovernanceLevel::Conservative, GovernanceLevel::Moderate, GovernanceLevel::Permissive, GovernanceLevel::Autonomous] {
        let result = engine.create_temporal_contract("test", level);
        assert_eq!(result.initial_level, level);
    }
}

// =========================================================================
// Section 11: Invention — Contract Inheritance
// =========================================================================

#[test]
fn test_invention_contract_inheritance() {
    let engine = populated_engine();
    let policies = engine.list_policies(None);
    let parent_id = policies[0].id;
    let child_id = policies[1].id;
    let result = engine.create_contract_inheritance(parent_id, child_id, true).unwrap();
    assert!(!result.id.to_string().is_empty());
    assert_eq!(result.parent_id, parent_id);
    assert_eq!(result.child_id, child_id);
    assert!(result.propagate_changes);
}

#[test]
fn test_invention_contract_inheritance_nonexistent() {
    let engine = fresh();
    let fake_a = ContractId::new();
    let fake_b = ContractId::new();
    let result = engine.create_contract_inheritance(fake_a, fake_b, false);
    assert!(result.is_err());
}

// =========================================================================
// Section 12: Invention — Smart Escalation
// =========================================================================

#[test]
fn test_invention_smart_escalation() {
    let engine = populated_engine();
    let result = engine.smart_escalate("Rate limit exceeded by 200%", 0.9);
    assert!(!result.recommended_approver.is_empty());
    assert_eq!(result.recommended_approver, "admin"); // urgency > 0.8 routes to admin
}

#[test]
fn test_invention_smart_escalation_low_urgency() {
    let engine = populated_engine();
    let result = engine.smart_escalate("Minor policy concern", 0.3);
    assert_eq!(result.recommended_approver, "manager"); // urgency <= 0.8 routes to manager
}

// =========================================================================
// Section 13: Invention — Violation Archaeology
// =========================================================================

#[test]
fn test_invention_violation_archaeology() {
    let engine = populated_engine();
    let result = engine.violation_archaeology("agent_1", 86400);
    // Should analyze the agent_1 violation ("Rate spike")
    assert_eq!(result.agent_id, "agent_1");
    assert_eq!(result.window_secs, 86400);
}

#[test]
fn test_invention_violation_archaeology_no_violations() {
    let engine = populated_engine();
    let result = engine.violation_archaeology("unknown_agent", 86400);
    assert!(result.clusters.is_empty());
    assert!(result.root_causes.is_empty());
}

// =========================================================================
// Section 14: Invention — Contract Simulation
// =========================================================================

#[test]
fn test_invention_contract_simulation() {
    let engine = populated_engine();
    let result = engine.simulate_contract(100);
    assert_eq!(result.scenario_count, 100);
    assert!(result.approval_rate >= 0.0 && result.approval_rate <= 1.0);
    assert!(result.denial_rate >= 0.0 && result.denial_rate <= 1.0);
    assert!(result.health_score >= 0.0 && result.health_score <= 1.0);
}

#[test]
fn test_invention_contract_simulation_empty() {
    let engine = fresh();
    let result = engine.simulate_contract(50);
    assert_eq!(result.scenario_count, 50);
    // Empty engine should have high health
    assert!(result.health_score >= 0.0);
}

// =========================================================================
// Section 15: Invention — Federated Governance
// =========================================================================

#[test]
fn test_invention_federated_governance() {
    let engine = populated_engine();
    let result = engine.create_federated_governance(
        "Org Alliance",
        vec![("org_1", "Finance"), ("org_2", "Engineering"), ("org_3", "Security")],
        TransparencyLevel::Full,
    );
    assert!(!result.id.to_string().is_empty());
    assert_eq!(result.members.len(), 3);
    assert_eq!(result.name, "Org Alliance");
}

#[test]
fn test_invention_federated_governance_all_transparency() {
    let engine = fresh();
    for level in [TransparencyLevel::Full, TransparencyLevel::Summary, TransparencyLevel::Minimal] {
        let result = engine.create_federated_governance(
            "test",
            vec![("a", "A"), ("b", "B")],
            level,
        );
        assert_eq!(result.transparency, level);
    }
}

// =========================================================================
// Section 16: Invention — Self-Healing Contracts
// =========================================================================

#[test]
fn test_invention_self_healing() {
    let engine = populated_engine();
    let policies = engine.list_policies(None);
    let base_id = policies[0].id;
    let result = engine.create_self_healing_contract(base_id).unwrap();
    assert!(!result.id.to_string().is_empty());
    assert_eq!(result.base_contract_id, base_id);
    assert!(!result.healing_rules.is_empty());
    assert!(result.health_score >= 0.0 && result.health_score <= 1.0);
}

#[test]
fn test_invention_self_healing_nonexistent_base() {
    let engine = fresh();
    let fake_id = ContractId::new();
    let result = engine.create_self_healing_contract(fake_id);
    assert!(result.is_err());
}

// =========================================================================
// Section 17: Large dataset stress
// =========================================================================

#[test]
fn test_stress_2000_policies() {
    let mut engine = fresh();
    for i in 0..2000 {
        let scope = match i % 3 {
            0 => PolicyScope::Global,
            1 => PolicyScope::Session,
            _ => PolicyScope::Agent,
        };
        let action = match i % 4 {
            0 => PolicyAction::Allow,
            1 => PolicyAction::Deny,
            2 => PolicyAction::RequireApproval,
            _ => PolicyAction::AuditOnly,
        };
        engine.add_policy(Policy::new(&format!("Policy {}", i), scope, action));
    }
    let policies = engine.list_policies(None);
    assert_eq!(policies.len(), 2000);
}

#[test]
fn test_stress_1000_risk_limits() {
    let mut engine = fresh();
    for i in 0..1000 {
        let lt = match i % 4 {
            0 => LimitType::Rate,
            1 => LimitType::Threshold,
            2 => LimitType::Budget,
            _ => LimitType::Count,
        };
        engine.add_risk_limit(RiskLimit::new(&format!("Limit {}", i), lt, (i + 1) as f64));
    }
    let limits = engine.list_risk_limits();
    assert_eq!(limits.len(), 1000);
}

#[test]
fn test_stress_2000_violations() {
    let mut engine = fresh();
    for i in 0..2000 {
        let sev = match i % 4 {
            0 => ViolationSeverity::Info,
            1 => ViolationSeverity::Warning,
            2 => ViolationSeverity::Critical,
            _ => ViolationSeverity::Fatal,
        };
        engine.report_violation(Violation::new(
            &format!("Violation {}", i),
            sev,
            &format!("agent_{}", i % 10),
        ));
    }
    let all = engine.list_violations(None);
    assert_eq!(all.len(), 2000);

    let critical = engine.list_violations(Some(ViolationSeverity::Critical));
    assert_eq!(critical.len(), 500);
}

#[test]
fn test_stress_500_obligations() {
    let mut engine = fresh();
    for i in 0..500 {
        let mut o = Obligation::new(
            &format!("Obligation {}", i),
            &format!("Description {}", i),
            &format!("agent_{}", i % 5),
        );
        if i % 2 == 0 {
            o = o.with_deadline(future_dt());
        }
        engine.add_obligation(o);
    }
    let obligations = engine.list_obligations(None);
    assert_eq!(obligations.len(), 500);
}

// =========================================================================
// Section 18: File format roundtrip under load
// =========================================================================

#[test]
fn test_file_roundtrip_loaded() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("roundtrip.acon");

    let mut engine = fresh();
    for i in 0..100 {
        engine.add_policy(Policy::new(&format!("P{}", i), PolicyScope::Global, PolicyAction::Deny));
        engine.add_risk_limit(RiskLimit::new(&format!("L{}", i), LimitType::Rate, 100.0));
        engine.report_violation(Violation::new(&format!("V{}", i), ViolationSeverity::Info, "a1"));
        engine.add_obligation(Obligation::new(&format!("O{}", i), "desc", "a1"));
    }

    engine.file.path = Some(path.clone());
    engine.file.save().unwrap();

    let loaded = ContractEngine::open(&path).unwrap();
    assert_eq!(loaded.file.policies.len(), 100);
    assert_eq!(loaded.file.risk_limits.len(), 100);
    assert_eq!(loaded.file.violations.len(), 100);
    assert_eq!(loaded.file.obligations.len(), 100);
}

#[test]
fn test_file_roundtrip_preserves_data() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("preserve.acon");

    let mut engine = fresh();
    let policy = Policy::new("test policy", PolicyScope::Agent, PolicyAction::RequireApproval)
        .with_description("Important policy");
    let pid = engine.add_policy(policy);
    engine.add_risk_limit(RiskLimit::new("API limit", LimitType::Budget, 99.99));
    engine.report_violation(Violation::new("Test violation", ViolationSeverity::Warning, "agent_x"));

    engine.file.path = Some(path.clone());
    engine.file.save().unwrap();

    let loaded = ContractEngine::open(&path).unwrap();
    let policy = loaded.file.policies.iter().find(|p| p.id == pid).unwrap();
    assert_eq!(policy.label, "test policy");
    assert_eq!(policy.scope, PolicyScope::Agent);
    assert_eq!(policy.action, PolicyAction::RequireApproval);
}

// =========================================================================
// Section 19: Error paths
// =========================================================================

#[test]
fn test_save_to_nonexistent_dir_fails() {
    // open() is lazy — creates in-memory. But save() to a bad path should fail.
    let engine = ContractEngine::open("/nonexistent_dir_xyz/nested/file.acon").unwrap();
    let result = engine.file.save();
    assert!(result.is_err());
}

#[test]
fn test_open_invalid_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("invalid.acon");
    std::fs::write(&path, b"this is not a valid acon file").unwrap();
    let result = ContractEngine::open(&path);
    assert!(result.is_err());
}

#[test]
fn test_open_empty_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("empty.acon");
    std::fs::write(&path, b"").unwrap();
    let result = ContractEngine::open(&path);
    assert!(result.is_err());
}

#[test]
fn test_open_truncated_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("truncated.acon");
    // Write just the magic bytes, not enough for a full header
    std::fs::write(&path, b"ACON").unwrap();
    let result = ContractEngine::open(&path);
    assert!(result.is_err());
}

// =========================================================================
// Section 20: Policy decision logic
// =========================================================================

#[test]
fn test_policy_check_deny_wins() {
    let mut engine = fresh();
    engine.add_policy(Policy::new("deploy", PolicyScope::Global, PolicyAction::Allow));
    engine.add_policy(Policy::new("deploy", PolicyScope::Global, PolicyAction::Deny));
    let result = engine.check_policy("deploy", PolicyScope::Global);
    // Deny should take precedence
    assert_eq!(result, PolicyAction::Deny);
}

#[test]
fn test_policy_check_scope_isolation() {
    let mut engine = fresh();
    engine.add_policy(Policy::new("action", PolicyScope::Global, PolicyAction::Allow));
    engine.add_policy(Policy::new("action", PolicyScope::Agent, PolicyAction::Deny));

    let global_result = engine.check_policy("action", PolicyScope::Global);
    assert_eq!(global_result, PolicyAction::Allow);

    let agent_result = engine.check_policy("action", PolicyScope::Agent);
    assert_eq!(agent_result, PolicyAction::Deny);
}

#[test]
fn test_policy_check_no_match() {
    let mut engine = fresh();
    engine.add_policy(Policy::new("deploy", PolicyScope::Global, PolicyAction::Deny));
    let result = engine.check_policy("read_file", PolicyScope::Global);
    // No matching policy = default allow
    assert_eq!(result, PolicyAction::Allow);
}

// =========================================================================
// Section 21: Risk limit check logic
// =========================================================================

#[test]
fn test_risk_limit_check_within() {
    let mut engine = fresh();
    engine.add_risk_limit(RiskLimit::new("api_calls", LimitType::Rate, 1000.0));
    let exceeded = engine.check_risk_limit("api", 500.0);
    assert!(exceeded.is_none());
}

#[test]
fn test_risk_limit_check_exceeded() {
    let mut engine = fresh();
    engine.add_risk_limit(RiskLimit::new("api_calls", LimitType::Rate, 100.0));
    let exceeded = engine.check_risk_limit("api", 150.0);
    assert!(exceeded.is_some());
}

// =========================================================================
// Section 22: Approval workflow
// =========================================================================

#[test]
fn test_approval_full_workflow() {
    let mut engine = fresh();

    // Add rule
    let rule_id = engine.add_approval_rule(ApprovalRule::new("deploy gate", "deploy:*"));

    // Request approval
    let request_id = engine.request_approval(rule_id, "Deploy v2.0", "agent_1").unwrap();

    // Decide approval
    let decision_id = engine.decide_approval(request_id, DecisionType::Approve, "admin", "LGTM").unwrap();
    assert!(!decision_id.to_string().is_empty());

    // List should show approved
    let requests = engine.list_approval_requests(Some(ApprovalStatus::Approved));
    assert_eq!(requests.len(), 1);
}

#[test]
fn test_approval_deny_workflow() {
    let mut engine = fresh();
    let rule_id = engine.add_approval_rule(ApprovalRule::new("gate", "action:*"));
    let request_id = engine.request_approval(rule_id, "Do something risky", "agent_1").unwrap();
    engine.decide_approval(request_id, DecisionType::Deny, "admin", "Too risky").unwrap();

    let denied = engine.list_approval_requests(Some(ApprovalStatus::Denied));
    assert_eq!(denied.len(), 1);
}

#[test]
fn test_approval_request_nonexistent_rule() {
    let mut engine = fresh();
    let fake_id = ContractId::new();
    let result = engine.request_approval(fake_id, "test", "agent_1");
    assert!(result.is_err());
}

// =========================================================================
// Section 23: Obligation lifecycle
// =========================================================================

#[test]
fn test_obligation_fulfill() {
    let mut engine = fresh();
    let o = Obligation::new("Report", "Submit report", "agent_1").with_deadline(future_dt());
    let oid = engine.add_obligation(o);
    engine.fulfill_obligation(oid).unwrap();

    let fulfilled = engine.list_obligations(Some(ObligationStatus::Fulfilled));
    assert_eq!(fulfilled.len(), 1);
}

#[test]
fn test_obligation_overdue_detection() {
    let mut engine = fresh();
    let past = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(30))
        .unwrap();
    let o = Obligation::new("Late report", "Was due last month", "agent_1").with_deadline(past);
    engine.add_obligation(o);

    let obligations = engine.list_obligations(Some(ObligationStatus::Pending));
    assert!(obligations.iter().any(|o| o.is_overdue()));
}

// =========================================================================
// Section 24: Unicode and edge cases in engine
// =========================================================================

#[test]
fn test_unicode_policy_labels() {
    let mut engine = fresh();
    let labels = [
        "禁止周五部署",
        "政策 🛡️",
        "مرحبا",
        "Ñoño",
        "日本語テスト",
        "한국어 정책",
        "Ελληνικά",
    ];
    for label in &labels {
        engine.add_policy(Policy::new(*label, PolicyScope::Global, PolicyAction::Deny));
    }
    assert_eq!(engine.list_policies(None).len(), labels.len());
}

#[test]
fn test_empty_string_handling() {
    let mut engine = fresh();
    engine.add_policy(Policy::new("", PolicyScope::Global, PolicyAction::Allow));
    engine.add_risk_limit(RiskLimit::new("", LimitType::Rate, 0.0));
    engine.report_violation(Violation::new("", ViolationSeverity::Info, ""));
    // Should not panic
    let stats = engine.stats();
    assert!(stats.policy_count >= 1);
}

// =========================================================================
// Section 25: Stats correctness
// =========================================================================

#[test]
fn test_stats_complete() {
    let engine = populated_engine();
    let stats = engine.stats();
    assert_eq!(stats.policy_count, 5);
    assert_eq!(stats.risk_limit_count, 3);
    assert_eq!(stats.violation_count, 2);
    assert_eq!(stats.obligation_count, 2);
    assert_eq!(stats.critical_violation_count, 1);
    assert!(stats.total_entities > 0);
}

#[test]
fn test_stats_empty_engine() {
    let engine = fresh();
    let stats = engine.stats();
    assert_eq!(stats.policy_count, 0);
    assert_eq!(stats.risk_limit_count, 0);
    assert_eq!(stats.violation_count, 0);
    assert_eq!(stats.total_entities, 0);
}

// =========================================================================
// Section 26: Inventions stress — all 16 on populated engine
// =========================================================================

#[test]
fn test_all_inventions_populated() {
    let engine = populated_engine();
    let policies = engine.list_policies(None);
    let p0 = policies[0].id;
    let p1 = policies[1].id;

    // 1. Policy Omniscience
    let _ = engine.policy_omniscience("agent_1", "deploy");
    // 2. Risk Prophecy
    let _ = engine.risk_prophecy("agent_1", 3600);
    // 3. Approval Telepathy
    let _ = engine.approval_telepathy("deploy");
    // 4. Obligation Clairvoyance
    let _ = engine.obligation_clairvoyance("agent_1", 86400);
    // 5. Violation Precognition
    let _ = engine.violation_precognition("deploy to prod");
    // 6. Contract Crystallization
    let _ = engine.crystallize_contract("agent governance");
    // 7. Policy DNA
    let _ = engine.extract_policy_dna(p0).unwrap();
    // 8. Trust Gradients
    let _ = engine.evaluate_trust_gradient("agent_1", "deploy");
    // 9. Collective Contracts
    let _ = engine.create_collective_contract(
        vec![("a", "A"), ("b", "B")],
        ArbitrationMethod::Automated,
    );
    // 10. Temporal Contracts
    let _ = engine.create_temporal_contract("test", GovernanceLevel::Conservative);
    // 11. Contract Inheritance
    let _ = engine.create_contract_inheritance(p0, p1, true).unwrap();
    // 12. Smart Escalation
    let _ = engine.smart_escalate("urgent", 0.95);
    // 13. Violation Archaeology
    let _ = engine.violation_archaeology("agent_1", 86400);
    // 14. Contract Simulation
    let _ = engine.simulate_contract(50);
    // 15. Federated Governance
    let _ = engine.create_federated_governance(
        "test",
        vec![("org1", "Org1")],
        TransparencyLevel::Summary,
    );
    // 16. Self-Healing Contracts
    let _ = engine.create_self_healing_contract(p0).unwrap();
}

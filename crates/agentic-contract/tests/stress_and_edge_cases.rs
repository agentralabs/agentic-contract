use agentic_contract::{
    ContractEngine, LimitType, Policy, PolicyAction, PolicyScope, RiskLimit, Violation,
    ViolationSeverity,
};

#[test]
fn stress_many_policies() {
    let mut engine = ContractEngine::new();
    for i in 0..1000 {
        let p = Policy::new(
            format!("stress-policy-{}", i),
            PolicyScope::Global,
            PolicyAction::Allow,
        );
        engine.add_policy(p);
    }
    assert_eq!(engine.list_policies(None).len(), 1000);
}

#[test]
fn stress_many_risk_limits() {
    let mut engine = ContractEngine::new();
    for i in 0..500 {
        let l = RiskLimit::new(format!("stress-limit-{}", i), LimitType::Threshold, 100.0);
        engine.add_risk_limit(l);
    }
    assert_eq!(engine.list_risk_limits().len(), 500);
}

#[test]
fn stress_many_violations() {
    let mut engine = ContractEngine::new();
    for i in 0..1000 {
        let v = Violation::new(
            format!("stress-violation-{}", i),
            ViolationSeverity::Info,
            "stress-agent",
        );
        engine.report_violation(v);
    }
    assert_eq!(engine.list_violations(None).len(), 1000);
}

#[test]
fn edge_empty_policy_label() {
    let mut engine = ContractEngine::new();
    let p = Policy::new("", PolicyScope::Global, PolicyAction::Deny);
    engine.add_policy(p);
    assert_eq!(engine.list_policies(None).len(), 1);
}

#[test]
fn edge_unicode_policy_labels() {
    let mut engine = ContractEngine::new();
    engine.add_policy(Policy::new(
        "政策：不允许",
        PolicyScope::Global,
        PolicyAction::Deny,
    ));
    engine.add_policy(Policy::new(
        "📋 Règle de sécurité",
        PolicyScope::Session,
        PolicyAction::Allow,
    ));
    engine.add_policy(Policy::new(
        "Richtlinie: Sicherheit 🔒",
        PolicyScope::Agent,
        PolicyAction::AuditOnly,
    ));
    assert_eq!(engine.list_policies(None).len(), 3);
}

#[test]
fn edge_zero_max_risk_limit() {
    let mut engine = ContractEngine::new();
    engine.add_risk_limit(RiskLimit::new("zero-limit", LimitType::Budget, 0.0));
    let limits = engine.list_risk_limits();
    assert_eq!(limits.len(), 1);
    assert_eq!(limits[0].max_value, 0.0);
}

#[test]
fn edge_very_large_risk_value() {
    let mut engine = ContractEngine::new();
    engine.add_risk_limit(RiskLimit::new("huge-limit", LimitType::Count, f64::MAX));
    let limits = engine.list_risk_limits();
    assert_eq!(limits[0].max_value, f64::MAX);
}

#[test]
fn edge_check_policy_empty_action() {
    let mut engine = ContractEngine::new();
    engine.add_policy(Policy::new(
        "block-empty",
        PolicyScope::Global,
        PolicyAction::Deny,
    ));
    let _ = engine.check_policy("", PolicyScope::Global);
}

#[test]
fn boundary_all_policy_scopes() {
    let mut engine = ContractEngine::new();
    engine.add_policy(Policy::new(
        "global",
        PolicyScope::Global,
        PolicyAction::Allow,
    ));
    engine.add_policy(Policy::new(
        "session",
        PolicyScope::Session,
        PolicyAction::Deny,
    ));
    engine.add_policy(Policy::new(
        "agent",
        PolicyScope::Agent,
        PolicyAction::RequireApproval,
    ));
    assert_eq!(engine.list_policies(None).len(), 3);
}

#[test]
fn boundary_all_violation_severities() {
    let mut engine = ContractEngine::new();
    engine.report_violation(Violation::new("info", ViolationSeverity::Info, "agent"));
    engine.report_violation(Violation::new("warn", ViolationSeverity::Warning, "agent"));
    engine.report_violation(Violation::new("crit", ViolationSeverity::Critical, "agent"));
    engine.report_violation(Violation::new("fatal", ViolationSeverity::Fatal, "agent"));
    assert_eq!(engine.list_violations(None).len(), 4);
}

#[test]
fn boundary_all_limit_types() {
    let mut engine = ContractEngine::new();
    engine.add_risk_limit(RiskLimit::new("rate", LimitType::Rate, 100.0));
    engine.add_risk_limit(RiskLimit::new("threshold", LimitType::Threshold, 200.0));
    engine.add_risk_limit(RiskLimit::new("budget", LimitType::Budget, 300.0));
    engine.add_risk_limit(RiskLimit::new("count", LimitType::Count, 400.0));
    assert_eq!(engine.list_risk_limits().len(), 4);
}

#[test]
fn stress_file_roundtrip_heavy() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("heavy.acon");

    let mut engine = ContractEngine::open(path.clone()).unwrap();
    for i in 0..200 {
        engine.add_policy(Policy::new(
            format!("heavy-policy-{}", i),
            PolicyScope::Global,
            PolicyAction::Allow,
        ));
        engine.add_risk_limit(RiskLimit::new(
            format!("heavy-limit-{}", i),
            LimitType::Threshold,
            100.0,
        ));
        engine.report_violation(Violation::new(
            format!("heavy-violation-{}", i),
            ViolationSeverity::Info,
            "agent",
        ));
    }

    engine.save().unwrap();

    let loaded = ContractEngine::open(path).unwrap();
    assert_eq!(loaded.list_policies(None).len(), 200);
    assert_eq!(loaded.list_risk_limits().len(), 200);
    assert_eq!(loaded.list_violations(None).len(), 200);
}

#[test]
fn edge_special_characters_in_labels() {
    let mut engine = ContractEngine::new();
    engine.add_policy(Policy::new(
        "policy with \"quotes\" and 'apostrophes'",
        PolicyScope::Global,
        PolicyAction::Allow,
    ));
    engine.add_policy(Policy::new(
        "policy\twith\ttabs\nand\nnewlines",
        PolicyScope::Session,
        PolicyAction::Deny,
    ));
    engine.add_policy(Policy::new(
        "policy/with/slashes\\and\\backslashes",
        PolicyScope::Agent,
        PolicyAction::AuditOnly,
    ));
    assert_eq!(engine.list_policies(None).len(), 3);
}

#[test]
fn heavy_concurrent_stats() {
    let mut engine = ContractEngine::new();
    for i in 0..500 {
        engine.add_policy(Policy::new(
            format!("stats-policy-{}", i),
            PolicyScope::Global,
            PolicyAction::Allow,
        ));
    }
    for _ in 0..100 {
        let stats = engine.stats();
        assert_eq!(stats.policy_count, 500);
    }
}

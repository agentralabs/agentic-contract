use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use agentic_contract::{
    ContractEngine, LimitType, Policy, PolicyAction, PolicyScope, RiskLimit, Violation,
    ViolationSeverity,
};

fn bench_policy_evaluate(c: &mut Criterion) {
    let mut engine = ContractEngine::new();
    engine.add_policy(Policy::new(
        "test-policy",
        PolicyScope::Global,
        PolicyAction::Deny,
    ));

    c.bench_function("policy_evaluate_single", |b| {
        b.iter(|| {
            engine.check_policy("test action", PolicyScope::Global);
        });
    });
}

fn bench_risk_limit_check(c: &mut Criterion) {
    let mut engine = ContractEngine::new();
    engine.add_risk_limit(RiskLimit::new("api-calls", LimitType::Rate, 1000.0));

    c.bench_function("risk_limit_check", |b| {
        b.iter(|| {
            engine.check_risk_limit("api-calls", 1.0);
        });
    });
}

fn bench_policy_evaluate_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("policy_evaluate_scale");

    for count in [10, 100, 1000] {
        let mut engine = ContractEngine::new();
        for i in 0..count {
            engine.add_policy(Policy::new(
                format!("policy-{}", i),
                PolicyScope::Global,
                if i % 2 == 0 {
                    PolicyAction::Allow
                } else {
                    PolicyAction::AuditOnly
                },
            ));
        }

        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, _| {
            b.iter(|| {
                engine.check_policy("deploy to production", PolicyScope::Global);
            });
        });
    }

    group.finish();
}

fn bench_violation_report(c: &mut Criterion) {
    let mut engine = ContractEngine::new();

    c.bench_function("violation_report", |b| {
        b.iter(|| {
            engine.report_violation(Violation::new(
                "test violation",
                ViolationSeverity::Warning,
                "agent-1",
            ));
        });
    });
}

fn bench_engine_stats(c: &mut Criterion) {
    let mut engine = ContractEngine::new();
    for i in 0..100 {
        engine.add_policy(Policy::new(
            format!("policy-{}", i),
            PolicyScope::Global,
            PolicyAction::Allow,
        ));
    }

    c.bench_function("engine_stats", |b| {
        b.iter(|| {
            engine.stats();
        });
    });
}

fn bench_file_roundtrip(c: &mut Criterion) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bench.acon");

    let mut engine = ContractEngine::open(path.clone()).unwrap();
    for i in 0..100 {
        engine.add_policy(Policy::new(
            format!("policy-{}", i),
            PolicyScope::Global,
            PolicyAction::Allow,
        ));
        engine.add_risk_limit(RiskLimit::new(
            format!("limit-{}", i),
            LimitType::Threshold,
            100.0,
        ));
    }

    c.bench_function("file_save_100_entities", |b| {
        b.iter(|| {
            engine.save().unwrap();
        });
    });

    engine.save().unwrap();

    c.bench_function("file_load_100_entities", |b| {
        b.iter(|| {
            let _ = ContractEngine::open(path.clone()).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_policy_evaluate,
    bench_risk_limit_check,
    bench_policy_evaluate_scale,
    bench_violation_report,
    bench_engine_stats,
    bench_file_roundtrip,
);
criterion_main!(benches);

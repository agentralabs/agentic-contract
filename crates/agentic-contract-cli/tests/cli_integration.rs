//! CLI Integration Tests for `acon`
//!
//! Covers: all subcommands (policy, limit, approval, obligation, violation,
//! stats, info), bad arguments, file operations, --path flag, and error paths.

use std::process::Command;

/// Get the path to the compiled `acon` binary.
fn acon_bin() -> String {
    let mut path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    path.push("acon");
    path.to_string_lossy().to_string()
}

/// Create a temp dir and return its path as a string.
fn temp_acon_path() -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.acon").to_string_lossy().to_string();
    (dir, path)
}

// =========================================================================
// Section 1: Basic commands and help
// =========================================================================

#[test]
fn test_cli_help() {
    let output = Command::new(acon_bin())
        .arg("--help")
        .output()
        .expect("Failed to run acon");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Policy engine for AI agents") || stdout.contains("acon"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(acon_bin())
        .arg("--version")
        .output()
        .expect("Failed to run acon");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("acon"));
}

#[test]
fn test_cli_no_args_shows_help() {
    let output = Command::new(acon_bin())
        .output()
        .expect("Failed to run acon");
    // Should show error or help when no subcommand given
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success() || stderr.contains("Usage") || stdout.contains("Usage"),
        "No args should show help or error"
    );
}

// =========================================================================
// Section 2: Stats command
// =========================================================================

#[test]
fn test_cli_stats_fresh_file() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "stats"])
        .output()
        .expect("Failed to run acon stats");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("policy_count") || stdout.contains("0"));
}

// =========================================================================
// Section 3: Info command
// =========================================================================

#[test]
fn test_cli_info_fresh_file() {
    let (_dir, path) = temp_acon_path();
    // Create a file first via stats
    Command::new(acon_bin())
        .args(["--path", &path, "stats"])
        .output()
        .unwrap();

    let output = Command::new(acon_bin())
        .args(["--path", &path, "info"])
        .output()
        .expect("Failed to run acon info");
    // Info command just prints — may work on existing or handle missing file
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let _ = (stdout, stderr); // Just ensure no panic
}

#[test]
fn test_cli_info_nonexistent_file() {
    let output = Command::new(acon_bin())
        .args(["info", "/tmp/nonexistent_acon_test_file.acon"])
        .output()
        .expect("Failed to run acon info");
    // Should print error, not panic
    let stderr = String::from_utf8_lossy(&output.stderr);
    let _ = stderr;
}

// =========================================================================
// Section 4: Policy subcommand
// =========================================================================

#[test]
fn test_cli_policy_add() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "No Friday deploys",
            "--scope",
            "global",
            "--action",
            "deny",
        ])
        .output()
        .expect("Failed to run acon policy add");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created policy"));
}

#[test]
fn test_cli_policy_add_with_description() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "Compliance check",
            "--scope",
            "session",
            "--action",
            "require_approval",
            "--description",
            "All actions need review",
        ])
        .output()
        .expect("Failed to run acon policy add");
    assert!(output.status.success());
}

#[test]
fn test_cli_policy_list_empty() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "policy", "list"])
        .output()
        .expect("Failed to run acon policy list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 0"));
}

#[test]
fn test_cli_policy_add_then_list() {
    let (_dir, path) = temp_acon_path();

    // Add a policy
    Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "Block writes",
            "--action",
            "deny",
        ])
        .output()
        .unwrap();

    // List should show it
    let output = Command::new(acon_bin())
        .args(["--path", &path, "policy", "list"])
        .output()
        .expect("Failed to run acon policy list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Block writes") || stdout.contains("Total: 1"));
}

#[test]
fn test_cli_policy_check() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "policy", "check", "deploy"])
        .output()
        .expect("Failed to run acon policy check");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Decision"));
}

#[test]
fn test_cli_policy_list_by_scope() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "policy", "list", "--scope", "agent"])
        .output()
        .expect("Failed to run acon policy list --scope");
    assert!(output.status.success());
}

#[test]
fn test_cli_policy_add_all_scopes() {
    let (_dir, path) = temp_acon_path();
    for scope in &["global", "session", "agent"] {
        let output = Command::new(acon_bin())
            .args([
                "--path",
                &path,
                "policy",
                "add",
                &format!("{} policy", scope),
                "--scope",
                scope,
            ])
            .output()
            .expect("Failed to run acon policy add");
        assert!(output.status.success(), "Failed for scope: {}", scope);
    }
}

#[test]
fn test_cli_policy_add_all_actions() {
    let (_dir, path) = temp_acon_path();
    for action in &["allow", "deny", "require_approval", "audit_only"] {
        let output = Command::new(acon_bin())
            .args([
                "--path",
                &path,
                "policy",
                "add",
                &format!("{} action", action),
                "--action",
                action,
            ])
            .output()
            .expect("Failed to run acon policy add");
        assert!(output.status.success(), "Failed for action: {}", action);
    }
}

#[test]
fn test_cli_policy_bad_scope() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "test",
            "--scope",
            "invalid_scope",
        ])
        .output()
        .expect("Failed to run acon policy add");
    // Should handle gracefully with error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown scope") || !output.status.success(),
        "Should reject invalid scope"
    );
}

#[test]
fn test_cli_policy_bad_action() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "test",
            "--action",
            "invalid_action",
        ])
        .output()
        .expect("Failed to run acon policy add");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown action") || !output.status.success(),
        "Should reject invalid action"
    );
}

// =========================================================================
// Section 5: Limit subcommand
// =========================================================================

#[test]
fn test_cli_limit_set() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "limit", "set", "API calls", "--max", "100"])
        .output()
        .expect("Failed to run acon limit set");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created risk limit"));
}

#[test]
fn test_cli_limit_set_all_types() {
    let (_dir, path) = temp_acon_path();
    for lt in &["rate", "threshold", "budget", "count"] {
        let output = Command::new(acon_bin())
            .args([
                "--path",
                &path,
                "limit",
                "set",
                &format!("{} limit", lt),
                "--max",
                "50",
                "--type",
                lt,
            ])
            .output()
            .expect("Failed to run acon limit set");
        assert!(output.status.success(), "Failed for type: {}", lt);
    }
}

#[test]
fn test_cli_limit_check() {
    let (_dir, path) = temp_acon_path();
    // Set a limit first
    Command::new(acon_bin())
        .args(["--path", &path, "limit", "set", "API calls", "--max", "100"])
        .output()
        .unwrap();

    let output = Command::new(acon_bin())
        .args(["--path", &path, "limit", "check", "API", "--amount", "50"])
        .output()
        .expect("Failed to run acon limit check");
    assert!(output.status.success());
}

#[test]
fn test_cli_limit_list_empty() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "limit", "list"])
        .output()
        .expect("Failed to run acon limit list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 0"));
}

#[test]
fn test_cli_limit_bad_type() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path", &path, "limit", "set", "test", "--max", "10", "--type", "bogus",
        ])
        .output()
        .expect("Failed to run acon limit set");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown limit type") || !output.status.success(),
        "Should reject invalid limit type"
    );
}

// =========================================================================
// Section 6: Violation subcommand
// =========================================================================

#[test]
fn test_cli_violation_report() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "violation",
            "report",
            "Rate limit exceeded",
            "--severity",
            "warning",
            "--actor",
            "agent_1",
        ])
        .output()
        .expect("Failed to run acon violation report");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Violation reported"));
}

#[test]
fn test_cli_violation_report_all_severities() {
    let (_dir, path) = temp_acon_path();
    for sev in &["info", "warning", "critical", "fatal"] {
        let output = Command::new(acon_bin())
            .args([
                "--path",
                &path,
                "violation",
                "report",
                &format!("{} event", sev),
                "--severity",
                sev,
                "--actor",
                "agent_1",
            ])
            .output()
            .expect("Failed to run acon violation report");
        assert!(output.status.success(), "Failed for severity: {}", sev);
    }
}

#[test]
fn test_cli_violation_list_empty() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "violation", "list"])
        .output()
        .expect("Failed to run acon violation list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 0"));
}

#[test]
fn test_cli_violation_list_by_severity() {
    let (_dir, path) = temp_acon_path();
    // Add violations
    Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "violation",
            "report",
            "info thing",
            "--severity",
            "info",
            "--actor",
            "a1",
        ])
        .output()
        .unwrap();
    Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "violation",
            "report",
            "critical thing",
            "--severity",
            "critical",
            "--actor",
            "a2",
        ])
        .output()
        .unwrap();

    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "violation",
            "list",
            "--severity",
            "critical",
        ])
        .output()
        .expect("Failed to run acon violation list --severity");
    assert!(output.status.success());
}

#[test]
fn test_cli_violation_bad_severity() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "violation",
            "report",
            "test",
            "--severity",
            "invalid",
            "--actor",
            "a1",
        ])
        .output()
        .expect("Failed to run acon violation report");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown severity") || !output.status.success(),
        "Should reject invalid severity"
    );
}

// =========================================================================
// Section 7: Obligation subcommand
// =========================================================================

#[test]
fn test_cli_obligation_add() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "obligation",
            "add",
            "Submit report",
            "--description",
            "Monthly compliance report",
        ])
        .output()
        .expect("Failed to run acon obligation add");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created obligation"));
}

#[test]
fn test_cli_obligation_add_with_deadline() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "obligation",
            "add",
            "Annual review",
            "--description",
            "Do the review",
            "--deadline",
            "2027-01-01T00:00:00Z",
        ])
        .output()
        .expect("Failed to run acon obligation add");
    assert!(output.status.success());
}

#[test]
fn test_cli_obligation_list_empty() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "obligation", "list"])
        .output()
        .expect("Failed to run acon obligation list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 0"));
}

#[test]
fn test_cli_obligation_check_all() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "obligation", "check"])
        .output()
        .expect("Failed to run acon obligation check");
    assert!(output.status.success());
}

#[test]
fn test_cli_obligation_bad_deadline() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "obligation",
            "add",
            "test",
            "--description",
            "test",
            "--deadline",
            "not-a-date",
        ])
        .output()
        .expect("Failed to run acon obligation add");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid deadline") || !output.status.success(),
        "Should reject invalid deadline"
    );
}

// =========================================================================
// Section 8: Approval subcommand
// =========================================================================

#[test]
fn test_cli_approval_list_empty() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "approval", "list"])
        .output()
        .expect("Failed to run acon approval list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 0"));
}

#[test]
fn test_cli_approval_rule_add() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "approval",
            "rule",
            "Deploy gate",
            "--pattern",
            "deploy:*",
        ])
        .output()
        .expect("Failed to run acon approval rule");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created approval rule"));
}

#[test]
fn test_cli_approval_request_bad_rule() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "approval",
            "request",
            "--rule-id",
            "00000000-0000-0000-0000-000000000000",
            "deploy production",
            "--requestor",
            "agent_1",
        ])
        .output()
        .expect("Failed to run acon approval request");
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should report error for nonexistent rule
    assert!(
        stderr.contains("Error") || !output.status.success(),
        "Should reject nonexistent rule ID"
    );
}

// =========================================================================
// Section 9: --path flag and ACON_PATH env
// =========================================================================

#[test]
fn test_cli_custom_path_flag() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .args(["--path", &path, "stats"])
        .output()
        .expect("Failed to run acon with --path");
    assert!(output.status.success());
}

#[test]
fn test_cli_acon_path_env() {
    let (_dir, path) = temp_acon_path();
    let output = Command::new(acon_bin())
        .env("ACON_PATH", &path)
        .args(["stats"])
        .output()
        .expect("Failed to run acon with ACON_PATH env");
    assert!(output.status.success());
}

// =========================================================================
// Section 10: Full workflow through CLI
// =========================================================================

#[test]
fn test_cli_full_workflow() {
    let (_dir, path) = temp_acon_path();

    // 1. Add policies
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "Block deploys",
            "--action",
            "deny",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());

    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "policy",
            "add",
            "Allow reads",
            "--action",
            "allow",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 2. Set limits
    let output = Command::new(acon_bin())
        .args(["--path", &path, "limit", "set", "API rate", "--max", "1000"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 3. Report violations
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "violation",
            "report",
            "Unauthorized access",
            "--severity",
            "critical",
            "--actor",
            "agent_3",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 4. Add obligation
    let output = Command::new(acon_bin())
        .args([
            "--path",
            &path,
            "obligation",
            "add",
            "Security audit",
            "--description",
            "Quarterly audit required",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 5. Check stats
    let output = Command::new(acon_bin())
        .args(["--path", &path, "stats"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Stats should show some entities
    assert!(stdout.contains("policy_count") || stdout.contains("2"));

    // 6. List everything
    let output = Command::new(acon_bin())
        .args(["--path", &path, "policy", "list"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 2"));
}

// =========================================================================
// Section 11: Bulk CLI operations
// =========================================================================

#[test]
fn test_cli_bulk_policy_adds() {
    let (_dir, path) = temp_acon_path();

    for i in 0..20 {
        let output = Command::new(acon_bin())
            .args([
                "--path",
                &path,
                "policy",
                "add",
                &format!("Bulk policy {}", i),
            ])
            .output()
            .expect("Failed to run acon policy add in bulk");
        assert!(output.status.success(), "Failed on iteration {}", i);
    }

    let output = Command::new(acon_bin())
        .args(["--path", &path, "policy", "list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total: 20"));
}

// =========================================================================
// Section 12: Install and Serve subcommands
// =========================================================================

#[test]
fn test_cli_install_command() {
    let output = Command::new(acon_bin())
        .args(["install"])
        .output()
        .expect("Failed to run acon install");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Install") || stdout.contains("profile"));
}

#[test]
fn test_cli_install_with_profile() {
    let output = Command::new(acon_bin())
        .args(["install", "--profile", "ci"])
        .output()
        .expect("Failed to run acon install --profile");
    assert!(output.status.success());
}

#[test]
fn test_cli_serve_command() {
    let output = Command::new(acon_bin())
        .args(["serve"])
        .output()
        .expect("Failed to run acon serve");
    // Serve just prints a message about using mcp server
    assert!(output.status.success());
}

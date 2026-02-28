//! Integration tests for the CLI binary.
//!
//! Verifies that the `acon` binary exists and responds to basic flags.
//!
//! This test is registered as a [[test]] in the agentic-contract-cli crate
//! so that CARGO_BIN_EXE_acon is available.

use std::process::Command;

/// Get a Command pointing to the `acon` binary.
fn acon_binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_acon"))
}

#[test]
fn cli_responds_to_help() {
    let output = acon_binary()
        .arg("--help")
        .output()
        .expect("failed to execute acon --help");

    assert!(
        output.status.success(),
        "acon --help should exit with success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("acon") || stdout.contains("AgenticContract") || stdout.contains("Usage"),
        "acon --help output should contain usage information, got: {stdout}"
    );
}

#[test]
fn cli_responds_to_version() {
    let output = acon_binary()
        .arg("--version")
        .output()
        .expect("failed to execute acon --version");

    assert!(
        output.status.success(),
        "acon --version should exit with success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("0.") || stdout.contains("agentic-contract"),
        "acon --version output should contain a version number, got: {stdout}"
    );
}

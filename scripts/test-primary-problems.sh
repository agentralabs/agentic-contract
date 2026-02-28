#!/usr/bin/env bash
# test-primary-problems.sh — Validate primary problem coverage for AgenticContract
set -euo pipefail

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

assert_contains() {
  local text="$1"
  local pattern="$2"
  local label="$3"
  if command -v rg >/dev/null 2>&1; then
    printf '%s' "$text" | rg -q --fixed-strings "$pattern" || fail "${label}: missing '${pattern}'"
  else
    printf '%s' "$text" | grep -q -F -- "$pattern" || fail "${label}: missing '${pattern}'"
  fi
}

assert_file() {
  [ -f "$1" ] || fail "Missing required file: $1"
}

run_acon() {
  cargo run --quiet -p agentic-contract-cli --bin acon -- "$@"
}

tmpdir="$(mktemp -d)"
acon_file="$tmpdir/primary.acon"

echo "[1/8] Create contract artifact"
# acon creates on first use via open_or_create
stats_out="$(run_acon --path "$acon_file" stats)"
assert_contains "$stats_out" "policies" "create stats"

echo "[2/8] Add policies (core governance)"
policy_out="$(run_acon --path "$acon_file" policy add --label "No unreviewed deploys" --action deny --scope global)"
assert_contains "$policy_out" "Created policy" "policy add"
policy_out2="$(run_acon --path "$acon_file" policy add --label "Allow read-only queries" --action allow --scope session)"
assert_contains "$policy_out2" "Created policy" "policy add session"

echo "[3/8] Policy enforcement check"
check_out="$(run_acon --path "$acon_file" policy check --action allow --scope session)"
assert_contains "$check_out" "allow" "policy check"

echo "[4/8] Risk limit lifecycle"
limit_out="$(run_acon --path "$acon_file" limit set --label "API calls per minute" --limit-type rate --max-value 100)"
assert_contains "$limit_out" "Created risk limit" "limit set"
limit_list="$(run_acon --path "$acon_file" limit list)"
assert_contains "$limit_list" "API calls per minute" "limit list"

echo "[5/8] Approval workflow"
rule_out="$(run_acon --path "$acon_file" approval rule --label "Deploy approval" --approvers admin --scope global)"
assert_contains "$rule_out" "Created approval rule" "approval rule"
req_out="$(run_acon --path "$acon_file" approval request --description "Deploy v1.2" --requestor dev-agent)"
assert_contains "$req_out" "Created approval request" "approval request"

echo "[6/8] Obligation tracking"
obligation_out="$(run_acon --path "$acon_file" obligation add --label "Complete audit before release" --deadline 2099-12-31)"
assert_contains "$obligation_out" "Created obligation" "obligation add"
obligation_check="$(run_acon --path "$acon_file" obligation check)"
assert_contains "$obligation_check" "Complete audit" "obligation check"

echo "[7/8] Violation reporting"
violation_out="$(run_acon --path "$acon_file" violation report --description "Unauthorized API access" --severity critical)"
assert_contains "$violation_out" "Recorded violation" "violation report"
violation_list="$(run_acon --path "$acon_file" violation list)"
assert_contains "$violation_list" "Unauthorized API access" "violation list"

echo "[8/8] Validate focused regression tests"
cargo test --quiet -p agentic-contract --lib
cargo test --quiet -p agentic-contract-mcp --test edge_cases_inventions

# Documentation existence check
assert_file "docs/public/primary-problem-coverage.md"
assert_file "docs/public/initial-problem-coverage.md"

echo "Primary contract problem checks passed (P01-ungoverned,P02-no-policy,P03-risk-blindness,P04-no-approval,P05-no-obligations,P06-no-violations)"

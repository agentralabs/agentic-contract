"""AgenticContract — Policy engine for AI agents.

This Python SDK wraps the `acon` CLI binary via subprocess.
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any, Optional


__version__ = "0.1.0"


class ContractEngine:
    """High-level wrapper around the acon CLI."""

    def __init__(self, path: Optional[str] = None, binary: str = "acon"):
        self._binary = binary
        self._path = path or str(
            Path.home() / ".agentic" / "contract.acon"
        )

    def _run(self, *args: str) -> str:
        """Run an acon CLI command and return stdout."""
        cmd = [self._binary, "--path", self._path, *args]
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=30
        )
        if result.returncode != 0:
            raise RuntimeError(
                f"acon failed (exit {result.returncode}): {result.stderr.strip()}"
            )
        return result.stdout.strip()

    def stats(self) -> dict[str, Any]:
        """Get contract statistics."""
        output = self._run("stats")
        return json.loads(output)

    def policy_add(
        self,
        label: str,
        scope: str = "global",
        action: str = "deny",
        description: Optional[str] = None,
    ) -> str:
        """Add a policy and return its ID."""
        args = ["policy", "add", label, "--scope", scope, "--action", action]
        if description:
            args.extend(["--description", description])
        output = self._run(*args)
        # Output: "Created policy: <uuid>"
        return output.split(": ", 1)[-1]

    def policy_check(self, action_type: str, scope: str = "global") -> str:
        """Check if an action is allowed. Returns the decision string."""
        output = self._run("policy", "check", action_type, "--scope", scope)
        return output

    def risk_limit_set(
        self,
        label: str,
        max_value: float,
        limit_type: str = "threshold",
    ) -> str:
        """Set a risk limit and return its ID."""
        output = self._run(
            "limit", "set", label, "--max", str(max_value), "--type", limit_type
        )
        return output.split(": ", 1)[-1]

    def violation_report(
        self,
        description: str,
        severity: str = "warning",
        actor: str = "unknown",
    ) -> str:
        """Report a violation and return its ID."""
        output = self._run(
            "violation", "report", description,
            "--severity", severity, "--actor", actor,
        )
        return output.split(": ", 1)[-1]

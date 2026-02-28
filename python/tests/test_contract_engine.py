"""Basic tests for the AgenticContract Python SDK."""

import pytest

from agentic_contract import ContractEngine


def test_import():
    """Verify the module imports correctly."""
    assert ContractEngine is not None


def test_version():
    """Verify version is set."""
    import agentic_contract
    assert agentic_contract.__version__ == "0.1.0"


def test_engine_init():
    """Verify engine can be instantiated."""
    engine = ContractEngine(path="/tmp/test.acon")
    assert engine._path == "/tmp/test.acon"


def test_engine_custom_binary():
    """Verify engine accepts custom binary path."""
    engine = ContractEngine(binary="/usr/local/bin/acon")
    assert engine._binary == "/usr/local/bin/acon"

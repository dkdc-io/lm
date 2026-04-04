"""Tests for dkdc-lm Python bindings."""

import pytest

import dkdc_lm


# -- constants -----------------------------------------------------------------


def test_default_port():
    assert dkdc_lm.default_port() == 8080


def test_default_builtin():
    assert dkdc_lm.default_builtin() == "gemma-4-26b-a4b-it"


def test_tmux_session():
    assert dkdc_lm.tmux_session() == "dkdc-lm"


# -- resolve_builtin -----------------------------------------------------------


def test_resolve_builtin_default():
    args = dkdc_lm.resolve_builtin(dkdc_lm.default_builtin())
    assert args == ["-hf", "ggml-org/gemma-4-26B-A4B-it-GGUF"]


def test_resolve_builtin_small():
    args = dkdc_lm.resolve_builtin("gemma-4-e4b-it")
    assert args == ["-hf", "ggml-org/gemma-4-E4B-it-GGUF"]


def test_resolve_builtin_unknown():
    with pytest.raises(RuntimeError, match="unknown built-in model"):
        dkdc_lm.resolve_builtin("nonexistent")


def test_resolve_builtin_empty():
    with pytest.raises(RuntimeError, match="unknown built-in model"):
        dkdc_lm.resolve_builtin("")


def test_resolve_builtin_case_sensitive():
    with pytest.raises(RuntimeError):
        dkdc_lm.resolve_builtin("Gemma-4-26b-a4b-it")


# -- is_running (no tmux session expected in CI) --------------------------------


def test_is_running_returns_bool():
    result = dkdc_lm.is_running()
    assert isinstance(result, bool)


# -- status (no server expected) ------------------------------------------------


def test_status_returns_tuple():
    result = dkdc_lm.status()
    assert isinstance(result, tuple)
    assert len(result) == 2
    assert all(isinstance(v, bool) for v in result)


def test_status_custom_port():
    result = dkdc_lm.status(port=19999)
    assert result == (False, False) or isinstance(result, tuple)


# -- exports -------------------------------------------------------------------


def test_all_exports_present():
    expected = [
        "start",
        "stop",
        "attach",
        "status",
        "is_running",
        "logs",
        "resolve_builtin",
        "default_port",
        "default_builtin",
        "tmux_session",
    ]
    for name in expected:
        assert hasattr(dkdc_lm, name), f"missing export: {name}"
        assert callable(getattr(dkdc_lm, name)), f"not callable: {name}"

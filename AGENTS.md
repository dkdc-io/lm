# dkdc-ai

Local LLM inference management via llama-server.

## Commands

```bash
bin/build          # Build all (Rust + Python)
bin/build-rs       # Build Rust crates
bin/build-py       # Build Python bindings (maturin develop)
bin/check          # Run all checks (format, lint, test)
bin/check-rs       # Rust checks (fmt, clippy, test)
bin/check-py       # Python checks (ruff, ty)
bin/test           # Run all tests
bin/format         # Format all code
bin/bump-version   # Bump version (--patch, --minor (default), --major)
```

## Architecture

```
crates/dkdc-ai-core/       # Core library (dkdc-ai on crates.io)
  src/lib.rs                 # Lifecycle (start/stop/status/logs), model resolution
crates/dkdc-ai-cli/        # CLI binary (dkdc-ai)
  src/main.rs                # Clap CLI wrapping core library
crates/dkdc-ai-py/         # PyO3 bindings (cdylib)
py/dkdc_ai/                # Python wrapper + type stubs
```

## Dependencies

- `dkdc-sh` — tmux session management
- `reqwest` — health check HTTP calls

## Default models

- `gemma-4-26b-a4b-it` — Gemma 4 26B MoE (4B active params, fast + high quality)
- `gemma-4-e4b-it` — Gemma 4 E4B dense (lighter option)

Both via ggml-org GGUF repos on HuggingFace.

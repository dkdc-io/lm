# dkdc-lm

[![GitHub Release](https://img.shields.io/github/v/release/dkdc-io/lm?color=blue)](https://github.com/dkdc-io/lm/releases)
[![PyPI](https://img.shields.io/pypi/v/dkdc-lm?color=blue)](https://pypi.org/project/dkdc-lm/)
[![crates.io](https://img.shields.io/crates/v/dkdc-lm?color=blue)](https://crates.io/crates/dkdc-lm)
[![CI](https://img.shields.io/github/actions/workflow/status/dkdc-io/lm/ci.yml?branch=main&label=CI)](https://github.com/dkdc-io/lm/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-8A2BE2.svg)](https://github.com/dkdc-io/lm/blob/main/LICENSE)

Language model service.

## Install

Recommended:

```bash
curl -LsSf https://dkdc.sh/lm/install.sh | sh
```

Pre-built binaries are available for Linux and macOS via Python (`uv`). Windows users should install via `cargo` or use macOS/Linux.

uv:

```bash
uv tool install dkdc-lm
```

cargo:

```bash
cargo install dkdc-lm-cli
```

Verify installation:

```bash
lm --version
```

You can use `uvx` to run it without installing:

```bash
uvx --from dkdc-lm lm
```

## Usage

### CLI

```bash
lm start                              # Start with default model (Gemma 4 26B-A4B)
lm start --builtin gemma-4-e4b-it     # Start with smaller model
lm start --hf ggml-org/some-model-GGUF  # Start with HuggingFace model
lm start --model /path/to/model.gguf  # Start with local model
lm status                             # Check server status
lm logs                               # View recent logs
lm attach                             # Attach to tmux session
lm stop                               # Stop server
```

### Rust

```rust
use dkdc_lm::{start, stop, status, resolve_builtin, DEFAULT_PORT};

let args = resolve_builtin("gemma-4-26b-a4b-it")?;
start(&args, DEFAULT_PORT, -1, 4096)?;

let (tmux, http) = status(DEFAULT_PORT);
println!("tmux: {}, http: {}", tmux, http);

stop()?;
```

### Python

```python
import dkdc_lm

args = dkdc_lm.resolve_builtin(dkdc_lm.default_builtin())
dkdc_lm.start(args)

tmux_running, http_responding = dkdc_lm.status()
print(f"tmux: {tmux_running}, http: {http_responding}")

dkdc_lm.stop()
```

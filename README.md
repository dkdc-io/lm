# dkdc-lm

Local LLM inference management via llama-server.

## Install

```bash
cargo install dkdc-lm-cli
```

```bash
uv add dkdc-lm
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
let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
start(&refs, DEFAULT_PORT, -1, 4096)?;

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

# dkdc-ai

Local LLM inference management via llama-server.

## Install

```bash
cargo install dkdc-ai-cli
```

```bash
uv add dkdc-ai
```

## Usage

### CLI

```bash
dkdc-ai start                              # Start with default model (Gemma 4 26B-A4B)
dkdc-ai start --builtin gemma-4-e4b-it     # Start with smaller model
dkdc-ai start --hf ggml-org/some-model-GGUF  # Start with HuggingFace model
dkdc-ai start --model /path/to/model.gguf  # Start with local model
dkdc-ai status                             # Check server status
dkdc-ai logs                               # View recent logs
dkdc-ai attach                             # Attach to tmux session
dkdc-ai stop                               # Stop server
```

### Rust

```rust
use dkdc_ai::{start, stop, status, resolve_builtin, DEFAULT_PORT};

let args = resolve_builtin("gemma-4-26b-a4b-it")?;
let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
start(&refs, DEFAULT_PORT, -1, 4096)?;

let (tmux, http) = status(DEFAULT_PORT);
println!("tmux: {}, http: {}", tmux, http);

stop()?;
```

### Python

```python
import dkdc_ai

args = dkdc_ai.resolve_builtin(dkdc_ai.default_builtin())
dkdc_ai.start(args)

tmux_running, http_responding = dkdc_ai.status()
print(f"tmux: {tmux_running}, http: {http_responding}")

dkdc_ai.stop()
```

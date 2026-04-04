def start(
    model_args: list[str],
    port: int = 8080,
    gpu_layers: int = -1,
    ctx_size: int = 4096,
) -> None:
    """Start llama-server in a tmux session."""
    ...

def stop() -> None:
    """Stop llama-server by killing its tmux session."""
    ...

def attach() -> None:
    """Attach to the llama-server tmux session."""
    ...

def status(port: int = 8080) -> tuple[bool, bool]:
    """Check status. Returns (tmux_running, http_responding)."""
    ...

def is_running() -> bool:
    """Whether the llama-server tmux session exists."""
    ...

def logs(lines: int | None = None) -> str:
    """Capture recent logs from the tmux session."""
    ...

def resolve_builtin(name: str) -> list[str]:
    """Resolve a built-in model name to llama-server args."""
    ...

def default_port() -> int:
    """Default port for llama-server (8080)."""
    ...

def default_builtin() -> str:
    """Default built-in model preset."""
    ...

def tmux_session() -> str:
    """Tmux session name for llama-server."""
    ...

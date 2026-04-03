//! Local LLM inference management via llama-server (llama.cpp).
//!
//! Manages the llama-server lifecycle (start/stop/status via tmux).
//! For the AI client library, see `stringflow`.

/// Default port for llama-server
pub const DEFAULT_PORT: u16 = 8080;

/// Tmux session name for llama-server
pub const TMUX_SESSION: &str = "dkdc-ai";

/// Default built-in model preset
pub const DEFAULT_BUILTIN: &str = "gemma-4-26b-a4b-it";

/// Known built-in model presets
pub const BUILTIN_MODELS: &[(&str, &str)] = &[
    ("gemma-4-26b-a4b-it", "-hf ggml-org/gemma-4-26B-A4B-it-GGUF"),
    ("gemma-4-e4b-it", "-hf ggml-org/gemma-4-E4B-it-GGUF"),
];

/// Start llama-server in a tmux session.
pub fn start(
    model_args: &[&str],
    port: u16,
    gpu_layers: i32,
    ctx_size: u32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dkdc_sh::require("llama-server")?;

    if dkdc_sh::tmux::has_session(TMUX_SESSION) {
        return Err(format!(
            "llama-server already running in tmux session '{}'",
            TMUX_SESSION
        )
        .into());
    }

    let args = model_args
        .iter()
        .map(|s| s.to_string())
        .chain([
            "--port".to_string(),
            port.to_string(),
            "-ngl".to_string(),
            gpu_layers.to_string(),
            "-c".to_string(),
            ctx_size.to_string(),
        ])
        .collect::<Vec<_>>()
        .join(" ");

    let cmd = format!("llama-server {}", args);
    dkdc_sh::tmux::new_session(TMUX_SESSION, &cmd)?;
    Ok(())
}

/// Stop llama-server by killing its tmux session.
pub fn stop() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !dkdc_sh::tmux::has_session(TMUX_SESSION) {
        return Ok(());
    }
    dkdc_sh::tmux::kill_session(TMUX_SESSION)?;
    Ok(())
}

/// Check llama-server status. Returns (tmux_running, http_responding).
pub fn status(port: u16) -> (bool, bool) {
    let tmux_running = dkdc_sh::tmux::has_session(TMUX_SESSION);
    let url = format!("http://localhost:{}/health", port);
    let http_responding = reqwest::blocking::get(&url)
        .map(|r| r.status().is_success())
        .unwrap_or(false);
    (tmux_running, http_responding)
}

/// Capture recent logs from the tmux session.
pub fn logs(lines: Option<usize>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    if !dkdc_sh::tmux::has_session(TMUX_SESSION) {
        return Err(format!("no tmux session '{}'", TMUX_SESSION).into());
    }
    let output = dkdc_sh::tmux::capture_pane(TMUX_SESSION, lines)?;
    Ok(output)
}

/// Resolve a built-in model name to llama-server args.
pub fn resolve_builtin(
    name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let args_str = BUILTIN_MODELS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, args)| *args)
        .ok_or_else(|| {
            let available: Vec<&str> = BUILTIN_MODELS.iter().map(|(n, _)| *n).collect();
            format!(
                "unknown built-in model '{}'. available: {}",
                name,
                available.join(", ")
            )
        })?;
    Ok(args_str.split_whitespace().map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_builtin_default() {
        let args = resolve_builtin(DEFAULT_BUILTIN).unwrap();
        assert_eq!(args, vec!["-hf", "ggml-org/gemma-4-26B-A4B-it-GGUF"]);
    }

    #[test]
    fn resolve_builtin_small() {
        let args = resolve_builtin("gemma-4-e4b-it").unwrap();
        assert_eq!(args, vec!["-hf", "ggml-org/gemma-4-E4B-it-GGUF"]);
    }

    #[test]
    fn resolve_builtin_unknown() {
        assert!(resolve_builtin("nonexistent").is_err());
    }

    #[test]
    fn constants() {
        assert_eq!(DEFAULT_PORT, 8080);
        assert_eq!(TMUX_SESSION, "dkdc-ai");
        assert_eq!(DEFAULT_BUILTIN, "gemma-4-26b-a4b-it");
    }
}

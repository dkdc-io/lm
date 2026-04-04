//! Local LLM inference management via llama-server (llama.cpp).
//!
//! Manages the llama-server lifecycle (start/stop/status via tmux).

use std::fmt;

/// Default port for llama-server.
pub const DEFAULT_PORT: u16 = 8080;

/// Tmux session name for llama-server.
pub const TMUX_SESSION: &str = "dkdc-lm";

/// Default built-in model preset.
pub const DEFAULT_BUILTIN: &str = "gemma-4-26b-a4b-it";

/// Known built-in model presets.
pub const BUILTIN_MODELS: &[(&str, &str)] = &[
    ("gemma-4-26b-a4b-it", "-hf ggml-org/gemma-4-26B-A4B-it-GGUF"),
    ("gemma-4-e4b-it", "-hf ggml-org/gemma-4-E4B-it-GGUF"),
];

/// Errors returned by dkdc-lm operations.
#[derive(Debug)]
pub enum Error {
    /// llama-server is already running.
    AlreadyRunning,
    /// No active tmux session found.
    NotRunning,
    /// The requested built-in model name is unknown.
    UnknownModel(String),
    /// An error from a shell command or dependency.
    Shell(dkdc_sh::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AlreadyRunning => write!(
                f,
                "llama-server already running in tmux session '{TMUX_SESSION}'"
            ),
            Error::NotRunning => write!(f, "no tmux session '{TMUX_SESSION}'"),
            Error::UnknownModel(name) => {
                let available: Vec<&str> = BUILTIN_MODELS.iter().map(|(n, _)| *n).collect();
                write!(
                    f,
                    "unknown built-in model '{name}'. available: {}",
                    available.join(", ")
                )
            }
            Error::Shell(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Shell(e) => Some(e),
            _ => None,
        }
    }
}

impl From<dkdc_sh::Error> for Error {
    fn from(e: dkdc_sh::Error) -> Self {
        Error::Shell(e)
    }
}

/// Whether the llama-server tmux session exists.
pub fn is_running() -> bool {
    dkdc_sh::tmux::has_session(TMUX_SESSION)
}

/// Start llama-server in a tmux session.
pub fn start(
    model_args: &[String],
    port: u16,
    gpu_layers: i32,
    ctx_size: u32,
) -> Result<(), Error> {
    dkdc_sh::require("llama-server")?;

    if is_running() {
        return Err(Error::AlreadyRunning);
    }

    let args = model_args
        .iter()
        .cloned()
        .chain([
            "--port".into(),
            port.to_string(),
            "-ngl".into(),
            gpu_layers.to_string(),
            "-c".into(),
            ctx_size.to_string(),
        ])
        .collect::<Vec<String>>()
        .join(" ");

    dkdc_sh::tmux::new_session(TMUX_SESSION, &format!("llama-server {args}"))?;
    Ok(())
}

/// Stop llama-server by killing its tmux session.
pub fn stop() -> Result<(), Error> {
    if !is_running() {
        return Ok(());
    }
    dkdc_sh::tmux::kill_session(TMUX_SESSION)?;
    Ok(())
}

/// Check llama-server status. Returns (tmux_running, http_responding).
pub fn status(port: u16) -> (bool, bool) {
    let tmux_running = is_running();
    let url = format!("http://localhost:{port}/health");
    let http_responding = reqwest::blocking::get(&url)
        .map(|r| r.status().is_success())
        .unwrap_or(false);
    (tmux_running, http_responding)
}

/// Attach to the llama-server tmux session.
pub fn attach() -> Result<(), Error> {
    if !is_running() {
        return Err(Error::NotRunning);
    }
    dkdc_sh::tmux::attach(TMUX_SESSION)?;
    Ok(())
}

/// Capture recent logs from the tmux session.
pub fn logs(lines: Option<usize>) -> Result<String, Error> {
    if !is_running() {
        return Err(Error::NotRunning);
    }
    Ok(dkdc_sh::tmux::capture_pane(TMUX_SESSION, lines)?)
}

/// Resolve a built-in model name to llama-server args.
pub fn resolve_builtin(name: &str) -> Result<Vec<String>, Error> {
    let args_str = BUILTIN_MODELS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, args)| *args)
        .ok_or_else(|| Error::UnknownModel(name.to_owned()))?;
    Ok(args_str.split_whitespace().map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- resolve_builtin ------------------------------------------------------

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
    fn resolve_builtin_all_models() {
        for (name, expected_args_str) in BUILTIN_MODELS {
            let args = resolve_builtin(name).unwrap();
            let expected: Vec<String> = expected_args_str
                .split_whitespace()
                .map(String::from)
                .collect();
            assert_eq!(args, expected, "mismatch for model '{name}'");
        }
    }

    #[test]
    fn resolve_builtin_unknown() {
        let err = resolve_builtin("nonexistent").unwrap_err();
        assert!(matches!(err, Error::UnknownModel(_)));
        let msg = err.to_string();
        assert!(msg.contains("nonexistent"));
        assert!(msg.contains("gemma-4-26b-a4b-it"));
    }

    #[test]
    fn resolve_builtin_empty_string() {
        let err = resolve_builtin("").unwrap_err();
        assert!(matches!(err, Error::UnknownModel(_)));
    }

    #[test]
    fn resolve_builtin_case_sensitive() {
        // Model names are case-sensitive — uppercase should fail
        assert!(resolve_builtin("Gemma-4-26b-a4b-it").is_err());
        assert!(resolve_builtin("GEMMA-4-26B-A4B-IT").is_err());
    }

    // -- constants ------------------------------------------------------------

    #[test]
    fn constants() {
        assert_eq!(DEFAULT_PORT, 8080);
        assert_eq!(TMUX_SESSION, "dkdc-lm");
        assert_eq!(DEFAULT_BUILTIN, "gemma-4-26b-a4b-it");
    }

    #[test]
    fn default_builtin_is_in_builtin_models() {
        assert!(
            BUILTIN_MODELS.iter().any(|(n, _)| *n == DEFAULT_BUILTIN),
            "DEFAULT_BUILTIN '{DEFAULT_BUILTIN}' not found in BUILTIN_MODELS"
        );
    }

    #[test]
    fn builtin_models_have_valid_args() {
        for (name, args_str) in BUILTIN_MODELS {
            assert!(!name.is_empty(), "model name should not be empty");
            assert!(!args_str.is_empty(), "args for '{name}' should not be empty");
            let parts: Vec<&str> = args_str.split_whitespace().collect();
            assert!(
                parts.len() >= 2,
                "args for '{name}' should have at least flag + value"
            );
        }
    }

    // -- Error Display --------------------------------------------------------

    #[test]
    fn error_display_already_running() {
        let msg = Error::AlreadyRunning.to_string();
        assert!(msg.contains(TMUX_SESSION));
        assert!(msg.contains("already running"));
    }

    #[test]
    fn error_display_not_running() {
        let msg = Error::NotRunning.to_string();
        assert!(msg.contains(TMUX_SESSION));
    }

    #[test]
    fn error_display_unknown_model() {
        let msg = Error::UnknownModel("bad-model".into()).to_string();
        assert!(msg.contains("bad-model"));
        assert!(msg.contains("available:"));
        // Should list all known models
        for (name, _) in BUILTIN_MODELS {
            assert!(msg.contains(name), "missing model '{name}' in error message");
        }
    }

    // -- Error trait -----------------------------------------------------------

    #[test]
    fn error_source_non_shell_is_none() {
        assert!(std::error::Error::source(&Error::AlreadyRunning).is_none());
        assert!(std::error::Error::source(&Error::NotRunning).is_none());
        assert!(
            std::error::Error::source(&Error::UnknownModel("x".into())).is_none()
        );
    }
}

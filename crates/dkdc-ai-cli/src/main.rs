use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dkdc-ai")]
#[command(about = "Local LLM inference management via llama-server")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start llama-server (in tmux)
    Start {
        /// Path to GGUF model file
        #[arg(short, long)]
        model: Option<String>,
        /// Hugging Face model repository (e.g. ggml-org/gemma-4-26B-A4B-it-GGUF)
        #[arg(long)]
        hf: Option<String>,
        /// Built-in model preset (e.g. gemma-4-26b-a4b-it, gemma-4-e4b-it)
        #[arg(long)]
        builtin: Option<String>,
        /// Port to listen on
        #[arg(short, long, default_value_t = dkdc_ai::DEFAULT_PORT)]
        port: u16,
        /// Number of GPU layers to offload (-1 = all)
        #[arg(long, default_value_t = -1, allow_hyphen_values = true)]
        gpu_layers: i32,
        /// Context size
        #[arg(short, long, default_value_t = 4096)]
        ctx_size: u32,
    },
    /// Stop llama-server (kill tmux session)
    Stop,
    /// Show llama-server status
    Status,
    /// Attach to llama-server tmux session
    Attach,
    /// Show recent logs from tmux session
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    match args.command {
        Commands::Start {
            model,
            hf,
            builtin,
            port,
            gpu_layers,
            ctx_size,
        } => {
            let model_args = resolve_model_source(&model, &hf, &builtin)?;
            let model_refs: Vec<&str> = model_args.iter().map(|s| s.as_str()).collect();
            dkdc_ai::start(&model_refs, port, gpu_layers, ctx_size)?;

            println!(
                "llama-server started in tmux session '{}'",
                dkdc_ai::TMUX_SESSION
            );
            println!();
            println!("Commands:");
            println!("  dkdc-ai attach    # View server output");
            println!("  dkdc-ai logs      # Show recent logs");
            println!("  dkdc-ai stop      # Stop server");
        }
        Commands::Stop => {
            dkdc_ai::stop()?;
            println!("llama-server stopped");
        }
        Commands::Status => {
            let (tmux_running, http_responding) = dkdc_ai::status(dkdc_ai::DEFAULT_PORT);

            if http_responding {
                println!("llama-server is running on port {}", dkdc_ai::DEFAULT_PORT);
                if tmux_running {
                    println!("Tmux session: {}", dkdc_ai::TMUX_SESSION);
                }
            } else if tmux_running {
                println!("Tmux session exists but llama-server may not be responding");
                println!("Use: dkdc-ai logs");
            } else {
                println!("llama-server is not running");
            }
        }
        Commands::Attach => {
            if !dkdc_sh::tmux::has_session(dkdc_ai::TMUX_SESSION) {
                println!(
                    "llama-server not running (no tmux session '{}')",
                    dkdc_ai::TMUX_SESSION
                );
                println!("Use: dkdc-ai start --builtin {}", dkdc_ai::DEFAULT_BUILTIN);
                return Ok(());
            }
            dkdc_sh::tmux::attach(dkdc_ai::TMUX_SESSION)?;
        }
        Commands::Logs { lines } => {
            let output = dkdc_ai::logs(Some(lines))?;
            print!("{}", output);
        }
    }
    Ok(())
}

fn resolve_model_source(
    model: &Option<String>,
    hf: &Option<String>,
    builtin: &Option<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(m) = model {
        Ok(vec!["-m".to_string(), m.clone()])
    } else if let Some(repo) = hf {
        Ok(vec!["-hf".to_string(), repo.clone()])
    } else {
        let name = builtin.as_deref().unwrap_or(dkdc_ai::DEFAULT_BUILTIN);
        dkdc_ai::resolve_builtin(name)
    }
}

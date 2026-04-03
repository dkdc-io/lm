use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "lm")]
#[command(about = "Local LLM inference management via llama-server")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
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
        #[arg(short, long, default_value_t = dkdc_lm::DEFAULT_PORT)]
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
    Status {
        /// Port to check
        #[arg(short, long, default_value_t = dkdc_lm::DEFAULT_PORT)]
        port: u16,
    },
    /// Attach to llama-server tmux session
    Attach,
    /// Show recent logs from tmux session
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value_t = 50)]
        lines: usize,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), dkdc_lm::Error> {
    let args = Args::parse();

    match args.command {
        None => {
            Args::parse_from(["lm", "--help"]);
        }
        Some(Commands::Start {
            model,
            hf,
            builtin,
            port,
            gpu_layers,
            ctx_size,
        }) => {
            let model_args =
                resolve_model_source(model.as_deref(), hf.as_deref(), builtin.as_deref())?;
            dkdc_lm::start(&model_args, port, gpu_layers, ctx_size)?;
            print_start_help();
        }
        Some(Commands::Stop) => {
            dkdc_lm::stop()?;
            println!("llama-server stopped");
        }
        Some(Commands::Status { port }) => {
            let (tmux_running, http_responding) = dkdc_lm::status(port);

            if http_responding {
                println!("llama-server is running on port {port}");
                if tmux_running {
                    println!("Tmux session: {}", dkdc_lm::TMUX_SESSION);
                }
            } else if tmux_running {
                println!("Tmux session exists but llama-server may not be responding");
                println!("Use: lm logs");
            } else {
                println!("llama-server is not running");
            }
        }
        Some(Commands::Attach) => {
            if !dkdc_lm::is_running() {
                println!(
                    "llama-server not running (no tmux session '{}')",
                    dkdc_lm::TMUX_SESSION
                );
                println!("Use: lm start --builtin {}", dkdc_lm::DEFAULT_BUILTIN);
                return Ok(());
            }
            dkdc_lm::attach()?;
        }
        Some(Commands::Logs { lines }) => {
            let output = dkdc_lm::logs(Some(lines))?;
            print!("{output}");
        }
    }
    Ok(())
}

fn print_start_help() {
    println!(
        "llama-server started in tmux session '{}'",
        dkdc_lm::TMUX_SESSION
    );
    println!();
    println!("Commands:");
    println!("  lm attach    # View server output");
    println!("  lm logs      # Show recent logs");
    println!("  lm stop      # Stop server");
}

fn resolve_model_source(
    model: Option<&str>,
    hf: Option<&str>,
    builtin: Option<&str>,
) -> Result<Vec<String>, dkdc_lm::Error> {
    if let Some(m) = model {
        Ok(vec!["-m".into(), m.into()])
    } else if let Some(repo) = hf {
        Ok(vec!["-hf".into(), repo.into()])
    } else {
        let name = builtin.unwrap_or(dkdc_lm::DEFAULT_BUILTIN);
        dkdc_lm::resolve_builtin(name)
    }
}

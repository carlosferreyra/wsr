use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "wsr",
    about = "A local, Wasm-sandboxed CI runner. Run your workflows on every git hook — before you push, not after.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan workflows, install git hook shims, and generate wsr.json
    Init,

    /// Run workflows locally
    Run {
        /// Specific workflow file to run
        file: Option<String>,

        /// Force a specific trigger event (e.g. push, pull_request)
        #[arg(long)]
        event: Option<String>,

        /// Print the execution plan without running anything
        #[arg(long)]
        dry_run: bool,

        /// Show expression evaluation, sandbox grants, and context dump
        #[arg(long)]
        verbose: bool,

        /// Skip interactive prompts
        #[arg(long)]
        yes: bool,

        /// Output format: human (default) or gha (GitHub Annotations)
        #[arg(long, default_value = "human")]
        format: String,
    },

    /// Watch workflow files and keep hooks in sync automatically
    Daemon {
        /// Register as a launchd / systemd service
        #[arg(long)]
        install: bool,
    },

    /// List all workflows and their hook mappings
    List,

    /// Parse and pretty-print a workflow, validate expressions
    Inspect {
        /// Workflow file to inspect
        file: String,
    },

    /// Manage the compiled Wasm module cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },

    /// Manually install or remove individual hook shims
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },

    /// Show active hook map, daemon state, and last sync time
    Status,
}

#[derive(Subcommand)]
enum CacheAction {
    /// List cached Wasm modules with their SHA-256 keys and sizes
    List,
    /// Verify integrity of all cached modules
    Verify,
    /// Remove all cached modules
    Purge,
}

#[derive(Subcommand)]
enum HookAction {
    /// Install a hook shim for a specific git hook
    Install {
        /// Git hook name (e.g. pre-push, pre-commit)
        hook: String,
    },
    /// Remove a hook shim
    Remove {
        /// Git hook name
        hook: String,
    },
}

fn main() {
    let _cli = Cli::parse();
    // TODO: delegate to wsr-engine, wsr-git, wsr-tracing
    eprintln!("wsr — not yet implemented");
}

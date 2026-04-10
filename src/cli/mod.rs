use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wsr", about = "A local, Wasm-sandboxed CI runner for git hooks")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scan workflows, generate wsr.json, and install git hook shims
    Init,

    /// Execute workflows locally
    Run {
        /// Run a specific workflow file
        file: Option<PathBuf>,

        /// Force a specific event type
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
    },
}

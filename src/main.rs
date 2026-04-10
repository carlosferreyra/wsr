use clap::Parser;
use wsr::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            println!("wsr init — not yet implemented");
        }
        Command::Run { file, event, dry_run, verbose, yes } => {
            println!(
                "wsr run — file={file:?} event={event:?} dry_run={dry_run} verbose={verbose} yes={yes}"
            );
        }
    }
}

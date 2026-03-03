//! Desktop Homunculus CLI (`hmcs`)

use clap::{Parser, Subcommand};

mod config;
mod mods;
mod prefs;

/// Top-level CLI structure for the `hmcs` command.
#[derive(Parser)]
#[command(name = "hmcs", version, about = "Desktop Homunculus CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Top-level subcommands available in `hmcs`.
#[derive(Subcommand)]
enum Commands {
    /// Manage application preferences
    Prefs(prefs::PrefsArgs),
    /// Manage application configuration
    Config(config::ConfigArgs),
    /// Manage mods.
    Mod(mods::ModsArgs),
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Prefs(args) => args.execute(),
        Commands::Config(args) => args.execute(),
        Commands::Mod(args) => args.execute(),
    };

    if let Err(e) = result {
        eprintln!("{e}");
    }
}

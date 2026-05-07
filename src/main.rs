/// Modules
mod commands;
mod config;
mod errors;
mod git;
mod macros;
mod resolver;

/// Imports
use clap::{Parser, Subcommand};

/// Defines CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

/// Defines a subcommand
#[derive(Subcommand)]
enum SubCommand {
    /// Adds egg from url
    Add { url: String },
    /// Runs project
    Run,
    /// Performs resolution of dependencies
    Solve,
    /// Creates and initializes new egg
    New { name: String },
    /// Initializes new egg in cwd
    Init,
    /// Clears cache of packages
    Clean,
}

///
fn main() {
    commands::run::run();
}

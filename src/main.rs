/// Modules
mod commands;
mod config;
mod errors;
mod git;
mod macros;
mod project;
mod resolver;

/// Imports
use crate::commands::{add, clean, new, run, solve};
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

/// The entry point
fn main() {
    // Parsing command
    match Cli::parse().command {
        SubCommand::Add { url } => add::run(url),
        SubCommand::Run => run::run(),
        SubCommand::Solve => solve::run(),
        SubCommand::New { name } => new::run(name),
        SubCommand::Init => todo!(),
        SubCommand::Clean => clean::run(),
    }
}

mod cli;
mod commands;
mod models;
mod store;

use crate::cli::Cli;
use crate::cli::Commands;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            ref title,
            ref url,
            ref tags,
        } => {
            commands::add(title, url, tags);
        }
        Commands::List => commands::list(),
        Commands::Search { ref query } => {
            commands::search(query);
        }
        Commands::Open { ref id } => {
            commands::open(id);
        }
        Commands::Remove { ref id } => {
            commands::remove(id);
        }
    }
}

mod cli;
mod commands;
mod models;
mod store;

use crate::cli::Cli;
use crate::cli::Commands;
use crate::models::LinkDB;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let mut db = LinkDB::new();

    match cli.command {
        Commands::Add {
            ref title,
            ref url,
            ref tags,
        } => {
            commands::add(&mut db, title, url, tags);
        }
        Commands::List => commands::list(&db),
        Commands::Search { ref query } => {
            commands::search(&db, query);
        }
        Commands::Open { ref target } => {
            commands::open(&db, target);
        }
        Commands::Remove { ref id } => {
            commands::remove(&mut db, id);
        }
    }
}

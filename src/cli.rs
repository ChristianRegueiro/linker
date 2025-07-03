use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "linker",
    author = "Christian Regueiro",
    version = "1.0",
    about = "📎 A simple CLI tool to save and search useful links"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Add a new link with a title, URL, and optional comma-separated tags")]
    Add {
        #[arg(help = "Title of the link")]
        title: String,

        #[arg(help = "The URL to save")]
        url: String,

        #[arg(
            short,
            long,
            value_delimiter = ',',
            help = "Comma-separated list of tags (e.g., --tags rust,cli,docs)"
        )]
        tags: Vec<String>,
    },

    #[command(about = "List all saved links")]
    List,

    #[command(about = "Search links by keyword in title, URL, or tags")]
    Search {
        #[arg(help = "Search query")]
        query: String,
    },

    #[command(about = "Open a link by ID or title")]
    Open {
        #[arg(help = "ID number or full title of the link to open")]
        target: String,
    },

    #[command(about = "Remove a link by ID or title")]
    Remove {
        #[arg(help = "ID number of the link to remove")]
        id: usize,
    },

    #[command(about = "Update a link by ID or title")]
    Edit {
        #[arg(help = "ID number or full title of the link to open")]
        target: String,
    },
}

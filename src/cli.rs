use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "linker", author = "ChristianRegueiro")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        title: String,
        url: String,
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    List,
    Search {
        query: String,
    },
    Open {
        id: usize,
    },
    Remove {
        id: usize,
    },
}

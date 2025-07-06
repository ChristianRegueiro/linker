use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub tags: Vec<String>,
    pub added: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LinkDB {
    pub links: Vec<Link>,
}

impl LinkDB {
    pub fn new() -> Self {
        crate::store::load_links()
    }

    pub fn add_link(&mut self, link: Link) {
        if self.links.iter().any(|l| l.url == link.url) {
            eprintln!("{} This URL is already saved.", "⚠️".yellow());
            return;
        }

        if self
            .links
            .iter()
            .any(|l| l.title.to_lowercase() == link.title.to_lowercase())
        {
            eprintln!(
                "{} This title already exists. Please use a unique title.",
                "⚠️".yellow()
            );
            return;
        }

        self.links.push(link);
        println!("{} Link added successfully.", "✔️".green().bold());
        crate::store::save_links(self);
    }

    pub fn remove_link(&mut self, id: usize) {
        self.links.retain(|link| link.id != id);
    }

    // TODO: Implement search functionality
    // pub fn search(&self, query: &str) -> Vec<&Link> {
    //     self.links
    //         .iter()
    //         .filter(|link| {
    //             link.title.to_lowercase().contains(&query.to_lowercase())
    //                 || link.url.to_lowercase().contains(&query.to_lowercase())
    //                 || link
    //                     .tags
    //                     .iter()
    //                     .any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
    //         })
    //         .collect()
    // }
}

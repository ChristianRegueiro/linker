use chrono::Utc;
use colored::Colorize;
use url::Url;

use crate::{
    models::Link,
    store::{load_links, save_links},
};

pub fn add(title: &str, url: &str, tags: &[String]) {
    if Url::parse(url).is_err() {
        eprintln!("{} Invalid URL: {}", "✘".red(), url);
        return;
    }

    let mut db = load_links();

    let new_id = db.links.iter().map(|l| l.id).max().unwrap_or(0) + 1;

    let link = Link {
        id: new_id,
        title: title.to_string(),
        url: url.to_string(),
        tags: tags.to_vec(),
        added: Utc::now(),
    };

    db.links.push(link);
    save_links(&db);
}

pub fn list() {
    let db = load_links();

    if db.links.is_empty() {
        println!("{}", "⚠️  There is no saved links yet.".yellow());
        return;
    }

    pretty_print(&db.links);
}

pub fn search(query: &str) {
    let db = load_links();

    let query_lower = query.to_lowercase();

    let results: Vec<_> = db
        .links
        .iter()
        .filter(|link| {
            link.title.to_lowercase().contains(&query_lower)
                || link.url.to_lowercase().contains(&query_lower)
                || link
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query_lower))
        })
        .collect();

    if results.is_empty() {
        println!("{} No results for: '{}'", "🕵️".yellow(), query);
        return;
    }

    println!(
        "{} Resultados para '{}':\n",
        "🔍".green().bold(),
        query.cyan()
    );

    pretty_print(results);
}

pub fn open(id: &usize) {
    let db = load_links();

    match db.links.iter().find(|l| l.id == *id) {
        Some(link) => {
            println!("{} Opening: {}", "🌐".green(), link.url.underline().blue());

            if let Err(e) = open::that(&link.url) {
                eprintln!("{} Cannot open in browser: {}", "✘".red(), e);
            }
        }
        None => {
            eprintln!("{} There is no link with id {}", "✘".red(), id);
        }
    }
}

pub fn remove(id: &usize) {
    let mut db = load_links();

    let initial_len = db.links.len();
    db.links.retain(|link| link.id != *id); // retain mantiene elementos que conciden

    if db.links.len() == initial_len {
        eprintln!("{} There is no link with ID: {}", "✘".red(), id);
    } else {
        save_links(&db);
        println!("{} Deleted link with ID {}", "✔".green(), id);
    }
}

pub fn pretty_print<'a, I>(items: I)
where
    I: IntoIterator<Item = &'a Link>,
{
    for link in items {
        let id = format!("[{:02}]", link.id).cyan();
        let title = link.title.bold();
        let url = link.url.underline().blue();
        let tags = if link.tags.is_empty() {
            "-".normal()
        } else {
            format!("[{}]", link.tags.join(", ")).green()
        };
        let date = link.added.format("%Y-%m-%d").to_string().dimmed();

        println!("{id} {title:25} {url:45} {tags:20} {date}");
    }
}

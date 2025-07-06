use chrono::Utc;
use colored::Colorize;
use std::io::{self, Write};
use url::Url;

use crate::models::{Link, LinkDB};

pub fn add(db: &mut LinkDB, title: &str, description: Option<&str>, url: &str, tags: &[String]) {
    if Url::parse(url).is_err() {
        eprintln!("{} Invalid URL: {}", "✘".red(), url);
        return;
    }

    let new_id = db.links.iter().map(|l| l.id).max().unwrap_or(0) + 1;

    let link = Link {
        id: new_id,
        title: title.to_string(),
        description: description.map(|d| d.to_string()),
        url: url.to_string(),
        tags: tags.to_vec(),
        added: Utc::now(),
    };

    db.add_link(link);
}

pub fn list(db: &LinkDB) {
    if db.links.is_empty() {
        println!("{}", "⚠️  There is no saved links yet.".yellow());
        return;
    }

    pretty_print(&db.links);
}

pub fn search(db: &LinkDB, query: &str) {
    let results = find_partial_matches(&db.links, query);

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

pub fn open(db: &LinkDB, target: &str) {
    let found = if let Ok(id) = target.parse::<usize>() {
        db.links.iter().find(|l| l.id == id)
    } else {
        db.links
            .iter()
            .find(|l| l.title.eq_ignore_ascii_case(target))
    };

    if let Some(link) = found {
        println!("{} Opening: {}", "🌐".green(), link.url.underline().blue());
        if let Err(e) = open::that(&link.url) {
            eprintln!("{} Cannot open in browser: {}", "✘".red(), e);
        }
        return;
    }

    let suggestions = find_partial_matches(&db.links, target);

    if suggestions.is_empty() {
        eprintln!("{} No link found with ID or title: '{}'", "✘".red(), target);
    } else {
        eprintln!(
            "{} No exact match found for '{}', but did you mean:",
            "🔎".yellow(),
            target.cyan()
        );
        for link in suggestions {
            println!("  - [{}] {}", link.id.to_string().cyan(), link.title.bold());
        }
    }
}

pub fn edit(db: &mut LinkDB, target: &str) {
    let found = if let Ok(id) = target.parse::<usize>() {
        db.links.iter_mut().find(|l| l.id == id)
    } else {
        db.links
            .iter_mut()
            .find(|l| l.title.eq_ignore_ascii_case(target))
    };

    if let Some(link) = found {
        println!(
            "{} Editing link: [{}] {}",
            "✏️".yellow(),
            link.id.to_string().cyan(),
            link.title.bold()
        );

        print!("Title [{}]: ", link.title);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let new_title = input.trim();
        if !new_title.is_empty() {
            link.title = new_title.to_string();
        }

        print!(
            "Description [{}]: ",
            link.description.as_deref().unwrap_or("-")
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let new_description = input.trim();
        if !new_description.is_empty() {
            link.description = Some(new_description.to_string());
        } else {
            link.description = None;
        }

        input.clear();
        print!("URL [{}]: ", link.url);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let new_url = input.trim();
        if !new_url.is_empty() {
            link.url = new_url.to_string();
        }

        input.clear();
        let current_tags = link.tags.join(", ");
        print!("Tags (comma-separated) [{}]: ", current_tags);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let new_tags = input.trim();
        if !new_tags.is_empty() {
            link.tags = new_tags
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        crate::store::save_links(db);
        println!("✅ Link updated.");
    } else {
        eprintln!("⚠️ Link not found.");
    }
}

pub fn remove(db: &mut LinkDB, id: &usize) {
    if db.links.iter().any(|link| link.id == *id) {
        db.remove_link(*id);
        println!("{} Deleted link with ID {}", "✔".green(), id);
    } else {
        eprintln!("{} There is no link with ID: {}", "✘".red(), id);
    }
}

pub fn pretty_print<'a, I>(items: I)
where
    I: IntoIterator<Item = &'a Link>,
{
    for link in items {
        let id = format!("[{:02}]", link.id).cyan();
        let title = link.title.bold();
        let description = link.description.as_deref().unwrap_or("-").dimmed();
        let url = link.url.underline().blue();
        let tags = if link.tags.is_empty() {
            "-".normal()
        } else {
            format!("[{}]", link.tags.join(", ")).green()
        };
        let date = link.added.format("%Y-%m-%d").to_string().dimmed();

        println!("{id} {title:25} {description:20} {url:30} {tags:20} {date}");
    }
}

pub fn find_partial_matches<'a>(links: &'a [Link], query: &str) -> Vec<&'a Link> {
    let query_lower = query.to_lowercase();

    links
        .iter()
        .filter(|link| {
            link.title.to_lowercase().contains(&query_lower)
                || link.url.to_lowercase().contains(&query_lower)
                || link
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query_lower))
        })
        .collect()
}

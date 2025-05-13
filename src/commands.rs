use chrono::Utc;
use colored::Colorize;
use url::Url;

use crate::models::{Link, LinkDB};

pub fn add(db: &mut LinkDB, title: &str, url: &str, tags: &[String]) {
    if Url::parse(url).is_err() {
        eprintln!("{} Invalid URL: {}", "✘".red(), url);
        return;
    }

    let new_id = db.links.iter().map(|l| l.id).max().unwrap_or(0) + 1;

    let link = Link {
        id: new_id,
        title: title.to_string(),
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

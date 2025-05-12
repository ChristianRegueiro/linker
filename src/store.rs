use crate::models::LinkDB;
use std::{fs, path::PathBuf};

use colored::Colorize;
use dirs::home_dir;

pub fn get_db_path() -> PathBuf {
    home_dir()
        .expect("Error al cargar la ruta de la db")
        .join(".linker")
        .join(".links.json")
}

pub fn load_links() -> LinkDB {
    let path = get_db_path();

    if path.exists() {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error {}", "{e}".red());
                return LinkDB::default();
            }
        };

        match serde_json::from_str(&content) {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Error parsing json file {}", e);
                LinkDB::default()
            }
        }
    } else {
        LinkDB::default()
    }
}

pub fn save_links(db: &LinkDB) {
    let path = get_db_path();
    let dir = path.parent().unwrap();

    fs::create_dir_all(dir).unwrap_or_else(|_| {
        eprintln!("Error creating directory: {}", dir.display());
    });

    println!("Saving to: {}", path.display());

    let data = match serde_json::to_string_pretty(db) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error serializing data: {}", e);
            return;
        }
    };

    match fs::write(&path, data) {
        Ok(_) => println!("Data saved successfully"),
        Err(e) => eprintln!("Error writing file: {}", e),
    }
}

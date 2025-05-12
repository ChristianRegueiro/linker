use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: usize,
    pub title: String, // 🆕 Título descriptivo del enlace
    pub url: String,
    pub tags: Vec<String>,
    pub added: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LinkDB {
    pub links: Vec<Link>,
}

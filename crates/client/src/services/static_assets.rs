use std::collections::BTreeSet;

use anyhow::Result;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::utils::browser::hostname;

use super::session_cache::{SessionCacheKey, SharedSessionCache};

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub url: String,
    pub cover: String,
    pub description: String,
    pub isbn_13: String,
    pub tags: BTreeSet<String>,
}

pub struct StaticAssets {
    hostname: Url,
    #[allow(dead_code)]
    ss_cache: SharedSessionCache,
}

impl StaticAssets {
    pub fn new(ss_cache: SharedSessionCache) -> Self {
        Self {
            hostname: hostname(),
            ss_cache,
        }
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>> {
        if let Ok(Some(books)) = self.ss_cache.get::<Vec<Book>>(&SessionCacheKey::Books) {
            return Ok(books);
        }

        let client = Client::new();
        let url = self.hostname.join("/assets/database/books.json")?;
        let response: Vec<Book> = client.get(url).send().await?.json().await?;

        if let Err(err) = self.ss_cache.set(&SessionCacheKey::Books, &response) {
            leptos::logging::error!("Failed to instert caché entry for Books: {err}");
        }

        Ok(response)
    }
}

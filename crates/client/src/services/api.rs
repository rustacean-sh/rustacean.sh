use std::collections::BTreeMap;

use anyhow::Result;
use reqwest::Url;

use proto::rustacean::Rustacean;

use crate::utils::browser::hostname;

use super::session_cache::{SessionCacheKey, SharedSessionCache};

pub type RustaceanDb = BTreeMap<String, Rustacean>;

pub struct Api {
    hostname: Url,
    #[allow(dead_code)]
    ss_cache: SharedSessionCache,
}

impl Api {
    pub fn new(ss_cache: SharedSessionCache) -> Self {
        Self {
            hostname: hostname(),
            ss_cache,
        }
    }

    /// Retrieves local `Rustaceans` database.
    ///
    /// > This could be cached in the Session Storage
    pub async fn list_rustaceans(&self) -> Result<RustaceanDb> {
        if let Ok(Some(gh_repo)) = self
            .ss_cache
            .get::<RustaceanDb>(&SessionCacheKey::Rustacean)
        {
            return Ok(gh_repo);
        }

        let client = reqwest::Client::new();
        let url = self.hostname.join("/assets/database/rustaceans.bin")?;
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        let entries = bincode::deserialize(&bytes)?;

        if let Err(err) = self.ss_cache.set(&SessionCacheKey::Rustacean, &entries) {
            leptos::logging::error!("Failed to caché Rustaceans Database: {err}");
        }

        Ok(entries)
    }
}

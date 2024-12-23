use std::collections::BTreeMap;

use anyhow::Result;
use reqwest::Url;

use proto::rustacean::Rustacean;

use crate::utils::browser::hostname;

pub struct Api {
    hostname: Url,
}

impl Default for Api {
    fn default() -> Self {
        Self::new()
    }
}

impl Api {
    pub fn new() -> Self {
        Self {
            hostname: hostname(),
        }
    }

    /// Retrieves local `Rustaceans` database.
    ///
    /// > This could be cached in the Session Storage
    pub async fn list_rustaceans(&self) -> Result<BTreeMap<String, Rustacean>> {
        let client = reqwest::Client::new();
        let url = self.hostname.join("/assets/database/rustaceans.bin")?;
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        let entries = bincode::deserialize(&bytes)?;

        Ok(entries)
    }
}

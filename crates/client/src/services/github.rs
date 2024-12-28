use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use super::session_cache::{SessionCacheKey, SharedSessionCache};

#[derive(Deserialize, Serialize)]
pub struct GitHubRepository {
    pub name: String,
    pub stargazers_count: u32,
}

pub struct GitHub {
    ss_cache: SharedSessionCache,
}

impl GitHub {
    pub fn new(ss_cache: SharedSessionCache) -> Self {
        Self { ss_cache }
    }

    pub async fn get_stars(&self) -> Result<GitHubRepository> {
        if let Ok(Some(gh_repo)) = self
            .ss_cache
            .get::<GitHubRepository>(&SessionCacheKey::GitHubStars)
        {
            return Ok(gh_repo);
        }

        let url = Url::parse("https://api.github.com/repos/rustacean-sh/rustacean.sh")?;
        let gh_repo = reqwest::get(url).await?.json::<GitHubRepository>().await?;

        if let Err(err) = self.ss_cache.set(&SessionCacheKey::GitHubStars, &gh_repo) {
            leptos::logging::error!("Failed to caché GitHub Repository: {err}");
        }

        Ok(gh_repo)
    }
}

use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GitHub {
    pub stars: Option<u32>,
}

impl Default for GitHub {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHub {
    pub fn new() -> Self {
        Self { stars: None }
    }
    pub async fn get_stars(&self) -> Result<Self> {
        let url = "https://api.github.com/repos/rustacean-sh/rustacean.sh";
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let v: GitHub = serde_json::from_str(res.as_str())?;

        Ok(GitHub { stars: v.stars })
        // Ok(v.stars.unwrap())
    }
}

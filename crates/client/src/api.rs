use anyhow::{Ok, Result};
use reqwest::Url;
use serde::Deserialize;
use serde_json;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Stars {
    pub amount: Option<u64>,
}

const API_REPO_URL = "https://api.github.com/repos/rustacean-sh/rustacean.sh";
    pub async fn get_amount() -> Result<Self> {
        let url = format!(API_REPO_URL);
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.text().await?;
        let v: Value = serde_json::from_str(res.as_str()).unwrap();

        // leptos::logging::log!("Stars: {}", v["stargazers_count"]);

        Ok(Stars {
            amount: v["stargazers_count"].as_u64(),
        })
    }
}

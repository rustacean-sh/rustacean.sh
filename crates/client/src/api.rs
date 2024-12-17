use exitfailure::ExitFailure;
use proto::Rustacean;
use reqwest::Url;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Stars {
    pub amount: Option<u64>,
}

impl Stars {
    pub async fn get_amount() -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/stargazers",
            owner = "rustacean-sh",
            repo = "rustacean.sh"
        );
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url)
            .await?
            .json::<Vec<Rustacean>>()
            .await?
            .len();

        Ok(Stars {
            amount: Some(res as u64),
        })
    }
}

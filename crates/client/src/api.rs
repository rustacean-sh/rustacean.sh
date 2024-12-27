// use anyhow::Result;
// use reqwest::Url;
// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct Stars {
//     pub amount: Option<u64>,
// }

// impl Stars {
//     pub async fn get_amount() -> Result<Self> {
//         let url = "https://api.github.com/repos/rustacean-sh/rustacean.sh";
//         let url = Url::parse(&*url)?;
//         let res = reqwest::get(url).await?.text().await?;
//         let v: Stars = serde_json::from_str(res.as_str())?;

//         Ok(Stars { amount: v.amount })
//     }
// }

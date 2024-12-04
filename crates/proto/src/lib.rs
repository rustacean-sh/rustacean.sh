use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rustacean {
    pub name: String,
    pub image: String,
    pub github_url: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rustacean {
    pub name: String,
    pub image: String,
    pub bio:String,
    pub github_url: String,
    pub email:String,
    pub geographical_location: String,
    pub personal_website:Option<String>,
    pub mastodon_fediverse:Option<String>, 
    pub IRC:Option<String>,
    pub preferred_social_network:Option<String>,
}

use std::collections::HashSet;
use std::hash::Hash;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rustacean {
    pub name: String,
    pub gh_user: String,
    pub image: Option<String>,
    pub social_networks: Option<HashSet<SocialNetwork>>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum SocialNetwork {
    /// https://www.discourse.org
    Discourse { username: String },
    /// Any email address
    Email { username: String },
    /// https://www.instagram.com
    Instagram { username: String },
    /// https://www.reddit.com
    Reddit { username: String },
    /// https://www.threads.net
    Threads { username: String },
    /// https://bsky.app
    BlueSky { username: String },
    /// https://mastodon.social/explore
    Mastodon { username: String },
    /// https://weird.one
    Weird { username: String },
    /// https://x.com
    X { username: String },
    /// Custom public source of contact
    Custom { url: Url },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Location {
    continent: Continent,
    country: Option<Country>,
    city: Option<City>,
    timezone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Continent {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Africa,
    Oceania,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Country {
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct City {
    name: String,
}

pub trait RustaceanExt {
    fn to_toml(&self) -> Result<String>;
    fn from_toml(toml: &str) -> Result<Rustacean>;
}

impl RustaceanExt for Rustacean {
    fn to_toml(&self) -> Result<String> {
        toml::to_string(&self).context("Failed to serialize Rustacean to TOML")
    }

    fn from_toml(toml: &str) -> Result<Rustacean> {
        Ok(toml::from_str(toml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rustacean_toml() {
        let mut social_networks = HashSet::new();

        social_networks.insert(SocialNetwork::BlueSky {
            username: String::from("johnapple"),
        });
        social_networks.insert(SocialNetwork::Custom {
            url: Url::parse("https://example.com").unwrap(),
        });

        let rustacean = Rustacean {
            name: String::from("John Apple"),
            gh_user: String::from("johnapple"),
            image: Some(String::from(
                "https://avatars.githubusercontent.com/u/1?v=4",
            )),
            social_networks: Some(social_networks),
            location: Some(Location {
                continent: Continent::NorthAmerica,
                country: Some(Country {
                    name: String::from("United States"),
                }),
                city: Some(City {
                    name: String::from("San Francisco"),
                }),
                timezone: Some(String::from("America/Los_Angeles")),
            }),
        };

        let have = rustacean.to_toml().unwrap();
        let want = r#"name = "John Apple"
gh_user = "johnapple"
image = "https://avatars.githubusercontent.com/u/1?v=4"

[[social_networks]]

[social_networks.Custom]
url = "https://example.com/"

[[social_networks]]

[social_networks.BlueSky]
username = "johnapple"

[location]
continent = "NorthAmerica"
timezone = "America/Los_Angeles"

[location.country]
name = "United States"

[location.city]
name = "San Francisco"
"#;

        assert_eq!(have, want);
    }
}

use std::collections::HashSet;
use std::hash::Hash;

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
#[serde(tag = "type", content = "meta")]
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

use std::collections::HashSet;
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rustacean {
    pub name: String,
    pub gh_user: String,
    pub image: Option<String>,
    pub social_networks: Option<HashSet<SocialNetwork>>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum SocialNetwork {
    Instagram { username: String },
    X { username: String },
    Custom { url: Url },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    continent: Continent,
    country: Option<Country>,
    city: Option<City>,
    timezone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Continent {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Africa,
    Oceania,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Country {
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct City {
    name: String,
}

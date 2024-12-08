use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rustacean<'a> {
    pub name: String,
    pub gh_user: String,
    pub image: Option<&'a str>,
    pub social_networks: Option<HashSet<SocialNetwork>>,
    pub geographical_location: Option<Location>,
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
    country: Country,
    city: City,
    timezone: String,
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

use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

use anyhow::{Context, Error, Result};
use leptos::window;
use serde::{de::DeserializeOwned, Serialize};
use web_sys::Storage;

pub type SharedSessionCache = Rc<SessionCache>;

pub enum SessionCacheKey {
    Rustacean,
    GitHubStars,
    Books,
}

impl Display for SessionCacheKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Rustacean => write!(f, "rustacean"),
            Self::GitHubStars => write!(f, "github_stars"),
            Self::Books => write!(f, "books"),
        }
    }
}

pub struct SessionCache {
    storage: Storage,
}

impl Default for SessionCache {
    fn default() -> Self {
        let storage = window()
            .session_storage()
            .expect("Unable to read Session Storage from Window")
            .expect("No Session Storage available");

        Self { storage }
    }
}

impl SessionCache {
    pub fn set<T: Serialize>(&self, key: &SessionCacheKey, value: T) -> Result<()> {
        let key_str = key.to_string();
        let value_json = serde_json::to_string(&value)
            .context("Failed to serialize while storing into Session Storage")?;

        self.storage
            .set(&key_str, &value_json)
            .map_err(|_| Error::msg(format!("Failed to set {} into Session Storage.", key_str)))?;

        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &SessionCacheKey) -> Result<Option<T>> {
        let key_str = key.to_string();

        let Ok(maybe_value) = self.storage.get(&key_str) else {
            return Err(Error::msg(format!(
                "Failed to get {} from Session Storage.",
                key_str
            )));
        };

        if let Some(value) = maybe_value {
            let value: T = serde_json::from_str(&value)
                .context("Failed to deserialize while reading from Session Storage")?;

            return Ok(Some(value));
        }

        Ok(None)
    }

    pub fn clear(&self) -> Result<()> {
        self.storage
            .clear()
            .map_err(|_| Error::msg("Failed to clear Session Storage."))?;

        Ok(())
    }
}

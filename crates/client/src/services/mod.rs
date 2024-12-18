mod api;
mod github;

use self::api::Api;
use self::github::GitHub;

pub struct Services {
    api: Api,
    github: GitHub,
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}

impl Services {
    pub fn new() -> Self {
        Self {
            api: Api::new(),
            github: GitHub::default(),
        }
    }

    #[inline]
    pub fn api(&self) -> &Api {
        &self.api
    }

    #[inline]
    pub fn github(&self) -> &GitHub {
        &self.github
    }
}

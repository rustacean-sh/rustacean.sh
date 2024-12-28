mod api;
mod github;
mod session_cache;

use std::rc::Rc;

use self::api::Api;
use self::github::GitHub;
use self::session_cache::SessionCache;

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
        let ss_cache = SessionCache::default();
        let ss_cache = Rc::new(ss_cache);

        Self {
            api: Api::new(Rc::clone(&ss_cache)),
            github: GitHub::new(Rc::clone(&ss_cache)),
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

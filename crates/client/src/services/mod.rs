pub mod api;
pub mod github;
pub mod session_cache;
pub mod static_assets;

use std::rc::Rc;

use self::api::Api;
use self::github::GitHub;
use self::session_cache::SessionCache;
use self::static_assets::StaticAssets;

pub struct Services {
    api: Api,
    github: GitHub,
    assets: StaticAssets,
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
            assets: StaticAssets::new(Rc::clone(&ss_cache)),
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

    #[inline]
    pub fn assets(&self) -> &StaticAssets {
        &self.assets
    }
}

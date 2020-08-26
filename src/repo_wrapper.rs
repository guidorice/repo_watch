use std::hash::{Hash, Hasher};

use hubcaps::repositories::Repo;

/// Wrapper for Repo to make it hash-able.
#[derive(Debug)]
pub struct RepoWrapper {
    pub value: Repo
}

impl RepoWrapper {
    pub fn new(repo: Repo) -> Self {
        RepoWrapper{ value: repo }
    }
}

impl Hash for RepoWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.url.hash(state);
    }
}

impl PartialEq for RepoWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.value.url == other.value.url
    }
}

impl Eq for RepoWrapper {
}

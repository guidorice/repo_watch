use futures::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::env;

use hubcaps::{
    Credentials, Result, Github,
    repositories::Repo,
};

pub mod repo_wrapper;
pub use repo_wrapper::RepoWrapper;

pub mod args;
pub use args::{Scope, Sort};

pub mod report;

pub fn sign_in() -> Result<Github> {
    let token = env::var("GITHUB_TOKEN").expect(
        "Missing GITHUB_TOKEN environment variable. Generate one at https://github.com/settings/tokens"
    );
    Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )
}

/// Fetch the set of all repositories being watched by the authenticated user (global listing for all of github).
pub async fn watching_repos_all(github: &Github) -> Result<HashSet<RepoWrapper>> {
    let repos: Vec<Repo> = github.activity().watching().iter().try_collect().await?;
    // wrap the Repos so they are hashable (may be put into a Set).
    let wrapper_iter = repos.into_iter().map(RepoWrapper::new);
    let set = HashSet::from_iter(wrapper_iter);
    Ok(set)
}

/// Fetch the set of all repositories in an organization, visible to the authenticated user.
pub async fn organization_repos(github: &Github, org_name: &str ) -> Result<HashSet<RepoWrapper>> {
    // let options = OrganizationRepoListOptions::builder()
    //     .repo_type(OrgRepoType::All)
    //     .build();
    let repos: Vec<Repo> = github.org_repos(org_name).iter(&Default::default()).try_collect().await?;
    // wrap the Repos so they are hashable (may be put into a Set).
    let wrapper_iter = repos.into_iter().map(RepoWrapper::new);
    let set = HashSet::from_iter(wrapper_iter);
    Ok(set)
}

/// Fetch the set of all repos belonging to (some) user.
pub async fn user_repos(github: &Github, github_username: &str) -> Result<HashSet<RepoWrapper>> {
    let repos: Vec<Repo> = github.user_repos(github_username).iter(&Default::default()).try_collect().await?;
    let wrapper_iter = repos.into_iter().map(RepoWrapper::new);
    let set = HashSet::from_iter(wrapper_iter);
    Ok(set)
}

/// Fetch the set of all repos owned by user (note: includes all repos across all your organizations!)
pub async fn all_repos(github: &Github) -> Result<HashSet<RepoWrapper>> {
    let repos: Vec<Repo> = github.repos().iter(&Default::default()).try_collect().await?;
    let wrapper_iter = repos.into_iter().map(RepoWrapper::new);
    let set = HashSet::from_iter(wrapper_iter);
    Ok(set)
}


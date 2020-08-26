use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;
use dotenv::dotenv;

use clap::{value_t};

use repo_watch::{
    watching_repos_all,
    organization_repos,
    all_repos,
    user_repos,
    sign_in,
    report::report,
    repo_wrapper::RepoWrapper,
    args::{get_args, check_args, Scope, Sort, SCOPE, SORT, NAME}
};

#[allow(clippy::panic_params)]
#[allow(unused_assignments)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let arg_matches = get_args();
    check_args(&arg_matches);
    let gh = sign_in()?;
    println!("fetching github user info...");
    let gh_user = gh.users().authenticated().await?;
    println!("hello, {}!", gh_user.login);
    println!("fetching list of repos you are watching...");
    let watching_repos = watching_repos_all(&gh).await?;
    let mut in_scope_watching_repos : HashSet<&RepoWrapper> = HashSet::default();
    println!("found {} watching repos", watching_repos.len());
    let scope = value_t!(arg_matches, SCOPE, Scope)?;
    let sort = value_t!(arg_matches, SORT, Sort)?;
    let name_arg = arg_matches.value_of(NAME);
    let mut in_scope_repos: HashSet<RepoWrapper> = HashSet::default();

    match scope {
        Scope::Org => {
            let org_name = match name_arg {
                Some(s) => s.to_lowercase(),
                None => panic!("organization name is required with org scope. try --name {org_name} .")
            };
            println!("fetching repos owned by organization {}...", org_name);
            in_scope_repos = organization_repos(&gh, &org_name).await?;
            let v: Vec<_> = watching_repos.iter().filter(|repo_wrapper| {
                let lower_url = &repo_wrapper.value.url.to_lowercase();
                let pattern = format!("/repos/{}/", org_name);
                lower_url.contains(&pattern)
            }).collect();
            in_scope_watching_repos = HashSet::from_iter(v);
        },
        Scope::User => {
            // consider repos in the username scope (defaults to current user)
            let github_username = match name_arg {
                Some(github_username) => github_username,
                None => &gh_user.login,
            };
            println!("fetching repos owned by user {}...", github_username);
            in_scope_repos = user_repos(&gh, github_username).await?;
            let v: Vec<_> = watching_repos.iter().filter(|repo_wrapper| {
                let lower_url = &repo_wrapper.value.url.to_lowercase();
                let pattern = format!("/repos/{}/", github_username);
                lower_url.contains(&pattern)
            }).collect();
            in_scope_watching_repos = HashSet::from_iter(v);
        },
        Scope::All => {
            // consider everything visible to user
            println!("fetching all repos x all orgs...");
            in_scope_repos = all_repos(&gh).await?;
            in_scope_watching_repos = HashSet::from_iter(watching_repos.iter());
        },
    };
    let not_watching_repos: HashSet<_> = in_scope_repos.difference(&watching_repos).collect();

    println!();
    report(in_scope_watching_repos, not_watching_repos, scope, name_arg, sort);

    Ok(())
}

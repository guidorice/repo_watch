use std::collections::HashSet;

use crate::repo_wrapper::RepoWrapper;
use crate::args::{Sort, Scope};
use std::cmp::Ordering;


/// Write to stdout a report like:
///
/// {scope description}
///
/// watching repos:
/// ...
/// not watching repos:
/// ...
///
pub fn report(
    watching_repos: HashSet<&RepoWrapper>,
    not_watching_repos: HashSet<&RepoWrapper>,
    scope: Scope,
    name_arg: Option<&str>,
    sort: Sort,
) {
    let sorter = match sort {
        Sort::Alpha => {
            |a: &&RepoWrapper, b: &&RepoWrapper| -> Ordering {
                a.value.name.cmp(&b.value.name)
            }
        },
        Sort::Active => {
            |a: &&RepoWrapper, b: &&RepoWrapper| -> Ordering {
                b.value.updated_at.cmp(&a.value.updated_at)
            }
        },
        Sort::Newest => {
            |a: &&RepoWrapper, b: &&RepoWrapper| -> Ordering {
                b.value.created_at.cmp(&a.value.created_at)
            }
        },
    };
    report_scope(scope, name_arg);
    if watching_repos.is_empty() {
        println!("\nwatching repos (none)");
    } else {
        let mut sorted_watching_repos: Vec<_> =  watching_repos.into_iter().collect();
        sorted_watching_repos.sort_by(sorter);
        println!("\nwatching repos ({})", sorted_watching_repos.len());
        for repo_wrapper in sorted_watching_repos {
            report_repo_line("😍", repo_wrapper);
        }
    }
    if not_watching_repos.is_empty() {
        println!("\nnot-watching repos (none)");
    } else {
        let mut sorted_not_watching_repos: Vec<_> =  not_watching_repos.into_iter().collect();
        sorted_not_watching_repos.sort_by(sorter);
        println!("\nnot-watching repos ({})", sorted_not_watching_repos.len());
        for repo_wrapper in sorted_not_watching_repos {
            report_repo_line("😴", repo_wrapper);
        }
    }
}

fn report_repo_line(prefix: &str, repo_wrapper: &RepoWrapper) {
    let repo = &repo_wrapper.value;
    let archived_descr = match repo.archived {
        true => String::from("(📁 archived) "),
        false => String::new()
    };
    let stars_descr = match repo.stargazers_count {
        0 => String::new(),
        1 => String::from("(✨ 1 star)"),
        n => format!("(✨ {} stars) ", n)
    };
    let forks_descr = match repo.forks_count {
        0 => String::new(),
        1 => String::from("(🍴 1 fork) "),
        n => format!("(🍴 {} forks) ", n),
    };
    println!("{} {} {}{}{}",
        prefix,
        repo.html_url,
        archived_descr,
        stars_descr,
        forks_descr,
    );
}

fn report_scope(scope: Scope, name_arg: Option<&str>) {
    match scope {
        Scope::All => {
            println!("scope: all your github organizations x all repositories");
        },
        Scope::Org => {
            match name_arg {
                Some(org_name) => println!("scope: {} github organization", org_name),
                None => unreachable!()
            }
        },
        Scope::User => {
            match name_arg {
                Some(org_name) => println!("Scope: github username {}", org_name),
                None => println!("scope: current authorized github user"),
            }
        }
    }
}
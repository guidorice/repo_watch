use clap::{Arg, App, ArgMatches, arg_enum, value_t};

pub const ACTIVE: &str = "active";
pub const ALL: &str = "all";
pub const SCOPE: &str = "scope";
pub const NEWEST: &str = "newest";
pub const ORG: &str = "org";
pub const SORT: &str = "sort";
pub const USER: &str = "user";
pub const NAME: &str = "name";

arg_enum!{
    #[derive(PartialEq, Debug)]
    /// Scope of github repositories to consider.
    pub enum Scope {
        Org,
        User,
        All
    }
}

arg_enum!{
    #[derive(PartialEq, Debug)]
    /// Available sort orders in report view.
    pub enum Sort {
        Alpha,
        Active,
        Newest
    }
}

pub fn get_args() -> ArgMatches<'static> {
     App::new("Repo 👀 Watch")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name(SCOPE)
            .possible_values(&Scope::variants())
            .case_insensitive(true)
            .help( "scope of search: github organization, github user, or all visible to user (all organizations)")
            .takes_value(true)
            .required(true)
        )
        .arg( Arg::with_name(NAME)
            .short("n")
            .long(NAME)
            .help("github user name or organization name (defaults to current authenticated user)")
            .takes_value(true)
        )
        .arg(
            Arg::with_name(SORT)
                .possible_values(&Sort::variants())
                .case_insensitive(true)
                .default_value("alpha")
                .help("sort by recent activity, or creation date (defaults to alpha (alphabetical)")
        )
        .after_help("EXAMPLES:

    # repos belonging to authenticated user
    repo_watch user

    # public repos belonging to another github username
    repo_watch user --name {username}

    # repos belonging to an organization name
    repo_watch org --name {orgname}

    # all repos * all orgs, which the authenticated user has visibility into
    repo_watch all

    # optionally specify sort order: alpha, active, or newest
    repo_watch all active
        ")
        .get_matches()
}

/// Early out if incompatible cli arguments are found.
pub fn check_args(args: &ArgMatches) {
    // TODO: clap config may be able to enforce this check?
    let scope = value_t!(args, SCOPE, Scope);
    if let Ok(scope) = scope {
        if let Scope::Org = scope {
            if value_t!(args, NAME, String).is_err() {
                panic!("organization name is required with org scope. try --name org_name .");
            }
        }
    }
}
# repo_watch

<a href="https://github.com/guidorice/repo_watch/https://github.com/guidorice/repo_watch/actionactions?query=workflow%3ARust" target="_blank">
    <img src="https://github.com/guidorice/repo_watch/workflows/Rust/badge.svg" alt="CI">
</a>

Manage watched vs unwatched GitHub repositories.

### Background

When working in a GitHub organization with many repositories it can be tough to discover:

- what new repos exist that could be useful?
- what repos are getting a lot of activity, that I need to know about?
- what repos am I watching that I do not care about anymore?

[GitHub's notifications web app](https://github.com/notifications) is great, but
as far as I can tell it does not provide these answers. `repo_watch` is a command-line tool for discovering this info!

### Get Started

1. Clone the source
    ```shell script
    git clone https://github.com/guidorice/repo_watch.git
    cd repo_watch/
    ```
2. Get [Rust](https://www.rust-lang.org/) for building this app. https://rustup.rs/ is a good source if you do not have Rust. 
    (This works on 1.46 stable)
3. Build and install using Rust's `cargo` command:
    ```shell script
    cargo install --path .
    # now a repo_watch binary should be on your PATH
    repo_watch --help   
    ```
4. `repo_watch` uses the GitHub API, so get a GitHub token if you do not have one already, see https://github.com/settings/tokens .
    Set your token in `GITHUB_TOKEN` environment variable. Alternatively, you set add the GITHUB_TOKEN in a `.env` file.
    ```shell script
    export GITHUB_TOKEN=******
    ```

### Usage

```
$ repo_watch --help
Repo 👀 Watch 0.1.0
Alex G Rice <alex@ricegeo.dev>
Manage watched vs unwatched github repositories.

USAGE:
    repo_watch [OPTIONS] <scope> [sort]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>    github user name or organization name (defaults to current authenticated user)

ARGS:
    <scope>    scope of search: github organization, github user, or all visible to user (all organizations)
               [possible values: Org, User, All]
    <sort>     sort by recent activity, or creation date (defaults to alpha (alphabetical) [default: alpha]
               [possible values: Alpha, Active, Newest]

EXAMPLES:

    # repos belonging to authenticated user
    repo_watch user

    # public repos belonging to another github username
    repo_watch user {username}

    # repos belonging to an organization name
    repo_watch org {orgname}

    # all repos * all orgs, which the authenticated user has visibility into
    repo_watch all

    # optionally specify sort order: alpha, active, or newest
    repo_watch all active
```

### Example

I am not currently a member of the georust organization, but I can still see all it's public repos!
Note: `repo_watch` works with private repositories as well, since it authenticates you to the Github API.

```
$ repo_watch org --name georust
fetching github user info...
hello, guidorice!
fetching list of repos you are watching...
found 69 watching repos
fetching repos owned by organization georust...

scope: georust github organization

watching repos (9)
😍 https://github.com/georust/docker-images 
😍 https://github.com/georust/gdal (✨ 89 stars) (🍴 28 forks) 
😍 https://github.com/georust/geo (✨ 577 stars) (🍴 71 forks) 
😍 https://github.com/georust/geocoding (✨ 35 stars) (🍴 9 forks) 
😍 https://github.com/georust/geohash (✨ 41 stars) (🍴 11 forks) 
😍 https://github.com/georust/geojson (✨ 105 stars) (🍴 22 forks) 
😍 https://github.com/georust/georust.org (🍴 1 fork) 
😍 https://github.com/georust/geotiff (✨ 7 stars) (🍴 5 forks) 
😍 https://github.com/georust/osm (✨ 12 stars) (🍴 3 forks) 

not-watching repos (17)
😴 https://github.com/georust/assets 
😴 https://github.com/georust/geographiclib-rs (✨ 3 stars) 
😴 https://github.com/georust/geos (✨ 37 stars) (🍴 17 forks) 
😴 https://github.com/georust/geos-sys (✨ 1 star)
😴 https://github.com/georust/gpx (✨ 31 stars) (🍴 15 forks) 
😴 https://github.com/georust/meta (✨ 1 star)
😴 https://github.com/georust/netcdf (✨ 25 stars) (🍴 11 forks) 
😴 https://github.com/georust/polyline (✨ 10 stars) (🍴 4 forks) 
😴 https://github.com/georust/proj (✨ 37 stars) (🍴 10 forks) 
😴 https://github.com/georust/proj-sys (🍴 5 forks) 
😴 https://github.com/georust/robust (✨ 5 stars) (🍴 3 forks) 
😴 https://github.com/georust/shapefile (✨ 4 stars) (🍴 2 forks) 
😴 https://github.com/georust/tilejson (✨ 1 star)(🍴 2 forks) 
😴 https://github.com/georust/topojson (✨ 5 stars) (🍴 2 forks) 
😴 https://github.com/georust/transitfeed (✨ 7 stars) (🍴 3 forks) 
😴 https://github.com/georust/wkt (✨ 25 stars) (🍴 11 forks) 
😴 https://github.com/georust/world-file (✨ 4 stars) 
```
In Terminal.app you can click through the github.com URLs to 
see additional info and of course, to Watch/
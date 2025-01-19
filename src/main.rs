use anyhow::Error;
use clap::{Arg, ArgMatches, Command, ArgAction};
use git2::Repository;
use std::path::Path;

fn get_https_url(matches: &ArgMatches, repo: &Repository) -> Result<String, Error> {
    let url = get_remote_url(matches, repo);
    if url.starts_with("http://") || url.starts_with("https://") {
        return Ok(format!("Browser URL: {}", url));
    } else if url.starts_with("git@") {
        let https_url = url.replace(":", "/").replace("git@", "https://");
        let https_url = https_url.trim_end_matches(".git");

        // println!("I am about to open this url: {}", https_url);

        // let path = https_url;
        return Ok(https_url.to_string());
    } else if url.starts_with("ssh://") {
        return Err(Error::msg(
            "The URL is using SSH, you might need to convert it manually.",
        ));
    } else {
        return Err(Error::msg(format!(
            "The URL is using another protocol. huh? {}",
            url
        )));
    }
}

fn get_remote_url(matches: &ArgMatches, repo: &Repository) -> String {
    let remote_name = matches
        .get_one::<String>("remote")
        .map(String::as_str)
        .unwrap_or("origin");

    let remote = match repo.find_remote(remote_name) {
        Ok(remote) => remote,
        Err(e) => {
            eprintln!("Failed to find remote: {}", e);
            return String::from("");
        }
    };

    if let Some(url) = remote.url() {
        return String::from(url);
    } else {
        eprintln!("Remote URL not found");
        return "".to_string();
    }
}

fn add_branch(repo: &Repository, url: String) -> String {
    let branch = repo
        .head()
        .ok()
        .and_then(|h| h.shorthand().map(String::from))
        .unwrap_or(String::from(""));

    if !branch.is_empty() {
        format!("{}/tree/{}", url, branch)
    } else {
        url
    }
}

fn open_url(path: &str) {
    match open::that(path) {
        Ok(()) => println!("Opened '{}' successfully.", path),
        Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
    }
}

fn main() {
    let matches = Command::new("git-open")
        .version("1.0")
        .about("Open the GitHub/GitLab/Bitbucket page for the repository or branch")
        .arg(
            Arg::new("branch")
                .short('b')
                .long("branch")
                .help("Branch name (default: current branch)"),
        )
        .arg(
            Arg::new("remote")
                .short('r')
                .long("remote")
                .help("Remote name (default: `origin`)"),
        )
        .arg(
            Arg::new("print")
                .action(ArgAction::SetTrue)
                .short('p')
                .long("print")
                .help("Print the URL instead of opening it"),
        )
        .get_matches();

    // NOTE: I want to get the name of the default branch
    // let config = Config::open_default().expect("Failed to load Git config");

    let repo = match Repository::open(Path::new(".")) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository or not in one: {}", e);
            return;
        }
    };

    match get_https_url(&matches, &repo) {
        Ok(url) => {
            let url = add_branch(&repo, url);
            if matches.get_flag("print") == true {
                println!("{}",url);
            } else {
                open_url(&url);
            }
        }
        Err(msg) => {
            eprintln!("{}", msg)
        }
    }
}

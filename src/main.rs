use clap::{arg, command, value_parser, ArgAction, Command};
use git2::Repository;
use std::path::Path;

fn main() {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .get_matches();

    open_remote();
}

fn open_remote() {
    let repo_path = Path::new(".");

    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository or not in one: {}", e);
            return;
        }
    };

    let remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(e) => {
            eprintln!("Failed to find remote: {}", e);
            return;
        }
    };

    if let Some(url) = remote.url() {
        println!("Remote URL: {}", url);

        if url.starts_with("http://") || url.starts_with("https://") {
            println!("Browser URL: {}", url);
        } else if url.starts_with("git@") {
            let https_url = url.replace(":", "/").replace("git@", "https://");
            let https_url = https_url.trim_end_matches(".git");
            println!("I am about to open this url: {}", https_url);
            let path = https_url;
            match open::that(path) {
                Ok(()) => println!("Opened '{}' successfully.", path),
                Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
            }
        } else if url.starts_with("ssh://") {
            println!("The URL is using SSH, you might need to convert it manually.");
        } else {
            println!("The URL is using another protocol. huh?");
        }
    } else {
        eprintln!("Remote URL not found");
    }
}

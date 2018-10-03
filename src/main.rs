
extern crate git2;
use git2::Repository;

use std::fs::*;
use std::path::{Path, PathBuf};

fn main() {
    println!("Hello, world!");

    let output_dir = PathBuf::from("/tmp/rstow");
    create_dir(output_dir.as_path());

    let repository_url = "https://github.com/qboileau/rstow.git";

    let repo = get_repository(repository_url, output_dir.as_path());
}



fn get_repository(url: &str, output_dir: &Path) -> Repository {
    let repo = match Repository::clone(url, output_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    repo
}

extern crate walkdir;

use std::collections::HashSet;

use clap::Parser;
use walkdir::{DirEntry, WalkDir};
use wildmatch::WildMatch;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(
    author = "Danny F. Pires",
    version,
    about = "File and Directory Search"
)]
struct Search {
    /// The path to start the search default is "./"
    starting_path: Option<String>,
    #[clap(long)]
    /// The pattern to search for
    name: Option<String>,
    #[clap(long = "type")]
    /// The type of search
    search_type: Option<String>,
    #[clap(long = "max-open")]
    /// Max open paths, doesn't impact final results but tradesoff memory for speed default = 3
    max_open: Option<usize>,
    #[clap(long = "exclude")]
    /// Paths you which to exclude can be set as a comma separated list
    excluded_paths: Option<String>,
}

fn search(file: &DirEntry, wildcard: &WildMatch, search_type: &str) -> bool {
    let is_matching: bool = wildcard.matches(file.file_name().to_str().unwrap_or(""));
    if is_matching {
        println!("{}", file.path().display());
    }
    return match search_type {
        "f" => file.file_type().is_file() && is_matching,
        "d" => file.file_type().is_dir() && is_matching,
        "l" => file.path_is_symlink() && is_matching, //TODO: Add test cases for symbolic links
        _ => is_matching,
    };
}

fn valid_file(exclusion_set: &HashSet<&str>, file_path: &str) -> bool {
    let mut wild_match: WildMatch;
    for exclusion_item in exclusion_set {
        if exclusion_item.starts_with('.') || exclusion_item.starts_with('/') {
            wild_match = WildMatch::new(&(exclusion_item.to_string() + "/*"))
        } else {
            wild_match = WildMatch::new(&("./".to_owned() + exclusion_item + "/*"));
        }
        if wild_match.matches(file_path) {
            return false;
        }
    }
    return true;
}

fn main() {
    let args: Search = Search::parse();
    let starting_dir: &str = &args.starting_path.unwrap_or(".".to_owned());
    let cleaned_starting_dir: &str =
        if starting_dir.starts_with('.') || starting_dir.starts_with('/') {
            starting_dir
        } else {
            &("./".to_owned() + starting_dir)
        };
    let search_term: &str = &args.name.unwrap_or("**".to_owned()); // Bound search by tearm by start and end
    let search_type: &str = &args.search_type.unwrap_or("".to_owned());
    let max_open: usize = match args.max_open {
        Some(x) => x,
        None => 3,
    };
    let exclude_string: &str = &args.excluded_paths.unwrap_or("".to_string());
    let wildcard: WildMatch = WildMatch::new(search_term);
    let exclusion_set: HashSet<&str> = exclude_string.split(",").collect::<HashSet<&str>>();
    for file in WalkDir::new(&cleaned_starting_dir)
        .max_open(max_open)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if valid_file(&exclusion_set, file.path().to_str().unwrap_or("")) {
            search(&file, &wildcard, &search_type);
        }
    }
}

#[cfg(test)]
mod test;

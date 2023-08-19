extern crate walkdir;
use std::collections::HashSet;

use walkdir::WalkDir;
use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("{}", args.len());
        println!("Invalid number of args!");
        println!("Two args required <starting-dir> <search-term> <search-type>");
        return;
    }
    let starting_dir: String = args.get(1).unwrap().to_string();
    let search_term: String = args.get(2).unwrap().to_string();
    let temp: &String = &String::from("");
    let search_type: String = args.get(3).unwrap_or_else(|| temp).to_string();
    println!("search_type: {}", search_type);
    let valid_set: HashSet<String> = HashSet::from([String::from("dir"), String::from("d"), String::from("file"), String::from("f")]);
    if !valid_set.contains(&search_type) {
        println!("Invalid search type, please provide arguments \"dir\" or \"file\"");
        return;
    }
    search(starting_dir, search_term, search_type);
}

fn search(starting_dir: String, search_term: String, search_type: String) -> () {
    let empty_str: &String = &String::from("");
    let regex_search_term: String = String::from("^") + search_term.as_str() + &String::from("$"); // Bound search by tearm by start and end
    let regex = Regex::new(&regex_search_term).unwrap();

    for file in WalkDir::new(starting_dir).into_iter().filter_map(|file| file.ok()) {
        if (search_type == "file" || search_type == "f") && regex.is_match(file.file_name().to_str().unwrap_or_else(|| empty_str)) {
            println!("Found matching File: {}", file.path().display());
        } else if (search_type == "dir" || search_type == "d") && file.path().to_string_lossy().ends_with(&search_term) {
            println!("Found matching directory: {}", file.path().to_str().unwrap());
        }
    }
}


use super::*;

fn walkdir_search(
    search_term: &str,
    starting_dir: &str,
    function: &dyn Fn(&DirEntry, &wildmatch::WildMatch) -> bool,
) -> Vec<bool> {
    let wildcard: WildMatch = WildMatch::new(&search_term);
    let mut result_arr: Vec<bool> = Vec::new();
    for file in WalkDir::new(&starting_dir)
        .max_open(3)
        .into_iter()
        .filter_map(|f| f.ok())
    {
        let func_result: bool = function(&file, &wildcard);
        result_arr.push(func_result);
    }
    return result_arr;
}

#[test]
fn test_file_search() {
    let search_term: &str = "main.c";
    let starting_dir: &str = "./test/test-dir";
    let func = &search_file;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    println!("results: {:?}", result_arr);
    let result: bool = result_arr.contains(&true);
    assert!(result); //Should return true
}

#[test]
fn test_dir_search() {
    let search_term: &str = "dir1";
    let starting_dir: &str = "./test/test-dir";
    let func = &search_dir;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    println!("results: {:?}", result_arr);
    let result: bool = result_arr.contains(&true);
    assert!(result); //Should return true
}

#[test]
fn test_all_types_search() {
    let search_term: &str = "main.c";
    let starting_dir: &str = "./test/test-dir";
    let func = &search_all_types;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    println!("results: {:?}", result_arr);
    let result: bool = result_arr.contains(&true);
    assert!(result); //Should return true
}

#[test]
fn test_file_search_wildcard() {
    let search_term: &str = "main*";
    let starting_dir: &str = "./test/test-dir";
    let func = &search_file;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    let mut count = 0;
    for res in result_arr.into_iter() {
        if res {
            count += 1;
        }
    }
    assert!(count == 1); //Should return true
}

#[test]
fn test_all_dir_search_default() {
    let search_term: &str = "**";
    let starting_dir: &str = "./test/test-dir";
    let func = &search_dir;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    let mut count = 0;
    for res in result_arr.into_iter() {
        if res {
            count += 1;
        }
    }
    assert!(count == 3); // Includes the starting dir

}

#[test]
fn test_dir_search_wildcard() {
    let search_term: &str = "dir*";
    let starting_dir: &str = "./test";
    let func = &search_dir;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    let mut count = 0;
    for res in result_arr.into_iter() {
        if res {
            count += 1;
        }
    }
    assert!(count == 2); //Should return true
}

#[test]
fn test_all_types_search_wildcard() {
    let search_term: &str = "main*";
    let starting_dir: &str = "./test/test-dir";
    let func = &search_all_types;

    let result_arr: Vec<bool> = walkdir_search(search_term, starting_dir, func);
    let mut count = 0;
    for res in result_arr.into_iter() {
        if res {
            count += 1;
        }
    }
    assert!(count == 1); //Should return true
}

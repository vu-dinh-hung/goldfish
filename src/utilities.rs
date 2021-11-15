//! # Common Utilities
use std::path::PathBuf;
use std::fs;
use std::env;
use nanoid::nanoid;

pub fn diff(lines1: Vec<String>, lines2: Vec<String>) -> Vec<(String, String)> {
    //! Returns a list of differences (line by line) in the two strings
    //! (e.g. [('+', 'this_line_only_in_2\n'), ('=', 'common_line\n'), ('-', 'this_line_not_in_2\n)])
    todo!()
}

pub fn generate_id() -> String {
    nanoid!(10, &nanoid::alphabet::SAFE)
}

pub fn map_path_to_snapshot<'a>(path: &'a str, snapshot_path: &'a str) -> String {
    let mut dir_path = env::current_dir().unwrap();
    let absolute_path = fs::canonicalize(PathBuf::from(path)).unwrap();
    let relative_path = absolute_path.to_str().unwrap()[dir_path.to_str().unwrap().chars().count()+1..].to_string();
    dir_path.push(snapshot_path);
    dir_path.push(relative_path);
    return dir_path.to_str().unwrap().to_string();
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_1_diff() {
        //! Check that the right differences are returned from
        todo!();
    }

    #[test]
    fn test_2_hash() {
        //! Check that several different strings return unique hashes
        todo!();
    }
}

//! # Common Utilities
use std::path::PathBuf;
use std::fs;
use std::env;
use nanoid::nanoid;
use sha2::{Sha256, Digest};

pub fn diff(lines1: Vec<String>, lines2: Vec<String>) -> Vec<(String, String)> {
    //! Returns a list of differences (line by line) in the two strings
    //! (e.g. [('+', 'this_line_only_in_2\n'), ('=', 'common_line\n'), ('-', 'this_line_not_in_2\n)])
    todo!()
}

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:X}", hasher.finalize())
}

pub fn generate_id() -> String {
    nanoid!(10, &nanoid::alphabet::SAFE)
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

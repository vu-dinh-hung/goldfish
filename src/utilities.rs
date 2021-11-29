//! # Common Utilities
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::hash::Hash;

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

// True if equal
pub fn compare_map<K: Eq + Hash, V: Eq>(m1: &HashMap<K, V>, m2: &HashMap<K, V>) -> bool {
    for (k, v) in m1 {
        if !m2.contains_key(&k) || !v.eq(&m2[&k]) {
            return false;
        }
    }
    return true;
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

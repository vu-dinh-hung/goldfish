// # Common Utilities
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::hash::Hash;

// Returns a list of differences (line by line) in the two strings
// (e.g. [('+', 'this_line_only_in_2\n'), ('=', 'common_line\n'), ('-', 'this_line_not_in_2\n)])
pub fn diff(a: Vec<String>, b: Vec<String>) -> Vec<(String, String)> {
    let lcs_result: Vec<String> = lcs(&a, &b);
    let mut diff: Vec<(String, String)> = vec![];
    let mut index_a: usize = 0;
    let mut index_b: usize = 0;
    let mut index: usize = 0;
    while index < lcs_result.len() && index_a < a.len() && index_b < b.len() {
        let equal_a = lcs_result[index] == a[index_a];
        let equal_b = lcs_result[index] == b[index_b];
        if equal_a && equal_b {
            diff.push(("=".to_owned(), lcs_result[index].to_string()));
            index_a += 1;
            index_b += 1;
            index += 1;
        } else if equal_a {
            diff.push(("+".to_owned(), b[index_b].to_owned()));
            index_b += 1;
        } else if equal_b {
            diff.push(("-".to_owned(), a[index_a].to_owned()));
            index_a += 1;
        } else {
            diff.push(("+".to_owned(), b[index_b].to_owned()));
            diff.push(("-".to_owned(), a[index_a].to_owned()));
            index_a += 1;
            index_b += 1;
        }
    }
    while index_a < a.len() && index_b < b.len() {
        diff.push(("+".to_owned(), b[index_b].to_owned()));
        diff.push(("-".to_owned(), a[index_a].to_owned()));
        index_a += 1;
        index_b += 1;
    }
    while index_b < b.len() {
        diff.push(("+".to_owned(), b[index_b].to_owned()));
        index_b += 1;
    }
    while index_a < a.len() {
        diff.push(("-".to_owned(), a[index_a].to_owned()));
        index_a += 1;
    }
    diff
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

// Longest common subsequence between two vectors of strings
fn lcs(a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for (i, str_a) in a.iter().enumerate() {
        for (j, str_b) in b.iter().enumerate() {
            let mut added: Vec<String> = vec![];
            if str_a == str_b {
                if i > 0 && j > 0 {
                    let prev_res = result.get(&format!("{}#{}", i - 1, j - 1)).unwrap();
                    for s in prev_res.iter() {
                        added.push(s.to_string());
                    }
                }
                added.push(str_a.to_string());
            } else {
                let len_i = if i > 0 { result.get(&format!("{}#{}", i - 1, j)).unwrap().len() } else { 0 };
                let len_j = if j > 0 { result.get(&format!("{}#{}", i, j - 1)).unwrap().len() } else { 0 };
                if len_i > len_j && len_i > 0 {
                    let prev_res = result.get(&format!("{}#{}", i - 1, j)).unwrap();
                    for s in prev_res.iter() {
                        added.push(s.to_string());
                    }
                } else if len_j > 0 {
                    let prev_res = result.get(&format!("{}#{}", i, j - 1)).unwrap();
                    for s in prev_res.iter() {
                        added.push(s.to_string());
                    }
                }
            }
            result.insert(format!("{}#{}", i, j), added);
        }
    }
    result.get(&format!("{}#{}", a.len() - 1, b.len() - 1)).unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::utilities::diff;
    #[test]
    fn test_1_diff() {
        //! Check that the right differences are returned from
        let a: Vec<String> = vec!["hello".to_string(), "hello".to_string(), "hello".to_string()];
        let b: Vec<String> = vec!["hello".to_string(), "hi".to_string(), "hello".to_string(), "hi".to_string()];
        let mut expected_result: Vec<(String, String)> = vec![];
        expected_result.push(("=".to_string(), "hello".to_string()));
        expected_result.push(("+".to_string(), "hi".to_string()));
        expected_result.push(("=".to_string(), "hello".to_string()));
        expected_result.push(("+".to_string(), "hi".to_string()));
        expected_result.push(("-".to_string(), "hello".to_string()));
        assert_eq!(expected_result, diff(a,b));
    }
}

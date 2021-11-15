//! Filesystem
//! Note that this module uses std::io::Result, not the Result class from prelude (std::result::Result).
use std::io::Result;
use std::path::Path;
use std::fs;


pub fn save_file(data: &str, path: &str) -> Result<()> {
    return fs::write(path, data);
}

pub fn read_file(path: &str) -> Result<String> {
    let content = fs::read_to_string(path);
    return content;
}

pub fn create_directory(path: &str) -> Result<()> {
    fs::create_dir_all(path)
}

pub fn remove_file(path: &str, recursive: bool) -> Result<()> {
    todo!()
}

pub fn move_file(source: &str, dest: &str, recursive: bool, copy: bool) -> Result<()> {
    todo!()
}

pub fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn parent(path: &str) -> Option<String> {
    match Path::new(path).parent() {
        Some(path_obj) => match path_obj.to_str() {
            Some(path_string) => Some(path_string.to_string()),
            None => None
        }
        None => None
    }
}

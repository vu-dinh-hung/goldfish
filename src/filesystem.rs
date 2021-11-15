//! Filesystem
//! Note that this module uses std::io::Result, not the Result class from prelude (std::result::Result).
use std::io::Result;


pub fn save_file(data: String, path: String) -> Result<()> {
    todo!()
}

pub fn read_file(path: String) -> Result<String> {
    todo!()
}

pub fn create_directory(path: String) -> Result<()> {
    todo!()
}

pub fn remove_file(path: String, recursive: bool) -> Result<()> {
    todo!()
}

pub fn move_file(source: String, dest: String, recursive: bool, copy: bool) -> Result<()> {
    todo!()
}

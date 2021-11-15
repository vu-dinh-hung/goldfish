//! Filesystem
use std::io;
use std::path::Path;
use std::fs;


pub fn write_file(data: &str, path: &str) -> io::Result<()> {
    //! Write data to the specified file
    //! If the file does not exist, create the file as well as all intermediate parent folders
    let parent_folder = parent(path);
    match parent_folder {
        Some(folder) => {
            if !is_directory(folder.as_str()) {
                create_directory(folder.as_str())?  // return early with the IO error if this errors out
            }
            fs::write(path, data)
        }
        None => Err(io::Error::new(io::ErrorKind::Other, "Cannot create the specified path"))
    }
}

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn create_directory(path: &str) -> io::Result<()> {
    //! Create the given path, including all intermediate directories
    fs::create_dir_all(path)
}

pub fn remove_file(path: &str, recursive: bool) -> io::Result<()> {
    todo!()
}

pub fn move_file(source: &str, dest: &str, recursive: bool, copy: bool) -> io::Result<()> {
    todo!()
}

pub fn is_directory(path: &str) -> bool {
    //! Check if path is a directory
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    //! Check if path is a file
    Path::new(path).is_file()
}

pub fn parent(path: &str) -> Option<String> {
    //! Get the parent path of the given path
    Path::new(path).parent()?.to_str().map(|path_string| path_string.to_string())
}

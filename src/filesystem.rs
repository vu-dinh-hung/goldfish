// pub fn test_1_read_file(){
// 	assert_eq!(dvcs.read_file(fileContainingHelloWorld), Ok("Hello World"))
// }
// pub fn test_2_write_file(){
// 	assert_eq!(dvcs.write_file(invalidpath, somedata), Err("InvalidPathError"))
// }
// pub fn test_3_move_file(){
// 	assert_eq!(dvcs.move_file(sourcepath, destpath, c=true), true)
// 	assert_eq!(dvcs.read_file(sourcepath), dvcs.read_file(destpath))
// }
// pub fn test_4_remove_file(){
// 	assert_eq!(dvcs.remove_file(path, r=true), true)
// 	pathprime = path + "/filename"
// 	assert_eq!(dvcs.write_file(pathprime), Err("InvalidPathError"))
// }

//! Filesystem
use pathdiff;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn pathbuf_to_string(path: PathBuf) -> String {
    //! Unsafe function (but mostly safe practically) to convert a PathBuf object into a String
    path.into_os_string().into_string().unwrap()
}

pub fn get_absolute_path(path: &str) -> PathBuf {
    return fs::canonicalize(path).unwrap();
}

pub fn get_relative_path_to_wd(base: &str, rel_path: &str) -> String {
    let abs_path = pathbuf_to_string(get_absolute_path(rel_path));
    return diff_path(base, abs_path.as_str()).unwrap();
}

pub fn diff_path(base: &str, path: &str) -> Option<String> {
    pathdiff::diff_paths(Path::new(path), Path::new(base)).map(|pb| pathbuf_to_string(pb))
}

pub fn list_files(path: &str, recursive: bool, exclude: &Vec<&str>) -> io::Result<Vec<String>> {
    //! List all files in the given directory. Returns an IO error if given invalid path(s)
    let mut result = vec![];
    'main_loop: for entry in fs::read_dir(path)? {
        let entry = entry?;
        let cur_path = pathbuf_to_string(entry.path());

        for ex in exclude.iter() {
            if canonicalize(cur_path.as_str())? == canonicalize(ex)? {
                continue 'main_loop;
            }
        }

        if is_dir(cur_path.as_str()) {
            if recursive {
                let mut inner_files = list_files(cur_path.as_str(), true, exclude)?;
                result.append(&mut inner_files);
            };
        } else {
            result.push(cur_path);
        }
    }
    Ok(result)
}

pub fn write_file(data: &str, path: &str) -> io::Result<()> {
    //! Write data to the specified file
    //! If the file does not exist, create the file as well as all intermediate parent folders
    let parent_folder = parent(path);
    match parent_folder {
        Some(folder) => {
            if !is_dir(folder.as_str()) {
                create_dir(folder.as_str())? // return early with the IO error if this errors out
            }
            fs::write(path, data)
        }
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "Cannot create the specified path",
        )),
    }
}

pub fn read_file(path: &str) -> io::Result<String> {
    //! Read all the contents of a file to a String
    fs::read_to_string(path)
}

pub fn join_path(paths: Vec<&str>) -> String {
    //! Join the paths into a path string (in the format of the host OS)
    //! Currently panics if the conversion from OsString to String (after joining)
    //! fails. This will be fixed to be safer later.
    paths
        .iter()
        .fold(Path::new("").to_path_buf(), |acc, path| acc.join(path))
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn create_dir(path: &str) -> io::Result<()> {
    //! Create the given path, including all intermediate directories
    fs::create_dir_all(path)
}

pub fn remove(path: &str) -> io::Result<()> {
    if is_dir(path) {
        fs::remove_dir_all(path)
    } else if is_file(path) {
        fs::remove_file(path)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Invalid path"))
    }
}

pub fn list(path: &str) -> io::Result<Vec<String>> {
    fs::read_dir(path).map(|iterator| {
        iterator
            .map(|dir_entry_result| pathbuf_to_string(dir_entry_result.unwrap().path()))
            .collect()
    })
}

pub fn copy(source: &str, dest: &str) -> io::Result<()> {
    if is_dir(source) {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            match copy(
                entry.path().to_str().unwrap(),
                Path::new(dest).join(entry.file_name()).to_str().unwrap(),
            ) {
                Ok(_x) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    } else if is_file(source) {
        let path = Path::new(dest);
        let prefix = path.parent().unwrap();
        match fs::create_dir_all(prefix) {
            Ok(_x) => (),
            Err(e) => return Err(e),
        }
        fs::copy(source, dest)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Invalid path"))
    }
}

pub fn is_dir(path: &str) -> bool {
    //! Check if path is a directory
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    //! Check if path is a file
    Path::new(path).is_file()
}

pub fn parent(path: &str) -> Option<String> {
    //! Get the parent path of the given path
    Path::new(path)
        .parent()?
        .to_str()
        .map(|path_string| path_string.to_string())
}

pub fn canonicalize(path: &str) -> io::Result<String> {
    //! Return the full canonical path for the given path
    fs::canonicalize(Path::new(path)).map(pathbuf_to_string)
}

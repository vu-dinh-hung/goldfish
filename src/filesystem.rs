
pub fn test_1_read_file(){
	asserteq!(dvcs.read_file(fileContainingHelloWorld), Ok("Hello World"))
}
pub fn test_2_write_file(){
	asserteq!(dvcs.write_file(invalidpath, somedata), Err("InvalidPathError"))
}
pub fn test_3_move_file(){

	asserteq!(dvcs.move_file(sourcepath, destpath, c=true), true)
	asserteq!(dvcs.read_file(sourcepath), dvcs.read_file(destpath))
}
pub fn test_4_remove_file(){
	asserteq!(dvcs.remove_file(path, r=true), true)
	pathprime = path + "/filename"
	asserteq!(dvcs.write_file(pathprime), Err("InvalidPathError"))
}

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
            if !is_dir(folder.as_str()) {
                create_dir(folder.as_str())?  // return early with the IO error if this errors out
            }
            fs::write(path, data)
        }
        None => Err(io::Error::new(io::ErrorKind::Other, "Cannot create the specified path"))
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
    paths.iter().fold(Path::new("").to_path_buf(), |acc, path| acc.join(path))
        .into_os_string().into_string().unwrap()
}

pub fn create_dir(path: &str) -> io::Result<()> {
    //! Create the given path, including all intermediate directories
    fs::create_dir_all(path)
}

pub fn remove_file(path: &str, recursive: bool) -> io::Result<()> {
    todo!()
}

pub fn copy_file(source: &str, dest: &str) -> io::Result<u64> {
    let path = Path::new(dest);
    let prefix = path.parent().unwrap();
    match fs::create_dir_all(prefix) {
        Ok(_x) => (),
        Err(e) => return Err(e),
    }
    fs::copy(source, dest)
}

pub fn copy_dir(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let typ = entry.file_type()?;
        if typ.is_dir() {
            copy_dir(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            copy_file(entry.path().to_str().unwrap(), dest.as_ref().join(entry.file_name()).to_str().unwrap())?;
        }
    }
    Ok(())
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
    Path::new(path).parent()?.to_str().map(|path_string| path_string.to_string())
}


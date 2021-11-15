use toml::Value;
use crate::filesystem;

const GOLDFISH: &str = ".goldfish/state.toml";

/* Public Struct */

// Data structure to interact with a file
pub struct VirtualFile {}

// Data structure to represent a revision
pub struct Revision {}

/* Public Enum */
pub enum LineChangeState {
    Delete,
    Add,
}

pub enum FileChangeState {
    Modify,
    Delete,
    Add,
}

#[derive(Debug)]
pub enum Error {
    FailToLoadGoldfish,
    SomethingWentWrong,
    NotFile,
    TrackedFileAlreadyAdded,
}

/* Internal methods */
fn diff_virtual_files(vf1: &VirtualFile, vf2: &VirtualFile) -> VirtualFile {
    todo!()
}

fn merge_virtual_files(vf1: &VirtualFile, vf2: &VirtualFile) -> VirtualFile {
    todo!()
}
fn get_list_of_track_files() -> Result<Vec<String>, Error> {
    todo!()
}

/* External methods */

/**
 * Add a file to track list, use FileSystem to update tracking list file
 *
 * @param path: path to file to add
 * @return: Some(Error) if failed, None otherwise
 */
pub fn add_track_file(path: &str) -> Option<Error> {
    // sanity check
    if !filesystem::is_file(path) {
        return Some(Error::NotFile);
    }
    // read Goldfish file into goldfish_raw
    let mut goldfish_raw: String;
    match filesystem::read_file(GOLDFISH) {
        Ok(raw) => goldfish_raw = raw,
        Err(_e) => return Some(Error::FailToLoadGoldfish)
    }
    // deserialize raw
    let mut goldfish_info: Value;
    match toml::from_str(goldfish_raw.as_str()) {
        Ok(val) => goldfish_info = val,
        Err(_e) => return Some(Error::FailToLoadGoldfish)
    }
    // add tracked file
    if goldfish_info.get("tracked_file").is_none() {
        goldfish_info.as_table_mut().unwrap().insert(String::from("tracked_file"), Value::try_from(Vec::new() as Vec<Value>).unwrap());
    }
    match goldfish_info["tracked_file"].as_array_mut() {
        Some(vector) => {
            vector.push(Value::try_from(path).unwrap());
            vector.dedup();
        }
        None => return Some(Error::FailToLoadGoldfish)
    }
    // serialize goldfish
    match toml::to_string(&goldfish_info) {
        Ok(s) => goldfish_raw = s,
        Err(_e) => return Some(Error::SomethingWentWrong)
    }
    // write back to Goldfish file
    match filesystem::write_file(goldfish_raw.as_str(), GOLDFISH) {
        Ok(_v) => None,
        Err(_e) => Some(Error::SomethingWentWrong),
    }
}

/**
 * Delete a file to track list, use FileSystem to update tracking list file
 *
 * @param path: path to file to delete
 * @return: Some(Error) if failed, None otherwise
 */
pub fn delete_track_file(path: String) -> Option<Error> {
    todo!()
}

/**
 * Get current revision, Use Filesystem to read HEAD file
 *
 * @return: Err(Error) if failed, Ok(Revision) otherwise
 */
pub fn get_current_revision() -> Result<Revision, Error> {
    todo!()
}

/**
 * Get current branch, Use Filesystem to read HEAD file
 *
 * @return: Err(Error) if failed, Ok(String) otherwise
 */
pub fn get_current_branch() -> Result<String, Error> {
    todo!()
}

/**
 * Read file and create a VirtualFile from it, use FileSystem to read the file
 *
 * @param path: path to file
 * @return: Err(Error) if failed, Ok(VirtualFile) otherwise
 */
pub fn create_virtual_file_from_path(path: String) -> Result<VirtualFile, Error> {
    // Read file from path
    todo!()
}

/**
 * Read file from a revision and create a VirtualFile from it, use FileSystem to read the file
 *
 * @param path: path to file
 * @param rev: revision of the file
 * @return: Err(Error) if failed, Ok(VirtualFile) otherwise
 */
pub fn create_virtual_file_from_revision_path(path: String, rev: Revision) -> Result<VirtualFile, Error> {
    todo!()
}

/**
 * Get list of revision, use FileSystem to read revision list file
 *
 * @return: Err(Error) if failed, Ok(Vec<Revision>) otherwise
 */
pub fn get_list_of_revisions() -> Result<Vec<Revision>, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_diff_virtual_files() {
        todo!();
    }

    #[test]
    fn test_merge_virtual_files() {
        todo!();
    }

    #[test]
    fn test_get_list_of_track_file_corrupted_dvcs() {
        todo!();
    }

    #[test]
    fn test_get_list_of_track_file_normal() {
        todo!();
    }

    #[test]
    fn test_add_file_not_exist() {
        todo!();
    }

    #[test]
    fn test_add_file_normal() {
        todo!();
    }

    #[test]
    fn test_delete_file_not_exist() {
        todo!();
    }

    #[test]
    fn test_delete_file_normal() {
        todo!();
    }

    #[test]
    fn test_get_current_revision_corrupted_dvcs() {
        todo!();
    }

    #[test]
    fn test_get_current_revision_normal() {
        todo!();
    }

    #[test]
    fn test_get_current_branch_corrupted_dvcs() {
        todo!();
    }

    #[test]
    fn test_get_current_branch_normal() {
        todo!();
    }

    #[test]
    fn create_virtual_file_from_path_not_exist() {
        todo!();
    }
}

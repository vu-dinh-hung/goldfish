use toml::Value;
use crate::filesystem;
use crate::utilities;
use std::collections::HashMap;

// root
pub const DVCS_ROOT_DIR: &str = ".goldfish";

// top-level directories
pub const BLOBS_DIR: &str = "blobs";
pub const STAGING_DIR: &str = "staging";
pub const COMMITS_DIR: &str = "commits";
pub const BRANCHES_DIR: &str = "branches";

// top-level files
pub const HEAD: &str = "HEAD";

// Loc's old stuff
const STATE: &str = ".goldfish/state.toml";
pub const STAGING: &str = ".goldfish/staging/";

/* Public Struct */
#[derive(Debug)]
pub struct Repository {
    working_path: String,
    dvcs_path: String,
}

impl Repository {
    pub fn find(path: &str) -> Option<Repository> {
        //! Find the root working path and .dvcs path of the current DVCS repository,
        //! and return a Repository object with those paths
        if !filesystem::is_dir(path) {
            return None
        }

        let current_dvcs_path = filesystem::join_path(vec![path, DVCS_ROOT_DIR]);
        if filesystem::is_dir(current_dvcs_path.as_str()) {
            return Some(Repository { working_path: path.to_owned(), dvcs_path: current_dvcs_path })
        }

        let parent = filesystem::parent(path)?;  // return None if there is no parent path
        Repository::find(parent.as_str())  // recursively find in parent path
    }

    pub fn get_dvcs_path<'a>(&'a self) -> &'a str {
        &self.dvcs_path
    }

    pub fn get_working_path<'a>(&'a self) -> &'a str {
        &self.working_path
    }
}

// Data structure to interact with a file
pub struct Blob {}

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

/* Internal functions */
fn diff_blobs(vf1: &Blob, vf2: &Blob) -> Option<Blob> {
    todo!()
}

fn merge_blobs(vf1: &Blob, vf2: &Blob) -> Blob {
    todo!()
}

fn get_list_of_track_files() -> Result<Vec<String>, Error> {
    todo!()
}

fn read_state() -> Result<Value, Error> {
    // read Goldfish file into goldfish_raw
    let goldfish_raw: String;
    match filesystem::read_file(STATE) {
        Ok(raw) => goldfish_raw = raw,
        Err(_e) => return Err(Error::FailToLoadGoldfish)
    }
    // deserialize raw
    let goldfish_state: Value;
    match toml::from_str(goldfish_raw.as_str()) {
        Ok(val) => goldfish_state = val,
        Err(_e) => return Err(Error::FailToLoadGoldfish)
    }
    return Ok(goldfish_state);
}

fn write_state(goldfish_state: Value) -> Option<Error> {
    // serialize goldfish
    let goldfish_raw: String;
    match toml::to_string(&goldfish_state) {
        Ok(s) => goldfish_raw = s,
        Err(_e) => return Some(Error::SomethingWentWrong)
    }
    // write back to Goldfish file
    match filesystem::write_file(goldfish_raw.as_str(), STATE) {
        Ok(_v) => return None,
        Err(_e) => return Some(Error::SomethingWentWrong),
    }
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
    // copy file to staging
    match filesystem::copy_file(path, utilities::map_path_to_snapshot(path, STAGING).as_str()) {
        Ok(_v) => (),
        Err(_e) => return Some(Error::SomethingWentWrong),
    }
    return None;
}

pub fn add_revision(rev_id: &String) -> Option<Error> {
    let mut goldfish_state: Value;
    match read_state() {
        Ok(state) => goldfish_state = state,
        Err(_e) => return Some(Error::FailToLoadGoldfish)
    }
    // add tracked file
    let cur_rev_id: String;
    match get_current_revision() {
        Ok(rev_id) => cur_rev_id = rev_id,
        Err(e) => return Some(e),
    }
    let mut rev: HashMap<String, Value> = HashMap::new();
    rev.insert("id".to_string(), Value::try_from(rev_id).unwrap());
    rev.insert("prev".to_string(), Value::try_from(cur_rev_id).unwrap());
    if goldfish_state.get("revisions").is_none() {
        goldfish_state.as_table_mut().unwrap().insert("revisions".to_string(), Value::try_from(Vec::new() as Vec<Value>).unwrap());
    }
    match goldfish_state["revisions"].as_array_mut() {
        Some(vector) => {
            vector.push(Value::try_from(rev).unwrap());
            vector.dedup();
        }
        None => return Some(Error::FailToLoadGoldfish)
    }
    goldfish_state.as_table_mut().unwrap().insert("HEAD".to_string(), Value::try_from(rev_id).unwrap());
    write_state(goldfish_state)
}

/**
 * Delete a file to track list, use FileSystem to update tracking list file
 *
 * @param path: path to file to delete
 * @return: Some(Error) if failed, None otherwise
 */
pub fn delete_track_file(path: &str) -> Option<Error> {
    todo!()
}

/**
 * Get current revision, Use Filesystem to read HEAD file
 *
 * @return: Err(Error) if failed, Ok(Revision) otherwise
 */
pub fn get_current_revision() -> Result<String, Error> {
    let mut goldfish_state: Value;
    match read_state() {
        Ok(state) => goldfish_state = state,
        Err(_e) => return Err(Error::FailToLoadGoldfish)
    }
    Ok(goldfish_state.get("HEAD")
        .or(Some(&Value::try_from("").unwrap()))
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
    )
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
pub fn create_virtual_file_from_path(path: String) -> Result<Blob, Error> {
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
pub fn create_virtual_file_from_revision_path(path: String, rev_id: String) -> Result<Blob, Error> {
    todo!()
}

/**
 * Get list of revision, use FileSystem to read revision list file
 *
 * @return: Err(Error) if failed, Ok(Vec<Revision>) otherwise
 */
pub fn get_list_of_revisions() -> Result<Vec<String>, Error> {
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

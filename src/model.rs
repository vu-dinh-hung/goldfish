use toml::Value;
use crate::filesystem;
use crate::utilities;
use std::collections::HashMap;
use std::io;

// root
pub const DVCS_ROOT_DIR: &str = ".goldfish";

// top-level directories
pub const BLOBS_DIR: &str = "blobs";
pub const STAGING_DIR: &str = "staging";
pub const COMMITS_DIR: &str = "commits";
pub const BRANCHES_DIR: &str = "branches";

// top-level files
pub const HEAD: &str = "HEAD";

// misc
pub const DIGEST_SIZE: usize = 256;

// Loc's old stuff
const STATE: &str = ".goldfish/state.toml";
pub const STAGING: &str = ".goldfish/staging/";

fn resolve_reference(reference: &str) -> Option<String> {
    //! If given a branch name, resolve that branch name to the associated commit id
    //! else if given a commit id, return that commit id
    Some(reference.to_string())
}

#[derive(Debug)]
pub struct Repository {
    working_path: String,
    repo_path: String,
}

impl Repository {
    pub fn find(path: &str) -> Option<Repository> {
        //! Find the root working path and .dvcs path of the current DVCS repository,
        //! and return a Repository object with those paths
        if !filesystem::is_dir(path) {
            return None
        }

        let current_repo_path = filesystem::join_path(vec![path, DVCS_ROOT_DIR]);
        if filesystem::is_dir(current_repo_path.as_str()) {
            return Some(Repository { working_path: path.to_owned(), repo_path: current_repo_path })
        }

        let parent = filesystem::parent(path)?;  // return None if there is no parent path
        Repository::find(parent.as_str())  // recursively find in parent path
    }

    pub fn get_current_commit_id(&self) -> io::Result<String> {
        //! Return the current commit id or an empty string if this is a fresh repository
        let head_content = filesystem::read_file(filesystem::join_path(vec![&self.repo_path, HEAD]).as_str())?;
        Ok(resolve_reference(head_content.trim()).unwrap())
    }

    pub fn get_repo_path<'a>(&'a self) -> &'a str {
        &self.repo_path
    }

    pub fn get_working_path<'a>(&'a self) -> &'a str {
        &self.working_path
    }

    pub fn get_head_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, HEAD])
    }

    pub fn get_commits_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, COMMITS_DIR])
    }

    pub fn get_staging_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, STAGING_DIR])
    }

    pub fn get_blobs_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, BLOBS_DIR])
    }
}

/**
 * This is an interface for interacting with commit files. Commit files' filename is
 * the hash digest of their file content, and they have the following format:
 * ```
 * commit\n
 * parent {direct_parent_id}\n
 * {{ zero or more lines of `parent {parent_id}\n' for any other (merged) parents` }}
 * file {file_path} {blob_id}\n
 * {{ more file lines if necessary }}
 * ```
 */
#[derive(Debug)]
pub struct Commit {
    id: String,
    direct_parent_id: String,
    secondary_parent_ids: Vec<String>,
    path: String,
}

impl Commit {
    pub fn create(repo: &Repository, direct_parent_id: String, secondary_parent_ids: Vec<String>, file_list: Vec<(String, String)>) -> io::Result<Commit> {
        // TODO: assert non-empty file_list; a commit cannot have no files

        let mut content = format!("commit\nparent {}\n", direct_parent_id);

        // add secondary parents
        for parent in secondary_parent_ids.iter() {
            content = format!("{}parent {}\n", content, parent);
        }

        // add file list
        for (file_path, blob_id) in file_list.iter() {
            content = format!("{}file {} {}\n", content, file_path, blob_id);
        }
        let commit_id = utilities::hash(content.as_str());
        let commit_path = filesystem::join_path(vec![repo.get_commits_path().as_str(), commit_id.as_str()]);

        // write commit file
        filesystem::write_file(content.as_str(), commit_path.as_str())?;

        // update HEAD file
        filesystem::write_file(commit_id.as_str(), repo.get_head_path().as_str())?;

        Ok(Commit {
            id: commit_id,
            direct_parent_id: direct_parent_id,
            secondary_parent_ids: secondary_parent_ids,
            path: commit_path
        })
    }

    pub fn get(repo: &Repository, id: &str) -> Option<Commit> {
        //! Find the commit file at the given path
        //! and return the Commit object loaded from that commit file
        let full_path = filesystem::join_path(vec![repo.get_commits_path().as_str(), id]);
        let content = filesystem::read_file(full_path.as_str()).ok()?;
        let mut lines = content.split('\n');
        if lines.nth(0)?.trim() != "commit" {
            return None
        }
        let mut parent = String::from("");
        let mut secondary_parents = vec![];
        for line in lines {
            if line.starts_with("parent") {
                let current_parent = line.split(" ").nth(1)?.to_string();
                if parent == "" {
                    parent = current_parent;
                } else {
                    secondary_parents.push(current_parent);
                }
            } else {
                break
            }
        }
        Some(Commit { id: id.to_string(), direct_parent_id: parent, secondary_parent_ids: secondary_parents, path: full_path })
    }

    pub fn load_file_list(&self) -> Option<Vec<(String, String)>> {
        //! Load all the file entries in the commit file
        let mut result = vec![];
        let content = filesystem::read_file(self.path.as_str()).ok()?;
        let lines = content.split('\n');
        for line in lines {
            if line.starts_with("file") {
                let file_path = line.split(' ').nth(0)?;
                let blob_id = line.split(' ').nth(1)?;
                result.push((file_path.to_string(), blob_id.to_string()));
            }
        }

        // There should be at least one file present for a commit, otherwise this is a defective commit file
        if result.is_empty() {
            return None
        }

        Some(result)
    }

    pub fn pretty_print(&self) -> String {
        format!("Commit: {}", &self.id)
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_direct_parent_id(&self) -> &str {
        &self.direct_parent_id
    }

    pub fn get_secondary_parent_ids(&self) -> &Vec<String> {
        &self.secondary_parent_ids
    }
}

/**
 * A blob is a file whose name is the hash of its MAIN CONTENT
 * Format:
 * ```
 * blob\n
 * {{ MAIN CONTENT }}
 * ```
 */
pub struct Blob {
    id: String,
    path: String,
}

impl Blob {
    pub fn create(repo: &Repository, blob_data: &str) -> io::Result<Blob> {
        //! Write a new blob file with the given content
        let content = format!("blob\n{}", blob_data);
        let blob_id = utilities::hash(blob_data);
        let blob_path = filesystem::join_path(vec![repo.get_blobs_path().as_str(), blob_id.as_str()]);
        filesystem::write_file(content.as_str(), blob_path.as_str())?;
        Ok(Blob { id: blob_id, path: blob_path })
    }

    pub fn get(repo: &Repository, id: &str) -> Option<Blob> {
        //! Find a blob at the given path
        let full_path = filesystem::join_path(vec![repo.get_blobs_path().as_str(), id]);
        let content = filesystem::read_file(full_path.as_str()).ok()?;
        let mut lines = content.split("\n");
        if lines.nth(0)?.trim() != "blob" {
            return None
        }

        Some(Blob { id: id.to_string(), path: full_path })
    }

    pub fn get_blob_content(&self) -> io::Result<String> {
        //! Read the main content of the blob
        let content = filesystem::read_file(&self.path.as_str())?;
        let lines = content.split("\n");
        Ok(lines.skip(1).map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

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

// pub fn add_revision(rev_id: &String) -> Option<Error> {
//     let mut goldfish_state: Value;
//     match read_state() {
//         Ok(state) => goldfish_state = state,
//         Err(_e) => return Some(Error::FailToLoadGoldfish)
//     }
//     // add tracked file
//     let cur_rev_id: String;
//     match get_current_revision() {
//         Ok(rev_id) => cur_rev_id = rev_id,
//         Err(e) => return Some(e),
//     }
//     let mut rev: HashMap<String, Value> = HashMap::new();
//     rev.insert("id".to_string(), Value::try_from(rev_id).unwrap());
//     rev.insert("prev".to_string(), Value::try_from(cur_rev_id).unwrap());
//     if goldfish_state.get("revisions").is_none() {
//         goldfish_state.as_table_mut().unwrap().insert("revisions".to_string(), Value::try_from(Vec::new() as Vec<Value>).unwrap());
//     }
//     match goldfish_state["revisions"].as_array_mut() {
//         Some(vector) => {
//             vector.push(Value::try_from(rev).unwrap());
//             vector.dedup();
//         }
//         None => return Some(Error::FailToLoadGoldfish)
//     }
//     goldfish_state.as_table_mut().unwrap().insert("HEAD".to_string(), Value::try_from(rev_id).unwrap());
//     write_state(goldfish_state)
// }

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

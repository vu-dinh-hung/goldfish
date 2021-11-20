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

    pub fn get_commits_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, COMMITS_DIR])
    }

    pub fn get_staging_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, STAGING_DIR])
    }

    pub fn clean_staging(&self) {
        filesystem::remove(&self.get_staging_path()).unwrap();
    }

    pub fn get_blobs_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, BLOBS_DIR])
    }

    pub fn get_head_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, HEAD])
    }

    pub fn read_HEAD(&self) -> Result<String, String> {
        match filesystem::read_file(&self.get_head_path()) {
            Ok(goldfish_HEAD) => Ok(goldfish_HEAD),
            Err(_e) => Err(String::from("Fail to load HEAD file"))
        }
    }

    pub fn write_HEAD(&self, goldfish_HEAD: String) -> Option<String> {
        match filesystem::write_file(goldfish_HEAD.as_str(), &self.get_head_path()) {
            Ok(_v) => return None,
            Err(_e) => return Some(String::from("Fail to save HEAD")),
        }
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
        repo.write_HEAD(String::from(&commit_id));

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
                let file_path = line.split(' ').nth(1)?;
                let blob_id = line.split(' ').nth(2)?;
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
#[derive(Debug)]
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

/* Internal functions */
fn diff_blobs(vf1: &Blob, vf2: &Blob) -> Option<Blob> {
    todo!()
}

fn merge_blobs(vf1: &Blob, vf2: &Blob) -> Blob {
    todo!()
}

fn get_list_of_track_files() -> Result<Vec<String>, String> {
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

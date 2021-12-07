use crate::filesystem;
use crate::utilities;
use std::collections::{HashMap, HashSet};
use std::io;

// root
pub const GOLDFISH_ROOT_DIR: &str = ".goldfish";

// top-level directories
pub const BLOBS_DIR: &str = "blobs";
pub const STAGING_DIR: &str = "staging";
pub const COMMITS_DIR: &str = "commits";
pub const BRANCHES_DIR: &str = "branches";

// top-level files
pub const HEAD: &str = "HEAD";
pub const TRACKEDFILES: &str = "tracked_files";


fn resolve_reference(reference: &str) -> Option<String> {
    //! If given a branch name, resolve that branch name to the associated commit id
    //! else if given a commit id, return that commit id
    Some(reference.to_string())
}

#[derive(Debug)]
pub struct Repository {
    // Working Dir path
    working_path: String,
    // .Goldfish path
    repo_path: String,
}

impl Repository {
    pub fn find(path: &str) -> Option<Repository> {
        //! Find the root working path and .dvcs path of the current DVCS repository,
        //! and return a Repository object with those paths
        if !filesystem::is_dir(path) {
            return None
        }

        let current_repo_path = filesystem::join_path(vec![path, GOLDFISH_ROOT_DIR]);
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

    pub fn get_blobs_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, BLOBS_DIR])
    }
}

// Interacting with HEAD
impl Repository {
    fn get_head_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, HEAD])
    }

    pub fn read_head(&self) -> Result<String, String> {
        match filesystem::read_file(&self.get_head_path()) {
            Ok(goldfish_head) => Ok(goldfish_head),
            Err(_e) => Err(String::from("Fail to load HEAD file"))
        }
    }

    pub fn write_head(&self, goldfish_head: String) -> Option<String> {
        match filesystem::write_file(goldfish_head.as_str(), &self.get_head_path()) {
            Ok(_v) => return None,
            Err(_e) => return Some(String::from("Fail to save HEAD")),
        }
    }
}

// Interacting with list of tracked files
impl Repository {
    fn get_track_files_path(&self) -> String {
        filesystem::join_path(vec![&self.repo_path, TRACKEDFILES])
    }

    pub fn get_staging_tracked_files(&self) -> Result<HashMap<String, String>, String> {
        match filesystem::read_file(&self.get_track_files_path()) {
            Ok(_raw_tracked_files) => {
                // parse tracked files
                let raw_tracked_files = filesystem::read_file(self.get_track_files_path().as_str()).unwrap_or(String::from(""));
                let mut tracked_file: HashMap<String, String> = HashMap::new();
                for line in raw_tracked_files.split_terminator('\n') {
                    let items: Vec<&str> = line.split(" ").collect();
                    tracked_file.insert(String::from(items[0]), String::from(items[1]));
                }
                Ok(tracked_file)
            },
            Err(_e) => Err(String::from("Fail to get staging list of tracked files")),
        }
    }

    pub fn get_file_content_hash(&self, rel_file_path_to_wd: &str) -> Option<String> {
        match self.get_staging_tracked_files() {
            Ok(map) => {
                if map.contains_key(rel_file_path_to_wd) {
                    Some(map[rel_file_path_to_wd].clone())
                } else {
                    None
                }
            },
            Err(_e) => None,
        }
    }

    pub fn save_staging_tracked_files(&self, tracked_file: HashMap<String, String>) -> Option<String> {
        let mut raw_new_tracked_files = String::new();
        for (k, v) in tracked_file {
            raw_new_tracked_files.push_str(format!("{} {}\n", k, v).as_str());
        }
        match filesystem::write_file(raw_new_tracked_files.as_str(), &self.get_track_files_path()) {
            Ok(_x) => None,
            Err(_e) => Some(String::from("Fail to save staging list of tracked file")),
        }
    }

    pub fn track_file(&self, abs_file_path: &str) -> Option<String> {
        // parse tracked files
        let mut tracked_file;
        match self.get_staging_tracked_files() {
            Ok(files) => tracked_file = files,
            Err(e) => return Some(e),
        }
        // track files
        let file_content = filesystem::read_file(abs_file_path).unwrap();
        let file_content_hash = utilities::hash(file_content.as_str());
        let rel_file_path_to_wd = filesystem::get_relative_path_from_base(self.get_working_path(), abs_file_path);
        tracked_file.insert(rel_file_path_to_wd, file_content_hash);
        // write back state
        self.save_staging_tracked_files(tracked_file)
    }

    pub fn untrack_file(&self, abs_file_path: &str) -> Option<String> {
        // parse tracked files
        let mut tracked_file;
        match self.get_staging_tracked_files() {
            Ok(files) => tracked_file = files,
            Err(e) => return Some(e),
        }
        // untrack files
        let rel_file_path_to_wd = filesystem::get_relative_path_from_base(self.get_working_path(), abs_file_path);
        tracked_file.remove(rel_file_path_to_wd.as_str());
        // write back state
        self.save_staging_tracked_files(tracked_file)
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
pub struct Commit<'a> {
    id: String,
    direct_parent_id: String,
    secondary_parent_ids: Vec<String>,
    repo: &'a Repository,
}




impl<'a> Commit<'a> {
    pub fn create(repo: &Repository, direct_parent_id: String, secondary_parent_ids: Vec<String>, tracked_files: HashMap<String, String>) -> io::Result<Commit> {
        // TODO: assert non-empty file_list; a commit cannot have no files

        let mut content = format!("commit\nparent {}\n", direct_parent_id);

        // add secondary parents
        for parent in secondary_parent_ids.iter() {
            content = format!("{}parent {}\n", content, parent);
        }

        // add tracked file list
        for (file_path, hash) in tracked_files.iter() {
            content = format!("{}tracked_file {} {}\n", content, file_path, hash);
        }

        let commit_id = utilities::hash(content.as_str());
        let commit_path = filesystem::join_path(vec![repo.get_commits_path().as_str(), commit_id.as_str()]);

        // write commit file
        filesystem::write_file(content.as_str(), commit_path.as_str())?;

        // update HEAD file
        repo.write_head(String::from(&commit_id));

        Ok(Commit {
            id: commit_id,
            direct_parent_id: direct_parent_id,
            secondary_parent_ids: secondary_parent_ids,
            repo: repo
        })
    }

    pub fn get(repo: &'a Repository, id: &str) -> Option<Commit<'a>> {
        //! Find the commit file with the given commit id
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
        Some(Commit { id: id.to_string(), direct_parent_id: parent, secondary_parent_ids: secondary_parents, repo: repo })
    }

    pub fn checkout(&self) -> Result<String, String> {
        let repo = self.get_repo();
        // load all the files of that commit
        match self.load_tracked_files() {
            Some(tracked_file_list) => {
                // populate the staging area with the files of the commit
                for (file_path, blob_id) in &tracked_file_list {
                    match Blob::get(&repo, blob_id.as_str()) {
                        Some(blob) => {
                            match filesystem::write_file(
                                blob.get_blob_content().unwrap().as_str(),
                                filesystem::join_path(vec![
                                    repo.get_staging_path().as_str(),
                                    file_path.as_str(),
                                ])
                                .as_str(),
                            ) {
                                Ok(_) => {}
                                Err(_) => return Err(
                                    String::from("Something went wrong when writing files. Please try again")
                                ),
                            }
                        }
                        None => return Err(
                            String::from("Something went wrong creating the committed files"),
                        ),
                    }
                }
                // populate staging tracked files
                repo.save_staging_tracked_files(tracked_file_list);
                // copy the staging area to the working path
                for file_path in
                    filesystem::list_files(repo.get_staging_path().as_str(), true, &vec![]).unwrap()
                {
                    let dest = filesystem::join_path(vec![
                        repo.get_working_path(),
                        filesystem::diff_path(repo.get_staging_path().as_str(), file_path.as_str())
                            .unwrap()
                            .as_str(),
                    ]);
                    match filesystem::write_file(
                        filesystem::read_file(file_path.as_str()).unwrap().as_str(),
                        dest.as_str(),
                    ) {
                        Ok(_) => {}
                        Err(_) => return Err(String::from("Something failed while writing to working area"))
                    }
                }
                // clean staging
                filesystem::remove(repo.get_staging_path().as_str()).unwrap();
            }
            None => return Err(String::from("Corrupt commit file")),
        }
        Ok(String::from(""))
    }

    pub fn load_tracked_files(&self) -> Option<HashMap<String, String>> {
        let mut result = HashMap::new();
        let commit_file_path = filesystem::join_path(vec![self.get_repo().get_commits_path().as_str(), self.get_id().as_str()]);
        let content = filesystem::read_file(commit_file_path.as_str()).ok()?;
        let lines = content.split('\n');
        for line in lines {
            if line.starts_with("tracked_file") {
                let file_path = line.split(' ').nth(1)?;
                let hash = line.split(' ').nth(2)?;
                result.insert(file_path.to_string(), hash.to_string());
            }
        }

        // There should be at least one file present for a commit, otherwise this is a defective commit file
        if result.is_empty() {
            return None
        }

        Some(result)
    }

    pub fn get_lowest_common_parent_with<'b>(&'b self, other: &'b Commit) -> Option<Commit<'b>> {
        let mut self_ancestors = HashSet::new();
        self_ancestors.insert(self.get_id());

        // populate the parents set with all the ancestors of the self Commit
        fn populate_ancestors(ancestors: &mut HashSet<String>, commit: &Commit) {
            let parent_results = commit.get_parents();
            for parent_result in parent_results {
                match parent_result {
                    Some(parent) => {
                        ancestors.insert(parent.get_id());
                        populate_ancestors(ancestors, &parent);
                    }
                    None => {}
                }
            }
        }
        populate_ancestors(&mut self_ancestors, self);

        if self_ancestors.contains(&other.get_id()) {
            return Commit::get(other.get_repo(), other.get_id().as_str())
        }

        // walk up the ancestor tree of the other Commit,
        // returning when an ancestor that exists in self's ancestor set is found

        fn find_match(ancestors: &mut HashSet<String>, commit: &Commit) -> Option<String> {
            let parent_results = commit.get_parents();
            for parent_result in parent_results {
                match parent_result {
                    Some(parent) => {
                        if ancestors.contains(&parent.get_id()) {
                            return Some(parent.get_id())
                        } else {
                            match find_match(ancestors, &parent) {
                                Some(id) => return Some(id),
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            }
            None
        }

        Commit::get(self.repo, find_match(&mut self_ancestors, &other)?.as_str())
    }





    pub fn pretty_print(&self) -> String {
        format!("Commit: {}", &self.id)
    }

    pub fn get_repo(&self) -> &Repository {
        &self.repo
    }

    pub fn get_id(&self) -> String {
        (&self).id.to_owned()
    }

    pub fn get_direct_parent_id(&self) -> &str {
        &self.direct_parent_id
    }

    pub fn get_direct_parent(&self) -> Option<Commit> {
        Commit::get(&self.repo, &self.direct_parent_id)
    }

    pub fn get_secondary_parents(&self) -> Vec<Option<Commit>> {
        let mut result = vec![];
        for parent_id in &self.secondary_parent_ids {
            result.push(Commit::get(&self.repo, parent_id))
        }
        result
    }

    pub fn get_parents(&self) -> Vec<Option<Commit>> {
        let mut parents = self.get_secondary_parents();
        parents.insert(0, self.get_direct_parent());
        parents
    }
}

#[derive(Debug)]
pub struct ChangeBin {
    tag: String,
    line_list: Vec<(String,String)>,
}

impl ChangeBin {
    pub fn create(t: String, l: Vec<(String,String)>) -> ChangeBin {
        ChangeBin{
            tag: t,
            line_list: l,
        }
    }

    pub fn get_tag(&self) -> &str{
        self.tag.as_str()
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
}

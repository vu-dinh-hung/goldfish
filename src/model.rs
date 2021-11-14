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

pub enum Error {}

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
pub fn add_track_file(path: String) -> Option<Error> {
    todo!()
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

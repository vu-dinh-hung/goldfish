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
fn diffVirtualFiles(vf1: &VirtualFile, vf2: &VirtualFile) -> VirtualFile;
fn mergeVirtualFiles(vf1: &VirtualFile, vf2: &VirtualFile) -> VirtualFile;
fn getListOfTrackFiles() -> Result<Vec<String>, Error>;

/* External methods */

/**
 * Add a file to track list, use FileSystem to update tracking list file
 * 
 * @param path: path to file to add
 * @return: Some(Error) if failed, None otherwise
 */
pub fn AddTrackFile(path: String) -> Option<Error>;

/**
 * Delete a file to track list, use FileSystem to update tracking list file
 * 
 * @param path: path to file to delete
 * @return: Some(Error) if failed, None otherwise
 */
pub fn DeleteTrackFile(path: String) -> Option<Error>;

/**
 * Get current revision, Use Filesystem to read HEAD file
 * 
 * @return: Err(Error) if failed, Ok(Revision) otherwise
 */
pub fn GetCurrentRevision() -> Result<Revision, Error>;

/**
 * Get current branch, Use Filesystem to read HEAD file
 * 
 * @return: Err(Error) if failed, Ok(String) otherwise
 */
pub fn GetCurrentBranch() -> Result<String, Error>;

/**
 * Read file and create a VirtualFile from it, use FileSystem to read the file
 * 
 * @param path: path to file
 * @return: Err(Error) if failed, Ok(VirtualFile) otherwise
 */
pub fn CreateVirtualFileFromPath(path: String)->Result<VirtualFile, Error>;

/**
 * Read file from a revision and create a VirtualFile from it, use FileSystem to read the file
 * 
 * @param path: path to file
 * @param rev: revision of the file
 * @return: Err(Error) if failed, Ok(VirtualFile) otherwise
 */
pub fn CreateVirtualFileFromRevisionPath(path: String, rev: Revision) -> Result<VirtualFile, Error>;

/**
 * Get list of revision, use FileSystem to read revision list file
 * 
 * @return: Err(Error) if failed, Ok(Vec<Revision>) otherwise
 */
pub fn GetListOfRevisions() -> Result<Vec<Revision>, Error>;
//! # Controller
use crate::model::{Repository};
use crate::filesystem;


pub fn init() {
    //! Create a new .dvcs folder inside the current directory (if it doesn't already exist)
    let current_working_directory = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    if Repository::find(current_working_directory.as_str()).is_some() {
        panic!("Already a DVCS repository");
    }
    match filesystem::create_directory(filesystem::join(vec![current_working_directory.as_str(), ".dvcs"]).as_str()) {
        Ok(_) => {
            assert!(Repository::find(current_working_directory.as_str()).is_some());
            let repo = Repository::find(current_working_directory.as_str()).unwrap();
            filesystem::write_file("", filesystem::join(vec![repo.get_dvcs_path(), "state.toml"]).as_str())
                .expect("Something went wrong creating the `state.toml` file");
            filesystem::create_directory(filesystem::join(vec![repo.get_dvcs_path(), "staging"]).as_str())
                .expect("Something went wrong creating the `staging` directory");
            filesystem::create_directory(filesystem::join(vec![repo.get_dvcs_path(), "objects"]).as_str())
                .expect("Something went wrong creating the `objects` directory");
            filesystem::create_directory(filesystem::join(vec![repo.get_dvcs_path(), "commits"]).as_str())
                .expect("Something went wrong creating the `commits` directory");
        },
        Err(_) => panic!("Something went wrong creating the .dvcs folder")
    }
}

pub fn clone(url: String) {
    //! Create a folder with the repo name, download the .dvcs folder from the specified url,
    //! and load the full directory into the folder
    todo!()
}

pub fn add(names: Vec<String>) {
    //! Add the specified file names to the "staging area", which is just a file in
    //! the .dvcs folder
    todo!()
}

pub fn remove(names: Vec<String>) {
    //! Remove the specified file names from the "staging area"
    todo!()
}

pub fn commit() {
    //! Create a new snapshot (full copy) of the current states of the current directory,
    //! excluding changed files that are not in the staging area
    todo!()
}

pub fn status() {
    //! Print the current changed files and staged files to the output display
    todo!()
}

pub fn heads() {
    //! Print out the current HEAD and the branch name of that HEAD, taken from the .dvcs folder
    todo!()
}

pub fn diff(commit1: String, commit2: String) {
    //! Takes in two commit hashes and use the `display` module to print out the changes
    //! between the two files
    todo!()
}

pub fn cat(commit: String, file: String) {
    //! Reads a file in the given commit (revision)
    todo!()
}

pub fn log() {
    //! Use the `display` module
    todo!()
}

pub fn checkout(commit: String) {
    //! Edit the commit (branch) name in the HEAD file, and load the full directory of the
    //! commit
    todo!()
}

pub fn merge(commit: String) {
    //! Merge the given commit with the current commit. Only works if the current directory
    //! does not have uncommited changes.
    todo!()
}

pub fn push() {
    //! Use the ServerContent interface in `networking` to make a push request to a different
    //! dvcs server
    todo!()
}

pub fn pull() {
    //! Use the ServerContent interface in `networking` to make a pull request to a different
    //! dvcs server
    todo!()
}

pub fn set_remote() {
    //! Sets the remote links for pull and push
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1_init() {
        //! Check and make sure the correct initial files and folders are created
        todo!();
    }

    #[test]
    fn test_2_clone() {
        //! Check and make sure the repository from the url is correctly copied over
        todo!();
    }

    #[test]
    fn test_3_add() {
        //! Check that the .dvcs folder records the staged files correctly
        todo!();
    }

    #[test]
    fn test_4_remove() {
        //! Check that the .dvcs folder records the files removed from staging correctly
        todo!();
    }

    #[test]
    fn test_5_commit() {
        //! Check that the .dvcs folder records a snapshot correctly
        todo!();
    }

    #[test]
    fn test_6_status() {
        //! Check that the right output is displayed to the shell
        todo!();
    }

    #[test]
    fn test_7_heads() {
        //! Check that the right output is displayed to the shell, given the current data inside the HEAD file
        todo!();
    }

    #[test]
    fn test_8_diff() {
        //! Check that the right output is displayed to the shell, given two test commits
        todo!();
    }

    #[test]
    fn test_9_cat() {
        //! Check that the right output is displayed to the shell, given a test file and test commit
        todo!();
    }

    #[test]
    fn test_10_log() {
        //! Check that the right output is displayed to the shell after a series of test commits
        todo!();
    }

    #[test]
    fn test_11_checkout() {
        //! Check that the state of the repo directory is correct, given a test commit
        todo!();
    }

    #[test]
    fn test_12_merge() {
        //! Check that the state of the repo directory is correct, given a test commit
        todo!();
    }

    #[test]
    fn test_13_push() {
        //! Check that the right request is sent to a mock server
        todo!();
    }

    #[test]
    fn test_14_pull() {
        //! Check that the right request is sent to a mock server, and that the directory
        //! is updated correctly
        todo!()
    }

    #[test]
    fn test_15_errors() {
        //! Check that the functions return/output the right errors if called incorrectly
        todo!();
    }
}

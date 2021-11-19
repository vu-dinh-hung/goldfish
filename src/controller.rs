//! # Controller
use crate::model;
use crate::model::{Repository};
use crate::utilities;
use crate::filesystem::*;
use crate::display::{print_output, print_error};
use std::path::PathBuf;


pub fn init() {
    //! Create a new .dvcs folder inside the current directory (if it doesn't already exist)
    let current_working_directory = pathbuf_to_string(std::env::current_dir().unwrap());
    if Repository::find(current_working_directory.as_str()).is_some() {
        print_error("Already a DVCS folder");
        return
    }
    match create_dir(join_path(vec![current_working_directory.as_str(), model::DVCS_ROOT_DIR]).as_str()) {
        Ok(_) => {
            assert!(Repository::find(current_working_directory.as_str()).is_some());
            let repo = Repository::find(current_working_directory.as_str()).unwrap();
            for file in [model::HEAD] {
                write_file("", join_path(vec![repo.get_dvcs_path(), file]).as_str())
                    .expect(format!("Something went wrong creating the `{}` file", file).as_str());
            }
            for folder in [model::BLOBS_DIR, model::BRANCHES_DIR, model::COMMITS_DIR, model::STAGING_DIR] {
                create_dir(join_path(vec![repo.get_dvcs_path(), folder]).as_str())
                    .expect(format!("Something went wrong creating the `{}` directory", folder).as_str());
            }
        },
        Err(_) => {
            print_error("Something went wrong creating the .dvcs folder");
            return
        }
    }
    print_output("Successfully initialized new repository")
}

pub fn clone(url: &str) {
    //! Create a folder with the repo name, download the .dvcs folder from the specified url,
    //! and load the full directory into the folder
    todo!()
}

pub fn commit() -> Option<model::Error> {
    let rev_id = utilities::generate_id();
    match model::add_revision(&rev_id) {
        Some(e) => return Some(e),
        None => (),
    }
    let mut rev_folder = PathBuf::from(model::DVCS_ROOT_DIR);
    rev_folder.push(rev_id);
    copy_dir(model::STAGING, rev_folder).unwrap();
    return None;
}

pub fn status() {
    //! Print the current changed files and staged files to the output display
    todo!()
}

pub fn heads() {
    //! Print out the current HEAD and the branch name of that HEAD, taken from the .dvcs folder
    todo!()
}

pub fn diff(commit1: &str, commit2: &str) {
    //! Takes in two commit hashes and use the `display` module to print out the changes
    //! between the two files
    todo!()
}

pub fn cat(commit: &str, file: &str) {
    //! Reads a file in the given commit (revision)
    todo!()
}

pub fn log() {
    //! Use the `display` module
    todo!()
}

pub fn checkout(commit: &str) {
    //! Edit the commit (branch) name in the HEAD file, and load the full directory of the
    //! commit
    todo!()
}

pub fn merge(commit: &str) {
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

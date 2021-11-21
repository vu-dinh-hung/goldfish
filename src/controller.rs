//! # Controller
use std::path::Path;
use crate::model;
use crate::model::{Repository, Blob, Commit};
use crate::utilities;
use crate::filesystem::*;
use crate::display::{print_output, print_error};
use std::path::PathBuf;


pub fn init() {
    //! Create a new .dvcs folder inside the current directory (if it doesn't already exist)
    let current_directory = pathbuf_to_string(std::env::current_dir().unwrap());
    match Repository::find(current_directory.as_str()) {
        None => {
            match create_dir(join_path(vec![current_directory.as_str(), model::DVCS_ROOT_DIR]).as_str()) {
                Ok(_) => {
                    assert!(Repository::find(current_directory.as_str()).is_some());
                    let repo = Repository::find(current_directory.as_str()).unwrap();
                    for file in [model::HEAD, model::TRACKEDFILES] {
                        write_file("", join_path(vec![repo.get_repo_path(), file]).as_str())
                            .expect(format!("Something went wrong creating the `{}` file", file).as_str());
                    }
                    for folder in [model::BLOBS_DIR, model::BRANCHES_DIR, model::COMMITS_DIR, model::STAGING_DIR] {
                        create_dir(join_path(vec![repo.get_repo_path(), folder]).as_str())
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
        Some(_) => print_error("Already a DVCS folder")
    }
}

pub fn clone(url: &str) {
    //! Create a folder with the repo name, download the .dvcs folder from the specified url,
    //! and load the full directory into the folder
    todo!()
}

pub fn commit() {
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            // list files in staging area
            match list_files(repo.get_staging_path().as_str(), true, &vec![]) {
                Ok(files) => {
                    let mut file_list = vec![];
                    // create blobs
                    for file_path in files {
                        let file_content = read_file(file_path.as_str()).expect("This file path should be valid");
                        match Blob::create(&repo, file_content.as_str()) {
                            Ok(blob) => {
                                file_list.push((diff_path(repo.get_staging_path().as_str(), file_path.as_str()).unwrap(), blob.get_id().to_string()))
                            },
                            Err(err) => {
                                print_error(format!("Something went wrong creating blob objects for the commit:\n{}", err).as_str());
                                return
                            }
                        }
                    }
                    // create commit
                    match repo.get_current_commit_id() {
                        Ok(current_commit_id) => {
                            let tracked_files;
                            match repo.get_staging_tracked_files() {
                                Ok(files) => tracked_files = files,
                                Err(e) => {
                                    print_error(e.as_str());
                                    return;
                                }
                            }
                            match Commit::create(&repo, current_commit_id, vec![], file_list, tracked_files) {
                                Ok(commit) => {
                                    print_output(format!("Created commit: {}", commit.get_id()).as_str());
                                    repo.clean_staging();
                                }
                                Err(err) => print_error(format!("Something went wrong writing the commit file:\n{}", err).as_str())
                            }
                        }
                        Err(err) => {
                            print_error(format!("Something went wrong reading the current commit id:\n{}", err).as_str());
                            return
                        }
                    }
                }
                Err(_) => {
                    print_error("No file found in staging area. Please `add` files before committing");
                    return
                }
            }
        }
        None => print_error("Not a DVCS folder")
    }
}

pub fn status() {
    //! Print the current changed files and staged files to the output display
    todo!()
}

pub fn heads() {
    //! Print out the current HEAD and the branch name of that HEAD, taken from the .dvcs folder
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            match repo.read_HEAD() {
                Ok(head) => print_output(format!("At commit {}", head).as_str()),
                Err(e) => print_error(e.as_str()),
            }
        }
        None => print_error("Not a Goldfish folder")
    }
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
    //! Print the ancestors of the current commit
    fn print_ancestor(repo: &Repository, commit_id: &str) {
        //! Print the current commit, then recursively print the ancestor of this commit
        if commit_id == "" {
            return
        }

        match Commit::get(repo, commit_id) {
            Some(commit) => {
                print_output(format!("---\n{}", commit.pretty_print()).as_str());
                print_ancestor(repo, commit.get_direct_parent_id());
            }
            None => print_error(format!("Invalid commit id: {}", commit_id).as_str())
        }
    }

    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            match repo.get_current_commit_id() {
                Ok(head_commit_id) => {
                    print_output("History:");
                    print_ancestor(&repo, head_commit_id.as_str());
                }
                Err(err) => print_error(format!("Something went wrong reading the current commit id:\n{}", err).as_str())
            }
        }
        None => print_error("Not a DVCS folder")
    }
}

pub fn checkout(commit_id: &str) {
    //! Edit the commit (branch) name in the HEAD file, and load the full directory of the commit
    // TODO: catch all errors
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            // get the Commit associated with the given commit_id
            match Commit::get(&repo, commit_id) {
                Some(commit) => {
                    // load all the files of that commit
                    match commit.load_file_list() {
                        Some(file_list) => {
                            // remove old files
                            match list(repo.get_working_path()) {
                                Ok(entry_list) => {
                                    for entry in entry_list {
                                        if canonicalize(entry.as_str()).unwrap() == canonicalize(repo.get_repo_path()).unwrap() {
                                            continue
                                        }
                                        remove(entry.as_str());
                                    }
                                }
                                Err(err) => {
                                    print_error(format!("Something went wrong deleting the current files:\n{}", err).as_str());
                                    return
                                }
                            }
                            // populate the staging area with the files of the commit
                            for (file_path, blob_id) in file_list {
                                match Blob::get(&repo, blob_id.as_str()) {
                                    Some(blob) => {
                                        write_file(
                                            blob.get_blob_content().unwrap().as_str(),
                                            join_path(vec![repo.get_staging_path().as_str(), file_path.as_str()]).as_str()
                                        );
                                    }
                                    None => print_error("Something went wrong creating the committed files")
                                }
                            }
                            // copy the staging area to the working path
                            for file_path in list_files(repo.get_staging_path().as_str(), true, &vec![]).unwrap() {
                                let dest = join_path(vec![repo.get_working_path(), diff_path(repo.get_staging_path().as_str() ,file_path.as_str()).unwrap().as_str()]);
                                write_file(read_file(file_path.as_str()).unwrap().as_str(), dest.as_str()).expect("Something failed while writing to working area");
                            }
                        }
                        None => print_error("Corrupt commit file")
                    }
                }
                None => print_error("Invalid commit_id")
            }
        }
        None => print_error("Not a DVCS folder")
    }
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

pub fn add_track_file(path: &str) {
    // sanity check
    if !is_file(path) && !is_dir(path) {
        print_error(format!("{} did not match any file or folder", path).as_str());
        return;
    }
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            let abs_path = pathbuf_to_string(get_absolute_path(path));
            let rel_path_to_wd = get_relative_path_to_wd(repo.get_working_path(), path);
            match copy(
                abs_path.as_str(), 
                Path::new(repo.get_staging_path().as_str())
                    .join(&rel_path_to_wd.as_str())
                    .to_str()
                    .unwrap()) {

                Ok(_v) => (),
                Err(_e) => {
                    print_error(format!("Fail to add {}", path).as_str());
                    return;
                },
            }
            match repo.trackFile(rel_path_to_wd.as_str()) {
                Some(e) => print_error(e.as_str()),
                None => (),
            }
        },
        None => print_error("Not a Goldfish folder")
    }
}

pub fn delete_track_file(path: &str) {
    // sanity check
    if !is_file(path) && !is_dir(path) {
        print_error(format!("{} did not match any file or folder", path).as_str());
        return;
    }
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            let abs_path = pathbuf_to_string(get_absolute_path(path));
            let rel_path_to_wd = get_relative_path_to_wd(repo.get_working_path(), path);
            match remove(Path::new(repo.get_staging_path().as_str())
                            .join(&rel_path_to_wd.as_str())
                            .to_str()
                            .unwrap()) {
                Ok(_v) => (),
                Err(_e) => {
                    print_error(format!("Fail to remove {}", path).as_str());
                    return;
                },
            }
            match repo.untrackFile(rel_path_to_wd.as_str()) {
                Some(e) => print_error(e.as_str()),
                None => (),
            }
        },
        None => print_error("Not a Goldfish folder")
    }
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

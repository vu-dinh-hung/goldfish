//! # Controller
use crate::display::{print_error, print_output, print_output_string, print_output_vec_string, print_error_string};
use crate::filesystem::*;
use crate::model;
use crate::model::{Blob, Commit, Repository};
use crate::utilities;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

pub fn init() {
    // Create a new .dvcs folder inside the current directory (if it doesn't already exist)
    let current_directory = pathbuf_to_string(std::env::current_dir().unwrap());
    match Repository::find(current_directory.as_str()) {
        None => {
            match create_dir(
                join_path(vec![current_directory.as_str(), model::GOLDFISH_ROOT_DIR]).as_str(),
            ) {
                Ok(_) => {
                    assert!(Repository::find(current_directory.as_str()).is_some());
                    let repo = Repository::find(current_directory.as_str()).unwrap();
                    for file in [model::HEAD, model::TRACKEDFILES] {
                        match write_file("", join_path(vec![repo.get_repo_path(), file]).as_str()) {
                            Ok(_) => {}
                            Err(_) => return print_error_string(format!("Something went wrong creating the `{}` file", file)),
                        }
                    }
                    for folder in [
                        model::BLOBS_DIR,
                        model::BRANCHES_DIR,
                        model::COMMITS_DIR,
                        model::STAGING_DIR,
                    ] {
                        match create_dir(join_path(vec![repo.get_repo_path(), folder]).as_str()) {
                            Ok(_) => {}
                            Err(_) => print_error_string(format!("Something went wrong creating the `{}` directory", folder)),
                        }
                    }
                }
                Err(_) => return print_error("Something went wrong creating the .goldfish folder"),
            }
            print_output("Successfully initialized new repository")
        }
        Some(_) => return print_error("Already a Goldfish folder"),
    }
}

pub fn clone(url: &str) {
    //! Create a folder with the repo name, download the .dvcs folder from the specified url,
    //! and load the full directory into the folder
    //! Example url: username@host:path/to/.goldfish

    Command::new("rsync")
            .arg("-avz")
            .arg(url)
            .arg("./")
            .output()
            .expect("Failed to clone.");

}



pub fn commit() {
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            // Comparing staging with HEAD to check if there is any change
            let staging_tracked_files;
            match repo.get_staging_tracked_files() {
                Ok(files) => staging_tracked_files = files,
                Err(e) => return print_error(e.as_str()),
            }
            let head_tracked_files;
            match repo.get_current_commit_id() {
                Ok(commit_id) => {
                    if commit_id.is_empty() {
                        head_tracked_files = HashMap::new();
                    } else {
                        match Commit::get(&repo, commit_id.as_str()) {
                            Some(commit) => match commit.load_tracked_files() {
                                Some(files) => head_tracked_files = files,
                                None => return print_error("Fail to load current commitx"),
                            },
                            None => return print_error("Fail to load current commity"),
                        }
                    }
                }
                Err(e) => head_tracked_files = HashMap::new(),
            }
            if utilities::compare_map(&staging_tracked_files, &head_tracked_files) {
                print_output("Nothing to commit")
            }
            // list files in staging area
            match list_files(repo.get_staging_path().as_str(), true, &vec![]) {
                Ok(files) => {
                    let mut file_list = vec![];
                    // create blobs
                    for file_path in files {
                        match read_file(file_path.as_str()) {
                            Ok(file_content) => {
                                match Blob::create(&repo, file_content.as_str()) {
                                    Ok(blob) => file_list.push((
                                        diff_path(repo.get_staging_path().as_str(), file_path.as_str())
                                            .unwrap(),
                                        blob.get_id().to_string(),
                                    )),
                                    Err(err) => return print_error_string(format!("Something went wrong creating blob objects for the commit:\n{}", err)),
                                }
                            }
                            Err(_) => return print_error("This file path should be valid"),
                        }
                    }
                    // clean staging
                    remove(repo.get_staging_path().as_str()).unwrap();
                    // create commit
                    match repo.get_current_commit_id() {
                        Ok(current_commit_id) => {
                            let tracked_files;
                            match repo.get_staging_tracked_files() {
                                Ok(files) => tracked_files = files,
                                Err(e) => return print_error(e.as_str()),
                            }
                            match Commit::create(&repo, current_commit_id, vec![], tracked_files) {
                                Ok(commit) => print_output(
                                    format!("Created commit: {}", commit.get_id()).as_str(),
                                ),
                                Err(err) => print_error(
                                    format!(
                                        "Something went wrong writing the commit file:\n{}",
                                        err
                                    )
                                    .as_str(),
                                ),
                            }
                        }
                        Err(err) => return print_error(
                                        format!(
                                            "Something went wrong reading the current commit id:\n{}",
                                            err
                                        )
                                        .as_str(),
                                    ),
                    }
                }
                Err(_) => return print_error(
                                "No file found in staging area. Please `add` files before committing",
                            ),
            }
        }
        None => return print_error("Not a Goldfish folder"),
    }
}

pub fn status() {
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            let mut change = false;
            // Comparing staging with HEAD
            let staging_tracked_files;
            match repo.get_staging_tracked_files() {
                Ok(files) => staging_tracked_files = files,
                Err(e) => return print_error(e.as_str()),
            }
            let head_tracked_files;
            match repo.get_current_commit_id() {
                Ok(commit_id) => {
                    if commit_id.is_empty() {
                        head_tracked_files = HashMap::new();
                    } else {
                        match Commit::get(&repo, commit_id.as_str()) {
                            Some(commit) => match commit.load_tracked_files() {
                                Some(files) => head_tracked_files = files,
                                None => return print_error("Fail to load current commit"),
                            },
                            None => return print_error("Fail to load current commity"),
                        }
                    }
                }
                Err(e) => head_tracked_files = HashMap::new(),
            }
            if !utilities::compare_map(&staging_tracked_files, &head_tracked_files) {
                change = true;
                print_output("Changes to be commit:");
                for (file_path, _hash) in &staging_tracked_files {
                    if !head_tracked_files.contains_key(file_path) {
                        print_output(format!("\tAdded:   \t{}", file_path).as_str());
                    }
                }
                for (file_path, _hash) in &head_tracked_files {
                    if !staging_tracked_files.contains_key(file_path) {
                        print_output(format!("\tDeleted: \t{}", file_path).as_str());
                    }
                }
                for (file_path, hash) in &staging_tracked_files {
                    if head_tracked_files.contains_key(file_path)
                        && !hash.eq(&head_tracked_files[file_path])
                    {
                        print_output(format!("\tModified:\t{}", file_path).as_str());
                    }
                }
            }
            // Comparing current WD with staging
            let mut wd_files = HashMap::new();
            for file_path in
                list_files(repo.get_working_path(), true, &vec![repo.get_repo_path()]).unwrap()
            {
                let hash;
                match read_file(&file_path) {
                    Ok(content) => hash = utilities::hash(content.as_str()),
                    Err(_e) => hash = "".to_string(),
                }
                wd_files.insert(
                    get_relative_path_from_base(repo.get_working_path(), file_path.as_str()),
                    hash,
                );
            }
            if !utilities::compare_map(&wd_files, &staging_tracked_files) {
                change = true;
                print_output("Changes not staged for commit:");
                for (file_path, _hash) in &wd_files {
                    if !staging_tracked_files.contains_key(file_path) {
                        print_output(format!("\tAdded:   \t{}", file_path).as_str());
                    }
                }
                for (file_path, _hash) in &staging_tracked_files {
                    if !wd_files.contains_key(file_path) {
                        print_output(format!("\tDeleted: \t{}", file_path).as_str());
                    }
                }
                for (file_path, hash) in &wd_files {
                    if staging_tracked_files.contains_key(file_path)
                        && !hash.eq(&staging_tracked_files[file_path])
                    {
                        print_output(format!("\tModified:\t{}", file_path).as_str());
                    }
                }
            }
            if !change {
                print_output("Nothing to commit, working directory clean");
            }
        }
        None => return print_error("Not a Goldfish folder"),
    }
}

pub fn heads() {
    //! Print out the current HEAD and the branch name of that HEAD, taken from the .dvcs folder
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => match repo.read_head() {
            Ok(head) => {
                if head == "" {
                    print_output("Empty repository")
                } else {
                    print_output(format!("At commit {}", head).as_str())
                }
            }
            Err(e) => return print_error(e.as_str()),
        },
        None => return print_error("Not a Goldfish folder"),
    }
}

pub fn diff(commit_id1: &str, commit_id2: &str) {
    fn get_diff_files(tracked_file_list1: &HashMap<String, String>,
                        tracked_file_list2: &HashMap<String, String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for (file_path1, _) in tracked_file_list1 {
            let mut is_file_in: bool = false;
            for (file_path2, _) in tracked_file_list2 {
                if file_path1 == file_path2 {
                    is_file_in = true;
                    break;
                }
            }
            if !is_file_in {
                result.push(file_path1.to_string());
            }
        }
        result
    }
    let mut result: Vec<String> = vec![];
    // Takes in two commit hashes and use the `display` module to print out the changes
    // between the two files
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            // get the Commit associated with the given commit_id
            match Commit::get(&repo, commit_id1) {
                Some(commit1) => {
                    match Commit::get(&repo, commit_id2) {
                        Some(commit2) => {
                            // load all the files of that commit
                            match commit1.load_tracked_files() {
                                Some(tracked_file_list1) => {
                                    match commit2.load_tracked_files() {
                                        Some(tracked_file_list2) => {
                                            let added_files = get_diff_files(&tracked_file_list2, &tracked_file_list1);
                                            let removed_files = get_diff_files(&tracked_file_list1, &tracked_file_list2);
                                            if !added_files.is_empty() {
                                                result.push(format!("Added files from commit {}:", commit_id1));
                                                result.extend(added_files);
                                            }
                                            if !removed_files.is_empty() {
                                                result.push(format!("Removed files from commit {}:", commit_id1));
                                                result.extend(removed_files);
                                            }
                                            for (file_path1, blob_id1) in &tracked_file_list1 {
                                                let mut is_file_in: bool = false;
                                                let mut blob_id2: String = "".to_string();
                                                for (file_path2, temp_blob_id) in &tracked_file_list2 {
                                                    if file_path1 == file_path2 {
                                                        is_file_in = true;
                                                        blob_id2 = temp_blob_id.to_string();
                                                        break;
                                                    }
                                                }
                                                if is_file_in {
                                                    match Blob::get(&repo, blob_id1.as_str()) {
                                                        Some(blob1) => {
                                                            match Blob::get(&repo, blob_id2.as_str()) {
                                                                Some(blob2) => {
                                                                    match blob1.get_blob_content() {
                                                                        Ok(content1) => {
                                                                            match blob2.get_blob_content() {
                                                                                Ok(content2) => {
                                                                                    let mut has_diff: bool = false;
                                                                                    let mut temp_result: Vec<String> = vec![];
                                                                                    let file_vec1: Vec<String> = content1.lines().collect::<Vec<&str>>()
                                                                                                                        .iter().map(|s| s.to_string()).collect();
                                                                                    let file_vec2: Vec<String> = content2.lines().collect::<Vec<&str>>()
                                                                                                                        .iter().map(|s| s.to_string()).collect();
                                                                                    let diff_content = utilities::diff(file_vec1, file_vec2);
                                                                                    for (_, content) in diff_content.iter().enumerate() {
                                                                                        if content.0 == "+" || content.0 == "-" {
                                                                                            has_diff = true;
                                                                                            temp_result.push(format!("{} {}", content.0, content.1));
                                                                                        } else {
                                                                                            temp_result.push(format!("{}", content.1));
                                                                                        }
                                                                                    }
                                                                                    if has_diff {
                                                                                        result.push(format!("Differences in file {}:", file_path1));
                                                                                        result.extend(temp_result);
                                                                                    }
                                                                                }
                                                                                Err(_) => return print_error("File not found"),
                                                                            }
                                                                        }
                                                                        Err(_) => return print_error("File not found"),
                                                                    }
                                                                }
                                                                None => return print_error(
                                                                    "Something went wrong reading file",
                                                                ),
                                                            }
                                                        }
                                                        None => return print_error(
                                                            "Something went wrong reading file",
                                                        ),
                                                    }
                                                }
                                            }
                                        }
                                        None => return print_error("Corrupted second commit file"),
                                    }
                                }
                                None => return print_error("Corrupted first commit file"),
                            }
                        }
                        None => return print_error("Invalid second commit id"),
                    }
                }
                None => return print_error("Invalid first commit id"),
            }
            if result.is_empty() {
                print_output("Two commits are identical");
            } else {
                print_output_vec_string(result);
            }
        }
        None => return print_error("Cannot find the repository"),
    }
}

pub fn cat(commit_id: &str, file: &str) {
    //! Reads a file in the given commit (revision)
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {  // found repo
            match Commit::get(&repo, commit_id) {
                Some(commit) => {  // found commit
                    match commit.load_tracked_files() {
                        Some(files_lookup) => {  // found committed file list
                            for (committed_file, blob_id) in &files_lookup {
                                if compare_paths(committed_file, file) {
                                    match Blob::get(&repo, blob_id) {
                                        Some(blob) => {
                                            match blob.get_blob_content() {
                                                Ok(content) => {
                                                    print_output(format!("File data for {}:\n{}", file, content).as_str());
                                                }
                                                Err(_) => return print_error("Blob object is corrupted")
                                            }
                                        }
                                        None => return print_error("Blob object is corrupted")
                                    }
                                }
                            }
                        }
                        None => return print_error("Commit object is corrupted")
                    }
                }
                None => return print_error(format!("Invalid commit id: {}", commit_id).as_str())
            }
        }
        None => return print_error("Not a Goldfish folder")
    }
}

pub fn log() {
    //! Print the ancestors of the current commit
    fn print_ancestor(repo: &Repository, commit_id: &str) {
        //! Print the current commit, then recursively print the ancestor of this commit
        if commit_id == "" {
            return;
        }

        match Commit::get(repo, commit_id) {
            Some(commit) => {
                print_output(format!("---\n{}", commit.pretty_print()).as_str());
                print_ancestor(repo, commit.get_direct_parent_id());
            }
            None => return print_error(format!("Invalid commit id: {}", commit_id).as_str()),
        }
    }

    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => match repo.get_current_commit_id() {
            Ok(head_commit_id) => {
                print_output("History:");
                print_ancestor(&repo, head_commit_id.as_str());
            }
            Err(err) => return print_error(
                format!(
                    "Something went wrong reading the current commit id:\n{}",
                    err
                )
                .as_str(),
            ),
        },
        None => print_error("Not a Goldfish folder"),
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
                    match commit.load_tracked_files() {
                        Some(tracked_file_list) => {
                            // populate the staging area with the files of the commit
                            for (file_path, blob_id) in &tracked_file_list {
                                match Blob::get(&repo, blob_id.as_str()) {
                                    Some(blob) => {
                                        write_file(
                                            blob.get_blob_content().unwrap().as_str(),
                                            join_path(vec![
                                                repo.get_staging_path().as_str(),
                                                file_path.as_str(),
                                            ])
                                            .as_str(),
                                        );
                                    }
                                    None => return print_error(
                                        "Something went wrong creating the committed files",
                                    ),
                                }
                            }
                            // populate staging tracked files
                            repo.save_staging_tracked_files(tracked_file_list);
                            // copy the staging area to the working path
                            for file_path in
                                list_files(repo.get_staging_path().as_str(), true, &vec![]).unwrap()
                            {
                                let dest = join_path(vec![
                                    repo.get_working_path(),
                                    diff_path(repo.get_staging_path().as_str(), file_path.as_str())
                                        .unwrap()
                                        .as_str(),
                                ]);
                                match write_file(
                                    read_file(file_path.as_str()).unwrap().as_str(),
                                    dest.as_str(),
                                ) {
                                    Ok(_) => {}
                                    Err(_) => return print_error("Something failed while writing to working area")
                                }
                            }
                            // clean staging
                            remove(repo.get_staging_path().as_str()).unwrap();
                        }
                        None => return print_error("Corrupt commit file"),
                    }
                }
                None => return print_error("Invalid commit_id"),
            }
        }
        None => return print_error("Not a Goldfish folder"),
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

fn copy_and_mark_fike_tracked(repo: &Repository, abs_path: &str, rel_path_to_wd: &str) {
    // if file hasn't changed, don't add it
    match repo.get_file_content_hash(rel_path_to_wd) {
        Some(hash) => {
            let file_content = read_file(abs_path).unwrap();
            let file_content_hash = utilities::hash(file_content.as_str());
            if file_content_hash == hash {
                return;
            }
        }
        None => (),
    }
    // copy file to staging
    match copy(
        abs_path,
        Path::new(repo.get_staging_path().as_str())
            .join(&rel_path_to_wd)
            .to_str()
            .unwrap(),
    ) {
        Ok(_v) => (),
        Err(_e) => return print_error(format!("Fail to add {}", abs_path).as_str()),
    }
    // add/update file in tracked list
    match repo.track_file(rel_path_to_wd) {
        Some(e) => return print_error(e.as_str()),
        None => (),
    }
}

pub fn add_track_file(path: &str) {
    // sanity check
    if !is_file(path) && !is_dir(path) {
        return print_error(format!("{} did not match any file or folder", path).as_str());
    }
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            let abs_path = pathbuf_to_string(get_absolute_path(path));
            let rel_path_to_wd =
                get_relative_path_from_base(repo.get_working_path(), abs_path.as_str());
            if !is_dir(path) {
                copy_and_mark_fike_tracked(&repo, abs_path.as_str(), rel_path_to_wd.as_str());
            } else {
                match list_files(abs_path.as_str(), true, &vec![repo.get_repo_path()]) {
                    Ok(files) => {
                        for file_path in files {
                            let abs_path = pathbuf_to_string(get_absolute_path(file_path.as_str()));
                            let rel_path_to_wd =
                                get_relative_path_from_base(repo.get_working_path(), abs_path.as_str());
                            copy_and_mark_fike_tracked(
                                &repo,
                                abs_path.as_str(),
                                rel_path_to_wd.as_str(),
                            );
                        }
                    }
                    Err(_e) => (),
                }
            }
        }
        None => print_error("Not a Goldfish folder"),
    }
}

pub fn delete_track_file(path: &str) {
    // sanity check
    if !is_file(path) && !is_dir(path) {
        return print_error(format!("{} did not match any file or folder", path).as_str());
    }
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            let abs_path = pathbuf_to_string(get_absolute_path(path));
            let rel_path_to_wd =
                get_relative_path_from_base(repo.get_working_path(), abs_path.as_str());
            match remove(
                Path::new(repo.get_staging_path().as_str())
                    .join(&rel_path_to_wd.as_str())
                    .to_str()
                    .unwrap(),
            ) {
                Ok(_v) => (),
                Err(_e) => (),
            }
            if !is_dir(path) {
                match repo.untrack_file(rel_path_to_wd.as_str()) {
                    Some(e) => return print_error(e.as_str()),
                    None => (),
                }
            } else {
                match list_files(abs_path.as_str(), true, &vec![repo.get_repo_path()]) {
                    Ok(files) => {
                        for file_path in files {
                            let abs_path = pathbuf_to_string(get_absolute_path(file_path.as_str()));
                            let rel_path_to_wd =
                                get_relative_path_from_base(repo.get_working_path(), abs_path.as_str());
                            match repo.untrack_file(rel_path_to_wd.as_str()) {
                                Some(e) => print_error(e.as_str()),
                                None => (),
                            }
                        }
                    }
                    Err(_e) => (),
                }
            }
        }
        None => return print_error("Not a Goldfish folder"),
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
    fn test_2_parse_url_clone() {
        //! Check and make sure the repository from the url is correctly copied over
        let url: &str = "username@host:path/to/.goldfish";
        let mut chunks: Vec<&str> = url.split(&['@','/',':'][..]).collect();
        let repo_name: &str = chunks.pop().unwrap();
        assert_eq!(repo_name, ".goldfish");
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

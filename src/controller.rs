//! # Controller
use crate::display::{print_error, print_output, print_output_string, print_output_vec_string, print_error_string};
use crate::filesystem::*;
use crate::filesystem;
use crate::model;
use crate::networking;
use crate::model::{Blob, Commit, Repository, Change_bin};
use crate::utilities;
use std::collections::HashMap;
use std::path::Path;

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

pub fn clone(url: &str, folder_name: &str) {
    //! Create a folder with the repo name, download the .dvcs folder from the specified url,
    //! and load the full directory into the folder
    //! Example url: username@host:path/to/.goldfish
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(_) => return print_error("Cannot clone, already a repository"),
        None => {
            // create the base folder for the new local repository
            let working_path = join_path(vec![".", folder_name]);
            if is_dir(working_path.as_str()) {
                return print_output_string(format!("The directory {} already exists", working_path))
            }
            if let Err(_) = create_dir(working_path.as_str()) {
                print_output_string(format!("Something went wrong creating the {} folder for the repository", working_path));
            }

            // download the .goldfish folder (repository data) from the given url
            let download_succeeded = networking::rsync(url, working_path.as_str());
            if download_succeeded {
                print_output("--> Finished downloading repository data; now populating working tree");
                match Repository::find(working_path.as_str()) {
                    Some(repo) => {
                        match repo.read_head() {
                            Ok(head_id) => {
                                match Commit::get(&repo, head_id.as_str()) {
                                    Some(commit) => {
                                        match commit.checkout() {
                                            Ok(_) => return print_output("Successfully cloned repository"),
                                            Err(err) => {
                                                print_error("Error checking out the HEAD commit:");
                                                print_error_string(err)
                                            }
                                        }
                                    }
                                    None => print_output("Something went wrong loading the HEAD commit")
                                }
                            }
                            Err(_) => print_output("Something went wrong leading the HEAD commit")
                        }
                    }
                    None => print_output("Something went wrong creating the repository")
                }
            } else {
                print_error(format!("Cannot fetch repository data from the given url: {}", url).as_str());
            }

            // cleanup
            if let Err(_) = remove(working_path.as_str()) {
                print_error_string(format!("Something went wrong cleaning up the {} repository folder", working_path));
            }
        }
    }
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
                            None => return print_error("Fail to load current commit"),
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

fn check_status() -> Option<bool> {
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            let mut change = false;
            // Comparing staging with HEAD
            let staging_tracked_files;
            match repo.get_staging_tracked_files() {
                Ok(files) => staging_tracked_files = files,
                Err(e) => {
                    print_error(e.as_str());
                    return None;
                }
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
                                None => {
                                    print_error("Fail to load current commit");
                                    return None;
                                }
                            },
                            None => {
                                print_error("Fail to load current commit");
                                return None;
                            }
                        }
                    }
                }
                Err(e) => head_tracked_files = HashMap::new(),
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
            return Some(!utilities::compare_map(&staging_tracked_files, &head_tracked_files)
                    || !utilities::compare_map(&wd_files, &staging_tracked_files));
        }
        None => {
            print_error("Not a Goldfish folder");
            return None;
        }
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
    fn print_ancestor(repo: &Repository, commit_id: &str) -> bool {
        //! Print the current commit, then recursively print the ancestor of this commit
        //! result
        if commit_id == "" {
            return false;
        }

        match Commit::get(repo, commit_id) {
            Some(commit) => {
                print_output(format!("---\n{}", commit.pretty_print()).as_str());
                print_ancestor(repo, commit.get_direct_parent_id());
            }
            None => {
                print_error(format!("Invalid commit id: {}", commit_id).as_str());
            },
        }
        true
    }

    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => match repo.get_current_commit_id() {
            Ok(head_commit_id) => {
                print_output("History:");
                if !print_ancestor(&repo, head_commit_id.as_str()) {
                    print_output("Empty, no commit found")
                };
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
            let mut id = commit_id.to_owned();
            if commit_id == "HEAD" {
                match repo.read_head() {
                    Ok(head_commit_id) => {
                        id = head_commit_id;
                    }
                    Err(_) => return print_error("Could not find the HEAD commit")
                }
            }
            // get the Commit associated with the given commit_id
            match Commit::get(&repo, id.as_str()) {
                Some(commit) => {
                    match commit.checkout() {
                        Ok(_) => {}
                        Err(err) => return print_error(err.as_str())
                    }
                }
                None => return print_error("Invalid commit_id"),
            }
            print_output_string(format!("Checked out commit {}", id))
        }
        None => return print_error("Not a Goldfish folder"),
    }
}




//returns a Hashmap mapping filename to Change_bin
pub fn commit_diff<'b>(a: &'b Commit, b: &'b Commit, repo: &Repository) -> Option<HashMap<String, Change_bin>> {
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


    let mut result : HashMap<String, Change_bin> = HashMap::new();
    // Takes in two commit hashes and use the `display` module to print out the changes
    // between the two files

    // load all the files of the commits
    match a.load_tracked_files() {
        Some(tracked_file_list1) => {
            match b.load_tracked_files() {
                Some(tracked_file_list2) => {
                    let added_files = get_diff_files(&tracked_file_list2, &tracked_file_list1);
                    let removed_files = get_diff_files(&tracked_file_list1, &tracked_file_list2);


                    for file in added_files {
                        result.insert(file, Change_bin::create(
                            String::from("+"),
                            vec![],
                        ));
                    }


                    for file in removed_files{
                        result.insert(file, Change_bin::create(
                            String::from("-"),
                            vec![],
                        ));
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
                                                            let file_vec1: Vec<String> = content1.lines().collect::<Vec<&str>>()
                                                                                            .iter().map(|s| s.to_string()).collect();
                                                            let file_vec2: Vec<String> = content2.lines().collect::<Vec<&str>>()
                                                                                            .iter().map(|s| s.to_string()).collect();
                                                            let diff_content = utilities::diff(file_vec1, file_vec2);
                                                            result.insert(file_path1.to_string(), Change_bin::create(
                                                                String::from("="),
                                                                diff_content,
                                                            ));
                                                        }
                                                        Err(_) => (),
                                                    }
                                                }
                                                Err(_) => (),
                                            }
                                        }
                                        None => (),
                                    }

                                }
                                None => (),
                            }
                        }
                    }
                }
                None => (),
            }
        }
        None => (),
    }








    Some(result)
}

// Helper functions for merge
fn get_blob_content_as_vec(repo: &Repository, blob_id: &str) -> Vec<String> {
    match Blob::get(&repo, blob_id) {
        Some(blob) => {
            match blob.get_blob_content() {
                Ok(content) => {
                    return content.lines()
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|s| s.to_string())
                                .collect();
                },
                Err(_) => ()
            }
        },
        None => ()
    }
    return vec!();
}

fn add_line(result: &mut String, line: &String) {
    if result.len() != 0 {
        result.push_str("\n");
    }
    result.push_str(line.as_str());
}

fn create_conflict(
    result: &mut String,
    blob1_content: &Vec<String>,
    blob2_content: &Vec<String>,
    blob1_id: &str,
    blob2_id: &str
) {
    if blob1_content.len() == 0 && blob2_content.len() == 0 {
        return;
    }
    if blob1_content.len() == 0 {
        for line in blob2_content {
            add_line(result, line);
        }
    }
    if blob2_content.len() == 0 {
        for line in blob1_content {
            add_line(result, line);
        }
    }
    // conflict!
    add_line(result, &format!("<<<<<<<<<< {}", blob1_id));
    for line in blob1_content {
        add_line(result, line);
    }
    add_line(result, &format!("===================="));
    for line in blob2_content {
        add_line(result, line);
    }
    add_line(result, &format!(">>>>>>>>>> {}", blob2_id));
}

fn merge_files(repo: &Repository, blob1_id: &str, blob2_id: &str, rev1_id: &str, rev2_id: &str) -> String {
    let blob1 = get_blob_content_as_vec(repo, blob1_id);
    let blob2 = get_blob_content_as_vec(repo, blob2_id);
    let diff_content = utilities::diff(blob1, blob2);
    let mut blob1_content: Vec<String> = vec!();
    let mut blob2_content: Vec<String> = vec!();
    let mut result: String = String::from("");
    for (typ, content) in diff_content {
        if typ == "-" {
            blob1_content.push(content);
        } else if typ == "+" {
            blob2_content.push(content);
        } else {
            create_conflict(&mut result, &blob1_content, &blob2_content, rev1_id, rev2_id);
            add_line(&mut result, &content);
            blob1_content.clear();
            blob2_content.clear();
        }
    }
    create_conflict(&mut result, &blob1_content, &blob2_content, rev1_id, rev2_id);
    return result;
}
// End of helper functions for merge

pub fn merge(commit: &str) {
    if check_status() == Some(true){
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            match Commit::get(&repo, repo.get_current_commit_id().unwrap().as_str()){
                Some(current) => {
                    match Commit::get(&repo, commit){
                        Some(update) => {

                            if current.get_lowest_common_parent_with(&update).unwrap().get_id() == update.get_id(){
                                update.checkout();
                                return ()
                            }

                            match update.load_tracked_files(){
                                Some(tracked_file_list2) => {
                                    match current.load_tracked_files(){
                                        Some(tracked_file_list1) => {
                                            match commit_diff(&current, &update, &repo){
                                                Some(map) => {
                                                    for(file, diff_list) in map.iter(){
                                                        match diff_list.get_tag() {
                                                            "-" => {

                                                            }
                                                            "+" => {




                                                                for(file_path2, blob_id2) in &tracked_file_list2{
                                                                    if file == file_path2{
                                                                        let content = Blob::get(&repo, blob_id2).unwrap().get_blob_content().unwrap();
                                                                        filesystem::write_file(
                                                                        content.as_str(),
                                                                        filesystem::join_path(
                                                                            vec![repo.get_working_path(), file.as_str()]
                                                                            ).as_str()

                                                                    );
                                                                        return ()
                                                                    }
                                                                }




                                                            }
                                                            "=" => {
                                                                for(file_path1, blob_id1) in &tracked_file_list1{
                                                                    for(file_path2, blob_id2) in &tracked_file_list2{
                                                                        if file == file_path1 && file == file_path2{
                                                                            filesystem::write_file(
                                                                                    merge_files(&repo, blob_id1.as_str(), blob_id2.as_str(), current.get_id().as_str(), update.get_id().as_str()).as_str(),
                                                                                    filesystem::join_path(
                                                                                        vec![repo.get_working_path(), file]
                                                                                        ).as_str()
                                                                                );
                                                                            return ()

                                                                        }
                                                                    }
                                                                }


                                                            }
                                                            _ => return print_error("Problem with file merge tag")

                                                        }
                                                    }
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {}
                            }
                         }
                        None => {}
                    }
                }
                None => {}
            }
        }
        None => {}
    }
    }else{
        return print_error("Can't merge, files in staging or WD")
    }



}






pub fn push(url: &str) {
    match Repository::find(pathbuf_to_string(std::env::current_dir().unwrap()).as_str()) {
        Some(repo) => {
            networking::rsync(repo.get_repo_path(), url);
        }
        None => print_error("Not a repository")
    }

}

pub fn pull(url: &str) {
    // Use `networking` to make a pull request to a different
    // dvcs server
    if check_status().is_none() {
        return print_error("Cannot pull. Working directory isn't clean or existed.");
    }
    let og_dir: String = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    if try_clone(url).is_some() {
        std::env::set_current_dir(&og_dir);
        remove(".goldfish_temp");
        networking::rsync(url, ".goldfish");
        match Repository::find(".goldfish") {
            Some(repo) => {
                match repo.read_head() {
                    Ok(head_id) => {
                        match Commit::get(&repo, head_id.as_str()) {
                            Some(commit) => {
                                match commit.checkout() {
                                    Ok(_) => return print_output("Pull successfully!"),
                                    Err(err) => {}
                                }
                            }
                            None => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            None => {}
        }
    } else {
        std::env::set_current_dir(&og_dir);
        remove(".goldfish_temp");
        return print_error("Failed to pull the content from specified URL/address");
    }
}

fn try_clone(url: &str) -> Option<bool> {
    create_dir(".goldfish_temp");
    std::env::set_current_dir(format!("{}{}", std::env::current_dir().unwrap().display(), "/.goldfish_temp"));
    let download_succeeded = networking::rsync(url, ".goldfish");
    if download_succeeded {
        match Repository::find(".goldfish") {
            Some(repo) => {
                match repo.read_head() {
                    Ok(head_id) => {
                        match Commit::get(&repo, head_id.as_str()) {
                            Some(commit) => {
                                match commit.checkout() {
                                    Ok(_) => return Some(true),
                                    Err(err) => return None,
                                }
                            }
                            None => return None
                        }
                    }
                    Err(_) => return None
                }
            }
            None => None
        }
    } else {
        return None;
    }
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

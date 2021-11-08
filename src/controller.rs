//! # Controller

pub fn init() {
    //! Create a new .dvcs folder inside the current directory (if it doesn't already exist)
    todo!()
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

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        unimplemented!();
    }

    #[test]
    fn test2() {
        unimplemented!();
    }

    #[test]
    fn test3() {
        unimplemented!();
    }

    #[test]
    fn test4() {
        unimplemented!();
    }
}

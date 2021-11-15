
use crate::controller;
use crate::display;
use crate::model;
/*
    Print welcome message, start receiving input from user and call other functions to process command
    Call PrintError() in display module if we have invalid command name

    @return: ! (never return)
*/


pub fn initialize() {


    match std::env::args().nth(1){
        Some(x) =>
            match x.as_str(){
            "init" => process_init("init".to_string()),
            "heads" => process_heads("heads".to_string()),
            "cat" => process_cat("cat".to_string()),
            "add" => process_add("add".to_string()),
            _ => display::print_error("unexpected command"),
            },
        None => display::print_error("Command not given"),
    }

}


/*
    Verify help command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_help(command: String) {}



/*
    Verify init command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/

pub fn process_init(command: String) {
    controller::init();
}


/*
    Verify clone command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_clone(command: String) {}

/*
    Verify add command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_add(command: String) {
    model::add_track_file(command.as_str());
}

/*
    Verify remove command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_remove(command: String) {}

/*
    Verify status command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_status(command: String) {}

/*
    Verify heads command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_heads(command: String) {
    controller::heads();
}

/*
    Verify diff command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_diff(command: String) {}

/*
    Verify cat command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_cat(command: String) {
    let commit = std::env::args().nth(2);
    let file = std::env::args().nth(3);
    if commit.is_some() & file.is_some(){
         controller::cat(commit.unwrap(), file.unwrap());
    }else{
        display::print_error("Commit or filename not given");
    }

}

/*
    Verify checkout command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_checkout(command: String) {}

/*
    Verify commit command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_commit(command: String) {}

/*
    Verify log command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_log(command: String) {}

/*
    Verify merge command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_merge(command: String) {}

/*
    Verify pull command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_pull(command: String) {}

/*
    Verify push command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn process_push(command: String) {}

#[cfg(test)]
mod tests {
    /*
        @input: user typed "init" command (valid)
        @expect: method Initialize calls method ProcessInit
    */
    #[test]
    fn test_initialize_given_valid_command() {
        todo!();
    }

    /*
        @input: user typed "hello" command (invalid)
        @expect: method Initialize calls PrintError in display module
    */
    #[test]
    fn test_initialize_given_invalid_command() {
        todo!();
    }

    /*
        @input: "help" command (valid)
        @expect: method ProcessHelp prints information about available commands
    */
    #[test]
    fn test_process_help_given_valid_command() {
        todo!();
    }

    /*
        @input: "help me" command (invalid)
        @expect: method ProcessHelp calls PrintError in display module
    */
    #[test]
    fn test_process_help_given_invalid_command() {
        todo!();
    }

    /*
        @input: "init" command (valid)
        @expect: method ProcessInit calls method in repository module
                to create an empty repository
    */
    #[test]
    fn test_process_init_given_valid_command() {
        todo!();
    }

    /*
        @input: "init pls" command (invalid)
        @expect: method ProcessInit calls PrintError in display module
    */
    #[test]
    fn test_process_init_given_invalid_command() {
        todo!();
    }

    /*
        @input: "clone [valid url]" command (valid)
        @expect: method ProcessClone calls method in repository module
                to create an empty repository
    */
    #[test]
    fn test_process_clone_given_valid_command() {
        todo!();
    }

    /*
        @input: "clone" command (invalid)
        @expect: method ProcessClone calls PrintError in display module
    */
    #[test]
    fn test_process_clone_given_invalid_command() {
        todo!();
    }

    /*
        @input: "add [valid file]" command (valid)
        @expect: method ProcessAdd calls method in repository module
                to add specific files that user want to track
    */
    #[test]
    fn test_process_add_given_valid_command() {
        todo!();
    }

    /*
        @input: "add @#$RA" command (invalid)
        @expect: method ProcessAdd calls PrintError in display module
    */
    #[test]
    fn test_process_add_given_invalid_command() {
        todo!();
    }

    /*
        @input: "remove [valid file]" command (valid)
        @expect: method ProcessRemove calls method in repository module
                to remove specific files from tracking list
    */
    #[test]
    fn test_process_remove_given_valid_command() {
        todo!();
    }

    /*
        @input:  "remove @#$abc^xyz" command (invalid)
        @expect: method ProcessRemove calls PrintError in display module
    */
    #[test]
    fn test_process_remove_given_invalid_command() {
        todo!();
    }

    /*
        @input: "status" command (valid)
        @expect: method ProcessStatus calls method in repository module
                to check the current status of current repository
    */
    #[test]
    fn test_process_status_given_valid_command() {
        todo!();
    }

    /*
        @input: "status code.java" command (invalid)
        @expect: method ProcessStatus calls PrintError in display module
    */
    #[test]
    fn test_process_status_given_invalid_command() {
        todo!();
    }

    /*
        @input: "heads" command (valid)
        @expect: method ProcessHeads calls method in repository module
                to show the current heads
    */
    #[test]
    fn test_process_heads_given_valid_command() {
        todo!();
    }

    /*
        @input: "heads goldfish" command (invalid)
        @expect: method ProcessHeads calls PrintError in display module
    */
    #[test]
    fn test_process_heads_given_invalid_command() {
        todo!();
    }

    /*
        @input: "diff [rev1] [rev2]" command (valid)
        @expect: method ProcessDiff calls method in repository module
                to check the changes between revisions
    */
    #[test]
    fn test_process_diff_given_valid_command() {
        todo!();
    }

    /*
        @input: "diff diff" command (invalid)
        @expect: method ProcessDiff calls PrintError in display module
    */
    #[test]
    fn test_process_diff_given_invalid_command() {
        todo!();
    }

    /*
        @input: "cat [file]" command (valid)
        @expect: method ProcessCat calls method in repository module
                to inspect a file of a given revision
    */
    #[test]
    fn test_process_cat_given_valid_command() {
        todo!();
    }

    /*
        @input: "cat meow meow" command (invalid)
        @expect: method ProcessCat calls PrintError in display module
    */
    #[test]
    fn test_process_cat_given_invalid_command() {
        todo!();
    }

    /*
        @input: "checkout [rev]" command (valid)
        @expect: method ProcessCheckout calls method in repository module
                to check out a specific revision
    */
    #[test]
    fn test_process_checkout_given_valid_command() {
        todo!();
    }

    /*
        @input: "checkout with cash" command (invalid)
        @expect: method ProcessCheckout calls PrintError in display module
    */
    #[test]
    fn test_process_checkout_given_invalid_command() {
        todo!();
    }

    /*
        @input: "commit" command (valid)
        @expect: method ProcessCommit calls method in repository module
                to commit changes and create a new revision
    */
    #[test]
    fn test_process_commit_given_valid_command() {
        todo!();
    }

    /*
        @input: "commit goldfish" command (invalid)
        @expect: method ProcessCommit calls PrintError in display module
    */
    #[test]
    fn test_process_commit_given_invalid_command() {
        todo!();
    }

    /*
        @input: "log" command (valid)
        @expect: method ProcessLog calls method in repository module
                to view the change log
    */
    #[test]
    fn test_process_log_given_valid_command() {
        todo!();
    }

    /*
        @input: "log goldfish" command (invalid)
        @expect: method ProcessLog calls PrintError in display module
    */
    #[test]
    fn test_process_log_given_invalid_command() {
        todo!();
    }

    /*
        @input: "merge [rev1] [rev2]" command (valid)
        @expect: method ProcessMerge calls method in repository module
                to merge two revisions
    */
    #[test]
    fn test_process_merge_given_valid_command() {
        todo!();
    }

    /*
        @input: "merge sort" command (invalid)
        @expect: method ProcessMerge calls PrintError in display module
    */
    #[test]
    fn test_process_merge_given_invalid_command() {
        todo!();
    }

    /*
        @input: "pull [repo]" command (valid)
        @expect: method ProcessPull calls method in repository module
                to pull the changes from another repository
    */
    #[test]
    fn test_process_pull_given_valid_command() {
        todo!();
    }

    /*
        @input: "pull and bear" command (invalid)
        @expect: method ProcessPull calls PrintError in display module
    */
    #[test]
    fn test_process_pull_given_invalid_command() {
        todo!();
    }

    /*
        @input: "pull [repo]" command (valid)
        @expect: method ProcessPush calls method in repository module
                to push changes into another repository
    */
    #[test]
    fn test_process_push_given_valid_command() {
        todo!();
    }

    /*
        @input: "pull and bear" command (invalid)
        @expect: method ProcessPush calls PrintError in display module
    */
    #[test]
    fn test_process_push_given_invalid_command() {
        todo!();
    }
}

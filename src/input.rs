
use crate::controller;
use crate::display;
use crate::model;
use std::io::{self, Write};
use regex::Regex;
use std::process;
use std::env;

/*
    Initialize input module
    Call print_error() in display module if we have invalid command name
*/
pub fn initialize() {
    let env_args = env::args().collect::<Vec<String>>();
    if env_args.len() == 1 {
        // initialize dvcs terminal to continuously read user input
        display::print_welcome();

        loop {
            print!("dvcs> ");
            io::stdout().flush();
            // read input
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            // process input by removing new line character at the end and spliting to list of args
            let re_space = Regex::new(r"[ ]+").unwrap();
            let args = re_space.split(&input[0..input.len() - 1]).collect::<Vec<&str>>();
            // match input with command
            process_command(args);
        }
    } else {
        // read input directly from user initial command
        let mut args = env_args.iter().map(|s| s as &str).collect::<Vec<&str>>();
        // remove executable file arg
        args.remove(0);
        process_command(args);
    }
}

/*
    Verify and process user commands
    Call print_error() in display module if we have invalid command

    @param args: list of arguments from user input
*/
pub fn process_command(args: Vec<&str>) {
    if !args.is_empty() {
        match args[0] {
            "" => return,
            "quit" => process_quit(args),
            "help" => process_help(args),
            "init" => process_init(args),
            "clone" => process_clone(args),
            "add" => process_add(args),
            "remove" => process_remove(args),
            "status" => process_status(args),
            "heads" => process_heads(args),
            "diff" => process_diff(args),
            "cat" => process_cat(args),
            "checkout" => process_checkout(args),
            "commit" => process_commit(args),
            "log" => process_log(args),
            "merge" => process_merge(args),
            "pull" => process_pull(args),
            "push" => process_push(args),
            _ => display::print_error("Invalid command. Please type help to see our supported commands")
        }
    }
}

/*
    Verify and process quit command
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_quit(args: Vec<&str>) {
    if args.len() == 1 {
        process::exit(0);
    } else {
        display::print_error_string(format!("Invalid number of arguments for quit. Expect 0 but got {}", args.len() - 1));
    }
}


/*
    Verify and process help command
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_help(args: Vec<&str>) {
    if args.len() == 1 {
        display::print_help();
    } else {
        display::print_error_string(format!("Invalid number of arguments for help. Expect 0 but got {}", args.len() - 1));
    }
}



/*
    Verify init command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/

pub fn process_init(args: Vec<&str>) {
    if args.len() == 1 {
        controller::init();
    } else {
        display::print_error_string(format!("Invalid number of arguments for init. Expect 0 but got {}", args.len() - 1));
    }
}


/*
    Verify clone command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_clone(args: Vec<&str>) {
    if args.len() == 2 {
        controller::clone(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for clone. Expect 1 but got {}", args.len() - 1));
    }
}

/*
    Verify add command and process by calling method in model module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_add(args: Vec<&str>) {
    if args.len() == 2 {
        model::add_track_file(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for add. Expect 1 but got {}", args.len() - 1));
    }
}

/*
    Verify remove command and process by calling method in model module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_remove(args: Vec<&str>) {
    if args.len() == 2 {
        model::delete_track_file(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for remove. Expect 1 but got {}", args.len() - 1));
    }
}

/*
    Verify status command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_status(args: Vec<&str>) {
    if args.len() == 1 {
        controller::status();
    } else {
        display::print_error_string(format!("Invalid number of arguments for status. Expect 0 but got {}", args.len() - 1));
    }
}

/*
    Verify heads command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_heads(args: Vec<&str>) {
    if args.len() == 1 {
        controller::heads();
    } else {
        display::print_error_string(format!("Invalid number of arguments for heads. Expect 0 but got {}", args.len() - 1));
    }
}

/*
    Verify diff command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_diff(args: Vec<&str>) {
    if args.len() == 3 {
        controller::diff(args[1], args[2]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for diff. Expect 2 but got {}", args.len() - 1));
    }
}

/*
    Verify cat command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_cat(args: Vec<&str>) {
    if args.len() == 3 {
        controller::cat(args[1], args[2]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for cat. Expect 2 but got {}", args.len() - 1));
    }
}

/*
    Verify checkout command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_checkout(args: Vec<&str>) {
    if args.len() == 2 {
        controller::checkout(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for checkout. Expect 1 but got {}", args.len() - 1));
    }
}

/*
    Verify commit command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_commit(args: Vec<&str>) {
    if args.len() == 1 {
        controller::commit();
    } else {
        display::print_error_string(format!("Invalid number of arguments for commit. Expect 0 but got {}", args.len() - 1));
    }
}

/*
    Verify log command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_log(args: Vec<&str>) {
    if args.len() == 1 {
        controller::log();
    } else {
        display::print_error_string(format!("Invalid number of arguments for log. Expect 0 but got {}", args.len() - 1));
    }
}

/*
    Verify merge command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_merge(args: Vec<&str>) {
    if args.len() == 2 {
        controller::merge(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for merge. Expect 1 but got {}", args.len() - 1));
    }
}

/*
    Verify pull command and process by calling method in repository module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_pull(args: Vec<&str>) {
    if args.len() == 1 {
        controller::pull();
    } else {
        display::print_error_string(format!("Invalid number of arguments for pull. Expect 0 but got {}", args.len() - 1));
    }
}

/*
    Verify push command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
pub fn process_push(args: Vec<&str>) {
    if args.len() == 1 {
        controller::push();
    } else {
        display::print_error_string(format!("Invalid number of arguments for push. Expect 0 but got {}", args.len() - 1));
    }
}

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

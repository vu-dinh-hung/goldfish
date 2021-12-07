
use crate::controller;
use crate::display;
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
            print!("Goldfish> ");
            match io::stdout().flush() {
                Ok(_) => {},
                Err(_) => {},
            }
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
fn process_command(args: Vec<&str>) {
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
fn process_quit(args: Vec<&str>) {
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
fn process_help(args: Vec<&str>) {
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

fn process_init(args: Vec<&str>) {
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
fn process_clone(args: Vec<&str>) {
    if args.len() == 3 {
        controller::clone(args[1], args[2]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for clone. Expect 2 but got {}", args.len() - 1));
    }
}

/*
    Verify add command and process by calling method in model module
    Call print_error() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
fn process_add(args: Vec<&str>) {
    if args.len() == 2 {
        controller::add_track_file(args[1]);
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
fn process_remove(args: Vec<&str>) {
    if args.len() == 2 {
        controller::delete_track_file(args[1]);
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
fn process_status(args: Vec<&str>) {
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
fn process_heads(args: Vec<&str>) {
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
fn process_diff(args: Vec<&str>) {
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
fn process_cat(args: Vec<&str>) {
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
fn process_checkout(args: Vec<&str>) {
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
fn process_commit(args: Vec<&str>) {
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
fn process_log(args: Vec<&str>) {
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
fn process_merge(args: Vec<&str>) {
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
fn process_pull(args: Vec<&str>) {
    if args.len() == 2 {
        controller::pull(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for pull. Expect 1 but got {}", args.len() - 1));
    }
}

/*
    Verify push command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command
    (invalid number of arguments, invalid argument, etc...)

    @param args: list of arguments from user input
*/
fn process_push(args: Vec<&str>) {
    if args.len() == 2 {
        controller::push(args[1]);
    } else {
        display::print_error_string(format!("Invalid number of arguments for push. Expect 1 but got {}", args.len() - 1));
    }
}
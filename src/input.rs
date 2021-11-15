
use crate::controller;
use crate::display;
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
            "head" => process_heads("head".to_string()),
            "cat" => process_cat("cat".to_string()),
            _ => display::print_error("unexpected command".to_string()),
            },
        None => display::print_error("Command not given".to_string()),
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
pub fn process_add(command: String) {}

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
        display::print_error("Commit or filename not given".to_string());
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

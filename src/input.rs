/*
    Print welcome message, start receiving input from user and call other functions to process command
    Call PrintError() in display module if we have invalid command name

    @return: ! (never return)
*/
pub fn initialize() {}

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
pub fn process_init(command: String) {}

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
pub fn process_heads(command: String) {}

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
pub fn process_cat(command: String) {}

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

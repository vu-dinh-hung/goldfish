/*
    Print welcome message, start receiving input from user and call other functions to process command
    Call PrintError() in display module if we have invalid command name
    
    @return: ! (never return)
*/
pub fn Initialize() -> !;

/*
    Verify help command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessHelp(String command) -> !;

/*
    Verify init command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessInit(String command) -> !;

/*
    Verify clone command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessClone(String command) -> !;

/*
    Verify add command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessAdd(String command) -> !;

/*
    Verify remove command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessRemove(String command) -> !;

/*
    Verify status command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessStatus(String command) -> !;

/*
    Verify heads command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessHeads(String command) -> !;

/*
    Verify diff command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessDiff(String command) -> !;

/*
    Verify cat command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessCat(String command) -> !;

/*
    Verify checkout command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessCheckout(String command) -> !;

/*
    Verify commit command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessCommit(String command) -> !;

/*
    Verify log command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessLog(String command) -> !;

/*
    Verify merge command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessMerge(String command) -> !;

/*
    Verify pull command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessPull(String command) -> !;

/*
    Verify push command and process by calling method in repository module
    Call PrintError() in display module if we have invalid command 
    (invalid number of arguments, invalid argument, etc...)

    @param command: command received from users
    @return: ! (never return)
*/
pub fn ProcessPush(String command) -> !;
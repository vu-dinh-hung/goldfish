/*
    Print output message (string literal) to terminal
    @param message: output message sent to user
*/
pub fn print_output(message: &str) {
    println!("{}", message);
}

/*
    Print output message (String struct) to terminal
    @param message: output message sent to user
*/
pub fn print_output_string(message: String) {
    println!("{}", message);
}

/*
    Print output message (vector of String struct) to terminal
    @param message: output message sent to user
*/
pub fn print_output_vec_string(messages: Vec<String>) {
    for message in messages {
        println!("{}", message);
    }
}

/*
    Print error message (string literal) to terminal
    @param message: error message sent to user
*/
pub fn print_error(message: &str) {
    println!("Error: {}", message);
}

/*
    Print error message (string struct) to terminal
    @param message: error message sent to user
*/
pub fn print_error_string(message: String) {
    println!("Error: {}", message);
}

/*
    Print welcome message when first initialize
*/
pub fn print_welcome() {
    println!("-------------------------------------------------------------");
    println!("Welcome to the Version Control System a.k.a the \"Fish Tank\"");
    println!("Our system is here to help you with managing your source code");
    println!("Please type \"help\" to see our supported commands");
    println!("Please type \"quit\" to exit the program");
    println!("Hope you enjoy the experience!");
    println!("-------------------------------------------------------------\n");
}

/*
    Print help message
*/
pub fn print_help() {
    println!("Here's the list of our supported commands:");
    println!("1. init: create an empty repository");
    println!("2. clone [url]: copy an existing repository from an url");
    println!("3. add [file]: add a specified file that you want to track");
    println!("4. remove [file]: remove a specified file from tracking list");
    println!("5. status: check the current status of current repository");
    println!("6. heads: show the current heads");
    println!("7. diff [rev1] [rev2]: check the changes between 2 revisions");
    println!("8. cat [commit] [file]: inspect a file of a given revision");
    println!("9. checkout [rev]: check out a specific revision");
    println!("10. commit: commit changes and create a new revision");
    println!("11. log: view the change log");
    println!("12. merge [rev1] [rev2]: merge two revisions");
    println!("13. pull [url]: pull changes from another repository");
    println!("14. push [url]: push changes into another repository");
    println!("15. quit: quit the program");
}

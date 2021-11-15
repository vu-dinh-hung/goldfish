/*
    Print output message to terminal

    @param message: output message sent to user
    @return: ! (never return)
*/
pub fn print_output(message: &str) {
    println!("Result: {}", message);
}

/*
    Print error message to terminal

    @param message: error message sent to user
    @return: ! (never return)
*/
pub fn print_error(message: &str) {
    println!("Error: {}", message);
}



/*
    Print output message to terminal

    @param message: output message sent to user
    @return: ! (never return)
*/

pub fn print_output(message: &str) {
    println!("{}", message);

}

/*
    Print error message to terminal

    @param message: error message sent to user
    @return: ! (never return)
*/

pub fn print_error(message: &str) {
    println!("Error: {}", message);
}

#[cfg(test)]
mod tests {
    /*
        @input: message "Clone Successfully!"
        @expect: method PrintOutput prints message to terminal
    */
    #[test]
    fn test_print_output_given_message() {
        todo!();
    }

    /*
        @input: message "Invalid number of arguments for command clone"
        @expect: method PrintError prints message to terminal
    */
    #[test]
    fn test_print_error_given_message() {
        todo!();
    }

}

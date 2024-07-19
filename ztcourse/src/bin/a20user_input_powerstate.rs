// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

// * Use an enum to store the possible power states
#[derive(Debug)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

// * Use a function with a match expression to print out the power messages
impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();

        // * state.as_str() is used to convert the string to a string slice (String -> &str)
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_message(state: &PowerState) {
    // * Use a match expression to print out the power messages
    use PowerState::*;
    match state {
        Off => println!("Powering off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    println!("Enter a power state:");
    let mut input = String::new();
    let user_input = io::stdin().read_line(&mut input);
    if user_input.is_ok() {
        match PowerState::new(&input) {
            Some(state) => print_power_message(&state),
            None => println!("Invalid power state"),
        }
    } else {
        println!("Error reading input");
    }
}

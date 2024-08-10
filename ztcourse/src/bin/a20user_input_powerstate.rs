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

#[derive(Debug)]
enum Power {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_uppercase().to_owned())
}

impl Power {
    fn new(state: &str) -> Option<Power> {
        let state = state;
        match state {
            "OFF" => Some(Power::Off),
            "SLEEP" => Some(Power::Sleep),
            "REBOOT" => Some(Power::Reboot),
            "SHUTDOWN" => Some(Power::Shutdown),
            "HIBERNATE" => Some(Power::Hibernate),
            _ => None,
        }
    }
}

fn print_power_action(state: Power) {
    use Power::*;
    match state {
        Off => println!("Turning off"),
        Sleep => println!("Going to sleep"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    println!("Enter a power state:");
    let input = get_user_input().expect("Failed to read line");
    let power_state = Power::new(&input);
    match power_state {
        Some(state) => print_power_action(state),
        None => println!("Invalid power state"),
    }
}

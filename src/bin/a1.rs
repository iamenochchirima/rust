// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

const FIRSY_NAME: &str = "Enoch";
const LAST_NAME: &str = "Chirima";

fn first_name() {
    println!("{FIRSY_NAME:?}");
}

fn last_name() {
    println!("{LAST_NAME:?}");
}

fn full_name() {
    println!("I am {:?} {:?}", FIRSY_NAME, LAST_NAME);
}

fn main() {
    first_name();
    last_name();
    full_name();
}

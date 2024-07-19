// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

const  FIRST: &str = "Enoch";
const LAST: &str = "Chirima";

fn first_name() {
    println!("{FIRST:?}");
}

fn last_name() {
    println!("{LAST:?}");
}

fn full_name() {
    println!("I am {:?} {:?}", FIRST, LAST);
}

fn main() {
    first_name();
    last_name();
    full_name();
}

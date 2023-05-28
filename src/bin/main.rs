// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

let first_name = "Enoch";
let last_name = "Chirima";

fn first_name() {
    println!("{first_name:?}");
}

fn last_name() {
    println!("{last_name:?}");
}

fn fullName() ->  {
    println!("I am {:?} {:?}", first_name, last_name);
}

fn main() {
    first_name();
    last_name();
    fullName();
}

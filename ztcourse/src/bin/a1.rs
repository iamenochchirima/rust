// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name() {
    println!("My first name is: {}", "John");
}

fn last_name() {
    println!("My last name is: {}", "Doe");
}

fn full_name() {
    println!("My full name is: {} {}", "John", "Doe");
}

fn main() {
    first_name();
    last_name();
    full_name();
}

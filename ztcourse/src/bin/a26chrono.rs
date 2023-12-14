// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now();

    println!("Local: {}, utc: {}", local, utc);

    println!("Local: {}" ,local.format("%Y-%m-%d %H:%M:%S").to_string())
}

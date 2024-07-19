// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn say_my_name() {
    let name = "Enoch";
    match name {
        "Enoch" => println!("You're goddanm right"),
        "Pamela" => println!("Say may name!!!"),
        "Ngoni" => println!("It's true"),
        "Tino" => println!("It's false"),
        _ => println!("We don't have business here!")
    }
}

fn main() {
    say_my_name()
}

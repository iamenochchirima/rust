// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn is_greater_that_100(input: bool) {

    match input {
        true => println!("its big"),
        _ => println!("its big")
    }
}

fn main() {
    let number = 101;

    // Will return a boolean value since it's an expression
    let is_gt_100 = number > 100;
    is_greater_that_100(is_gt_100)
}

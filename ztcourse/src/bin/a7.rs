// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Black,
    Blue,
    White,
}

fn what_color(){
    let color = Colors::Black;
    match color {
        Colors::Black => println!("Black"),
        Colors::Blue => println!("Blue"),
        Colors::White => println!("White"),
    }
}

fn main() {
    what_color()
}

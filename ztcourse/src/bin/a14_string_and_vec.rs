// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

enum Color {
    Blue,
    White,
    Black
}

struct Person {
    age: i32,
    name: String,
    fav_color: Color,
}

fn print_name(name: &str) {
    println!("{:?}", name)
}
fn print_color(color: Color) {
    match color {
        Color::Black => println!("Black"),
        Color::Blue => println!("Blue"),
        Color::White => println!("White"),
    }
}

fn main() {
    let people = vec![
        Person {
            age: 12,
            name: String::from("Enoch"),
            fav_color: Color::Black,
        },
        Person {
            age: 6,
            name: String::from("Tawana"),
            fav_color: Color::White,
        },
        Person {
            age: 10,
            name: ("Karl").to_owned(),
            fav_color: Color::Blue,
        },
    ];

    for person in people {
        if person.age <= 10 {
            println!("Age: {:?}", person.age);
            print_name(&person.name);
            print_color(person.fav_color);
        }
    }
}

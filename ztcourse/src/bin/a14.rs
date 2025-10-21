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


struct Person {
    name : String,
    age : u8,
    color : String
}

impl Person {
    fn new(name: String, age: u8, color: String) -> Self {
        Self {
            name,
            age,
            color
        }
    }

    fn print_name_and_color(&self) {
        println!("Name : {:?}, Color : {:?}", self.name, self.color)
    }
 }

fn main() {

    let person1 = Person::new("Tadi".to_owned(), 16, "Green".to_owned());
    let person2 = Person::new("Enoch".to_owned(), 6, "Purple".to_owned());
    let person3 = Person::new("Wande".to_owned(), 4, "Green".to_owned());

    let mut people: Vec<Person>  = Vec::new();
    people.push(person1);
    people.push(person2);
    people.push(person3);

    for person in people {
        if person.age < 10 {
            println!("Age : {:?}", person.age);
            person.print_name_and_color();
        }
    }
}


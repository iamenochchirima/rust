// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Orange,
    Pineapple,
    Guava,
}

struct Drink {
    flavor: Flavors,
    ounces: f64,
}

fn gimme_the_drink(drink: Drink) {
    match drink.flavor {
        Flavors::Guava => println!("Guava"),
        Flavors::Orange => println!("Orange"),
        Flavors::Pineapple => println!("Pineapple"),
    }

    println!("oz: {:?}", drink.ounces)
}

fn main() {
    let guava = Drink {
        flavor: (Flavors::Guava),
        ounces: (0.4),
    };
    gimme_the_drink(guava);
    let orange = Drink {
        flavor: (Flavors::Orange),
        ounces: (1.4),
    };
    gimme_the_drink(orange);
}

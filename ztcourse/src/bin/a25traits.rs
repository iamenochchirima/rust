// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter


trait Perimeter {
    fn calculate_perimeter(&self);
}

struct Square {
    side: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) {
        let perimeter = self.side * 4.0;
        println!("Perimeter of square: {}", perimeter);
    }
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) {
        let perimeter =  self.a + self.b + self.c;
        println!("Perimeter of the traingle is : {}", perimeter)
    }
}

fn calper (args: impl Perimeter) { 
    args.calculate_perimeter()
}

fn main() {
    let square = Square { side: 4.0 };
    let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };

    calper(square);
    calper(triangle);
}

// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Blue,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("brown"),
            Color::White => println!("white"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Length: {:?}", self.length);
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
    }
}
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new_box(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight)
    }
}

fn main() {
    let small_dimensions = Dimensions {
        length: 4.0,
        width: 2.5,
        height: 2.5,
    };
    let small_box = ShippingBox::new_box(3.3, Color::Blue, small_dimensions);
    small_box.print()
}

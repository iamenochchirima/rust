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

#[derive(Debug)]
enum BoxColor {
    Red,
    Yellow,
    Blue,
}

struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}

impl Dimensions {
    fn print(&self) {
        println!("Width : {:?}", self.width);
        println!("Length : {:?}", self.length);
        println!("Height : {:?}", self.height);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    color: BoxColor,
    weight: u32,
}

impl ShippingBox {
    fn new() -> Self {
        let color = BoxColor::Blue;
        let dimensions = Dimensions {
            length: 34,
            width: 23,
            height: 53,
        };
        Self {
            color,
            dimensions,
            weight: 43,
        }
    }

    fn display_charaters(&self) {
        self.dimensions.print();
        println!("Color : {:?}", self.color);
        println!("Weight : {:?}", self.weight)
    }
}

fn main() {
    let box_item = ShippingBox::new();
    box_item.display_charaters();
}

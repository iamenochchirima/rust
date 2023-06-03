// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: i32,
    name: String,
    price: i32,
    quantity: i32,
}

fn item_quantity(item: &Grocery) {
    println!("Quantity: {:?}", item.quantity)
}
fn item_id(item: &Grocery) {
    println!("Identity: {:?}", item.id)
}

fn main() {
    let item = Grocery {
        id: 1,
        quantity: 3,
        name: ("bananas").to_string(),
        price: 2,
    };
    item_quantity(&item);
    item_id(&item);


}

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

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("The quantity is: {:?}", item.quantity)
}

fn display_id(item: &GroceryItem) {
    println!("The id is {:?}", item.id)
}



fn main() {
    let item  = GroceryItem {
        id: 45,
        quantity : 34,
    };  
    display_quantity(&item);
    display_id(&item);

}

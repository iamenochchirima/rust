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

struct Item {
    id : i32,
    quantity : i32
}

fn display_quantity(args : &Item){
    println!("Quantity : {:?}", args.quantity);
}

fn display_id(args : &Item) {
    println!("Id : {:?}", args.id)
}

fn main() {
    let item  = Item {
        id : 2,
        quantity : 3
    };
    display_quantity(&item);
    display_id(&item);
}


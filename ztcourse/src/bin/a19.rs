// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

struct Stock {
    num_items: u32,
    name: String,
}

fn main() {
    let mut stocks: HashMap<u8, Stock> = HashMap::new();
    stocks.insert(0, Stock { num_items: 5, name: "Chairs".to_owned() });
    stocks.insert(1, Stock { num_items: 3, name: "Beds".to_owned() });
    stocks.insert(2, Stock { num_items: 2, name: "Tables".to_owned() });
    stocks.insert(3, Stock { num_items: 0, name: "Couches".to_owned() });

    for stock in stocks.values() {
        match stock.num_items {
            0 => {
                println!("{:?} is out of stock", stock.name);
            }
            _ => println!("{:?} items available of : {:?}", stock.num_items, stock.name)
        }
    }
}

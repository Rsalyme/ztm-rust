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


fn main() {
    let mut store_items = HashMap::new();
    store_items.insert("Chairs", 5);
    store_items.insert("Beds", 3);
    store_items.insert("Tables", 2);
    store_items.insert("Couches", 0);

    for (item, quantity) in store_items.iter() {
        match quantity {
            0 => println!("item: {item} | out of stock"),
            _ => println!("item: {item} quantity: {quantity}"),
          
        }
    }
}

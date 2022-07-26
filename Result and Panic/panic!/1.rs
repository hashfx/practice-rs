/*

// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        __
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink(__);

    println!("Exercise Failed if printing out this line!");
}

*/

use core::panic;

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("drinked, duang.....peng!")
    }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}

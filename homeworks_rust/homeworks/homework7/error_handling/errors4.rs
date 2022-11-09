// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

// I AM NOT DONE

use std::num::ParseIntError;

#[derive(Debug)]
struct Purchase {
    quantity: i32,
    total_cost: i32,
}

fn total_cost(item_quantity: &str) -> Result<Purchase, ParseIntError> {
    let tokens_per_item = 5;
    let processing_fee = 1;
    let quantity = item_quantity.parse::<i32>()?;
    Ok(Purchase {
        quantity,
        total_cost: quantity * tokens_per_item + processing_fee,
    })
}

fn main() {
    let result = "Error: 404".parse::<i32>();
    match result {
        Ok(v) => println!("The answer is: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}


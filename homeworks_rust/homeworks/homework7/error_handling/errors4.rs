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
    let purchase = total_cost("10");
    println!("{:?}", purchase);
}


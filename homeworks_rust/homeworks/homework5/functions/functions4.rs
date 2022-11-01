// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

// I AM NOT DONE

fn main() {
    let mut price = 0;
    let mut rustbucks = 0;
    let mut discount = 0;
    let mut final_price = 0;

    price = 10;
    rustbucks = 10;
    discount = calculate_discount(price, rustbucks);
    final_price = price - discount;
    println!("The final price is {}", final_price);

    price = 11;
    rustbucks = 10;
    discount = calculate_discount(price, rustbucks);
    final_price = price - discount;
    println!("The final price is {}", final_price);
}

fn calculate_discount(price: i32, rustbucks: i32) -> i32 {
    if price % 2 == 0 {
        rustbucks - 10
    } else {
        rustbucks - 3
    }
}

// I AM DONE

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

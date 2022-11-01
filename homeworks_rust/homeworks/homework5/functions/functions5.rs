// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

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

fn square(num: i32) -> i32 {
    num * num
}

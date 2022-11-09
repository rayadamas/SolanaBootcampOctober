// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// I AM NOT DONE

// you can modify anything EXCEPT for this function's signature


fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    let optional_value = Some(String::from("rustlings"));
    if let Some(value) = optional_value {
        println!("The value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }
}
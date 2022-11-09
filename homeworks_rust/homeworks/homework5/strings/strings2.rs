// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

// I AM NOT DONE

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    println!("The first word is: {}", word);
    let my_string_literal = "hello world";
}

fn is_a_color_word(attempt: &str) -> bool {
    match attempt {
        "red" => true,
        "green" => true,
        "blue" => true,
        _ => false,
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

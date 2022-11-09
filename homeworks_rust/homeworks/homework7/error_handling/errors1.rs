// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` for hints!

// I AM NOT DONE

fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        None
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_given() {
        assert_eq!(
            generate_nametag_text("Bors".to_string()),
            Some("Hi! My name is Bors".to_string())
        );
    }

    #[test]
    fn test_empty_name() {
        assert_eq!(generate_nametag_text("".to_string()), None);
    }
}

// I have a question about the HashMap exercise.
// I am not sure what the .to_string() is doing in the insert statements.
// I tried removing it and it still compiles and passes the tests.
// I also tried removing the .to_string() from the test statements and it still passes the tests.
// I am confused as to why it is there and what it is doing.
// Thanks for the help!

// It's converting the string literal to a String , so that it can be inserted into the map.

// If you remove it, you get an error like this:

// error[E0308]: mismatched types
//   --> src/lib.rs:15:5
//    |
// 15 |     basket.insert("apple", 2);
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::string::String`, found `&str`
//    |
//    = note: expected type `std::string::String`
//               found type `&'static str`

// The reason this works with the test is because the test is comparing the string literal "apple" to a String , and the compiler allows it.

// Thanks for the response. I was not aware that string literals were a different type than strings.
// I also did not know that the compiler allowed the test to compare a string literal to a string.

// I am curious as to why the compiler allows the test to compare a string literal to a string but not a string literal to a string.

// Thanks again!

// The compiler allows it because the test is comparing a String to a &str , not a String to a String.
// The test uses the to_string() method on the string literal, which converts it to a String.
// The compiler doesn't allow it in the other case because it's comparing

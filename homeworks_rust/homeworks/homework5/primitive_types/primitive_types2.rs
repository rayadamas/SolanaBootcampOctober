// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

// I AM NOT DONE

fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // Characters (`char`)
    let a = 'a';
    let face = '😀';

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

// // No test changes needed!
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn check_if_morning() {
//         assert!(is_morning);
//     }

//     #[test]
//     fn check_a_is_a() {
//         assert_eq!(a, 'a');
//     }

//     #[test]
//     fn check_face_is_face() {
//         assert_eq!(face, '😀');
//     }

//     #[test]
//     fn check_x_is_500() {
//         assert_eq!(x, 500);
//     }

//     #[test]
//     fn check_y_is_6_4() {
//         assert_eq!(y, 6.4);
//     }

//     #[test]
//     fn check_z_is_1() {
//         assert_eq!(z, 1);
//     }

//     #[test]
//     fn check_first_is_1() {
//         assert_eq!(first, 1);
//     }

//     #[test]
//     fn check_second_is_2() {
//         assert_eq!(second, 2);
//     }
// }

// // errors4.rs
// // Make this test pass! Execute `rustlings hint errors4` for hints :)

// // I AM NOT DONE

// use std::num::ParseIntError;

// #[derive(Debug)]
// struct Purchase {
//     quantity: i32,
//     total_cost: i32,
// }

// fn total_cost(item_quantity: &str) -> Result<Purchase, ParseIntError> {
//     let tokens_per_item = 5;
//     let processing_fee = 1;
//     let quantity = item_quantity.parse::<i32>()?;
//     Ok(Purchase {
//         quantity,
//         total_cost: quantity * tokens_per_item + processing_fee,
//     })
// }

// fn main() {
//     let purchase = total_cost("10");
//     println!("{:
// fn main() {
//     // Characters (`char`)

//     // Note the _single_ quotes, these are different from the double quotes
//     // you've been seeing around.
//     let my_first_initial = 'C';
//     if my_first_initial.is_alphabetic() {
//         println!("Alphabetical!");
//     } else if my_first_initial.is_numeric() {
//         println!("Numerical!");
//     } else {
//         println!("Neither alphabetic nor numeric!");
//     }

//     let // Finish this line like the example! What's your favorite character?
//     // Try a letter, try a number, try a special character, try a character
//     // from a different language than your own, try an emoji!
//     if your_character.is_alphabetic() {
//         println!("Alphabetical!");
//     } else if your_character.is_numeric() {
//         println!("Numerical!");
//     } else {
//         println!("Neither alphabetic nor numeric!");
//     }
// }
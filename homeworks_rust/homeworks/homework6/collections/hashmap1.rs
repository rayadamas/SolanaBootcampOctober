// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    basket.insert("apple".to_string(), 2);
    basket.insert("banana".to_string(), 2);
    basket.insert("mango".to_string(), 1);
    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basket_has_three_fruits() {
        let basket = fruit_basket();
        assert_eq!(basket.len(), 3);
    }

    #[test]
    fn basket_has_five_fruits() {
        let basket = fruit_basket();
        assert_eq!(basket.values().sum::<u32>(), 5);
    }
}


// I have a question about the HashMap exercise. I am not sure what the .to_string() is doing in the insert statements. I tried removing it and it still compiles and passes the tests.

// I also tried removing the .to_string() from the test statements and it still passes the tests. I am confused as to why it is there and what it is doing.

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

// I would like to know if there is a way to use the .to_string() in the insert statements without having to put it in the test statements. I tried using the .to_string() in the test statements and it still passes the tests.

// You can use the .to_string() in the test statements, but you don't need to. The test statements are comparing the string literal "apple" to a String , so the compiler allows it. If you tried to compare the string literal "apple" to a &str , you would get an error like this:

// error[E0308]: mismatched types
// use std::collections::HashMap;

// fn fruit_basket() -> HashMap<String, u32> {
//     let mut basket = // TODO: declare your hash map here.

//     // Two bananas are already given for you :)
//     basket.insert(String::from("banana"), 2);

//     // TODO: Put more fruits in your basket here.

//     basket
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn at_least_three_types_of_fruits() {
//         let basket = fruit_basket();
//         assert!(basket.len() >= 3);
//     }

//     #[test]
//     fn at_least_five_fruits() {
//         let basket = fruit_basket();
//         assert!(basket.values().sum::<u32>() >= 5);
//     }
// }

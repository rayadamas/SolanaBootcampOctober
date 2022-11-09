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



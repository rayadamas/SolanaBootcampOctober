// errors2.rs
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy,
// and the `total_cost` function will calculate the total number of tokens.
// Since the player typed in the quantity, though, we get it as a string-- and
// they might have typed anything, not just numbers!

// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is:
// if we call the `parse` function on a string that is not a number, that
// function will return a `ParseIntError`, and in that case, we want to
// immediately return that error from our function and not try to multiply
// and add.

// There are at least two ways to implement this that are both correct-- but
// one is a lot shorter! Execute `rustlings hint errors2` for hints to both ways.

// I AM NOT DONE

fn total_cost(item_quantity: &str) -> Result<i32, std::num::ParseIntError> {
    let tokens_per_item = 5;
    let processing_fee = 1;
    let quantity = item_quantity.parse::<i32>()?;
    Ok(quantity * tokens_per_item + processing_fee)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_total_cost() {
//         assert_eq!(total_cost("1"), Ok(6));
//         assert_eq!(total_cost("12"), Ok(61));
//         assert_eq!(total_cost("0"), Ok(1));
//         assert_eq!(total_cost("123"), Ok(616));
//     }
// }

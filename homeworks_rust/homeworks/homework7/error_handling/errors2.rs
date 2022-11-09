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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_cost() {
        assert_eq!(total_cost("1"), Ok(6));
        assert_eq!(total_cost("12"), Ok(61));
        assert_eq!(total_cost("0"), Ok(1));
        assert_eq!(total_cost("123"), Ok(616));
    }
}

// Ok, so I guess I am not understanding the concept of error handling. I thought that the question was asking to handle the error when the user inputs a string that is not a number. But the solution is to return the ParseIntError. So, do we always return the error if it occurs? What if we want to handle the error and not return it? In this case, we would want to prompt the user to enter a number again, right? So, why return the error?

// Thanks for the help!

// You can handle the error however you want, but you need to return it somehow. You can't just ignore it.

// The test wants you to return it, and the short solution does that. The long solution handles it, but then returns a custom error instead of the one that was returned by the parse function.

// I see. So, if we want to handle the error and not return it, we have to create our own error?

// Yes, or you can use an error type that already exists. The std::io::Error type is a good example of this.

// I see. Thanks for the help!

// Path: homeworks_rust\homeworks\homework7\error_handling\errors3.rs
// This exercise is similar to errors2, but instead of returning a number
// from the `total_cost` function, we want to return a `Purchase`
// struct. This struct will have two fields: `quantity` and `total_cost`.
// The `quantity` field will be the number of items the player wants to buy,
// and the `total_cost` will be the total number of tokens the player will
// need to pay.
// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// I AM NOT DONE

fn main() {
    call_this(3);
}

// This is a function that takes a number as an argument and prints out "Loop now {number}" for each number in the range 0..num. The function is called from main().
fn call_this(num: i32) {
    for i in 0..num {
        println!("Loop! number {}", i + 1);
    }
}



// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` for hints :D

// I AM NOT DONE

fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
    let mut my_fav_fruits_iter = my_fav_fruits.iter();
    
    assert_eq!(my_fav_fruits_iter.next(), Some(&"banana"));
    assert_eq!(my_fav_fruits_iter.next(), Some(&"custard apple"));
    assert_eq!(my_fav_fruits_iter.next(), Some(&"avocado"));
    assert_eq!(my_fav_fruits_iter.next(), Some(&"peach"));
    assert_eq!(my_fav_fruits_iter.next(), Some(&"raspberry"));
    assert_eq!(my_fav_fruits_iter.next(), None);
}


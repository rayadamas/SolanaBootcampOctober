// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// I AM NOT DONE

mod my_mod {
    pub fn function() {
        println!("called `my_mod::function()`");
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Absolute path
    my_mod::function();

    // Relative path
    function();
}

// mod sausage_factory {
//     // Don't let anybody outside of this module see this!
//     fn get_secret_recipe() -> String {
//         String::from("Ginger")
//     }

//     fn make_sausage() {
//         get_secret_recipe();
//         println!("sausage!");
//     }
// }

// fn main() {
//     sausage_factory::make_sausage();
// }

// This is a `mod` declaration
mod sound {
    // This is also a `mod` declaration, but it's inside another `mod`
    // declaration, so this is called a "nested module"
    mod instrument {
        // This is a `fn` declaration inside a `mod` declaration
        fn clarinet() {
            // Let's use `super` to go up one level relative to the current
            // module, then we'll use the `self` keyword to go down one level
            // relative to the `super` level
            super::breathe_in();
            self::breathe_out();
        }

        fn breathe_out() {
            // Methods can call other methods through `self`:
            self::breathe_in();
        }

        fn breathe_in() {
            // We can use `super` again to go back to the parent module
            super::breathe_in();
        }
    }

    fn breathe_in() {
        // This is a function defined directly in the parent module
    }
}

// This is a `use` declaration that brings all public items from the parent
// module into the current module
use crate::sound::instrument;

// This is a `use` declaration that brings all public items from the sibling
// module into the current module
use self::sound::instrument;

// This is a `use` declaration that brings a specific item into scope
use self::sound::instrument::clarinet;

// This is a `use` declaration that brings a specific item into scope with a
// different name
use self::sound::instrument::clarinet as woodwind;

// This is a `use` declaration that brings
// all public items from a path into scope
use self::sound::instrument::*;

// mod delicious_snacks {

//     // TODO: Fix these use statements
//     use self::fruits::PEAR as ???
//     use self::veggies::CUCUMBER as ???

//     mod fruits {
//         pub const PEAR: &'static str = "Pear";
//         pub const APPLE: &'static str = "Apple";
//     }

//     mod veggies {
//         pub const CUCUMBER: &'static str = "Cucumber";
//         pub const CARROT: &'static str = "Carrot";
//     }
// }

// fn main() {
//     println!(
//         "favorite snacks: {} and {}",
//         delicious_snacks::fruit,
//         delicious_snacks::veggie
//     );
// }

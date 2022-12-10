# Rust Book: Module System

The module system includes:

- Packages: build, test, and share crates.
- Creates: library or executable.
- Modules and use: control scope and privacy of paths.
- Paths: a way of naming an item, such as a struct, function, or module.

## Packages and Crates

**Binary Crate**
- Must have a `main` function
- Can be compiled to create an executable
- e.g. command line tool or server

**Library Crate**
- Does not have a `main` function
- When we say `crate`, we usually mean a library crate

**Crate Root**
- Source file that the Rust compiler starts from and makes up the root module of your crate

**Package**
- Bundle of one or more crates

If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package.

Filename: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting { // module is public, but its contents are private
        pub fn add_to_waitlist() {}  // function is public
        fn seat_at_table() {}  // function is private
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```


 - src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

Module tree

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment



```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // we can use super to go to the parent module of back_of_house
        super::deliver_order(); 
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public
We can use `pub` to make structs an enums public, but th fields and methods will be private unless we explicitly make them public.


```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast); // we can use dot notation because field is public

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

- If we make an enum public, all of its variants are then public.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## Use Keyword to Bring Paths into Scope

```rust
use std::fmt::Result; // use keyword to bring a path into scope
use std::io::Result as IoResult; // `as` as alias to avoid name collisions

pub use crate::front_of_house::hosting; // re-exporting

use std::collections::*; // bring all public items into scope
```
use std::io::{self, Write};
use std::collections::*;

mod front_of_house;

fn deliver_order() {}

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

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// eat_at_restaurant and front_of_house are siblings, so front_of_house doesn't need pub
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}


// use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // This doesn't work unless use crate::front_of_house::hosting is inside mod customer block
        // hosting::add_to_waitlist();
        // Alternatively, you can use super, like this:
        super::hosting::add_to_waitlist();
    }
}

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}

use std::fmt;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
    fmt::Result::Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    IoResult::Ok(())
}
/*
If you plan on sharing your library crate so other projects can use your code,
your public API is your contract with users of your crate that determines how 
they can interact with your code. 
There are many considerations around managing changes to your public API to 
make it easier for people to depend on your crate. These considerations are 
out of the scope of this book; if you’re interested in this topic, see The 
Rust API Guidelines (https://rust-lang.github.io/api-guidelines/).
*/

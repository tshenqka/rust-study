// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}



// we can improve on above

// In rust parent modules cant access private items inside child modules but can access pub items
// this is to hide implementation details and utilize API


// mod customer {
//     use crate::front_of_house::hosting;

//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist(); // this is idiomatic and we know module 
//     }
// }

// two items with the same name
// use std::fmt::Result;
// use std::io::Result as IoResult;

// use std::{cmp::Ordering, io}; // this works too

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }


mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {

    hosting::add_to_waitlist();

    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // use crate::front_of_house::hosting;
    // // shortcut path convention for functions
    // hosting::add_to_waitlist();

    // shortcut path convention for enums/structs/other itmes
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(1, 2);

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // Change our mind about what bread we'd like
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal._seasonal_fruit = String::from("blueberries");

    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
}

pub fn _deliver_order() {}

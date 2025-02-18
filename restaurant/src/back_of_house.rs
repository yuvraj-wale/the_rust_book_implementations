pub struct Breakfast {
    pub toast: String,
    _seasonal_fruit: String,
}
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast{
        return Breakfast{
            toast: String::from(toast),
            _seasonal_fruit: String::from("peaches")
        };
    }
}

pub enum Appetizer {
    Soup,
    Salad
}

fn _fix_incorrect_order() {
    _cook_order();
    super::_deliver_order();
}

fn _cook_order() {}
use crate::garden::vegetable::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("Vegetable: {plant:?}");
}

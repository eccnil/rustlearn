use std::{cmp::Ordering, io, self};

mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }
    fn cook_order() {}
}

fn deliver_order() {}

pub use crate::back_of_house::Breakfast as Desayuno;

pub fn eat_at_restaurat() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    let mut meal = Desayuno::summer("centeno");
    meal.toast = String::from("trigo");
    println!("tomaré una tosta de {:?}", meal.toast);
    println!("have a nice day!{}", String::from("my friend"));
}

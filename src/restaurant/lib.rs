// A mod statement imports a module.
// A module only needs to be included once in the module tree.
// use does not import.
// Instead it creates an alias for less verbose access to module components.
mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Since toast is a public field, we can access it here (write (for mut Breakfast) and read).
    meal.toast = String::from("Wheat");
    println!("I'd like {} please!", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {
}

mod back_of_house {
    fn fix_incorrent_order() {
        cook_order();
        // The super keyword references the parent package.
        super::deliver_order();
    }

    fn cook_order() {}

    // This makes the struct public, but all fields are still private by default.
    pub struct Breakfast {
        // This makes a field public.
        pub toast: String,
        // This field can only be read/written from within the module.
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // When an enum is public, all of its variants are public as well.
    pub enum Appetizer {
        Soup,
        Salad
    }
}
mod front_of_house {
    /**
     * - a child module is private by default in the perspective of the parent
     * - a child can see other functions inside its parent
     * - so we use 'pub' keyword to make the module public
     */
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("✅ Added to waitlist");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
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
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // field `toast` of struct `Breakfast` is private field
    // so we can't access a struct field if its not public
    meal.toast = String::from("Wheat");

    // after making toast public, we can access it

    // we can't create a breakfast using the Breakfast struct
    // because it has a private field 'seasonal_fruit'

    // if enum is marked as public, all of its variants are public
    let app = back_of_house::Appetizer::Soup;

    println!("Appetizer: {:?}", app);

    front_of_house::hosting::add_to_waitlist();
}

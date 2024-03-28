mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    mod serving {
        fn take_order() {
            println!("take_order");
        }

        fn serve_order() {
            println!("serve_order");
        }

        fn take_payment() {
            println!("take_payment");
        }
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    mod test {
        fn testfn() {
            super::fix_incorrect_order();

            super::super::deliver_order();
        }
    }

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
}

use crate::front_of_house::hosting;

mod customer {

    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // // 절대 경로
    // crate::front_of_house::hosting::add_to_waitlist();

    // // 상대 경로
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat"); // struct 안 pub 라서 오픈되어 있기에 가능
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");   // struct 안 pub 로 열려있지 않기에 불가능

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad; // enum 이 pub 면 안의 배리언트들 다 가능

    // hosting::add_to_waitlist();
}

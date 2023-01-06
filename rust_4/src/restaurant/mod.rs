mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        fn serve_customer(customer_pizza: super::Pizza) {
            println!(
                "The customer was served a regular pizza with {}",
                customer_pizza.topping
            )
        }
        pub fn take_order() {
            seat_at_table();
            let customer_pizza: super::Pizza = super::Pizza::lunch("ham and mushroom");
            serve_customer(customer_pizza);
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}

use crate::garden::flower::Flower;
use crate::restaurant::restaurant_function;
//use std::collections::{HashMap,LinkedList}; //This is how you use multiple items from a namespace
//use std::io::{self,Write}; //This includes Write but also std::io itself by using the "self" keyword
//use std::collections::*; //This uses every single item in the collections namespace
use rand::Rng; //Uses the trait Rng from the rand crate.

fn main() {
    println!("\n~Using the random crate.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The randomly generated number is {}", secret_number);

    println!("\n~Testing importing custom modules from other files.");
    let mut test_flower = Flower::new(String::from("Tulip"),5,(240,64,96));
    test_flower.display_flower();
    for _i in 0..7 { test_flower.she_loves_me(); }
    test_flower.display_flower();
    //let using_field = test_flower.PetalsLeft; //Cannot access this field because, while the struct is public, the field has not been made public

    println!("\n~Testing importing custom modules from the crate root.");
    restaurant_function();

    println!("\n~Testing calling parent functions in relative paths.");
    module_level_1::module_level_2::module_level_3::level_3_fn(); //Accessing the function with the path, without using the "use" keyword

    println!("\n~Testing using a module name.");
    use outside::using_mod_name::name; //You can access a module name in shorhand by using "use" on that module.
    name::function();                  //Calling the function in the used module
    name::inner::function();           //Calling a function in a module inside the used module
    println!("You can also rename a function when using it with the \"as\" keyword");
    use outside::using_mod_name::name::inner::function as inside_function;
    name::function();
    inside_function();
}

mod garden; //Defines a random module named garden. 
//The mod keyword simply defines that an item is now a module, but that module can be anything from a variable, function, struct, enum, module, constant, or anything else that consititutes an item

mod restaurant { //Defines a module and gives it module code. Does not need to be declared public since it and the main function are in the same context, and are siblings. Siblings can interact with each other.
    pub mod hosting {
        pub fn add_to_waitlist() { println!("Somebody has been added to the watchlist."); seat_at_table() }
        fn seat_at_table() { println!("Sit, heathen."); }
    }
    pub mod serving {
        pub fn take_order() { println!("Fuck you want?"); take_payment(); }
        fn serve_order() { println!("Here's your food. It was hard but fun to make, you made a good choice."); }
        fn take_payment() { println!("You paid. Wonderful."); serve_order(); }
    }
    fn leaving() { println!("Thank you for eating at mad-to-glad restaurant."); }
    use hosting::add_to_waitlist;
    use serving::take_order;
    pub fn restaurant_function() { 
        println!("Entering the restaurant.");
        add_to_waitlist(); take_order(); leaving();
    }
}

fn outside_function() { println!("Outside function"); }

mod module_level_1 {
    pub mod module_level_2 {
        pub mod module_level_3 {
            pub fn level_3_fn() { 
                println!("~~~Function on level 3");
                super::level_2_fn();
                super::super::level_1_fn();
                super::super::super::outside_function(); //"super" acts like "cd .." for the module tree, and can be stacked
            }
        }
        fn level_2_fn() { println!("~~Function on level 2"); } //Functions being accessed through "super" don't have to be public
    }
    fn level_1_fn() { println!("~Function on level 1"); }
}

mod outside {
    pub mod using_mod_name {
        pub mod name {
            pub fn function() { println!("This is a function"); }
            pub mod inner { pub fn function() { println!("This is an inner function"); } }
        }
    }
}
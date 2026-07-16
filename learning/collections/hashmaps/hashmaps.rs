// stores key-value pairs in rust. similar to json objects, dict in python or hashmaps in java.
// methods: insert, get, remove and clear

use std::{collections::HashMap};

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("Sabharish"), 21);
    users.insert(String::from("Harkirat"), 22);

    let first_users_age = users.get("ayesha");
    
    match first_users_age {
        Some(age) => println!("Age; {}", age),
        None => println!("User not found!"),
    }
}
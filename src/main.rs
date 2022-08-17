#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use playground::library::{first::{first, User}, second::second, rectangle};
use std::collections::HashMap;

fn main() {

    println!();

    // first();

    let user = User {
        active: true,
        username: String::from("Charles Darwin"),
        age: 65
    };

    // println!("{} is {}", user.username, user.age);

    rectangle::test_rectangle();

    let rect = rectangle::Rectangle::new(1080.0, 480.0);

    let string = String::from("peter");

    let str = String::from("Evolution").as_str();

    let key = Key { };

    let mut hash_map: HashMap<String, f32> = HashMap::new();

    hash_map.insert(String::from("Count"), 32.0);

    println!("rect: {}", rect);
    println!("\tdiagonal: {}\n", rect.diag_len());

    println!("hash map: {:?}", hash_map);

}

struct Key {

}

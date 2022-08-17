#![allow(dead_code)]
#![allow(unused_variables)]

pub fn first() {

    let user = User {
        age: 40,
        active: true,
        username: String::from("John_Doe")
    };

    println!("end first");

}

pub struct User {
    pub active: bool,
    pub username: String,
    pub age: i32
}

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

// MARK: Type Memebrs

impl Rectangle {

    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }

}

// MARK: Instance Members

impl Display for Rectangle {

     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle(width: {}, height: {})", self.width, self.height)
    }

}

impl Hash for Rectangle {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.to_bits().hash(state);
        self.height.to_bits().hash(state);
    }

}

impl Rectangle {

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn diag_len(&self) -> f32 {
        // d = √(l² + w²)
        (self.width.powf(2.0) + self.height.powf(2.0)).sqrt()
    }

}


pub fn test_rectangle() {

    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

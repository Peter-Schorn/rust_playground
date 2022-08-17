#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use std::io::Write;

pub fn second() {

    print!("\nGuess the Number: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()  // \/ references, like variables, are immutable by default
        .read_line(&mut guess)
        .expect("Failed to read line");

    // println!("You guessed: {}", guess);
    let message = format!("You guessed: {}", guess);

    println!("{}", message);

}

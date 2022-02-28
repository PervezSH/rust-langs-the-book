// Import input/output library
use std::io;

fn main() {
    println!("---Guess the number 🔢!---");
    println!("Input your guess : ");

    // Create a mutable variable that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    // Recieving user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline! 🤕");

    println!("You Guessed: {}", guess);
}

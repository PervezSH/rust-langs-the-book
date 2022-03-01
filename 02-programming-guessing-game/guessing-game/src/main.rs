// include the input/output functionality from the standard library
use std::io;
// include random number generator from random number library
use rand::Rng;

fn main() {
    println!("---Guess the number 🔢!---");
    println!("Input your guess : ");

    // prepare a non-deterministic random number generator:
    let mut rng = rand::thread_rng();
    // generate random number between 0 - 100
    let secret_number = rng.gen_range(1..101);
    println!("The secret number is {}", secret_number);

    // create a mutable variable that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    // recieving user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline! 🤕");

    println!("You Guessed: {}", guess);
}

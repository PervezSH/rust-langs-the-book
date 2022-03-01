// include the input/output functionality from the standard library
use std::io;
// include random number generator from random number library
use rand::Rng;
// include Ordering from the standard library
use std::cmp::Ordering; // enum has the variants, Less, Greater and Equal

fn main() {
    println!("---Guess the number 🔢!---");

    // prepare a non-deterministic random number generator:
    let mut rng = rand::thread_rng();
    // generate random number between 0 - 100
    let secret_number = rng.gen_range(1..101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your guess : ");
        // create a mutable variable that is currently bound to a new, empty instance of a String.
        let mut guess = String::new();

        // recieving user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline! 🤕");

        // convert guess to number so that it can be compared
        let guess: u32 = guess.trim().parse().expect("Please type a number!🤕");

        println!("You Guessed: {}", guess);

        // compare guess and secret number
        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("You Won!🥳"),
            Ordering::Greater => println!("Too Big!💪"),
            Ordering::Less => {
                println!("Too Small!🤏");
                break;
            }
        }
    }
}

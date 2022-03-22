enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Penny is {}", value_in_cents(Coin::Penny));
    println!("Nickel is {}", value_in_cents(Coin::Nickel));
    println!("Dime is {}", value_in_cents(Coin::Dime));
    println!("Quarter is {}", value_in_cents(Coin::Quarter));
}

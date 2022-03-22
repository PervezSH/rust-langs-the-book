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

    // Matching with Option enum
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    println!("Six is {:?}", six.unwrap()); // Returns the contained Some value
    println!("None is {:?}", plus_one(None));
}

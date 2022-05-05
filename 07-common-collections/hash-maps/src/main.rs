// bring hashmap type into scope from standard library
use std::collections::HashMap;
fn main() {
    /* Crearing HashMaps */
    // creating empty hashmap
    let mut scores: HashMap<String, i32> = HashMap::new();
    // populating hashmap
    scores.insert(String::from("TeamBlue"), 50);
    scores.insert(String::from("TeamRed"), 30);

    /* Retrieving value from HashMap */
    // get() takes reference and returns Option
    let team_blue_score = scores.get(&String::from("TeamBlue"));
    println!("Score of TeamBlue is {}", team_blue_score.unwrap());

    /* Iterating over all the elements in a HashMap */
    println!("Iterating all the key-val pairs:");
    for (key, val) in &scores {
        println!("{} : {}", key, val);
    }

    /* Updating HashMap */
    // or_insert enters a new entry, if there isn't one for the given key
    scores.entry(String::from("TeamRed")).or_insert(100);

    /* Updating value based on old value */
    // or_insert returns a mutable reference
    let team_green_score = scores.entry(String::from("TeamGreen")).or_insert(10);
    *team_green_score += 10;
    println!(
        "Printing updating scores of TeamGreen : {}",
        &team_green_score
    );

    println!("{:?}", scores);
}

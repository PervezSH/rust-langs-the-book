fn main() {
    // break and continue control flow in loops
    break_and_continue();
}

fn break_and_continue() {
    let mut counter = 0;
    // labeled loop
    'outer_loop: loop {
        let mut inner_counter = 0;
        loop {
            if inner_counter == 2 {
                println!("Inner Counter : {}", inner_counter);
                counter += 2;
                break 'outer_loop;
            }
            inner_counter += 1;
        }
    }
    println!("Outer Counter : {}", counter);
}

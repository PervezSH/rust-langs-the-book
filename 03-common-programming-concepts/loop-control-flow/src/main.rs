fn main() {
    // break and continue control flow in loops
    break_and_continue();
    // returning values from loops
    return_loops();
    // display first 10 fibonacii number
    generate_nth_fibonacci(10);
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

fn return_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("Loop result {}", result);
}

fn generate_nth_fibonacci(n: i32) {
    if n < 1 {
        println!("{}", n);
    }
    let mut f1 = 0;
    let mut f2 = 1;
    print!("{} ", f1);
    for i in 1..n {
        print!("{} ", f2);
        let next = f1 + f2;
        f1 = f2;
        f2 = next;
    }
}

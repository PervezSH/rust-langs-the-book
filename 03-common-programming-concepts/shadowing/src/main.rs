fn main() {
    // implement the concept of shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Value of x in inner scope is : {}", x);
    }
    println!("Value of x is : {}", x);
}

fn main() {
    // implement the concept of shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Value of x in inner scope is : {}", x);
    }
    println!("Value of x is : {}", x);

    // difference between mut and shadowing
    // this will work
    let spaces = "          ";
    let spaces = spaces.len(); // The first spaces variable is a string type and the second spaces variable is a number type
                               // however, this will not work
    let mut spaces = "          ";
    spaces = spaces.len();
}

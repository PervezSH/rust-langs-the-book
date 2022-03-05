fn main() {
    // a statement
    let x = 10;

    // A new scope block created with curly braces is an expression
    let y = {
        let a = 5;
        a + 5 // expression do not include semicolons, if you add one, you turn it into statement
    };

    println!("Value of x and y = {}, {}", x, y);
}

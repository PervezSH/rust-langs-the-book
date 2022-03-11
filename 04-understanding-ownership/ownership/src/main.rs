fn main() {
    // Move
    implementMove();
}

fn implementMove() {
    let s1 = String::from("hello");
    let s2 = s1;
    // here rust make s1, no longer valid, to ensure memory safet, double free error
    println!("{}, world!", s1);
}

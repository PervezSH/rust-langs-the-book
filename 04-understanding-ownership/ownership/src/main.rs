fn main() {
    // Move
    implementMove();
    // Clone
    implementClone();
}

fn implementMove() {
    let s1 = String::from("hello");
    let s2 = s1;
    // here rust make s1, no longer valid, to ensure memory safet, double free error
    println!("{}, world!", s1);
}

fn implementClone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deeply copies heap data of s1
    println!("{} from s1, {} from, world!", s1, s2);
}

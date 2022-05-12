/* Unit Test */
// Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
// It is convention to write unit test in the same file
#[cfg(test)] // annotating with [cfg(test)], so that they shouldn't be included in the compile result
mod tests {
    use super::*;
    #[test]
    // testing private function
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

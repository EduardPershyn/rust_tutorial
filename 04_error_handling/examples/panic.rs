// Run: cargo run --example panic
// Then:  RUST_BACKTRACE=1 cargo run --example panic     (shows the call stack)
// Look at the message, the file:line it points to, and the exit code (echo $? prints 101).

fn average(values: &[u32]) -> u32 {
    let total: u32 = {
        let mut sum = 0;
        for v in values {
            sum += v;
        }
        sum
    };
    // An empty slice here is a caller BUG, not a normal outcome, so a panic is appropriate.
    // (If empty input were normal, the function should return Option or Result instead.)
    assert!(!values.is_empty(), "average() called with an empty slice");
    total / values.len() as u32
}

fn main() {
    println!("average of [2, 4, 9] = {}", average(&[2, 4, 9]));
    println!("now calling average(&[]) ...");
    let result = average(&[]);
    println!("never printed: {result}");
}

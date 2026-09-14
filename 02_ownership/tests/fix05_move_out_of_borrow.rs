//! FIX 05: moving out of borrowed data.
//! Make it compile WITHOUT `.clone()`: return a reference instead of a new String.
//! Don't change the test.
//! Run: cargo test --test fix05_move_out_of_borrow

fn longest_name(names: &[String]) -> String {
    let mut best = names[0];
    for name in names {
        if name.len() > best.len() {
            best = *name;
        }
    }
    best
}

#[test]
fn finds_longest() {
    let names = vec![String::from("ann"), String::from("charlie"), String::from("bob")];
    assert_eq!(longest_name(&names), "charlie");
    assert_eq!(names.len(), 3);
}

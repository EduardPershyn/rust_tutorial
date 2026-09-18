//! FIX 06: matching and destructuring MOVE non-Copy data.
//! Make it compile WITHOUT `.clone()`. Change only the `match` / `let` lines. Don't change the asserts.
//! Run: cargo test --test fix06_match_moves

struct User {
    name: String,
    age: u32,
}

#[test]
fn match_then_reuse() {
    let nickname: Option<String> = Some(String::from("neo"));
    let len = match nickname {
        Some(n) => n.len(),
        None => 0,
    };
    assert_eq!(len, 3);
    assert_eq!(nickname, Some(String::from("neo")));
}

#[test]
fn destructure_then_reuse() {
    let user = User { name: String::from("ann"), age: 30 };
    let User { name, .. } = user;
    assert_eq!(name, "ann");
    assert_eq!(user.age, 30); // this line is fine even now. Why?
    assert_eq!(user.name, "ann");
}

// Run: cargo run --example borrowing

// Shared borrow: read-only access, the caller keeps ownership.
fn count_spaces(s: &str) -> usize {
    let mut n = 0;
    for c in s.chars() {
        if c == ' ' {
            n += 1;
        }
    }
    n
}

// Mutable borrow: can modify the caller's value.
fn add_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

// Returns a reference INTO the argument. Fine: it can't outlive the input.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// Newly created data must be returned as an owned value.
fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

// fn dangling() -> &String {       // ❌ E0106: no input to borrow from
//     let s = String::from("hi");   //    and `s` dies at the closing brace
//     &s
// }

fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n; // i32 += &i32 works: arithmetic is implemented for references too
    }
    total
}

fn main() {
    // ---------- & and &mut ----------
    let mut text = String::from("borrow me please");
    let spaces = count_spaces(&text); // lend read access
    add_suffix(&mut text, "!"); // lend write access
    println!("text={text:?}, spaces={spaces}, first_word={:?}", first_word(&text));
    println!("{}", greeting("Rust"));

    // ---------- many readers OR one writer ----------
    let r1 = &text;
    let r2 = &text; // ✅ any number of shared references
    println!("r1={r1}, r2={r2}");
    // r1 and r2 are never used again → their borrows end here

    let w = &mut text; // ✅ no live shared references, so this is allowed
    w.push_str("!!");
    // println!("{r1}");              // ❌ E0502 if uncommented: r1 would still be alive
    println!("after w: {text}");

    // ---------- the classic: reference invalidation ----------
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("first={first}"); // use it BEFORE mutating v...
    v.push(4); // ...then this is fine
    // println!("{first}");           // ❌ E0502: push may reallocate → first would dangle (UB in C++)
    println!("v={v:?}");

    // ---------- dereference ----------
    let mut counter = 0;
    let c = &mut counter;
    *c += 10;
    println!("counter={counter}");

    let s = String::from("auto");
    let rs = &s;
    let rrs = &rs; // &&String
    println!("rrs.len()={}", rrs.len()); // method calls auto-deref through any number of &

    let a = 5;
    let ra = &a;
    println!("*ra == 5 → {}", *ra == 5); // compare values: deref the reference
    // println!("{}", ra == 5);        // ❌ can't compare `&i32` with `i32`

    // ---------- iterating by reference ----------
    let mut scores = vec![70, 85, 90];
    for s in &scores {
        print!("{s} "); // s: &i32
    }
    println!();
    for s in &mut scores {
        *s += 5; // s: &mut i32
    }
    println!("curved: {scores:?}");
    for (i, s) in scores.iter().enumerate() {
        println!("  #{i}: {s}");
    }
    println!("sum={}, scores still ours: {scores:?}", sum(&scores));

    // ---------- mem::take: move a value out through &mut ----------
    let mut buffer = String::from("data");
    let taken = std::mem::take(&mut buffer); // buffer is left as ""
    println!("taken={taken:?}, buffer={buffer:?}");
}

// Run: cargo run --example slices_strings

fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn zero_out(part: &mut [i32]) {
    for x in part {
        *x = 0;
    }
}

fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    // ---------- slices of arrays and Vecs ----------
    let arr = [1, 2, 3, 4, 5];
    let v = vec![10, 20, 30, 40];
    println!("arr[1..4]={:?} arr[..2]={:?} arr[3..]={:?}", &arr[1..4], &arr[..2], &arr[3..]);
    // one function accepts arrays, Vecs and sub-slices:
    println!("sum(arr)={} sum(v)={} sum(v[1..3])={}", sum(&arr), sum(&v), sum(&v[1..3]));

    let mut data = [5, 3, 8, 1];
    zero_out(&mut data[1..3]); // mutable view of just the middle
    println!("data={data:?}");

    println!("v.get(1)={:?} v.get(99)={:?}", v.get(1), v.get(99)); // Option<&i32>, no panic

    // ---------- String vs &str ----------
    let owned: String = String::from("hello"); // owns a growable heap buffer
    let view: &str = &owned[1..4]; // borrowed view: "ell"
    let literal: &str = "static text"; // stored in the binary
    println!("owned={owned} view={view} literal={literal}");
    println!("{} {} {}", shout(&owned), shout(view), shout(literal)); // &String → &str automatically

    // building strings
    let mut s = String::new();
    s.push_str("Rust");
    s.push(' ');
    s += "is"; // String += &str
    let s = s + " fun"; // String + &str: moves s and reuses its buffer
    let s2 = format!("{s}, really {}", "fun");
    println!("{s2}");

    // ---------- UTF-8: bytes vs chars ----------
    let word = "Привіт"; // 6 chars, 12 bytes (each Cyrillic letter is 2 bytes)
    println!("{word}: len()={} bytes, chars().count()={}", word.len(), word.chars().count());
    // let c = word[0];                // ❌ doesn't compile: strings can't be indexed
    println!("chars().next()={:?}", word.chars().next()); // Some('П')
    println!("&word[0..2]={}", &word[0..2]); // 'П' is 2 bytes
    // let bad = &word[0..1];          // 💥 panic: byte index 1 is not a char boundary
    for (i, c) in word.char_indices() {
        print!("{c}@{i} "); // char and its byte offset
    }
    println!();
    for b in "hi!".bytes() {
        print!("{b} ");
    }
    println!();

    // ---------- common &str methods ----------
    let line = "  name = Alice  ";
    let trimmed = line.trim(); // a &str into `line`: no allocation
    match trimmed.find('=') {
        Some(i) => {
            let key = trimmed[..i].trim();
            let value = trimmed[i + 1..].trim();
            println!("key={key:?} value={value:?}");
        }
        None => println!("no '='"),
    }
    for w in "the  quick brown".split_whitespace() {
        print!("[{w}] ");
    }
    println!();
    println!("{}", "a,b,c".replace(',', ";")); // returns a new String
    println!("{}", "RuSt".to_lowercase());
    println!("starts_with: {}", "rustacean".starts_with("rust"));

    // ---------- converting ----------
    let a: String = "x".to_string(); // &str → String: allocates
    let b: &str = a.as_str(); // String → &str: free
    let c: &str = &a; // same, via deref coercion
    println!("{a} {b} {c}");
}

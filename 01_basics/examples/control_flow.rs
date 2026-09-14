// Run: cargo run --example control_flow

fn main() {
    let n = 7;

    // ---------- if ----------
    let parity = if n % 2 == 0 { "even" } else { "odd" }; // no ternary operator
    println!("{n} is {parity}");

    if n < 0 {
        println!("negative");
    } else if n < 10 {
        println!("single digit");
    } else {
        println!("big");
    }

    // ---------- loop with a value ----------
    let mut p = 1;
    let first_pow2_over_100 = loop {
        p *= 2;
        if p > 100 {
            break p;
        }
    };
    println!("first power of 2 > 100: {first_pow2_over_100}");

    // ---------- while ----------
    let mut countdown = 3;
    while countdown > 0 {
        print!("{countdown}... ");
        countdown -= 1;
    }
    println!("go!");

    // ---------- for over ranges ----------
    for i in 0..3 {
        print!("{i} "); // 0 1 2
    }
    println!();
    for i in (1..=3).rev() {
        print!("{i} "); // 3 2 1
    }
    println!();
    for i in (0..10).step_by(3) {
        print!("{i} "); // 0 3 6 9
    }
    println!();

    // ---------- for over arrays ----------
    let fruits = ["apple", "banana", "cherry"];
    for f in fruits {
        print!("{f} ");
    }
    println!();
    for (idx, f) in fruits.into_iter().enumerate() {
        println!("  {idx}: {f}");
    }
    // (`.iter()` instead of `.into_iter()` yields references, see section 02)

    // ---------- continue + labeled break ----------
    'outer: for a in 1..10 {
        for b in 1..10 {
            if b > a {
                continue 'outer; // next `a`
            }
            if a * b == 42 {
                println!("found {a} * {b} = 42");
                break 'outer; // leave both loops
            }
        }
    }

    // ---------- match: exhaustive, returns a value ----------
    for x in [-5, 0, 1, 2, 7, 42] {
        let desc = match x {
            i32::MIN..=-1 => "negative",
            0 => "zero",
            1 | 2 => "one or two",
            3..=9 => "single digit",
            _ => "big",
        };
        println!("{x:>3}: {desc}"); // {:>3} = right-align, width 3
    }

    // ---------- match on tuples: destructuring + guards ----------
    for point in [(0, 0), (5, 0), (3, -3), (1, 2)] {
        match point {
            (0, 0) => println!("{point:?} origin"),
            (x, 0) | (0, x) => println!("{point:?} on an axis at {x}"),
            (x, y) if x == -y => println!("{point:?} on the anti-diagonal"),
            (x, y) => println!("{point:?} somewhere, x+y={}", x + y),
        }
    }

    // ---------- Option preview ----------
    let arr = [4, 8, 15, 16, 23, 42];
    let mut found = None; // type inferred later as Option<usize>
    for (i, v) in arr.into_iter().enumerate() {
        if v == 15 {
            found = Some(i);
            break;
        }
    }
    match found {
        Some(i) => println!("15 is at index {i}"),
        None => println!("15 not found"),
    }
}

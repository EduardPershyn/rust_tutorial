// Run: cargo run --example functions

// Last expression without `;` is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Return several values with a tuple.
fn div_rem(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// `return` is for early exit.
fn abs(x: i32) -> i32 {
    if x >= 0 {
        return x;
    }
    -x
}

// No `-> T` means it returns `()`.
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// `if` is an expression, so it can be the whole function body.
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Recursion works as usual.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// A classic mistake:
// fn broken(a: i32) -> i32 {
//     a + 1;   // ERROR: expected `i32`, found `()`. Remove the `;`
// }

fn main() {
    println!("add(2, 3) = {}", add(2, 3));

    let (q, r) = div_rem(17, 5);
    println!("17 / 5 = {q} remainder {r}");

    println!("abs(-4) = {}, max(3, 9) = {}, gcd(48, 18) = {}", abs(-4), max(3, 9), gcd(48, 18));
    greet("Rust");

    // Blocks are expressions.
    let y = {
        let x = 3;
        x * x + 1
    };
    println!("y = {y}");

    // Functions can be declared inside functions.
    fn square(x: i32) -> i32 {
        x * x
    }
    println!("square(7) = {}", square(7));

    // Closures (anonymous functions, section 08).
    let mul = |a: i32, b: i32| a * b;
    let offset = 100;
    let add_offset = |v: i32| v + offset; // captures `offset` from the environment
    println!("mul(6, 7) = {}, add_offset(1) = {}", mul(6, 7), add_offset(1));

    // dbg! prints [file:line] expr = value to stderr and returns the value.
    let z = dbg!(add(1, 1) * 10);
    println!("z = {z}");
}

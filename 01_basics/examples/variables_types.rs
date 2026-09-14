// Run: cargo run --example variables_types

fn main() {
    // ---------- immutability & shadowing ----------
    let x = 5;
    // x = 6;                    // ERROR: cannot assign twice to immutable variable
    let mut counter = 0;
    counter += 1;
    let x = x * 2; // shadowing: new variable, old one is gone
    let spaces = "   ";
    let spaces = spaces.len(); // shadowing may change the type: &str -> usize
    println!("x={x}, counter={counter}, spaces={spaces}");

    // ---------- constants ----------
    const MAX_SUPPLY: u64 = 21_000_000; // `_` is a digit separator
    println!("MAX_SUPPLY={MAX_SUPPLY}");

    // ---------- integers & literals ----------
    let a = 42; // i32 by default
    let b: u8 = 255;
    let c = 1_000_000i64; // type suffix
    let hex = 0xff;
    let bin = 0b1010;
    let byte = b'A'; // u8 = 65
    println!("{a} {b} {c} {hex} {bin} {byte}");
    println!("i32 range: {}..={}", i32::MIN, i32::MAX);
    println!("u128 max:  {}", u128::MAX);

    // ---------- conversions: never implicit ----------
    let small: i32 = 300;
    let truncated = small as u8; // 44: `as` silently truncates
    let checked = u8::try_from(small); // Err(...): checked
    let widened: i64 = small.into(); // lossless, always OK
    println!("as u8 = {truncated}, try_from = {checked:?}, into i64 = {widened}");
    // let bad: i64 = small;     // ERROR: expected i64, found i32

    // ---------- overflow control ----------
    let big: u8 = 250;
    println!("checked_add:    {:?}", big.checked_add(10)); // None
    println!("wrapping_add:   {}", big.wrapping_add(10)); // 4
    println!("saturating_add: {}", big.saturating_add(10)); // 255
    // let boom = big + 10;      // panics in debug (here even rejected at compile time)

    // ---------- floats, bool, char ----------
    let ratio = 2.5_f64;
    println!("ratio={ratio:.3}, sqrt(2)={}, pi={}", 2f64.sqrt(), std::f64::consts::PI);
    let seven = 7;
    println!("int -> float: {}", seven as f64 / 2.0); // 3.5
    let is_active: bool = true;
    let heart = '❤'; // char = Unicode scalar, 4 bytes
    println!(
        "is_active={is_active}, heart={heart}, size_of::<char>()={}",
        std::mem::size_of::<char>()
    );

    // ---------- integer division gotchas ----------
    println!("7 / 2 = {}", 7 / 2); // 3
    println!("-7 / 2 = {}", -7 / 2); // -3 (toward zero)
    println!("-7 % 3 = {}", -7 % 3); // -1
    println!("(-7).rem_euclid(3) = {}", (-7i32).rem_euclid(3)); // 2
    println!("i32::MIN.unsigned_abs() = {}", i32::MIN.unsigned_abs());

    // ---------- tuples ----------
    let person: (&str, u32) = ("Alice", 30);
    let (name, age) = person; // destructuring
    println!("{name} is {age}; person.0 = {}", person.0);
    let unit = (); // empty tuple = "unit", the type of "no value"
    println!("unit = {unit:?}");

    // ---------- arrays ----------
    let arr = [10, 20, 30];
    let zeros = [0u8; 4];
    println!("arr={arr:?}, len={}, arr[0]={}, zeros={zeros:?}", arr.len(), arr[0]);
    let mut grid = [[0; 3]; 2];
    grid[1][2] = 7;
    println!("grid={grid:?}");
    // arr[10];                  // index out of bounds: compile error for constants, panic at runtime
}

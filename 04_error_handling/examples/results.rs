// Run: cargo run --example results

use std::num::ParseIntError;

// `?` passes the error up; the success value is wrapped in Ok at the end.
fn parse_sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.trim().parse()?;
    let y: i32 = b.trim().parse()?;
    Ok(x + y)
}

// Option → Result: give a missing value an error message with ok_or.
fn first_word(text: &str) -> Result<&str, String> {
    text.split_whitespace().next().ok_or("empty text".to_string())
}

// Handling instead of propagating: decide what an error means right here.
fn parse_or_zero(s: &str) -> i32 {
    s.parse().unwrap_or(0)
}

// A function whose only job is a side effect returns Result<(), E>.
fn withdraw(balance: &mut u64, amount: u64) -> Result<(), String> {
    if amount > *balance {
        return Err(format!("need {amount}, have {balance}"));
    }
    *balance -= amount;
    Ok(()) // "success, nothing to return"
}

fn main() {
    // ---------- match on a Result ----------
    for input in ["42", "-7", "abc", "99999999999"] {
        match input.parse::<i32>() {
            Ok(n) => println!("{input:>12} → Ok({n})"),
            Err(e) => println!("{input:>12} → Err: {e}"), // ParseIntError implements Display
        }
    }

    // ---------- ? inside a function ----------
    println!("parse_sum(\"2\", \" 3 \") = {:?}", parse_sum("2", " 3 "));
    println!("parse_sum(\"2\", \"x\")   = {:?}", parse_sum("2", "x"));

    // ---------- converting between Option and Result ----------
    println!("first_word(\"hi there\") = {:?}", first_word("hi there"));
    println!("first_word(\"   \")      = {:?}", first_word("   "));
    let maybe: Option<i32> = "7".parse().ok(); // Result → Option: forget the error
    println!("\"7\".parse().ok() = {maybe:?}");

    // ---------- quick handling ----------
    println!("parse_or_zero(\"12\")={} parse_or_zero(\"?\")={}", parse_or_zero("12"), parse_or_zero("?"));
    let r: Result<i32, ParseIntError> = "5".parse();
    println!("is_ok={} is_err={}", r.is_ok(), r.is_err());

    // ---------- Result<(), E> and must_use ----------
    let mut balance = 100;
    // withdraw(&mut balance, 30);     // ⚠️ warning: unused `Result` that must be used
    if let Err(e) = withdraw(&mut balance, 30) {
        println!("failed: {e}");
    }
    match withdraw(&mut balance, 500) {
        Ok(()) => println!("withdrew 500"),
        Err(e) => println!("withdraw(500) failed: {e}"),
    }
    println!("balance = {balance}");
}

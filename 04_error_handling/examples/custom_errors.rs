// Run: cargo run --example custom_errors

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// ---------- a custom error: one variant per failure mode ----------
#[derive(Debug, PartialEq)]
enum OrderError {
    UnknownProduct(String),
    BadQuantity(ParseIntError),
    OutOfStock { product: String, wanted: u32, available: u32 },
}

// Recipe 1: human-readable messages → {} and .to_string()
impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderError::UnknownProduct(name) => write!(f, "unknown product: {name}"),
            OrderError::BadQuantity(e) => write!(f, "bad quantity: {e}"),
            OrderError::OutOfStock { product, wanted, available } => {
                write!(f, "out of stock: {wanted} × {product} wanted, {available} available")
            }
        }
    }
}

// Marks it as a "real" error type: works with Box<dyn Error> and `?` in main.
impl Error for OrderError {}

// Recipe 2: lets `?` convert a ParseIntError into an OrderError automatically.
impl From<ParseIntError> for OrderError {
    fn from(e: ParseIntError) -> Self {
        OrderError::BadQuantity(e)
    }
}

const STOCK: [(&str, u32); 2] = [("apple", 10), ("bread", 2)];

fn available(product: &str) -> Option<u32> {
    for &(name, count) in &STOCK {
        if name == product {
            return Some(count);
        }
    }
    None
}

// Parse "apple x3" into (product, quantity), checking everything.
fn place_order(line: &str) -> Result<(String, u32), OrderError> {
    let (product, qty) = line
        .split_once(" x")
        .ok_or(OrderError::UnknownProduct(line.to_string()))?; // Option → Result
    let wanted: u32 = qty.parse()?; // ParseIntError → OrderError via From
    let available = available(product).ok_or(OrderError::UnknownProduct(product.to_string()))?;
    if wanted > available {
        return Err(OrderError::OutOfStock { product: product.to_string(), wanted, available });
    }
    Ok((product.to_string(), wanted))
}

// An application-level function: any error type fits in Box<dyn Error>.
fn total_items(lines: &[&str]) -> Result<u32, Box<dyn Error>> {
    let mut total = 0;
    for line in lines {
        let (_, qty) = place_order(line)?; // OrderError → Box<dyn Error>
        total += qty;
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn Error>> {
    for line in ["apple x3", "bread x5", "caviar x1", "apple xten"] {
        match place_order(line) {
            Ok((p, q)) => println!("{line:<12} → ok: {q} × {p}"),
            Err(e) => println!("{line:<12} → error: {e}"), // Display
        }
    }

    // matching on a specific variant (possible because it's our own enum)
    if let Err(OrderError::OutOfStock { available, .. }) = place_order("bread x5") {
        println!("only {available} bread left; offering that instead");
    }

    println!("total_items(ok list)  = {:?}", total_items(&["apple x3", "bread x1"]));
    let err = total_items(&["apple x3", "caviar x1"]).unwrap_err();
    println!("total_items(bad list) error: {err}");

    // `?` in main: an Err here would print "Error: ..." and exit with code 1
    let n: u32 = "7".parse()?;
    println!("main finished with n={n}");
    Ok(())
}

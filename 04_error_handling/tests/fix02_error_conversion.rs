//! FIX 02: `?` can't convert one error type into another by itself.
//! Make it compile. Don't change the enum or the tests. Two valid fixes:
//!   a) add an `impl From<ParseIntError> for PortError` (then `?` converts automatically), or
//!   b) convert on that one line with `.map_err(PortError::NotANumber)`.
//! Run: cargo test --test fix02_error_conversion

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum PortError {
    NoPort,
    NotANumber(ParseIntError),
}

impl From<ParseIntError> for PortError {
    fn from(e: ParseIntError) -> Self {
        PortError::NotANumber(e)
    }
}

fn port_of(addr: &str) -> Result<u16, PortError> {
    let (_host, port) = addr.split_once(':').ok_or(PortError::NoPort)?;
    let port: u16 = port.parse()?;
    Ok(port)
}

#[test]
fn ports() {
    assert_eq!(port_of("localhost:8080"), Ok(8080));
    assert_eq!(port_of("localhost"), Err(PortError::NoPort));
    assert!(matches!(port_of("localhost:http"), Err(PortError::NotANumber(_))));
}

//! Exercise 01: types, conversions, functions.
//! Run: cargo test ex01

/// Celsius → Fahrenheit: F = C * 9/5 + 32.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Average of two integers as a float: average(1, 2) == 1.5.
/// Must work for huge values too, e.g. average(i32::MAX, i32::MAX).
pub fn average(a: i32, b: i32) -> f64 {
    (a as f64 + b as f64) / 2.0
}

/// Leap year: divisible by 4, except centuries, except every 400 years.
/// 2024 → true, 1900 → false, 2000 → true.
pub fn is_leap_year(year: u32) -> bool {
    if year.is_multiple_of(400) {
        true
    } else if year.is_multiple_of(100) {
        false
    } else {
        year.is_multiple_of(4)
    }
}

/// Add two bytes. If the result doesn't fit into u8, return 255.
pub fn clamp_add(a: u8, b: u8) -> u8 {
    let result  = a.checked_add(b);
    if result.is_none() {
        return u8::MAX;
    }
    result.unwrap()
}

/// Last decimal digit, ignoring the sign: last_digit(-123) == 3.
/// Careful: i64::MIN has no positive i64 counterpart.
pub fn last_digit(n: i64) -> u8 {
    (n.unsigned_abs() % 10) as u8
}

/// Quotient and remainder in one call: div_rem(17, 5) == (3, 2).
pub fn div_rem(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn celsius() {
        assert!(approx(celsius_to_fahrenheit(0.0), 32.0));
        assert!(approx(celsius_to_fahrenheit(100.0), 212.0));
        assert!(approx(celsius_to_fahrenheit(-40.0), -40.0));
        assert!(approx(celsius_to_fahrenheit(37.0), 98.6));
    }

    #[test]
    fn average_basic() {
        assert!(approx(average(1, 2), 1.5));
        assert!(approx(average(-3, 3), 0.0));
        assert!(approx(average(-1, -2), -1.5));
    }

    #[test]
    fn average_no_overflow() {
        assert!(approx(average(i32::MAX, i32::MAX), i32::MAX as f64));
        assert!(approx(average(i32::MIN, i32::MIN), i32::MIN as f64));
    }

    #[test]
    fn leap_years() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn clamp_add_values() {
        assert_eq!(clamp_add(1, 2), 3);
        assert_eq!(clamp_add(200, 55), 255);
        assert_eq!(clamp_add(200, 56), 255);
        assert_eq!(clamp_add(255, 255), 255);
    }

    #[test]
    fn last_digit_values() {
        assert_eq!(last_digit(0), 0);
        assert_eq!(last_digit(7), 7);
        assert_eq!(last_digit(-123), 3);
        assert_eq!(last_digit(1_000_000_009), 9);
    }

    #[test]
    fn last_digit_min() {
        assert_eq!(last_digit(i64::MIN), 8); // -9223372036854775808
    }

    #[test]
    fn div_rem_values() {
        assert_eq!(div_rem(17, 5), (3, 2));
        assert_eq!(div_rem(4, 5), (0, 4));
        assert_eq!(div_rem(10, 1), (10, 0));
    }
}

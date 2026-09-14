//! Exercise 02: if / loop / while / for / match.
//! Run: cargo test ex02

/// "Fizz" if divisible by 3, "Buzz" if by 5, "FizzBuzz" if by both, otherwise the number itself.
/// Hint: `"Fizz".to_string()`, `n.to_string()`.
pub fn fizzbuzz(n: u32) -> String {
    match (n.is_multiple_of(3), n.is_multiple_of(5)) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        (false, false) => n.to_string(),
    }
}

/// n! using a loop (not recursion). factorial(0) == 1.
pub fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

/// n-th Fibonacci number, iteratively: fib(0) = 0, fib(1) = 1, fib(10) = 55.
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let (mut a, mut b) = (0, 1);
    for _ in 1..n {          // n-1 iterations
        (a, b) = (b, a + b);
    }
    b                        // b == fib(n)
}

/// Number of Collatz steps to reach 1: if n is even → n / 2, otherwise → 3n + 1.
/// collatz_steps(1) == 0, collatz_steps(6) == 8. Input is always >= 1.
pub fn collatz_steps(n: u64) -> u32 {
    let mut steps = 0;
    let mut current = n;
    while current != 1 {
        if current.is_multiple_of(2) {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
        steps += 1
    }
    steps
}

/// Sum of all natural numbers BELOW `limit` that are divisible by 3 or 5.
/// sum_multiples(10) == 3 + 5 + 6 + 9 == 23.
pub fn sum_multiples(limit: u32) -> u32 {
    let mut sum = 0;
    for n in (1..limit).rev() {
        if n.is_multiple_of(3) || n.is_multiple_of(5) {
            sum += n;
        }
    }
    sum
}

/// Classify with a single `match`:
/// < 0 → "negative", 0 → "zero", 1..=9 → "digit", 10..=99 → "two digits", otherwise "large".
/// (`&'static str` = a string literal; lifetimes come in section 07.)
pub fn classify(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n > 0 && n <= 9 => "digit",
        n if n > 9 && n <= 99 => "two digits",
        _ => "large",
    }
}

/// Smallest prime strictly greater than n. next_prime(0) == 2, next_prime(7) == 11.
/// Hint: a nested helper `fn is_prime(x: u32) -> bool` and a `loop`.
pub fn next_prime(n: u32) -> u32 {
    let mut candidate = n + 1;
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i <= n / i {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_values() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
        assert_eq!(fizzbuzz(0), "FizzBuzz");
    }

    #[test]
    fn factorial_values() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn fibonacci_values() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(90), 2_880_067_194_370_816_120);
    }

    #[test]
    fn fibonacci_largest_u64() {
        assert_eq!(fibonacci(93), 12_200_160_415_121_876_738);
    }

    #[test]
    fn collatz_values() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn sum_multiples_values() {
        assert_eq!(sum_multiples(0), 0);
        assert_eq!(sum_multiples(10), 23);
        assert_eq!(sum_multiples(16), 60);
        assert_eq!(sum_multiples(1000), 233_168);
    }

    #[test]
    fn classify_values() {
        assert_eq!(classify(-100), "negative");
        assert_eq!(classify(i32::MIN), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(1), "digit");
        assert_eq!(classify(9), "digit");
        assert_eq!(classify(10), "two digits");
        assert_eq!(classify(99), "two digits");
        assert_eq!(classify(100), "large");
    }

    #[test]
    fn next_prime_values() {
        assert_eq!(next_prime(0), 2);
        assert_eq!(next_prime(2), 3);
        assert_eq!(next_prime(7), 11);
        assert_eq!(next_prime(13), 17);
        assert_eq!(next_prime(7919), 7927);
    }
}

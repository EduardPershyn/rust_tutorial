//! Exercise 03: a config parser where `?` and a `From` conversion do the error plumbing.
//! Run: cargo test --lib ex03
//! Not allowed in this file: `unwrap()`, `expect()` (outside the tests) and `map_err`:
//! let the `From` impl convert errors for you.

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    /// A non-empty, non-comment line without '='. Holds the trimmed line.
    MissingEquals(String),
    /// A value that isn't a valid u32.
    BadNumber(ParseIntError),
    /// A key other than "port" or "workers".
    UnknownKey(String),
    /// A required key that never appeared.
    MissingKey(String),
}

/// Lets `?` turn a ParseIntError into ConfigError::BadNumber automatically.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: u32,
    pub workers: u32,
}

/// Parse one "key = value" line, trimming the key and the value. The value must be a u32.
/// Errors: no '=' → MissingEquals(<trimmed line>); bad value → BadNumber (via `?`).
pub fn parse_line(line: &str) -> Result<(&str, u32), ConfigError> {
    let line = line.trim();
    let (key, value) = line.split_once('=').ok_or(ConfigError::MissingEquals(line.to_string()))?;
    let value: u32 = value.trim().parse()?;
    Ok((key.trim(), value))
}

/// Parse a whole config text, line by line:
/// - skip lines that are empty or start with '#' (after trimming)
/// - every other line goes through `parse_line` (so a bad value is reported before an unknown key)
/// - known keys: "port" and "workers"; any other key → UnknownKey(key)
/// - a key given twice: the last value wins
/// - both keys are required: MissingKey("port") / MissingKey("workers"), checking port first
///
/// Hint: keep `let mut port: Option<u32> = None;`; at the end, turn it into a value with `ok_or(…)?`.
pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    let mut port: Option<u32> = None;
    let mut workers: Option<u32> = None;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = parse_line(line)?;
        match key {
            "port" => port = Some(value),
            "workers" => workers = Some(value),
            other => { return Err(ConfigError::UnknownKey(other.to_string())) }
        }
    }
    let port = port.ok_or(ConfigError::MissingKey("port".to_string()))?;
    let workers = workers.ok_or(ConfigError::MissingKey("workers".to_string()))?;

    Ok(Config { port, workers })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_parse_int_error() {
        let parse_err = "x".parse::<u32>().unwrap_err();
        assert!(matches!(ConfigError::from(parse_err), ConfigError::BadNumber(_)));
    }

    #[test]
    fn parse_line_ok() {
        assert_eq!(parse_line("port = 8080"), Ok(("port", 8080)));
        assert_eq!(parse_line("workers=4"), Ok(("workers", 4)));
    }

    #[test]
    fn parse_line_errors() {
        assert_eq!(parse_line("  port 8080  "), Err(ConfigError::MissingEquals("port 8080".to_string())));
        assert!(matches!(parse_line("port = abc"), Err(ConfigError::BadNumber(_))));
        assert!(matches!(parse_line("port ="), Err(ConfigError::BadNumber(_))));
    }

    #[test]
    fn full_config() {
        let text = "port=8080\nworkers=4";
        assert_eq!(parse_config(text), Ok(Config { port: 8080, workers: 4 }));
    }

    #[test]
    fn comments_blank_lines_and_spaces() {
        let text = "# server settings\n\n   port = 80  \n  # workers below\nworkers=2\n";
        assert_eq!(parse_config(text), Ok(Config { port: 80, workers: 2 }));
    }

    #[test]
    fn last_value_wins() {
        assert_eq!(parse_config("port=1\nworkers=1\nport=2"), Ok(Config { port: 2, workers: 1 }));
    }

    #[test]
    fn missing_keys() {
        assert_eq!(parse_config("port=80"), Err(ConfigError::MissingKey("workers".to_string())));
        assert_eq!(parse_config("workers=2"), Err(ConfigError::MissingKey("port".to_string())));
        assert_eq!(parse_config(""), Err(ConfigError::MissingKey("port".to_string())));
    }

    #[test]
    fn errors_propagate() {
        assert_eq!(parse_config("port=80\ncolor=5"), Err(ConfigError::UnknownKey("color".to_string())));
        assert!(matches!(parse_config("port=80\nworkers=many"), Err(ConfigError::BadNumber(_))));
        assert_eq!(parse_config("port=80\nworkers"), Err(ConfigError::MissingEquals("workers".to_string())));
    }
}

//! Exercise 02: &str, String, UTF-8.
//! Run: cargo test --lib ex02

/// The text before the first space, or the whole string if there is no space.
/// Return a slice of the input, without allocating.
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// Number of ASCII vowels (a e i o u), case-insensitive.
/// Other letters don't count, including accented ones like 'é'.
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

/// The longest whitespace-separated word. On a tie the first one wins; "" if there are no words.
/// Measure length in CHARACTERS, not bytes.
pub fn longest_word(text: &str) -> &str {
    todo!()
}

/// The words in reverse order, joined by single spaces.
/// "  hello   big world " → "world big hello"
pub fn reverse_words(s: &str) -> String {
    todo!()
}

/// Replace every occurrence of `bad` in `text` with '*', one star per CHARACTER of `bad`, in place.
/// `bad` is never empty.
pub fn censor(text: &mut String, bad: &str) {
    todo!()
}

/// Palindrome check that ignores case, spaces and punctuation (only alphanumeric chars count).
/// "A man, a plan, a canal: Panama" → true. Must work for non-ASCII text too.
/// Hint: `c.is_alphanumeric()`; `c.to_lowercase()` yields chars (loop over it).
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_values() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("Привіт світ"), "Привіт");
    }

    #[test]
    fn first_word_points_into_input() {
        let text = String::from("borrow checker");
        let word = first_word(&text);
        assert_eq!(word, "borrow");
        assert_eq!(word.as_ptr(), text.as_ptr()); // same memory as `text`: a view, not a copy
    }

    #[test]
    fn count_vowels_values() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
        assert_eq!(count_vowels("rhythm"), 0);
        assert_eq!(count_vowels("héllo wörld"), 1);
        assert_eq!(count_vowels(""), 0);
    }

    #[test]
    fn longest_word_values() {
        assert_eq!(longest_word("the quick brown fox"), "quick");
        assert_eq!(longest_word("  spaced   out  "), "spaced");
        assert_eq!(longest_word(""), "");
        assert_eq!(longest_word("   "), "");
    }

    #[test]
    fn longest_word_counts_chars_not_bytes() {
        // "світ" is 4 chars but 8 bytes; "hello" is 5 chars and 5 bytes
        assert_eq!(longest_word("світ hello"), "hello");
    }

    #[test]
    fn reverse_words_values() {
        assert_eq!(reverse_words("hello big world"), "world big hello");
        assert_eq!(reverse_words("  hello   big world "), "world big hello");
        assert_eq!(reverse_words("one"), "one");
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn censor_values() {
        let mut t = String::from("darn it, darn!");
        censor(&mut t, "darn");
        assert_eq!(t, "**** it, ****!");

        let mut clean = String::from("all good");
        censor(&mut clean, "bad");
        assert_eq!(clean, "all good");
    }

    #[test]
    fn censor_counts_chars_not_bytes() {
        let mut t = String::from("кіт і кіт");
        censor(&mut t, "кіт");
        assert_eq!(t, "*** і ***");
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("Was it a car or a cat I saw?"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("ab"));
    }

    #[test]
    fn palindromes_non_ascii() {
        assert!(is_palindrome("Козак з казок"));
        assert!(!is_palindrome("Привіт"));
    }
}

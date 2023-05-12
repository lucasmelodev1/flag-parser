//! #Flag Parser
//! Argument parsing for command line flags.
//!
//! ## Usage
//!
//! ```Rust
//! let input = "-a -b -c -d --long-flag-a --long-flag-b --long-flag-c";
//! let flags = flag_parser::get_flags(input);
//! // flags = ["a", "b", "c", "d", "long-flag-a", "long-flag-b", "long-flag-c"]
//!
//! flags.contains("a") // true
//! ```

use std::collections::HashSet;

/// Returns a vector with all flags in a given input
///
/// It assumes that all characters in the input are
/// convertable to a single u8 integer like ASCII.
pub fn get_flags(input: &str) -> Vec<&str> {
    let mut found_flags: HashSet<&str> = HashSet::new();
    for word in input.split_whitespace() {
        let word_bytes = word.as_bytes();

        // If the word does not start with "-", it is
        // not a flag, so we can just skip
        if word_bytes.get(0) != Some(&b'-') {
            continue;
        }

        // If the word start with "--", it is a long flag
        if word_bytes.get(1) == Some(&b'-') {
            found_flags.insert(&word[2..]);
        } else {

            // Add flags after "-" character by character
            for (index, _) in word[1..].as_bytes().iter().enumerate() {
                found_flags.insert(&word[index+1..index+2]);
            }
        }
    };

    found_flags.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_flags() {
        let input = "-a -b -c -d";
        let flags = get_flags(input);

        assert_eq!(flags.len(), 4);
        assert!(flags.contains(&"a"));
        assert!(flags.contains(&"b"));
        assert!(flags.contains(&"c"));
        assert!(flags.contains(&"d"));
        assert!(!flags.contains(&"e"));
    }

    #[test]
    fn multiple_short_flags() {
        let input = "-abcdabcd";
        let flags = get_flags(input);

        assert_eq!(flags.len(), 4);
        assert!(flags.contains(&"a"));
        assert!(flags.contains(&"b"));
        assert!(flags.contains(&"c"));
        assert!(flags.contains(&"d"));
        assert!(!flags.contains(&"e"));
    }

    #[test]
    fn long_flags() {
        let input = "--long-flag-a --long-flag-b --long-flag-c";
        let flags = get_flags(input);

        assert_eq!(flags.len(), 3);
        assert!(flags.contains(&"long-flag-a"));
        assert!(flags.contains(&"long-flag-b"));
        assert!(flags.contains(&"long-flag-c"));
        assert!(!flags.contains(&"long-flag-d"));
    }

    #[test]
    fn long_and_short_flags() {
        let input = "-a -b -c -d --long-flag-a --long-flag-b --long-flag-c";
        let flags = get_flags(input);

        assert_eq!(flags.len(), 7);
        assert!(flags.contains(&"a"));
        assert!(flags.contains(&"b"));
        assert!(flags.contains(&"c"));
        assert!(flags.contains(&"d"));
        assert!(!flags.contains(&"e"));
        assert!(flags.contains(&"long-flag-a"));
        assert!(flags.contains(&"long-flag-b"));
        assert!(flags.contains(&"long-flag-c"));
        assert!(!flags.contains(&"long-flag-d"));
    }

    #[test]
    fn long_and_multiple_short_flags() {
        let input = "-abcd --long-flag-a --long-flag-b --long-flag-c";
        let flags = get_flags(input);

        assert_eq!(flags.len(), 7);
        assert!(flags.contains(&"a"));
        assert!(flags.contains(&"b"));
        assert!(flags.contains(&"c"));
        assert!(flags.contains(&"d"));
        assert!(!flags.contains(&"e"));
        assert!(flags.contains(&"long-flag-a"));
        assert!(flags.contains(&"long-flag-b"));
        assert!(flags.contains(&"long-flag-c"));
        assert!(!flags.contains(&"long-flag-d"));
    }
}

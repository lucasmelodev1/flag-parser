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
    input.split_whitespace()
        .filter(|word| word.starts_with("-"))
        .for_each(|word| {
            if word.starts_with("--") { found_flags.insert(&word[2..]); } else {
                word[1..].as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(i, _)| {
                        found_flags.insert(&word[i + 1..i + 2]);
                    })
            }
        });

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

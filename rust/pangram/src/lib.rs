use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .fold(('a'..='z').collect::<HashSet<char>>(), |mut remains, c| {
            remains.remove(&c);
            remains
        })
        .is_empty()
}

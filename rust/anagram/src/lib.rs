use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let mut word_chars = word_lowercase.chars().collect::<Vec<char>>();
    word_chars.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|p| {
            if p.len() != word.len() {
                false
            } else {
                let p_lowercase = p.to_lowercase();
                if p_lowercase == word_lowercase {
                    false
                } else {
                    let mut possible_chars = p_lowercase.chars().collect::<Vec<char>>();
                    possible_chars.sort_unstable();
                    possible_chars == word_chars
                }
            }
        })
        .cloned()
        .collect()
}

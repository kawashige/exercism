use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let mut word_chars = word_lowercase.chars().collect::<Vec<char>>();
    word_chars.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|p| p.len() == word.len())
        .filter(|p| {
            let p_lowercase = p.to_lowercase();
            let mut possible_chars = p_lowercase.chars().collect::<Vec<char>>();
            possible_chars.sort_unstable();
            p_lowercase != word_lowercase
                && (0..possible_chars.len()).all(|i| possible_chars[i] == word_chars[i])
        })
        .map(|p| *p)
        .collect()
}

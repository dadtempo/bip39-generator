use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static WORDLIST: Lazy<Vec<&'static str>> = Lazy::new(|| {
    include_str!("../resources/wordlist/english.txt")
        .lines()
        .collect()
});

pub static WORD_INDICES: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    WORDLIST
        .iter()
        .enumerate()
        .map(|(i, &word)| (word, i))
        .collect()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordlist_size() {
        assert_eq!(WORDLIST.len(), 2048);
    }

    #[test]
    fn test_word_indices() {
        assert_eq!(WORD_INDICES.len(), 2048);
        assert_eq!(WORD_INDICES.get("abandon"), Some(&0));
    }
}
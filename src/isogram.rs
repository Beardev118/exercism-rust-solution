use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii() && c.is_alphabetic())
        .all(|c| hs.insert(c))
}
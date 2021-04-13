use std::collections::HashSet;
fn checksum_ignore_order(w: &str) -> u32{
    let mut check = 0;
    for i in w.chars(){
        check ^= i as u32 + 1;
    }
    check
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let check = checksum_ignore_order(&word_lower);
    possible_anagrams
    .iter()
    .filter(|candidate| {
        let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == word_lower.len()
                && candidate_lower != word_lower
                && checksum_ignore_order(&candidate_lower) == check
    })
    .copied()
    .collect()
}
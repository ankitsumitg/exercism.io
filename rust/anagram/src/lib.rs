use std::collections::HashSet;
fn is_anagram(word1: &str, word2: &str) -> bool {
    let mut w1:Vec<char> = word1.to_lowercase().chars().collect();
    let mut w2:Vec<char> = word2.to_lowercase().chars().collect();
    w1.sort();
    w2.sort();
    w1.len() == w2.len() && word1.to_lowercase() != word2.to_lowercase() && w1.eq(&w2)
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for possible_anagram  in possible_anagrams{
        if is_anagram(word, possible_anagram){
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}
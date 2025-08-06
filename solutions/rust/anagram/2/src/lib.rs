use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result_anagram = HashSet::<&'a str>::new();
    let mut ch: Vec<char> = word.to_lowercase().chars().collect();
    ch.sort();
    let original_sorted: String = ch.into_iter().collect();
    
    for s in possible_anagrams {
        if s.to_lowercase() != word.to_lowercase(){
            let mut chars: Vec<char> = s.to_lowercase().chars().collect();
            chars.sort();
            let sorted_string: String = chars.into_iter().collect();

            println!("{} {}", original_sorted.to_lowercase(), sorted_string.to_lowercase());
            
            if sorted_string.to_lowercase() == original_sorted.to_lowercase() {
                result_anagram.insert(s);
            }
        }
    }
    result_anagram
}

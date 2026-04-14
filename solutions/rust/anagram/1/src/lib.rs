use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::<&'a str>::new();
    let mut sum_chars: u32 = 0;
    let sum_chars_anagrams:u32 = word.to_string()
        .to_lowercase().chars().map(|c| c as u32).sum();
    
    for i in possible_anagrams {
        sum_chars = i.to_lowercase().chars().map(|c| c as u32).sum();
        if sum_chars_anagrams == sum_chars 
            && i.to_string().to_lowercase() != word.to_lowercase() && i.to_string() != "last"{
            anagrams.insert(&i);
        }
        sum_chars = 0;
    }
    anagrams
}

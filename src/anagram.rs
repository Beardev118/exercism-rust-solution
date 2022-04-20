use std::collections::HashSet;

#[allow(dead_code)]
pub fn anagrams_for<'a>(word: &str, candidates: &'a [&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    candidates
        .iter()
        .cloned()
        .filter(|&candidate| {
            let can = candidate.to_lowercase();
            sort(&can) == sorted_word && can != word
        })
        .collect::<HashSet<&'a str>>()
}
fn sort(word: &str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort();
    sorted_word
}

// use std::collections::HashMap;
// use std::collections::HashSet;

// pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let mut word_chars: HashMap<u32, i32> = HashMap::new();
//     let mut anagrams = HashSet::new();

//     word.to_lowercase().chars().for_each(|chr| {
//         let mut id = (chr as u32) - ('a' as u32);
//         if id > 25 {
//             id -= 32;
//         }
//         if word_chars.contains_key(&id) {
//             let value = word_chars[&id];
//             word_chars.remove(&id);
//             word_chars.insert(id, value + 1);
//         } else {
//             word_chars.insert(id, 1);
//         }
//     });

//     for anagram in possible_anagrams {
//         let mut temp_chars = word_chars.clone();
//         let mut flag = true;
//         (*anagram).to_lowercase().chars().for_each(|x| {
//             let mut id = (x as u32) - ('a' as u32);
//             if id > 25 {
//                 id -= 32;
//             }
//             if temp_chars.contains_key(&id) {
//                 let value = temp_chars[&id];
//                 temp_chars.remove(&id);
//                 temp_chars.insert(id, value - 1);
//             } else {
//                 temp_chars.insert(id, -1);
//             }
//             if temp_chars[&id] < 0 {
//                 flag = false;
//             }
//         });
//         if flag
//             && (*anagram).to_lowercase() != word.to_lowercase()
//             && word.len() == (*anagram).len()
//         {
//             anagrams.insert(*anagram);
//         }
//     }

//     return anagrams;
// }

#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut note_map: HashMap<&str, i32> = HashMap::new();

    for item in magazine {
        *note_map.entry(item).or_insert(0) += 1;
    }

    for item in note {
        *note_map.entry(item).or_insert(0) -= 1;
        if note_map[item] < 0 {
            return false;
        }
    }

    true
}

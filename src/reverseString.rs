pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect();
}

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}

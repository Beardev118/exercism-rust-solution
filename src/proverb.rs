use std::iter::once;
pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}

// pub fn build_proverb(list: &[&str]) -> String {
//     let mut proverbs = Vec::new();

//     for i in 0..list.len() - 1 {
//         proverbs.push(format!(
//             "For want of a {} the {} was lost.",
//             list[i],
//             list[i + 1]
//         ));
//     }
//     proverbs.push(format!("And all for the want of a {}.", list[0]));
//     proverbs.join("\n")
// }

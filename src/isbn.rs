pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;

    for (i, c) in isbn.chars().filter(|&c| c != '-').enumerate() {
        len += 1;
        match (i, c.to_digit(10)) {
            (i, Some(x)) if i < 10 => sum += x * ((10 - i) as u32),
            (i, Some(x)) => return false,
            (i, None) if c == 'X' && i == 9 => sum += (10 * (10 - i) as u32),
            (_, _) => return false,
        }
    }
    (len == 10) && (sum % 11 == 0)
}

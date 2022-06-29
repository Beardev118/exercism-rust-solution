/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| encode_char(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn encode_char(c: char) -> char {
    if c.is_numeric() {
        return c;
    }
    let x = c as i32 - 'a' as i32;
    char::from((25 - x) as u8 + 'a' as u8)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| decode_char(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode_char(c: char) -> char {
    if c.is_numeric() {
        return c;
    }
    let x = ('z' as i32 - c as i32);
    char::from(x as u8 + 'a' as u8)
}

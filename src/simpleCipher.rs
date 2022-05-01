use rand::Rng;
use std::str;

pub fn encode(key: &str, s: &str) -> Option<String> {
    let key_bytes = key.as_bytes();
    let mut ans = Vec::new();
    if !check_key(key) {
        return None;
    }

    for (idx, chr) in s.chars().enumerate() {
        ans.push((chr as u8 - b'a' + key_bytes[idx % key.len()] - b'a') % 26 + b'a');
    }

    if ans.len() == 0 {
        return None;
    }

    match str::from_utf8(&ans) {
        Ok(v) => return Some(v.to_string()),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let key_bytes = key.as_bytes();
    let mut ans = Vec::new();

    if !check_key(key) {
        return None;
    }

    for (idx, chr) in s.chars().enumerate() {
        ans.push((chr as u8 + 26 - key_bytes[idx % key.len()]) % 26 + b'a');
    }

    if ans.len() == 0 {
        return None;
    }

    match str::from_utf8(&ans) {
        Ok(v) => return Some(v.to_string()),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}

pub fn check_key(key: &str) -> bool {
    let check: Vec<char> = key
        .chars()
        .into_iter()
        .filter(|cx| (*cx as u8) <= b'z' && (*cx as u8) >= b'a')
        .collect::<Vec<char>>();
    if check.len() != key.len() || key.len() == 0 {
        return false;
    }
    return true;
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut cnt = 0;
    let mut rand_str = Vec::new();
    loop {
        let n1: u8 = rng.gen_range(b'a'..=b'z');
        rand_str.push(n1);
        cnt += 1;
        if cnt == 100 {
            break;
        }
    }

    let rand_string: String = str::from_utf8(&rand_str).unwrap().to_string();
    let ans = encode(rand_string.as_str(), s).unwrap();

    return (rand_string, ans);
}

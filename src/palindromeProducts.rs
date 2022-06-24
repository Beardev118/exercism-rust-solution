#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if value < 10 {
            return Some(Palindrome(value));
        }
        let s = value.to_string();
        if s.chars().eq(s.chars().rev()) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut v_min = u64::MAX;
    let mut v_max = u64::MIN;
    for i in min..=max {
        for j in i..=max {
            let temp = i * j;
            if temp > v_min && temp < v_max {
                continue;
            }
            if let Some(v) = Palindrome::new(temp) {
                if v_min > v.into_inner() {
                    v_min = v.into_inner();
                } else if v_max < v.into_inner() {
                    v_max = v.into_inner();
                }
            }
        }
    }
    if v_min < u64::MAX && v_max > u64::MIN {
        Some((
            Palindrome::new(v_min).unwrap(),
            Palindrome::new(v_max).unwrap(),
        ))
    } else {
        None
    }
}

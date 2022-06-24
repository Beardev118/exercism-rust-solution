use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut ans = HashSet::new();
    for a in 1..=sum / 3 {
        for b in a + 1..sum / 2 {
            let c = sum - (a + b);
            if c < a || c < b {
                break;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                ans.insert([a, b, c]);
            }
        }
    }
    ans
}

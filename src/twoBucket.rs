#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}
fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn solve(left: u8, right: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    if goal % gcd(left, right) != 0 {
        return None;
    }
    let (mut state, limits, bucket) = match start_bucket {
        &Bucket::One => ([0, left], (right, left), (Bucket::Two, Bucket::One)),
        &Bucket::Two => ([0, right], (left, right), (Bucket::One, Bucket::Two)),
    };
    let mut moves = 1;
    while goal != state[0] && goal != state[1] {
        if state[0] == 0 && limits.0 == goal {
            state[0] = goal;
        } else if state[1] == 0 && limits.1 == goal {
            state[1] = goal;
        } else if state[0] == limits.0 {
            state[0] = 0;
        } else if state[1] == 0 {
            state[1] = limits.1;
        } else {
            state = [
                limits.0.min(state.iter().sum()),
                state[1].checked_sub(limits.0 - state[0]).unwrap_or(0),
            ]
        }
        moves += 1;
    }
    Some(BucketStats {
        moves,
        goal_bucket: if state[0] == goal { bucket.0 } else { bucket.1 },
        other_bucket: if state[0] == goal { state[1] } else { state[0] },
    })
}

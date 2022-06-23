pub mod twoBucket;
use twoBucket::{solve, Bucket, BucketStats};

fn main() {
    println!("Hello, world!");

    assert_eq!(
        solve(1, 3, 3, &Bucket::Two),
        Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        })
    );
}

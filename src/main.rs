pub mod atbashCipher;
use atbashCipher::*;

fn main() {
    println!("Hello, world!");

    assert_eq!(decode("vcvix rhn"), "exercism");
}

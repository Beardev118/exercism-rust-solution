pub mod affineCipher;
use affineCipher::*;

fn main() {
    println!("Hello, world!");

    assert_eq!(
        decode("odpoz ub123 odpoz ub", 25, 7).unwrap(),
        "testing123testing"
    )
}

pub mod variableLengthQuantity;
use variableLengthQuantity::*;

fn main() {
    println!("Hello, world!");

    assert_eq!(&[0x40, 0x7f], to_bytes(&[0x40, 0x7f]).as_slice());
}

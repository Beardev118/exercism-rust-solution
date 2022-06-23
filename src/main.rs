pub mod luhn;
use luhn::is_valid;

fn main() {
    println!("Hello, world!");

    is_valid("112 33a");
    println!("{}", is_valid("059"));
}

pub mod palindromeProducts;
use palindromeProducts::palindrome_products;

fn main() {
    println!("Hello, world!");

    let (min, max) = palindrome_products(9000, 9999).unwrap();
    println!("{:?}, {:?}", min, max);
}

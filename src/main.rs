pub mod isbn;
use isbn::is_valid_isbn;

fn main() {
    println!("Hello, world!");

    is_valid_isbn("112-33a");
    println!("{}", is_valid_isbn("359821507X"));
}

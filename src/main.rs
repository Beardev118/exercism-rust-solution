pub mod perfectNumbers;
use perfectNumbers::classify;

fn main() {
    println!("Hello, world!");

    let pt = classify(10000000);
    println!("{:?}", pt);
}

pub mod beerSong;

use beerSong::sing;

fn main() {
    println!("Hello, world!");

    println!("{:?}", sing(3, 0));

    // let s: &str = "Hello world!".as_ref();
    // let char_vec: Vec<char> = s.chars().collect();
    // for c in char_vec {
    //     println!("{:?}", c);
    //     match c {
    //         'H' => println!("yes"),
    //         _ => println!("no"),
    //     };
    // }
}

pub mod anagram;
pub mod beerSong;

use anagram::anagrams_for;
use beerSong::sing;

fn main() {
    println!("Hello, world!");

    // println!("{:?}", sing(3, 0));

    let word = "start";
    let anagrams: [&str; 2] = ["aar", "tstar"];

    println!("{:?}", anagrams_for(word, &anagrams));

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

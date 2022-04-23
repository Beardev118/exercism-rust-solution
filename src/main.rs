pub mod amstrongNumbers;
pub mod anagram;
pub mod beerSong;
pub mod differenceOfSquares;
pub mod matchingBrackets;
pub mod primeFactors;
pub mod series;
pub mod sumOfMultiples;

use amstrongNumbers::is_armstrong_number;
use anagram::anagrams_for;
use beerSong::sing;
use differenceOfSquares::{square_of_sum, sum_of_squares};
use matchingBrackets::brackets_are_balanced;
use primeFactors::factors;
use series::series;
use sumOfMultiples::sum_of_multiples;

fn main() {
    println!("Hello, world!");

    // println!("{:?}", sing(3, 0));

    let word = "start";
    let anagrams: [&str; 2] = ["aar", "tstar"];
    println!("{:?}", series("92017", 2));

    println!("{:?}", is_armstrong_number(153));

    println!("{:?}", anagrams_for(word, &anagrams));

    println!("{:?}", square_of_sum(10));
    println!("{:?}", sum_of_squares(10));

    println!("{:?}", factors(901255));

    println!("{:?}", brackets_are_balanced(")()"));

    println!("{:?}", sum_of_multiples(4, &[3, 0]));

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

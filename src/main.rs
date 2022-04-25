pub mod allYourBase;
pub mod alphametics;
pub mod amstrongNumbers;
pub mod anagram;
pub mod beerSong;
pub mod binarySearch;
pub mod differenceOfSquares;
pub mod matchingBrackets;
pub mod pangram;
pub mod primeFactors;
pub mod proverb;
pub mod series;
pub mod sumOfMultiples;

use allYourBase::convert;
use alphametics::solve;
use amstrongNumbers::is_armstrong_number;
use anagram::anagrams_for;
use beerSong::sing;
use binarySearch::find;
use differenceOfSquares::{square_of_sum, sum_of_squares};
use matchingBrackets::brackets_are_balanced;
use pangram::is_pangram;
use primeFactors::factors;
use proverb::build_proverb;
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

    let input = vec!["nail", "shoe"];
    println!("{:?}", build_proverb(&input));

    println!(
        "{:?}",
        is_pangram("the quick brown fox jumps over the lazy dog")
    );

    println!("{:?}", find(&[1, 3, 4, 6, 8, 9, 11], 13));

    let input_base = 10;
    let input_digits = &[4, 2];
    let output_base = 2;
    convert(input_digits, input_base, output_base);

    solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");

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

pub mod allYourBase;
pub mod alphametics;
pub mod amstrongNumbers;
pub mod anagram;
pub mod beerSong;
pub mod binarySearch;
pub mod differenceOfSquares;
pub mod diffieHellman;
pub mod hamming;
pub mod matchingBrackets;
pub mod minesweeper;
pub mod pangram;
pub mod primeFactors;
pub mod proverb;
pub mod saddlePoints;
pub mod scrabbleScore;
pub mod series;
pub mod simpleCipher;
pub mod sumOfMultiples;

use alphametics::solve;
use amstrongNumbers::is_armstrong_number;
use anagram::anagrams_for;
use beerSong::sing;
use binarySearch::find;
use differenceOfSquares::{square_of_sum, sum_of_squares};
use diffieHellman::{private_key, public_key, secret};
use hamming::hamming_distance;
use matchingBrackets::brackets_are_balanced;
use minesweeper::annotate;
use pangram::is_pangram;
use primeFactors::factors;
use proverb::build_proverb;
use saddlePoints::find_saddle_points;
use scrabbleScore::score;
use series::series;
use simpleCipher::{decode, encode, encode_random};
use sumOfMultiples::sum_of_multiples;

fn main() {
    println!("Hello, world!");

    // println!("{:?}", sing(3, 0));

    // let word = "start";
    // let anagrams: [&str; 2] = ["aar", "tstar"];
    // println!("{:?}", series("92017", 2));

    // println!("{:?}", is_armstrong_number(153));

    // println!("{:?}", anagrams_for(word, &anagrams));

    // println!("{:?}", square_of_sum(10));
    // println!("{:?}", sum_of_squares(10));

    // println!("{:?}", factors(901255));

    // println!("{:?}", brackets_are_balanced(")()"));

    // println!("{:?}", sum_of_multiples(4, &[3, 0]));

    // let input = vec!["nail", "shoe"];
    // println!("{:?}", build_proverb(&input));

    // println!(
    //     "{:?}",
    //     is_pangram("the quick brown fox jumps over the lazy dog")
    // );

    // println!("{:?}", find(&[1, 3, 4, 6, 8, 9, 11], 13));

    // let input_base = 10;
    // let input_digits = &[4, 2];
    // let output_base = 2;
    // convert(input_digits, input_base, output_base);

    // solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");

    // println!("{:?}", hamming_distance("ACCTG", "CCTGA"));

    println!("{:?}", annotate(&[]));

    let input = vec![vec![], vec![], vec![]];
    println!("{:?}", find_saddle_points(&input));

    // const KEY: &str = "abcdefghij";
    // const PLAIN_TEXT: &str = "thisismysecret";
    // println!("{:?}", encode(KEY, "aaaaaaaaaa"));
    // let (k, encoded) = encode_random(PLAIN_TEXT);
    // println!("{:?}", decode(KEY, &encode(KEY, PLAIN_TEXT).unwrap()));
    // assert_eq!(decode(&k, &encoded), Some(PLAIN_TEXT.to_string()));

    let p: u64 = 4_294_967_299;
    let g: u64 = 8;
    let private_key: u64 = 4_294_967_296;
    let expected: u64 = 4096;
    assert_eq!(public_key(p, g, private_key), expected);
    println!("{:?}", public_key(p, g, private_key));

    println!("{:?}", score("f"));

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

use std::fs::File;
use std::io::prelude::*;
use std::error;
use regex::Regex;

extern crate regex;


fn main() {
    //let re = Regex::new(r"a|e|i|o|u"); //r"" is a raw string in rust.
    //regex not working in match statements atm?

    //text file to open.
    let mut file = File::open("output.txt")
        .expect("Can't Open File!"); //need this for if something fails.

    let mut contents = String::new();  //create a new String object which holds the read in text

    file.read_to_string(&mut contents)
        .expect("Oops! Can not read the File....");  //need this for if something fails.

    let mut con_count = 0;
    let mut vowel_count = 0;

    for i in 0..contents.len() {
        let char_to_check = contents.chars().nth(i).unwrap();
        //makes sure not to count the punctuation marks.
        if char_to_check.is_alphabetic() {
            //
            match char_to_check {
                'a' => vowel_count = vowel_count + 1,
                'e' => vowel_count = vowel_count + 1,
                'i' => vowel_count = vowel_count + 1,
                'o' => vowel_count = vowel_count + 1,
                'u' => vowel_count = vowel_count + 1,
                'A' => vowel_count = vowel_count + 1,
                'E' => vowel_count = vowel_count + 1,
                'I' => vowel_count = vowel_count + 1,
                'O' => vowel_count = vowel_count + 1,
                'U' => vowel_count = vowel_count + 1,
                _ => con_count = con_count + 1
            }
        }
    }
    //vowel and consonant count
    println!("Consonants {}, Vowels {} ", con_count, vowel_count);
}






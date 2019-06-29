use std::fs::File;

use std::io::prelude::*;



fn main() {

    let mut file = File::open("output.txt")

        .expect("Can't Open File!"); //need this for if something fails.

    let mut contents = String::new();  //create a new String object which holds the read in text



    file.read_to_string(&mut contents)

        .expect("Oops! Can not read the File....");  //need this for if something fails.



    let pattern = "patronizing intonation natural";  //word to be search

    let pattern_length = pattern.len();  //store the length of the word being search.

    let text_length = contents.len(); //length of the Entire text we are searching



    let mut word_count = 0;



    for i in 0..contents.len() - pattern_length {

        let mut j = 0;  //reset J back to 0 when the pattern int he while loop returns false.



        while j < pattern_length && contents.chars().nth(i + j).unwrap() == pattern.chars().nth(j).unwrap() {

            j = j + 1;  //increment J, which will check that the next part of text matches pattern.

        }



        //if J is the same size as pattern then we know that we've found the pattern

        if j == pattern_length {

            //println!("Found '{}'", pattern);

           word_count = word_count + 1;

        }

    }

    println!("{} appears {} times",pattern, word_count);

}

use std::env;
use std::error;
use std::io::{self, Write};
use std::process;
use std::fs;

mod vigenere_cipher;

type Error = Box<error::Error + Send + Sync>;

//Slightly more advanced Cipher
fn main() {
    match run() {
        Ok(count) => println!("{}", count),
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "{}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<u64, Error> {
    //path and query input into the console. First is path to the text file to search,
    //second is the the pattern to search for.
    let (file_path, key) = match (env::args_os().nth(1), env::args().nth(2)) {
        (None, _) => return Err(Error::from("No file to search")),
        (_, None) => return Err(Error::from("missing pattern")),
        (Some(file_path), Some(key)) => (file_path, key),
    };

    let mut input = String::new();
    println!("1. Encryption \n2. Decryption");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    //throw an error the user inputted text isn't a unsigned int.
    let number: u8 = input.trim().parse().expect("Expected a Number");

    let plain_text = fs::read_to_string(file_path).expect("Unable to read file");

    //Key is used for encrypting, path is the file to encrypt with the key.
    vigenere_cipher::start_cipher(&plain_text, &key, number);

    let count = 0;
    Ok(count)  //return value.
}







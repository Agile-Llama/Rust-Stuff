use std::env;
use std::error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process;
use std::ptr::null;
use std::fs;

//mod vigenere_cipher;

type Error = Box<error::Error + Send + Sync>;

//super basic rot13 encryption/Decryption.
fn main() {
    //Can implement asking user for a string to encrypt.
    //let mut user_input = String::new();
    //println!("Enter word to Encrypt/Decrypt");

    //io::stdin()
    //.read_line(&mut user_input)
    //  .expect("Failed to read input");

    //user_input = user_input.trim().to_string();

    //println!("Encrypted {} ", rot13_letter_encrypt(&user_input));

    match run() {
        Ok(count) => println!("{}", count),
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "{}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<u64, Error> {
    let mut rot_13_encrypted_text = String::new();
    let mut text = String::new();

    let (file_path) = match env::args_os().nth(1) {
        (None) => return Err(Error::from("No file to search")),
        (Some(file_path)) => (file_path),
    };

    let count = 0;
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    for i in 0..data.len() {
        rot_13_encrypted_text.push(encrypt(&data.chars().nth(i).unwrap()));
    }

    println!("Output: \n {}", rot_13_encrypted_text);

    //this is where the file will write too, could add a second parameter above to have a place to write.
    fs::write("/Users/AgileLlama/Desktop/RustProjects/TestProject-cargo/test_file.txt",
              &rot_13_encrypted_text).expect("Unable to write file");

    //vigenere_cipher::encrypt_cipher(&rot_13_encrypted_text);
    Ok(count)
}

//can't think of a better way to deal with capitals?
//encrypt/decrypt with rot13
fn encrypt(word: &char) -> char {
    let mut rot_13_encrypted_text = String::new();
    if word.is_alphabetic() {
        match word {
            'a' => return 'n',
            'b' => return 'o',
            'c' => return 'p',
            'd' => return 'q',
            'e' => return 'r',
            'f' => return 's',
            'g' => return 't',
            'h' => return 'u',
            'i' => return 'v',
            'j' => return 'w',
            'k' => return 'x',
            'l' => return 'y',
            'm' => return 'z',
            'n' => return 'a',
            'o' => return 'b',
            'p' => return 'c',
            'q' => return 'd',
            'r' => return 'e',
            's' => return 'f',
            't' => return 'g',
            'u' => return 'h',
            'v' => return 'i',
            'w' => return 'j',
            'x' => return 'k',
            'y' => return 'l',
            'z' => return 'm',
            //capitals
            'A' => return 'N',
            'B' => return 'O',
            'C' => return 'P',
            'D' => return 'Q',
            'E' => return 'R',
            'F' => return 'S',
            'G' => return 'T',
            'H' => return 'U',
            'I' => return 'V',
            'J' => return 'W',
            'K' => return 'X',
            'L' => return 'Y',
            'M' => return 'Z',
            'N' => return 'A',
            'O' => return 'B',
            'P' => return 'C',
            'Q' => return 'D',
            'R' => return 'E',
            'S' => return 'F',
            'T' => return 'G',
            'U' => return 'H',
            'V' => return 'I',
            'W' => return 'J',
            'X' => return 'K',
            'Y' => return 'L',
            'Z' => return 'M',
            _ => println!("Error unexpected Letter")
        }
    } else {
        match word {
            ',' => return ',',
            '.' => return '.',
            ' ' => return ' ',
            '!' => return '!',
            '"' => return '"',
            '?' => return '?',
            '-' => return '-',
            '(' => return '(',
            ')' => return ')',
            ':' => return ':',
            ';' => return ';',
            '\'' => return '\'',
            '\n' => return '\n',
            '1' => return '1',
            '2' => return '2',
            '3' => return '3',
            '4' => return '4',
            '5' => return '5',
            '6' => return '6',
            '7' => return '7',
            '8' => return '8',
            '9' => return '9',
            '0' => return '0',
            _ => println!("Error Non expected Grammar {}", word)
        }
    }
    return '*';
}








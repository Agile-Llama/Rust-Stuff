use std::fs;

static A: u8 = 'A' as u8;

fn uppercase_and_filter(input: &str) -> Vec<u8> {
    let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut result = Vec::new();

    for c in input.chars() {
        // Ignore anything that is not in our short list of chars
        if alphabet.iter().any(|&x| x as char == c) {
            result.push(c.to_ascii_uppercase() as u8);
        }
    }

    return result;
}

fn vigenere(key: &str, plaintext: &str, is_encoding: bool) -> String {
    let key_bytes = uppercase_and_filter(key);
    let text_bytes = uppercase_and_filter(plaintext);

    let mut result_bytes = Vec::new();

    for (i, c) in text_bytes.iter().enumerate() {
        //if we want to decrypt the file
        let c2 = if is_encoding {
            (c + key_bytes[i % key_bytes.len()] - 2 * A) % 26 + A
         //if we want to decrypt the file.
        } else {
            (c + 26 - key_bytes[i % key_bytes.len()]) % 26 + A
        };
        result_bytes.push(c2);
    }

    String::from_utf8(result_bytes).unwrap()
}

//true or false is whether we are encoding or decoding.
pub fn start_cipher(plaintext: &String, key: &String, answer: u8) {

    // if answer == 1 we are encoding, else we are decoding.
    if answer == 1 {
        let encoded = vigenere(key, plaintext, true);
        println!("Key used '{}' Text Encrypted {}", key, encoded);

        fs::write("/Users/AgileLlama/Desktop/RustProjects/TestProject-cargo/test_file.txt",
                  &encoded).expect("Unable to write file");
    } else if answer == 2 {
        let decoded = vigenere(key, &plaintext, false);
        println!("Key use '{}' Text Decrypted {}", key, decoded);

        fs::write("/Users/AgileLlama/Desktop/RustProjects/TestProject-cargo/test_file.txt",
                  &decoded).expect("Unable to write file");
    }
}



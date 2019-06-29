use std::env;
use std::error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process;

type Error = Box<error::Error + Send + Sync>;

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
    let (file_path, pattern) = match (env::args_os().nth(1), env::args().nth(2)) {
        (None, _) => return Err(Error::from("No file to search")),
        (_, None) => return Err(Error::from("missing pattern")),
        (Some(file_path), Some(pattern)) => (file_path, pattern),
    };

    let mut count = 0;
    let reader = io::BufReader::new(File::open(file_path)?);
    //iterate over each line, check if the line contains the pattern to search for
    //increment the count of it does.
    for line in reader.lines() {
        let line = line?;
        if line.contains(&pattern) {
            count = count + 1;
        }
    }
    Ok(count)
}





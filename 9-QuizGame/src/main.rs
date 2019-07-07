extern crate colored; 
use std::env;
use std::error;
use std::io::{self, BufRead, Write};
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

type Error = Box<error::Error + Send + Sync>;  //type casting like from c++

fn run() -> Result<u64, Error> {
    let file_path = match env::args_os().nth(1) {
        None => return Err(Error::from("No file to search")),
        Some(file_path) => (file_path),
    };
    let timeout = 10;
    let mut answers = 0;
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let total_questions = data.lines().count();
    let score = data.lines()
    .map(|line| {
          let mut q_a = line.split('\t').map(|s| s.to_string());
          let question = q_a.next().expect("No question found.");
          let answer = q_a.next().expect("No answer found.");
           (question, answer)
    })
        .map(|(question, answer)| read_question(&question, &answer, timeout))
        // `take_while` continues the iterator while the condition is true.
        .take_while(|o| o.is_some())
        .map(|o| o.unwrap())
		// Now we want to count the correct answers by filtering out the false bool.
        .filter(|p| *p)
        .count();
        
    println!("Score: {} / {}", score, total_questions);
     Ok(answers)
}

fn read_question(question: &str, answer: &str, timeout: i32 )  -> Option<bool>{  //return bool?
    
    // Set up our transmitter and receiver to use between threads.
    let (transmitter, receiver) = mpsc::channel();
    //ask question.
    println!("Question {} Enter your answer", question);

    // Spawn a thread with the user input code.
    thread::spawn(move || {
        let mut user_input = String::new();
        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
        user_input = user_input.trim().to_string();
        //transmits it to the reciver if the other thread has ended first.
        transmitter.send(user_input).expect("Failed to send user input");  
    });

   receiver
        .recv_timeout(Duration::new(timeout as u64, 0))
		.or_else(|o| {
			// If there is an error we print this and re-wrap the error.
			// Hack to use an error as a side effect.
			println!("\nYou ran out of time! :(");
			Err(o)
		})
		// `ok` changes `Result<A,B>` into `Option<A>`.
        .ok()
		// Now we can transform the Option<String> to an Option<bool> using a mapping function.
        .map(|buffer| buffer == answer)

}

fn main() {
    run();
}


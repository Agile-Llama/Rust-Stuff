extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 100);
    let mut correct = false;
    let mut guesses = 0;

    while correct == false {
        let mut input = String::new();

        println!("Guess the number");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");


        // Don't crash on expected error!
        let guess: i32 = match input.trim().parse() {
            Ok(random_number) => random_number,
            Err(_) => return
        };

        let difference = (random_number - guess).abs();

        if difference == 0 {
            println!("Guessed the number {}", random_number);
            correct = true;
            guesses = guesses + 1;
            break;
        } else {
            guesses = guesses + 1;
        }
        //prints out how far the guess was off the number
        match difference {
            1...10 => println!("Off by 1-10"),
            11...25 => println!("Off by 11-25"),
            26...50 => println!("Off by 25-50"),
            50...100 => println!("Off by Greater then 50"),
            _ => println!("Nope.")
        }

        //checks guess compared to random number, prints appropriate message depending on higher or lower.
        match guess.cmp(&random_number) {
            Ordering::Less => println!("{} is too low! Try again.", guess),
            Ordering::Greater => println!("{} is too high! Try again.", guess),
            Ordering::Equal => {
                correct = true;
            }
        }
    }
    println!("Took you {} guesses ", guesses);
}
use std::io;
//primes
//2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101
fn main() {
    let mut input = String::new();
    println!("Enter number");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    //throw an error the user inputted text isn't a unsigned int.
    let number: u16 = input.trim().parse().expect("Expected a Number");
    calculate_primes(number);
}

//calculate all primes of the number, smaller then the number given.
//return a vector of u16.
fn calculate_primes(number: u16) {
    let mut my_vector: Vec<u16> = Vec::new();

    //fill vector with numbers. Easier to remove numbers from array then to check if prime then add.
    for i in 2..number + 1 {
        my_vector.push(i);
    }
    //this is where we remove non prime numbers
    for p in 2..number + 1 {
        my_vector.retain(|&x| x <= p || x % p != 0);
    }
    //calls a function to get the primes of the number
    prime_pairs(& mut my_vector, number);
}

//finds all pairs of prime which equal the number inputted by the user.
fn prime_pairs(vector_of_primes: &mut Vec<u16>, number: u16) {
    //holds list of numbers which are duplicates.
    let mut duplicates: Vec<u16> = Vec::new();
    for i in vector_of_primes.iter() {
        for j in vector_of_primes.iter() {
            //dont want 5 + 5 as its reusing the same number.
            if (i + j) == number && (i != j) {
                //removes duplicate numbers. EG the number 10 can have 3+7 and 7+3 only want one.
                if !duplicates.contains(i) || !duplicates.contains(j) {
                    duplicates.push(i.clone());
                    duplicates.push(j.clone());
                    println!("{} + {} = {}", i, j, number);
                }
            }
        }
    }
    //println!("Primes of {} {:?}", number, vector_of_primes);
}








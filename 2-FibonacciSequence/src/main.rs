use std::io;

//returns a string
fn fib(target: u64) -> String {
    let mut output_as_string = String::new();  //string to hold the Fibonacci sequence

    let mut first = 0;
    let mut next = 1;
    let mut output = 0;
    let mut i = 0;

    while output < target {
        if i <= 1 {
            output = i;
        } else {
            output = first + next;
            first = next;
            next = output;
        }

        if output < target {
            output_as_string = format!("{} {}", output_as_string, output);
        }
        i += 1;
    }
    return output_as_string.trim().to_string();
}


fn main() {
    let mut input = String::new();
    println!("Enter max of the Fibonacci sequence");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    //throw an error the user inputted text isn't a unsigned int.
    let number: u64 = input.trim().parse().expect("Expected a Number");

    println!("Fibonacci sequence {} ", fib(number));
}
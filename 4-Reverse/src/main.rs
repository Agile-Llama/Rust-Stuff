use std::io;

fn main() {
    let mut user_input = String::new();
    println!("Enter sentence");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    user_input = user_input.trim().to_string();

    let mut output = String::new();
    println!("You entered {}", user_input);

    //need to pass output as a mutable reference as we'll be modifying it in the recursive call.
    println!("{}", reverse(&mut output, &user_input, user_input.len() as i32));
}
//'a is the lifetime of the variables. We dont give the lifetime to 'input' but it would be fine if we did.
//the reason for this is that we won't be returning the reference of input from this function.
//the return value of reverse must match the lifetime value of 'a, or else it must return a value which was created within.
fn reverse<'a>(output: &'a mut String, input: &String, n: i32) -> &'a mut String {
    if n == 0 {
        return output;
    }

    output.push(input.chars().nth((n - 1) as usize).unwrap());
    //recursive call, reduce n by 1 as we've already reversed one of the characters.
    reverse(output, input, n - 1)
}







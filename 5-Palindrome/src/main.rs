use std::io;

fn main() {
    let mut user_input = String::new();
    println!("Enter word to check");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    user_input = user_input.trim().to_string();

    //create a copy of the reference of user_input. Call a duplicate function. Takes a generic.
    let user_input_copy = duplicate(&user_input);
    let mut output = String::new();
    //reverse the user_input. Compare this with user_input_copy.
    let reversed = reverse(&mut output, &user_input, user_input.len() as i32);

    //if the reverse and the copy are correct, then its a Palindrome. Ignore the casing.
    if reversed.to_lowercase() == user_input_copy.to_lowercase() {
        println!("{} is a Palindrome", user_input_copy);
    } else {
        println!("{} isn't a Palindrome", user_input_copy);
    }
}

fn reverse<'a>(output: &'a mut String, input: &String, n: i32) -> &'a mut String {
    if n == 0 {
        return output;
    }

    output.push(input.chars().nth((n - 1) as usize).unwrap());
    //recursive call, reduce n by 1 as we've already reversed one of the characters.
    reverse(output, input, n - 1)
}

//copies contents of a variable into another.
fn duplicate<T>(x: T) -> T {
    return x;
}







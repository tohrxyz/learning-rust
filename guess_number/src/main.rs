use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Counter variable for the number of steps to guess right
    let mut counter = 0;

    // game looping until we guess the right number
    loop {
        println!("Please input your guess: ");

        // Create a mutable variable to store the user input of type String
        let mut guess = String::new();

        // Read the user input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess) // reads the input from cli and stores it in guess of type String
            .expect("Failed to read line"); // if wrong input, crash and display the error message

        // Convert the input to a number
        let guess: u32 = match guess
            .trim() // remove the whitespaces
            .parse(){ // try to convert the input to a number
                Ok(num) => num, // if the input is a number, store it in the guess variable
                Err(_) => continue, // if the input is not a number, restart the loop
            };

        // compare the user input with the secret number
        match guess.cmp(&secret_number) { // cmp() compares the two numbers and returns an Ordering enum
            Ordering::Less => { // if the user input is less than the secret number
                println!("Too small!");
                counter += 1;
            }
            Ordering::Greater => {  // if the user input is greater than the secret number
                println!("Too big!");
                counter += 1;
            }
            Ordering::Equal => { // if the user input is equal to the secret number
                println!("You win!");
                println!("Steps {counter}");
                break; // break the loop
            }
        }
    }
}

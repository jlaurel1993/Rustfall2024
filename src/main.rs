use std::io::{self, Write}; // For user input

// Function to check the guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // The secret number (hardcoded)
    let secret_number = 42;

    // Track the number of guesses
    let mut guess_count = 0;

    // Game loop
    loop {
        // Ask for user input (simulating dynamic guesses)
        print!("Enter your guess: ");
        io::stdout().flush().unwrap(); // Ensures the prompt is displayed

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = guess.trim().parse().expect("Please enter a valid number");

        guess_count += 1;

        // Check the guess
        let result = check_guess(guess, secret_number);

        // Print feedback based on the result
        if result == 0 {
            println!("Correct! The secret number is {}.", secret_number);
            break; // Exit the loop
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }
    }

    // Print the number of guesses it took
    println!("You guessed the number in {} tries.", guess_count);
}

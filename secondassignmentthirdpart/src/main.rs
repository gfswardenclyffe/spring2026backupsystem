// Assignment 3: Guessing Game
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut secret: i32 = 42;
    let mut guesses_taken: i32 = 0;

    let mut guess: i32 = 10; // this serves as user guess input

    loop {
        guesses_taken = guesses_taken + 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {} is correct!", guess);
            break;
        } else if result == 1 {
            println!("Guess {} is too high.", guess);
            guess = guess - 5; // next guess down
        } else {
            println!("Guess {} is too low.", guess);
            guess = guess + 16; // next guess up
        }
    }

    println!("It took {} guesses.", guesses_taken);
}
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

                // Convert the guess from String to an integer
                let guess: i32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Sorry, {}, Please input a valid number.", err);
                        continue;
                    }
                };
                if guess == secret_number {
                    println!("You guessed the correct number {}!", guess);
                    break;
                } else if guess < secret_number {
                    println!("Too small!");
                } else {
                    println!("Too big!");
                }
            }
}
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Guess the Number!");

    // Ask for player's name
    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim(); 

    println!("Hello, {name}! Let's play.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("\nPlease input your guess (1–100):");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) if (1..=100).contains(&num) => num,
            Ok(_) => {
                println!("Please enter a number between 1 and 100.");
                continue;
            }
            Err(_) => {
                println!("Invalid input! Please type a number.");
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\nCongratulations, {name}! You win after {attempts} attempts!");
                break;
            }
        }
    }

    println!("Thanks for playing, {name}!");
}





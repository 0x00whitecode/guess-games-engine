use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("{}", "Welcome to the Guessing Game!".cyan());
    
    loop {
        println!("\nSelect an option:");
        println!("1. Play Game");
        println!("2. Game Instructions");
        println!("3. Exit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        match choice.trim() {
            "1" => play_game(),
            "2" => game_instructions(),
            "3" => {
                println!("{}", "Goodbye!".green());
                break;
            }
            _ => println!("{}", "Invalid choice. Please enter 1, 2, or 3.".red()),
        }
    }
}

fn play_game() {
    println!("\n{}", "######### Starting Game #########".yellow());
    let mut lives = 5;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    while lives > 0 {
        println!("\nYou have {} lives remaining.", lives);
        println!("Enter your guess (1-100): ");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                println!("{}", "Invalid input! Please enter a number between 1 and 100.".red());
                continue;
            }
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small!".red());
                lives -= 1;
            },
            Ordering::Greater => {
                println!("{}", "Too big!".blue());
                lives -= 1;
            },
            Ordering::Equal => {
                println!("{}", "Congratulations! You win!".green());
                return;
            }
        }
    }
    
    println!("{}", "Game Over! You ran out of lives.".purple());
    println!("The secret number was: {}", secret_number);
}

fn game_instructions() {
    println!("\n{}", "######### Game Instructions #########".blue());
    println!("1. The game generates a random number between 1 and 100.");
    println!("2. You have 5 attempts to guess the correct number.");
    println!("3. If your guess is too high or too low, you will be notified.");
    println!("4. If you guess the correct number, you win!");
    println!("5. If you run out of lives, the game is over.");
    println!("6. You can exit the game anytime by selecting the Exit option.");
}

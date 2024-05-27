use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input! try again."); 
                continue;
            }
        };
        
        println!("you guessed: {guess}");

        let has_won = comp_guess(secret_number, guess);
        if has_won { break; }
    }
}

//function that compares the guess and the secret number.
//displays a message and returns true if they are equal.
fn comp_guess(secret_number: u32, guess: u32) -> bool{
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    return false;
}

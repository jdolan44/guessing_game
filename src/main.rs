use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //configure game
    println!("Enter the upper bound for the random number:");
    let upper_bound: u32 = get_number();
    println!("Enter the number of guesses:");
    let num_guesses: u32 = get_number();

    let secret_number: u32 = rand::thread_rng().gen_range(1..=upper_bound);

    play_game(secret_number, num_guesses);
}

fn get_number() -> u32{
    loop{
        let mut num_limit = String::new();

        io::stdin()
                .read_line(&mut num_limit)
                .expect("Failed to read line!");
        
        let num_limit: u32 = match num_limit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input! try again."); 
                continue;
            }
        };
        return num_limit;
    }
}

fn play_game(secret_number: u32, num_guesses: u32){
    let mut num_guesses = num_guesses;
    while num_guesses > 0{
        println!("Enter your guess ({} guesses remaining):", num_guesses);

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
        num_guesses-=1;
    }
    if num_guesses == 0{
        println!("You lost!")
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

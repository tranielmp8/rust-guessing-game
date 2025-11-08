use std::cmp::Ordering;
use std::io; 
use rand::Rng; // this is for random_range() 
use rand::rng;

fn main() {
    println!("Guessing Game!");

    let mut how_many = String::new();
    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for _ in 0..num_guesses { // i is not used so you can use _ instead
        correct.push(rng().random_range(1..=10));
    }

    println!("{correct:?}");
    
    let mut guesses_made = 0;

    println!("Guess a Number between 1 - 10");
    while guesses_made < num_guesses {

        let mut guess  = String::new(); // empty string that can be updated
        io::stdin()
            .read_line(&mut guess)
            .expect("Error with guess input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again! {e}");
                continue;
            }
        };     

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guess too high!"),
            Ordering::Less => println!("You guessed to low!"),
            Ordering::Equal => {
                println!("You are correct! 🦀");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Let's now try the next number!");
                }
            }
        }; // semicolon because of the match

    }

    println!("Thanks for playing! the correct answers were:");
    for item in correct {
        println!("{item}")
    }

    
}

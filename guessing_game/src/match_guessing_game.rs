use std::cmp::Ordering;
use std::io; 
use rand::Rng; // this is for random_range() 
use rand::rng;

fn main() {
    println!("Guessing Game!");

    let correct: u32 = rng().random_range(1..=10);

    println!("Guess a Number between 1 - 10");
    let mut guess  = String::new(); // empty string that can be updated

    io::stdin()
        .read_line(&mut guess)
        .expect("Error with guess input");

    let guess: u32 = guess.trim().parse().expect("Parsing error");

    // cmp is for compare, so with compare we can compare guess to correct
    let message = match guess.cmp(&correct) {
        Ordering::Greater => "You guessed to High.",
        Ordering::Less => "You guessed too low",
        Ordering::Equal => "You guessed CORRECT 🦀",
    };

    println!("{message}, {guess} is the guess while {correct} is the random number!")
    
}

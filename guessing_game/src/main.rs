use std::cmp::Ordering;
use std::io; 
use rand::Rng; // this is for random_range() 
use rand::rng;

fn main() {
    println!("Guessing Game!");

    let correct: u32 = rng().random_range(1..=10);

    println!("Guess a Number between 1 - 10");
    

    loop {
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

        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guess too high!"),
            Ordering::Less => println!("You guessed to low!"),
            Ordering::Equal => {
                println!("You are correct! 🦀");
                break;
            }
        }   

    }


    
}

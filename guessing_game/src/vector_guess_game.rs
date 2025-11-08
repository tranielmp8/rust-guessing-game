use std::cmp::Ordering;
use std::io; 
use rand::Rng; // this is for random_range() 
use rand::rng;

fn vector_guess() {
    println!("Guessing Game!");

    let mut how_many = String::new();
    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for i in 0..num_guesses { // i is not used so you can use _ instead
        correct.push(rng().random_range(1..=10));
    }

    println!("{correct:?}");

}
// use std::io;
// use rand::Rng;
// use rand::rng;
// // gen_thread() has been deprecated, so has gen_range and thread_rng()

// fn guessing_game() {
//     println!("Rust Guessing Game!");
//     // let correct = rand::thread_rng().gen_range(1..=10); //  .9+ rand::thread_rng has been depracated as well we would just use rng

//     let correct = rng().random_range(1..=10);
//     println!("Correct: {correct}");

//     println!("Guess the number (1-10): ");
//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("Error reading input");
//     let guess: u32 = guess.trim().parse().expect("Error with parse!");

//     // cool way with rust ASLO bc this expression is in a variable you put the semicolon at the very end
//     let  message = if correct > guess {
//         String::from("You guessed to low")
//     } else if correct < guess {
//         String::from("You guessed to high!")
//     } else {
//         String::from("YOU ARE CORRECT!")
//     };

//     println!("{message}")


//     // old way of doing it which is still ok to do
//     // if correct > guess {
//     //     println!("You guessed to low!");
//     // } else if correct < guess {
//     //     println!("You guessed to high.")
//     // } else {
//     //     println!("You guessed Correct!")
//     // }
// }

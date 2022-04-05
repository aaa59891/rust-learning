use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = match guess.trim().parse::<u32>() {
            Ok(result) => result,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Your are right!");
                break;
            }
        }
    }
}

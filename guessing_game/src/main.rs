use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","Please enter a number!".red());
                continue;
            },
        };
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".yellow()),
            Ordering::Greater => println!("{}","Too big!".yellow()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            },
        }
    }

}
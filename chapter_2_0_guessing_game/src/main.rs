use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {}.", secret_number);

    let mut retries: u32 =  10;

    while retries > 0 {
        let mut guess: String = String::new();

        println!("Please input your guess: ");

        // Use this if you want your input to be on the
        // same line as the print your print output.
        // print!("Please input your guess: ");
        // let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Failed to parse input. Err: {}", err.to_string());
                continue;
            }
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        retries -= 1;

        println!("Retries left: {}", retries);
    }
}
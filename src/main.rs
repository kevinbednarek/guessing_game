use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let lower = 1;
    let upper = 20;

    println!("Guess a number between {} and {}!", lower, upper);

    let secret_number = rand::thread_rng().gen_range(lower..=upper);

    //println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number.");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

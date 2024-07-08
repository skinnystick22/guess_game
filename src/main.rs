use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number: u32 = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!")
        };
    }
}

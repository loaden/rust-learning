use std::io;
use rand::Rng;

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    print!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    let guess: u32 = guess.trim().parse().expect("Please enter a valid number");
    println!("You guessed: {}", guess);
}

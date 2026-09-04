use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::random_range(1..=100);
    println!("The secret number was: {}", secret_number);
    println!(
        "Uniform i8 sample: {}",
        match rand::random() {
            0i8 => "zero",
            i if i > 0 => "positive",
            _ => "negative",
        }
    );

    print!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    let guess: u32 = guess.trim().parse().expect("Please enter a valid number");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

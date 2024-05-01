use std::io; // bring io library into scope

fn main() {
    println!("Guess the Number");
    println!("Please input your guess.");
    let mut guess = String::new(); // mutable variable guess
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

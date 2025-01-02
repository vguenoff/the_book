use std::io;

fn main() {
    let x = 5;
    let y = 10;
    println!("x is {x}, y is {}, soo x + y = {}", y, x + y);

    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}

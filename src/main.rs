extern crate rand;

use std::io;

fn main() {
    game(10)
}

enum Input {
    Quit,
    Unparseable,
    Guess(i32),
}

fn input_number() -> Input {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => { buffer.pop(); parse_input(&buffer) },
        Err(err) => {
            println!("{}", err);
            Input::Unparseable
        }
    }
}

fn parse_input(i: &str) -> Input {
    use Input::*;
    
    match i {
        "q" => Quit,
        n => {
            match n.parse::<i32>() {
                Ok(i) => Guess(i),
                Err(_) => Unparseable,
            }
        }
    }
}

fn game(upper: i32) -> () {
    use rand::Rng;

    let secret_number = rand::thread_rng().gen_range(1, upper + 1);
    println!("Guess a number between 1 and {}, 'q' to quit", upper);
    try_guess(secret_number);
}

fn try_guess(secret: i32) -> () {
    use Input::*;

    match input_number() {
        Guess(n) => {
            if n == secret {
                println!("You guessed right, the number was {}", secret);
            } else { println!("Unlucky, try again!"); try_guess(secret); }
        },
        Unparseable => {
            println!("Sorry, could not understand your number");
            try_guess(secret);
        },
        Quit => println!("Bye!"),
    }
}
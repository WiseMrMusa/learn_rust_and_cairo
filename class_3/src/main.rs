use std::io;
use rand::Rng;


fn main() {
    println!("Welcome to CVIII Rust Game");

    let random_number: u8 = rand::thread_rng().gen_range(0..100);

    println!("Our random number is {}", random_number);

    loop {
        
        let guess: u8 = your_guess();

        if guess == random_number {
            println!("Congratulations, you guessed right");
            break;
        } else {
            println!("Try again");
        }
        
    }
}

fn your_guess() -> u8 {
    println!("Guess a number");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading line");
    let x: u8 = x.trim().parse().unwrap();
    println!("You guessed {x}");
    x
    
}
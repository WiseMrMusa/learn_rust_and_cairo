use std::io;


fn main() {
    loop {
        the_game();
    }
}

fn the_game() {
    println!("Welcome to CVIII Rust Game");
    println!("Guess a number");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading line");
    let x: u8 = x.trim().parse().unwrap();
    println!("You guessed {x}");
}
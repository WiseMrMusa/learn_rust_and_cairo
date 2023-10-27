use std::{io, cmp::Ordering, fmt::{Display,Result, Formatter, write}};
use rand::Rng;


fn main() {
    // match => enum 
    enum Cars {
        Tesla,
        Nokia,
        Honda,
        BMW
    }

    impl Display for Cars {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Cars::Tesla => write!(f,"The car is Tesla"),
                Cars::Nokia => write!(f,"The car is Nokia"),
                Cars::Honda => write!(f,"The car is Honda"),
                Cars::BMW => write!(f,"The car is BMW")
            }
        }
    }

    let m: Cars = Cars::Nokia;
    println!("Welcome to CVIII Rust Game {m}");

    let random_number: u8 = rand::thread_rng().gen_range(0..100);

    // println!("Our random number is {}", random_number);

    loop {
        let guess: u8 = your_guess();

        match guess.cmp(&random_number){
            Ordering::Less => println!("Your guess is lesser"),
            Ordering::Greater => println!("Your guess is greater"),
            Ordering::Equal => {
                println!("Congratulations");
                break;
            }
        }



        // if guess < random_number {
        //     println!("Your guess is lesser");
        //     println!("Try again");
        //     continue;
        // }
        // if guess > random_number {
        //     println!("Your guess is bigger");
        //     println!("Try again");
        //     continue;
        // }

        // if guess == random_number {
        //     println!("Congratulations, you guessed right");
        //     break;
        // } else {
        //     println!("Try again");
        // }
        
    }
}

fn your_guess() -> u8 {
    println!("Guess a number");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading line");
    let x: u8 = match x.trim().parse(){
        Ok(x) => x,
        Err(_) => 50,
    };
    println!("You guessed {x}");
    x
    
}
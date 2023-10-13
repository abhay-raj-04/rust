use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

loop {
    println!("Guess the number:");

    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("The random number was : {guess}");
                println!("You won!");
                println!("\n\n To play again press 1.\n To exit press 2.");
                break;
            }
        }
    }
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess : u8 = guess.trim().parse().expect("Press 1 or 2.");
    if guess == 2{
        break;
    }
}
}
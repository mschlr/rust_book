use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_100() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut counter = 1;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("..{} is too small!", guess),
            Ordering::Greater => println!("..{} is too big!", guess),
            Ordering::Equal => {
                println!("You win!");
                println!("Bravo, you guessed {} in {} attempts!", guess, counter);
                break;
            }
        }
        counter += 1;
    }
}

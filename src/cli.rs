use std::env;
use crate::guess::guess_100;
use crate::common::run;

pub fn args() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // if a single argument was passed
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "guess" => guess_100(),
                "common" => run(),
                "help" => help(),
                _ => { println!("Sorry, I do not know this option 🤨"); help()},
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

fn help() {
    println!("Usage: rust_book [OPTION]
Options:
    guess       Chapter 2: Guessing Game
    common      Chapter 3: Common concepts

Martin Schuler <martin.schuler@outlook.com>");
}
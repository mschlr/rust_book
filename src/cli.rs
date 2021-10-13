use std::env;
use crate::guess::guess_100;


pub fn read() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("Please pass some arguments: 'guess', 'help'.");
        },
        // one argument passed
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "guess" => guess_100(),
                _ => println!("This is not the answer."),
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
    guess   Chapter 2: Guessing Game
    loops   Chapter 3: Loop examples");
}
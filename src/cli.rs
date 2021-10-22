use std::env;
use crate::guess::guess_100;
use crate::common::run;
use crate::ownership::hello;
use crate::structs::rec;
use crate::hashmap::exercise;

pub fn args() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // if a single argument was passed
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "guess" => guess_100(),
                "common" => run(),
                "ownership" => hello(),
                "rectangles" => rec(),
                "hashmap" => exercise(),
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
    ownership   Chapter 4: Rust concept of `ownership`
    rectangles  Chapter 5: Example Program Using Structs
    hashmap     Chapter 8: Exercises with Vectors and HashMap

Martin Schuler <martin.schuler@outlook.com>");
}
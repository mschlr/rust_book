use std::io;
use std::{thread, time};

pub fn run() {
    c2f();
    nth_fibonacci();
    twelve();
}

fn c2f() {
    // Convert °C to Fahrenheit
    // Formula: fahrenheit = (celsius * 9 / 5) + 32
    println!("\nConvert degree Celcius to Fahrenheit!");

    loop {
        println!("Please, enter current degrees:");

        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: i32 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fahrenheit: i32 = (&celsius * 9 / 5) + 32;

        // if-else-if-Alternative to match with cmp::Ordering
        if celsius < 0 {
            println!("...so icy!");
        } else if celsius <= 15 {
            println!("...that's pretty chilly...");
        } else if celsius <= 25 {
            println!("...alright,");
        } else {
            println!("...isn't it a bit hot?");
        }
        println!("\n{} °C are {} Fahrenheit.", celsius, fahrenheit);
        break;
    }
}

const MAX_FIBN: u32 = 93; // fibonacci(93) -> u64

fn nth_fibonacci() {
    // Call standard Fibonacci function
    // for the n-th number
    println!("\nLet's calculate the n-th Fibonacci-number!");

    loop {
        println!("Please enter n for Fibonacci(n):");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("failed to read line");
    
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n >= MAX_FIBN { 
            println!("We might risk an u64 overflow! Skipped...");
            continue;
        } else {
            println!("\nFibonacci_std({}) => {}", n, fibonacci(n));
            println!("Fibonacci_rec({}) => {}", n, fibonacci_rec(n));
            break;
        }  
    }  
}

fn fibonacci(n: u32) -> u64 {
    // Standard Fibonacci Algorithm
    let mut x = 1;
    let mut y = 1;

    for _ in 2..n {
        let n0 = x;
        x = y;
        y += n0;
    }

    return y;
}

fn fibonacci_rec(n: u32) -> u64 {
    // Calculate Fibonacci-Number using recursion,
    // probably a bad idea!
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}

fn twelve() {
    // Twelve Days of Christmas Song.
    // Solution uses nested loops and static arrays

    println!("\nTwelve Days of Christmas 🎼");
    thread::sleep(time::Duration::from_secs(3));

    let day: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    let gift: [&str; 11] = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eigth maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..day.len() {
        println!("\nOn the {} day of Christmas,", day[i]);
        println!("my true love sent to me");
        if i == 0 {
            println!("A partridge in a pear tree.");
            thread::sleep(time::Duration::from_secs(1));
        } else {
            for j in 0..i {
                println!("{},", gift[j]);
            }
            println!("And a partridge in a pear tree.");
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

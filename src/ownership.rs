pub fn hello() {
    // Book Example
    let my_string = String::from("Flughafen Zürich (ZRH)");
    println!("{}", first_word(&my_string));

    // Some more String and UTF-8 Examples
    let mut hello = String::from("Hello");

    // Get string length
    println!("Get the length of '{}': {}", hello, hello.len());

    hello.push(' '); // `empty` char
    hello.push('W');
    hello.push_str("orld.");

    print!("{}", hello);

    // String capacity
    println!(": has a capacity of {}", hello.capacity());

    // From Rust Book
    let strastwujtie = "Здравствуйте";
    let sawadeekrap = "สวัสดีครับ";
    let bind = format!("Thai: {}\nRussian: {}", sawadeekrap, strastwujtie);

    println!("\n{}\n", bind);
    println!("{}", &strastwujtie[0..4]);
    println!("{}", &sawadeekrap[0..6]);
}

fn first_word(s: &str) -> &str {
    // Chapter 4 - related function covering
    // Ownership, borrowing and slices
    
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

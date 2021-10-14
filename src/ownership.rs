pub fn hello() {
    let my_string = String::from("Flughafen Zürich (ZRH)");
    println!("{}", first_word(&my_string));
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

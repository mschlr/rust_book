use std::mem;
use std::collections::HashMap;

pub fn exercise() {
    arrays(); // repeat some stuff with arrays

    // First Exercise of Chapter 8
    // Calculate mean, median and the mode (using hashmap) of a Vector
    let mut v: Vec<i32> = vec![3, 1, 4, 7, 5, 9, 2, 2, 9, 4, 1, 3, 9, 8];

    println!("Our vector v is {:?}", v);
    println!("mean(v) => {}", mean(&v));
    println!("median(v) => {}", median(&mut v));
    println!("mode(v) => {}", mode(&v));
}

fn arrays () {
    let actg: [char; 4] = ['A', 'T', 'C', 'G'];

    println!("{:?}", actg);
    println!("Array length is {}", actg.len());
    println!("Size of array is {}", mem::size_of_val(&actg));

    let dna = [b'A', b'T', b'C', b'G', b'N'];

    println!("Print DNA info...");
    for i in 0..dna.len() {
        println!("{j:b}, {j}", j = dna[i]);
    }

    println!("Array length is {}", dna.len());
    println!("Size of array is {}", mem::size_of_val(&dna));
    
    print_type_of(&actg);
    print_type_of(&dna);
}

fn  print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn mean(v: &[i32]) -> f32 {
    v.iter().sum::<i32>() as f32 / v.len() as f32

    // lets try to forget all about what we learned in C++ course
    //     let mut total = 0;
    //     for n in v {
    //         total += n;
    //     }
    //     let total = total as f64;
    //     total / v.len() as f64
}

fn median(v: &mut [i32]) -> f32 {
    v.sort_unstable();
    let middle = v.len() / 2;
    if v.len() % 2 == 0 {
        mean(&v[middle - 1..middle + 1])
    } else {
        v[middle] as f32
    }
}

fn mode(v: &[i32]) -> i32 {
    // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
    let mut hs = HashMap::new();
    for number in v {
        let count = hs.entry(number).or_insert(0);
        *count += 1;
    }
    // Return
    *hs.into_iter()
    .max_by_key(|&(_, count)| count)
    .map(|(val, _)| val)
    .expect("Cannot compute the mode of zero numbers")
}

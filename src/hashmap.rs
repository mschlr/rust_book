pub fn exercise() {
    //vectors();
    let mut v: Vec<i32> = vec![3, 1, 4, 9, 2, 9, 3, 1, 3];

    println!("Our vector v is {:?}", v);
    println!("mean(v) => {}", mean(&v));
    println!("median(v) => {}", median(&mut v));
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

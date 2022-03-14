// vectors: resizeable arrays

use std::mem::size_of_val;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;

    // add to vector
    numbers.push(5);
    numbers.push(6);
    numbers.pop(); // remove last

    println!("whole array: {:?}", numbers);
    println!("single value: {}", numbers[0]);
    println!("length: {}", numbers.len());
    println!("memory used: {} bytes", size_of_val(&numbers));

    // slice arrays
    let slice = &numbers[1..3];
    println!("array slice: {:?}", slice);

    // loop through values
    for x in numbers.iter() {
        println!("{}", x);
    }
    // mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}

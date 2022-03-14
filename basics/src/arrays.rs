// array: fixed list of same type items
// primitive type: stored on the stack

use std::mem::size_of_val;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // type can be inferred, but must contain the size
    numbers[2] = 20;

    println!("whole array: {:?}", numbers);
    println!("single value: {}", numbers[0]);
    println!("length: {}", numbers.len());
    println!("memory used: {} bytes", size_of_val(&numbers));

    // slice arrays
    let slice = &numbers[1..3];
    println!("array slice: {:?}", slice);
}

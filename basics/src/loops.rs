// types of loops:
// loop: infinite loop
// while: standard while loop, () also not needed
// for: like a foreach, loops over an iterator

pub fn run() {
    // infinite loop
    let mut count = 0;
    loop {
        count += 1;
        println!("number {}", count);

        if count == 20 {
            break;
        }
    }

    // fizzbuzz
    let mut count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}

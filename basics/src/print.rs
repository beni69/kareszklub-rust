pub fn run() {
    println!("hello world");
    println!("number: {}", 1);
    println!("My {} is {}", "peepee", "poopoo");
    println!("{0} {0} {1}", "doo", "fart"); // arguments can be indexed
    println!("{name} likes {item}", name = "Joe", item = "ice cream"); // named arguments
    println!("Binary: {:b}, Hex: {:x}, Octal:{:o}", 10, 10, 10); // placeholder traits
    println!("{:?}", (12, true, "hello")); // debug trait
}

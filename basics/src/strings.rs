// Primitive &str = Immutable fixed-length, string slice. literally a slice of the final binary
// String = Growable, heap-allocated data - when you need to modify or own string data

pub fn run() {
    let hello = "hello";
    let mut cool_hello = String::from("hello again");

    // add a string
    cool_hello.push_str(" my friend!");
    println!("{:?}", (hello, &cool_hello, hello.len()));

    println!("Capacity in bytes: {}", cool_hello.capacity());
    println!("Is it empty? {}", cool_hello.is_empty());
    println!("Contains world? {}", cool_hello.contains("world"));
    println!("Replace: {}", cool_hello.replace("again", "world"));

    // loop through string
    for word in cool_hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10); // fixed length
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}

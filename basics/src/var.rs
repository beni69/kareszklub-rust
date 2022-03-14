pub fn run() {
    let name = "Joe";
    let mut age = 22;

    println!("My name is {} and I'm {}", name, age);

    // birthday
    age += 1;
    println!("My name is {} and I'm {}", name, age);

    // constants
    // this works but they're usually declard at the top of the file
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple variables
    let (my_name, my_age) = ("Tom", 69);
    println!("{} is {}", my_name, my_age);
}

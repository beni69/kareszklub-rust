// tuples: multiple values of any type
// max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Joe", "Mama", 69);
    println!("{} {} is {}", person.0, person.1, person.2);
}

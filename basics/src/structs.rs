// structs: custom data types

// normal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct a Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name -> tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    // println!("Color: {:?}", c);

    let mut c = ColorTuple(0, 255, 0);
    c.2 = 100;
    println!("ColorTuple: {} {} {}", c.0, c.1, c.2);
    // println!("ColorTuple: {:?}", c);

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);

    p.set_last_name("Williams");
    println!("{}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());
}
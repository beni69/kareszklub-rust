// rust is not a fully oop language
// no inheritance, we have traits
// https://doc.rust-lang.org/book/ch10-02-traits.html

pub fn run() {
    let mut p = Person::new("Joe", "Nuts");
    println!("Person {} {}", p.first_name, p.last_name);

    p.set_last_name("Williams");
    println!("{}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());

    // println!("{:?}", p); // !

    let talker = get_talker();
    talk(talker);

    // _debug_talker(talker); // !
}

// define data on a struct
struct Person {
    first_name: String,
    last_name: String,
}
// add functions in an impl block
impl Person {
    // constructors don't exist in rust
    // we just make a new() function
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
    fn to_tuple(&self) -> (&str, &str) {
        (&self.first_name, &self.last_name)
    }
}

trait CanTalk {
    fn talk(&self) -> String;
}

impl CanTalk for Person {
    fn talk(&self) -> String {
        format!("Hi, I'm {} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct Dog {
    _name: String,
}
impl Dog {
    fn _catch_ball() -> String {
        "caught ball".to_string()
    }
}

impl CanTalk for Dog {
    fn talk(&self) -> String {
        String::from("woof!")
    }
}

fn get_talker() -> impl CanTalk {
    Dog {
        _name: String::from("Spot"),
    }
}

// to the compiler these all mean the exact same thing
fn talk(t: impl CanTalk) {
    println!("{}", t.talk());
}
// fn talk<T: CanTalk>(t: T) {
//     println!("{}", t.talk());
// }
// fn talk<T>(t: T)
// where
//     T: CanTalk,
// {
//     println!("{}", t.talk());
// }

use std::fmt::Debug;

fn _debug_talker<T>(t: T)
where
    T: CanTalk + Debug,
{
    println!("{:?}", t);
}

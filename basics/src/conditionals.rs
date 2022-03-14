// if - else if - else
// works like any other lang
// note: no () needed

pub fn run() {
    let age = 19;
    let check_id = false;
    let knows_person_of_age = true;

    // if/else
    if age >= 18 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if check_id {
        println!("Sorry you can't drink!");
    } else {
        println!("Let me see your ID!");
    }

    // shorthand if
    // the closest we have to a ternary operator
    let is_of_age = if age >= 18 { true } else { false };
    println!("is of age: {}", is_of_age);

    // match - a switch statement but better
    let num = 1;
    match num {
        1 => println!("one"),
        2 | 3 | 5 | 7 => println!("prime"),
        10..=100 => println!("big"),
        _ => println!("something else"),
    }
}

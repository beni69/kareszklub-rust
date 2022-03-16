// structs: custom data types

// normal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct ColorTuple(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = ColorTuple(0, 255, 0);
    c.2 = 100;
    println!("ColorTuple: {} {} {}", c.0, c.1, c.2);
}

/*
Primitive Types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (int/unsigned int + bits)
Floats: f32, f64
Boolean (bool)
Characters (char)
String slices (&str)
Tuples
Arrays (not vectors)
References (but not the thing they point to)
*/

pub fn run() {
    let _x = 1; // will be i32 by default
    let _y = 2.5; // f64

    let _z: u8 = 123; // explicit types
    let _z = 123 as u8;
    let _z = 123u8;

    // max int values
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);
    println!("max i128: {}", std::i128::MAX);

    // booleans
    let active = false;
    let is_greater = 10 > 5;

    // char
    let ch = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (active, is_greater, ch, face));

    // custom types
    type MyType = (u8, f64, char);
    let _my_type: MyType = (1, 2.5, 'a');
}

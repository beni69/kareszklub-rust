#![allow(dead_code)]

// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
fn second_line(s: &str) -> &str {
    s.lines().nth(1).unwrap()
}
// fn second_line<'a>(s: &'a str) -> &'a str

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

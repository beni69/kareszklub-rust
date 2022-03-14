pub mod arrays;
pub mod cli;
pub mod conditionals;
pub mod enums;
pub mod functions;
pub mod loops;
pub mod print;
pub mod references;
pub mod strings;
pub mod structs;
pub mod tuples;
pub mod types;
pub mod var;
pub mod vectors;

fn main() {
    let fns: [(&str, fn()); 14] = [
        ("print", print::run),
        ("var", var::run),
        ("types", types::run),
        ("strings", strings::run),
        ("tuples", tuples::run),
        ("arrays", arrays::run),
        ("vectors", vectors::run),
        ("conditionals", conditionals::run),
        ("loops", loops::run),
        ("functions", functions::run),
        ("references", references::run),
        ("structs", structs::run),
        ("enums", enums::run),
        ("cli", cli::run),
    ];

    std::env::args().skip(1).for_each(|arg| {
        let mut i = 0;
        for f in fns.iter() {
            if arg == f.0 {
                f.1();
                break;
            }
            i += 1;
        }
        if i == fns.len() {
            println!("{} is not a valid function", arg);
        }
        print!("\n");
    });
}

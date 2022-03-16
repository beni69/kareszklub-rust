mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod oop;
mod print;
mod references;
mod result;
mod strings;
mod structs;
mod tuples;
mod types;
mod var;
mod vectors;

fn main() {
    let fns: [(&str, fn()); 16] = [
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
        ("result", result::run),
        ("cli", cli::run),
        ("oop", oop::run),
    ];

    let cmd = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("help".to_string());

    match cmd.as_ref() {
        "help" => {
            println!(
                "Available functions:\n{}",
                fns.iter().map(|f| f.0).collect::<Vec<&str>>().join("\n")
            );
        }
        "all" => {
            for f in fns.iter() {
                print!("\r\x1b[2J\r\x1b[H"); // clear terminal
                println!("\n{}\n", f.0);
                f.1();
                println!("\nPress enter to continue...");
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
        _ => {
            let mut i = 0;
            for f in fns.iter() {
                if cmd == f.0 {
                    f.1();
                    break;
                }
                i += 1;
            }
            if i == fns.len() {
                println!("{} is not a valid function", cmd);
            }
        }
    }
}

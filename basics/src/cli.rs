// std::env::args() - command line arguments

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    println!("Args {:?}", args);
    println!("Command {:?}", command);
}

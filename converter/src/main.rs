use clap::Parser;
use serde_json::Value;
use serde_yaml;
use std::{error::Error, fs::File, io::Write, path::PathBuf};
use termcolor::{StandardStream, WriteColor};
use toml;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,
    output: PathBuf,
    #[clap(short, long)]
    pretty: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    std::panic::set_hook(Box::new(panic_handler));

    let args = Args::parse();

    let file = std::fs::read(&args.input).expect("file not found");

    let v: Value = match args
        .input
        .extension()
        .expect("invalid input file extension")
        .to_str()
        .unwrap()
    {
        "json" => serde_json::from_slice(&file)?,
        "yml" | "yaml" => serde_yaml::from_slice(&file)?,
        "toml" => toml::from_slice(&file)?,
        _ => panic!("unknown file extension"),
    };

    match (
        args.output
            .extension()
            .expect("invalid output file extension")
            .to_str()
            .unwrap(),
        args.pretty,
    ) {
        ("json", false) => serde_json::to_writer(File::create(&args.output)?, &v)?,
        ("json", true) => serde_json::to_writer_pretty(File::create(&args.output)?, &v)?,
        ("yml" | "yaml", false) => serde_yaml::to_writer(File::create(&args.output)?, &v)?,
        ("yml" | "yaml", true) => panic!("pretty output not supported for yaml"),
        ("toml", false) => write!(File::create(&args.output)?, "{}", toml::to_string(&v)?)?,
        ("toml", true) => write!(
            File::create(&args.output)?,
            "{}",
            toml::to_string_pretty(&v)?
        )?,
        _ => panic!("unknown file extension"),
    }

    Ok(())
}

fn panic_handler(info: &std::panic::PanicInfo) {
    let mut stderr = StandardStream::stderr(termcolor::ColorChoice::Auto);
    stderr
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
        .unwrap();

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        writeln!(stderr, "error: {:?}", msg).unwrap();
    }
    if let Some(msg) = info.payload().downcast_ref::<String>() {
        writeln!(stderr, "error: {:?}", msg).unwrap();
    }

    std::process::exit(1);
}

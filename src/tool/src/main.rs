#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    text: String,
    file: std::path::PathBuf
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file).expect("could not read file");

    for line in content.lines() {
        println!("{}", line);
    }
}
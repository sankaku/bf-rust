use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect(&usage());

    let content = fs::read_to_string(filename);
    match content {
        Result::Ok(val) => println!("{:?}", val),
        Result::Err(err) => println!("[ERROR] {:?}", err),
    }
}

fn usage() -> String {
    String::from("cargo run <FILENAME>")
}

extern crate bf_rust;

use bf_rust::modules::interpreter::Interpreter;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect(&usage());
    let content = fs::read_to_string(filename);
    match content {
        Result::Ok(val) => {
            Interpreter::interpret(&val, 100);
            ()
        }
        Result::Err(err) => {
            println!("[ERROR] {:?}", err);
        }
    }
}

fn usage() -> String {
    String::from("cargo run <FILENAME>")
}

//! A small program to make temp mail from a dev env of crates.io readable

extern crate clap;
extern crate rustc_serialize;

use clap::{App, Arg};
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("crateiomailtrans")
        .version("0.1.0")
        .arg(
            Arg::with_name("PATH")
                .required(true)
                .multiple(false)
                .help("Path to a mail to be translated"),
        )
        .get_matches();
    let file_name = matches.value_of("PATH").unwrap();
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
    let json = match Json::from_str(&data) {
        Ok(j) => j,
        Err(e) => panic!("{}", e),
    };
    let message_json = match json.find_path(&["message"]) {
        Some(m) => m,
        None => panic!("No message in file found!"),
    };
    let message_array = if let Json::Array(a) = message_json {
        a
    } else {
        panic!("No array found!");
    };
    let mut message_vec = Vec::<u8>::new();
    for byte in message_array {
        if let Json::U64(b) = byte {
            message_vec.push(*b as u8)
        };
    }
    let message_str = match std::str::from_utf8(&message_vec) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", message_str);
}

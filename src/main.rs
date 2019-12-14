//! A small program to make temp mail from a dev env of crates.io readable

extern crate clap;
extern crate rustc_serialize;

use clap::{App, Arg};
use rustc_serialize::json::Json;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("crateiomailtrans")
        .version("0.1.0")
        .arg(
            Arg::with_name("PATH")
                .required(true)
                .multiple(false)
                .help("Path to a mail to be translated"),
        )
        .get_matches();
    let json = Json::from_str(&read_to_string(matches.value_of("PATH").unwrap())?)?;
    let message_json = json
        .find_path(&["message"])
        .expect("No Message in File found!");
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
    let message_str = std::str::from_utf8(&message_vec)?;
    println!("{}", message_str);
    Ok(())
}

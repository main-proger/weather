extern crate lazy_static;

use std::env;

pub mod utils;
pub mod weather;

use crate::weather::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "save" => {
                Config::parse_args(args).save();
            },
            _ => {
                println!("Unknown comand {}", args[1]);
            },
        }
    } else {
        println!("Arguments count error");
    }
}

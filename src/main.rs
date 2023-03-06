extern crate lazy_static;

use std::env;

pub mod utils;
pub mod weather;

use crate::weather::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "help" => {
                println!("Usage information:
    weather <command> [<name> <param>]
Commands:
    help - view usage information
    save - save default configuration
        example:
            weather save -provider OpenWeather -term_type C
            weather save -address \"your address\"
    get - view weather
        example:
            weather get -address -date now
            weather get -provider OpenWeather -date \"weather date\"
    providers - view all providers
        example:
            weather providers
Configuration:
    provider [OpenWeather] - weather provider
    date [now, <date>] - weather date
    address [<address>] - weather address
    term_type [C, F, K] - temperature type (Fahrenheit, Celsius, Kelvin)");
            },
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

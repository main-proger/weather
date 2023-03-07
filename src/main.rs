extern crate serde_json;
extern crate serde_xml_rs;

use std::env;

pub mod utils;
pub mod weather;

use weather::provider::view_weather_info;
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
            weather save -provider OpenWeather -temp C
            weather save -address \"your address\"
    get - view weather
        example:
            weather get -address -date now
            weather get -day 0 -hour 17
            weather get -date \"1, 17\"
            weather get -provider OpenWeather -date \"weather date\"
    providers - view all providers
        example:
            weather providers
Configuration:
    -provider [OpenWeather, WeatherApi] - weather provider, default OpenWeather
    -date [now, \"<day>, <hour>\"] - weather date, <day> - day for weather from current day, <hour> - hour of day, default now
    -day <day> - weather day, <day> - day for weather from current day, default now
    -hour <hour> - weather date, <hour> - hour of day, default now
    -address <address> - weather address
    -speed [meter, miles] - wind speed type, default meter
    -temp [C, F, K] - temperature type (Fahrenheit, Celsius, Kelvin), default Celsius");
            },
            "save" => {
                Config::parse_args(args).save();
            },
            "get" => {
                let config = Config::parse_args(args);
                view_weather_info(config);
            },
            "providers" => {
                println!("Weather providers:");
                println!("  OpenWeather");
                println!("  WeatherApi");
            },
            _ => {
                println!("Unknown comand {}", args[1]);
            },
        }
    } else {
        println!("Arguments count error");
    }
}

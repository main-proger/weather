extern crate lazy_static;

use std::env;

pub mod utils;
pub mod weather;

use weather::info::WeatherInfo;

use crate::weather::{config::Config, apis::open_weather::OpenWeatherProvider, provider::Provider};

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
            weather get -provider OpenWeather -date \"weather date\"
    providers - view all providers
        example:
            weather providers
Configuration:
    -provider [OpenWeather] - weather provider, default OpenWeather
    -date [now, <date>] - weather date
    -address [<address>] - weather address
    -speed [meter, miles] - wind speed type, default meter
    -temp [C, F, K] - temperature type (Fahrenheit, Celsius, Kelvin), default Celsius");
            },
            "save" => {
                Config::parse_args(args).save();
            },
            "get" => {
                let config = Config::parse_args(args);
                match OpenWeatherProvider::get_info(config) {
                    Ok(info) => {
                        info.print();
                    },
                    Err(_) => {},
                }
            },
            "providers" => {
                println!("Weather providers:");
                println!("  OpenWeather");
            },
            _ => {
                println!("Unknown comand {}", args[1]);
            },
        }
    } else {
        println!("Arguments count error");
    }
}

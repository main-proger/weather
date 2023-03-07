use serde::{Serialize, Deserialize};

use crate::weather::info::WeatherInfo;
use super::{config::Config, apis::{open_weather::OpenWeatherProvider, weather_api::WeatherApiProvider}};

pub trait Provider<I> {
    fn get_info(config: Config) -> Option<I>
    where 
    I: WeatherInfo;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProviderType {
    OpenWeather,
    WeatherApi,
}

impl ProviderType {
    pub fn parse(str: &str) -> Option<ProviderType> {
        match str {
            "OpenWeather" => {
                Some(ProviderType::OpenWeather)
            },
            "WeatherApi" => {
                Some(ProviderType::WeatherApi)
            },
            _ => None
        }
    }
}

pub fn view_weather_info(config: Config) {
    if config.address.is_none() {
        println!("Weather address must be present!");
    } else {
        match config.provider.as_ref().unwrap() {
            ProviderType::OpenWeather => {
                if let Some(info) = OpenWeatherProvider::get_info(config) {
                    info.print();
                    return;
                }
            },
            ProviderType::WeatherApi => {
                if let Some(info) = WeatherApiProvider::get_info(config) {
                    info.print();
                    return;
                }
            },
        }
        println!("Get weather info error");
    }
}

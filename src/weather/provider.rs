use serde::{Serialize, Deserialize};

use crate::weather::info::WeatherInfo;
use super::{config::Config, apis::open_weather::OpenWeatherProvider};

pub trait Provider<I> {
    fn get_info(config: Config) -> Option<I>
    where 
    I: WeatherInfo;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProviderType {
    OpenWeather
}

impl ProviderType {
    pub fn parse(str: &str) -> Option<ProviderType> {
        match str {
            "OpenWeather" => {
                Some(ProviderType::OpenWeather)
            },
            _ => None
        }
    }
}

pub fn get_weather_info(config: Config) -> Option<impl WeatherInfo> {
    match config.provider.as_ref().unwrap() {
        ProviderType::OpenWeather => {
            OpenWeatherProvider::get_info(config)
        },
    }
}
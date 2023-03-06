use serde::{Serialize, Deserialize};

use crate::weather::info::WeatherInfo;
use super::{config::Config, apis::open_weather::OpenWeatherProvider};

pub trait Provider<I> {
    fn get_info(config: Config) -> Result<I, ()>
    where 
    I: WeatherInfo;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProviderType {
    OpenWeather
}

impl ProviderType {
    pub fn parse(str: &str) -> Result<ProviderType, ()> {
        match str {
            "OpenWeather" => {
                Ok(ProviderType::OpenWeather)
            },
            _ => Err(()),
        }
    }
}

impl ProviderType {
    pub fn view_info(&self, config: Config) {
        match self {
            Self::OpenWeather => {
                if let Ok(info) = OpenWeatherProvider::get_info(config) {
                    info.print();
                }
            },
        }
    }
}

impl Clone for ProviderType {
    fn clone(&self) -> Self {
        match self {
            ProviderType::OpenWeather => {
                ProviderType::OpenWeather
            }
        }
    }
}
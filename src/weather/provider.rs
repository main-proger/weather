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

impl ProviderType {
    pub fn view_info(&self, config: Config) {
        match self {
            Self::OpenWeather => {
                if let Some(info) = OpenWeatherProvider::get_info(config) {
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
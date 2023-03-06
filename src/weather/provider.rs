use serde::{Serialize, Deserialize};

use crate::weather::info::WeatherInfo;
use super::config::Config;

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
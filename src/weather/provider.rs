use serde::{Serialize, Deserialize};

use crate::weather::info::WeaterInfo;
use super::config::Config;

trait Provider {
    fn create<P>(config: Config) -> Self;
    fn get_type(&self) -> ProviderType;
    fn get_info<I>(&self, address: &str) -> I
    where 
    I: WeaterInfo;
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
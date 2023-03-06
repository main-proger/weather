use reqwest::{blocking::Response};

use crate::weather::{provider::{Provider}, config::Config, info::{WeatherInfo, Temp}};

struct OpenWeatherProvider;
struct JsonWeatherInfo(serde_json::Value);

impl Provider<JsonWeatherInfo> for OpenWeatherProvider {
    fn get_info(config: Config) -> Result<JsonWeatherInfo, ()> {
        Err(())
    }
}

impl WeatherInfo for JsonWeatherInfo {
    fn temp() -> Option<Temp> {
        None
    }
    fn humidity() -> Option<f64> {
        None
    }
    fn description() -> Option<String> {
        None
    }
    fn address() -> Option<String> {
        None
    }
    fn print() {
    }
}
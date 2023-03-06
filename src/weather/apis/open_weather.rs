use reqwest::{blocking::Response};

use crate::weather::{provider::{Provider}, config::Config, info::{WeatherInfo, Temp, TempType}};

static API_KEY: &'static str = "45cfaf82f367c32a0ae8a1c1f12f7300";
static API_URL: &'static str = "https://api.openweathermap.org/data/2.5/weather?q={CITY}&appid={API_KEY}";

pub struct OpenWeatherProvider;
pub struct JsonWeatherInfo{
    config: Config,
    json: serde_json::Value,
}

impl Provider<JsonWeatherInfo> for OpenWeatherProvider {
    fn get_info(config: Config) -> Result<JsonWeatherInfo, ()> {
        if let Some(address) = config.address.as_ref() {
            let url = API_URL
                .replace("{API_KEY}", API_KEY)
                .replace("{CITY}", address);
            match reqwest::blocking::get(&url) {
                Ok(res) => {
                    match res.json::<serde_json::Value>() {
                        Ok(json) => {
                            return Ok(JsonWeatherInfo{
                                config: config,
                                json: json
                            });
                        },
                        Err(err) => {
                            println!("Provider response data error!");
                        }
                    }
                },
                Err(err) => {
                    println!("Provider request error!");
                }
            }
        } else {
            println!("Weather address must be present!");
        }
        Err(())
    }
}

impl WeatherInfo for JsonWeatherInfo {
    fn temp(&self) -> Option<Temp> {
        Some(Temp {
            temp_min: self.json["main"]["temp_min"].as_f64().unwrap(),
            temp_max: self.json["main"]["temp_max"].as_f64().unwrap(),
            temp_type: TempType::Kelvin,
        }.to_type(self.config.temp_type.as_ref().unwrap()))
    }
    fn humidity(&self) -> Option<f64> {
        Some(self.json["main"]["humidity"].as_f64().unwrap())
    }
    fn description(&self) -> Option<String> {
        Some(self.json["weather"][0]["description"].as_str().unwrap().to_string())
    }
    fn address(&self) -> Option<String> {
        self.config.address.clone()
    }
    fn print(&self) {
        println!("Weather in {}:", self.address().unwrap());
        println!("Description: {}", self.description().unwrap());
        println!("Temp: {}", self.temp().unwrap().to_string());
        println!("Humidity: {}%", self.humidity().unwrap());
    }
}
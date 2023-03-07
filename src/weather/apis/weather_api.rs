use crate::weather::{config::Config, info::{Date, Temp, WeatherInfo, Speed}, provider::Provider};

static API_KEY: &str = "9c7e143cdf584874a59173938230703";
static API_CURRENT_URL: &str = "https://api.weatherapi.com/v1/current.xml?key={API_KEY}&q={CITY}&aqi=no";
static API_FORECAST_URL: &str = "https://api.weatherapi.com/v1/forecast.xml?key={API_KEY}&q={CITY}&days={DAYS}&aqi=no&alerts=no";

#[derive(Debug)]
enum ApiType {
    Current,
    Forecast,
}

pub struct WeatherApiProvider;
pub struct XmlWeatherInfo {
    config: Config,
    api_type: ApiType,
    xml: String,
}

impl ApiType {
    fn from(date: &Date) -> Option<ApiType> {
        if date.day > 10 {
            None
        }else if date.day == 0 {
            Some(ApiType::Current)
        } else {
            Some(ApiType::Forecast)
        }
    }

    fn api_url(&self) -> &str {
        match self {
            ApiType::Current => API_CURRENT_URL,
            ApiType::Forecast => API_FORECAST_URL,
        }
    }

    fn get_url(&self, config: &Config) -> String {
        let url = self.api_url()
            .replace("{API_KEY}", API_KEY)
            .replace("{CITY}", config.address.as_ref().unwrap());
        match self {
            ApiType::Current => {
                url
            },
            ApiType::Forecast => {
                url.replace("{DAYS}", &config.date.as_ref().unwrap().day.to_string())
            },
        }
    }
}


impl Provider<XmlWeatherInfo> for WeatherApiProvider {
    fn get_info(config: Config) -> Option<XmlWeatherInfo> {
        match ApiType::from(config.date.as_ref().unwrap()) {
            Some(api_type) => {
                let url = api_type.get_url(&config);
                match reqwest::blocking::get(url) {
                    Ok(res) => {
                        if res.status() != 200 {
                            println!("Provider response error: Api key or url error!");
                            return None;
                        }
                        match res.json::<serde_json::Value>() {
                            Ok(json) => {
                                return Some(XmlWeatherInfo {
                                    api_type,
                                    config,
                                    xml: "".to_string()
                                });
                            },
                            Err(_) => {
                                println!("Provider response data error!");
                            }
                        }
                    },
                    Err(_) => {
                        println!("Provider request error!");
                    }
                }
            },
            None => {
                println!("Weather day must be less than 10 for this provider!");
            }
        };
        None
    }
}


impl WeatherInfo for XmlWeatherInfo {
    fn temp(&self) -> Option<Temp> {
        None
    }
    fn feels_like(&self) -> Option<Temp> {
        None
    }

    fn wind_speed(&self) -> Option<Speed> {
        None
    }
    fn wind_deg(&self) -> Option<f64> {
        None
    }
    fn wind_gust(&self) -> Option<Speed> {
        None
    }
    
    fn humidity(&self) -> Option<f64> {
        None
    }
    fn pressure(&self) -> Option<f64> {
        None
    }

    fn description(&self) -> Option<String> {
        None
    }
    fn date(&self) -> Option<String> {
        None
    }
    fn address(&self) -> Option<String> {
        self.config.address.clone()
    }
    fn print(&self) {
        match self.api_type {
            ApiType::Current => {
                println!("weather in {}:", self.address().unwrap());
            },
            ApiType::Forecast => {
                println!("weather in {}, on {}:", self.address().unwrap(), self.date().unwrap());
            },
        }
        println!("description: {}", self.description().unwrap());
        println!("main:");
        println!("  temp: {}", self.temp().unwrap().to_string());
        println!("  feels like temp: {}", self.feels_like().unwrap().to_string());
        println!("  humidity: {}%", self.humidity().unwrap());
        println!("  pressure: {}p", self.pressure().unwrap());
        println!("wind:");
        println!("  speed: {}", self.wind_speed().unwrap().to_string());
        println!("  deg: {}", self.wind_deg().unwrap());
        println!("  gust: {}", self.wind_gust().unwrap().to_string());
    }
}
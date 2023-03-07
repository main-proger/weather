use serde_json::Value;

use crate::weather::{config::Config, info::{Date, Temp, WeatherInfo, Speed, SpeedType, TempType}, provider::Provider};

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
    xml: Value,
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
                        match res.text() {
                            Ok(text) => {
                                match serde_xml_rs::from_str::<Value>(&text) {
                                    Ok(xml) => {
                                        return Some(XmlWeatherInfo {
                                            api_type,
                                            config,
                                            xml,
                                        });
                                    },
                                    Err(_) => {
                                        println!("Parse data response from api error!")
                                    }
                                }
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
        let current = self.xml["current"].as_object().unwrap();
        let temp = current["temp_c"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        Some(Temp {
            temp_min: temp,
            temp_max: temp,
            temp_type: TempType::Celsius,
        }.to_type(self.config.temp.as_ref().unwrap()))
    }
    fn feels_like(&self) -> Option<Temp> {
        let current = self.xml["current"].as_object().unwrap();
        let feelslike_c = current["feelslike_c"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        Some(Temp {
            temp_min: feelslike_c,
            temp_max: feelslike_c,
            temp_type: TempType::Celsius,
        }.to_type(self.config.temp.as_ref().unwrap()))
    }

    fn wind_speed(&self) -> Option<Speed> {
        let current = self.xml["current"].as_object().unwrap();
        let speed_kph = current["wind_kph"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        let speed_mps = speed_kph * 1000f64 / 3600f64;
        Some(Speed {
            speed: speed_mps,
            speed_type: SpeedType::Meter,
        }.to_type(self.config.speed.as_ref().unwrap()))
    }
    fn wind_deg(&self) -> Option<f64> {
        let current = self.xml["current"].as_object().unwrap();
        let wind_degree = current["wind_degree"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        Some(wind_degree)
    }
    fn wind_gust(&self) -> Option<Speed> {
        let current = self.xml["current"].as_object().unwrap();
        let gust_kph = current["gust_kph"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        let gust_mps = gust_kph * 1000f64 / 3600f64;
        Some(Speed {
            speed: gust_mps,
            speed_type: SpeedType::Meter,
        }.to_type(self.config.speed.as_ref().unwrap()))
    }
    
    fn humidity(&self) -> Option<f64> {
        let current = self.xml["current"].as_object().unwrap();
        let humidity = current["humidity"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        Some(humidity)
    }
    fn pressure(&self) -> Option<f64> {
        let current = self.xml["current"].as_object().unwrap();
        let pressure_mb = current["pressure_mb"]["$value"].as_str().unwrap().parse::<f64>().unwrap();
        Some(pressure_mb)
    }

    fn description(&self) -> Option<String> {
        None
    }
    fn date(&self) -> Option<String> {
        None
    }
    fn address(&self) -> Option<String> {
        let location = self.xml["location"].as_object().unwrap();
        let name = location["name"]["$value"].as_str().unwrap();
        let region = location["region"]["$value"].as_str().unwrap();
        let country = location["country"]["$value"].as_str().unwrap();
        Some(format!("{country}, {region}, {name}"))
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
        println!("main:");
        println!("  temp: {}", self.temp().unwrap().to_string());
        println!("  feels like temp: {}", self.feels_like().unwrap().to_string());
        println!("  wind speed: {}", self.wind_speed().unwrap().to_string());
        println!("  humidity: {}%", self.humidity().unwrap());
        println!("  pressure: {}p", self.pressure().unwrap());
        println!("wind:");
        println!("  speed: {}", self.wind_speed().unwrap().to_string());
        println!("  degree: {}°", self.wind_deg().unwrap());
        println!("  gust: {}", self.wind_gust().unwrap().to_string());
    }
}
use crate::weather::{provider::{Provider}, config::Config, info::{WeatherInfo, Temp, TempType, Speed, SpeedType, Date}};

static API_KEY: &str = "101b778ea4a1c56d89d54aecf4541207";
static API_CURRENT_URL: &str = "https://api.openweathermap.org/data/2.5/weather?q={CITY}&appid={API_KEY}";
static API_HOURLY_URL: &str = "api.openweathermap.org/data/2.5/forecast?q={CITY}&cnt={CNT}&appid={API_KEY}";
static API_DAILY_URL: &str = "https://api.openweathermap.org/data/2.5/forecast/daily?q={CITY}&cnt={CNT}&appid={API_KEY}";

#[derive(Debug)]
enum ApiType {
    Current,
    Hourly,
    Daily,
}

pub struct OpenWeatherProvider;
pub struct JsonWeatherInfo {
    config: Config,
    api_type: ApiType,
    json: serde_json::Value,
}

impl ApiType {
    fn from(date: &Date) -> Option<ApiType> {
        if date.day > 15 {
            None
        }else if date.hours.is_none() {
            if date.day == 0 {
                Some(ApiType::Current)
            } else if date.day < 5 {
                Some(ApiType::Hourly)
            } else {
                Some(ApiType::Hourly)
            }
        } else if date.day <= 5 {
            Some(ApiType::Hourly)
        } else {
            Some(ApiType::Daily)
        }
    }

    fn api_url(&self) -> &str {
        match self {
            ApiType::Current => API_CURRENT_URL,
            ApiType::Hourly => API_HOURLY_URL,
            ApiType::Daily => API_DAILY_URL,
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
            ApiType::Daily => {
                url.replace("{CNT}", &(config.date.as_ref().unwrap().day + 1).to_string())
            },
            ApiType::Hourly => {
                url.replace("{CNT}", &(config.date.as_ref().unwrap().day + 1).to_string())
            },
        }
    }
}

impl Provider<JsonWeatherInfo> for OpenWeatherProvider {
    fn get_info(config: Config) -> Option<JsonWeatherInfo> {
        match ApiType::from(config.date.as_ref().unwrap()) {
            Some(api_type) => {
                let url = api_type.get_url(&config);
                match reqwest::blocking::get(url) {
                    Ok(res) => {
                        if res.status() != 200 {
                            println!("Provider response error: Api key or url error!");
                            println!("Api key is payment!");
                            return None;
                        }
                        match res.json::<serde_json::Value>() {
                            Ok(json) => {
                                return Some(JsonWeatherInfo{
                                    api_type,
                                    config,
                                    json
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
                println!("Weather day must be less than 16 for this provider!");
            }
        };
        None
    }
}

impl WeatherInfo for JsonWeatherInfo {
    fn temp(&self) -> Option<Temp> {
        match self.api_type {
            ApiType::Current => {
                Some(Temp {
                    temp_min: self.json["main"]["temp_min"].as_f64().unwrap(),
                    temp_max: self.json["main"]["temp_max"].as_f64().unwrap(),
                    temp: TempType::Kelvin,
                }.to_type(self.config.temp.as_ref().unwrap()))
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(Temp {
                    temp_min: info["temp"]["min"].as_f64().unwrap(),
                    temp_max: self.json["temp"]["max"].as_f64().unwrap(),
                    temp: TempType::Kelvin,
                }.to_type(self.config.temp.as_ref().unwrap()))
            },
        }
    }
    fn feels_like(&self) -> Option<Temp> {
        match self.api_type {
            ApiType::Current => {
                let time = self.json["main"]["feels_like"].as_f64().unwrap();
                Some(Temp {
                    temp_min: time,
                    temp_max: time,
                    temp: TempType::Kelvin,
                }.to_type(self.config.temp.as_ref().unwrap()))
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                let time = info["feels_like"]["day"].as_f64().unwrap();
                Some(Temp {
                    temp_min: time,
                    temp_max: time,
                    temp: TempType::Kelvin,
                }.to_type(self.config.temp.as_ref().unwrap()))
            },
        }
    }

    fn wind_speed(&self) -> Option<Speed> {
        match self.api_type {
            ApiType::Current => {
                Some(Speed {
                    speed: self.json["wind"]["speed"].as_f64().unwrap(),
                    speed_type: SpeedType::Meter,
                }.to_type(self.config.speed.as_ref().unwrap()))
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(Speed {
                    speed: info["speed"].as_f64().unwrap(),
                    speed_type: SpeedType::Meter,
                }.to_type(self.config.speed.as_ref().unwrap()))
            },
        }
    }
    fn wind_deg(&self) -> Option<f64> {
        match self.api_type {
            ApiType::Current => {
                Some(self.json["wind"]["deg"].as_f64().unwrap())
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(info["deg"].as_f64().unwrap())
            },
        }
    }
    fn wind_gust(&self) -> Option<Speed> {
        match self.api_type {
            ApiType::Current => {
                Some(Speed {
                    speed: self.json["wind"]["gust"].as_f64().unwrap(),
                    speed_type: SpeedType::Meter,
                }.to_type(self.config.speed.as_ref().unwrap()))
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(Speed {
                    speed: info["gust"].as_f64().unwrap(),
                    speed_type: SpeedType::Meter,
                }.to_type(self.config.speed.as_ref().unwrap()))
            },
        }
    }
    
    fn humidity(&self) -> Option<f64> {
        match self.api_type {
            ApiType::Current => {
                Some(self.json["main"]["humidity"].as_f64().unwrap())
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(info["humidity"].as_f64().unwrap())
            },
        }
    }
    fn pressure(&self) -> Option<f64> {
        match self.api_type {
            ApiType::Current => {
                Some(self.json["main"]["pressure"].as_f64().unwrap())
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(info["pressure"].as_f64().unwrap())
            },
        }
    }

    fn description(&self) -> Option<String> {
        match self.api_type {
            ApiType::Current => {
                Some(self.json["weather"][0]["description"].as_str().unwrap().to_string())
            },
            ApiType::Hourly => {
                None
            },
            ApiType::Daily => {
                println!("{:?}", self.json);
                let info = self.json["list"][(self.json["cnt"].as_u64().unwrap() - 1u64) as usize].as_object().unwrap();
                Some(info["weather"][0]["description"].as_str().unwrap().to_string())
            },
        }
    }
    fn address(&self) -> Option<String> {
        self.config.address.clone()
    }
    fn print(&self) {
        match self.api_type {
            ApiType::Current => {
                println!("weather in {}:", self.address().unwrap());
            },
            ApiType::Hourly => {
                println!("weather in {}, on {}:", self.address().unwrap(), "");
            },
            ApiType::Daily => {
                println!("weather in {}, on day {}:", self.address().unwrap(), self.config.date.as_ref().unwrap().day);
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
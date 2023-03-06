use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::{fs::File, io::Write, io::Read};

use super::{info::{TempType, SpeedType}, provider::ProviderType};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub date: Option<String>,
    pub address: Option<String>,
    pub temp: Option<TempType>,
    pub speed: Option<SpeedType>,
    pub provider: Option<ProviderType>,
}

enum Error {
    JsonToString,
    CreateFile,
    WriteFile,
    OpenFile,
    ReadFile,
    StringToJson,
}

impl Default for Config {
    fn default() -> Self {
        match read_json("config.json") {
            Ok(config) => {
                let mut config: Self = config;

                config.date = None;
                if let None = config.temp {
                    config.temp = Some(TempType::Celsius);
                }
                if let None = config.speed {
                    config.speed = Some(SpeedType::Meter);
                }
                if let None = config.provider {
                    config.provider = Some(ProviderType::OpenWeather);
                }

                config
            },
            Err(err) => {
                Config {
                    date: None,
                    address: None,
                    temp: Some(TempType::Celsius),
                    speed: Some(SpeedType::Meter),
                    provider: Some(ProviderType::OpenWeather),
                }
            },
        }
    }
}

impl Config {
    pub fn set_param(&mut self, name: String, param: String) -> Result<(), String> {
        match name.as_str() {
            "-date" => {
                self.date = if param == "now" {
                    None
                } else {
                    Some(param)
                }
            },
            "-address" => {
                self.address = Some(param);
            },
            "-temp" => {
                self.temp = match TempType::parse(&param) {
                    Ok(res) => Some(res),
                    Err(err) => {
                        return Err(String::from("Argument 'temp' error, value must be (F, K, C)"));
                    },
                };
            },
            "-speed" => {
                self.speed = match SpeedType::parse(&param) {
                    Ok(res) => Some(res),
                    Err(err) => {
                        return Err(String::from("Argument 'speed' error, value must be (meter, miles)"));
                    },
                };
            },
            "-provider" => {
                self.provider = match ProviderType::parse(&param) {
                    Ok(res) => Some(res),
                    Err(err) => {
                        return Err(String::from("Argument 'provider' error, value must be (OpenWeather)"));
                    },
                };
            },
            _ => {
                return Err(format!("Unknown argument '{name}'"));
            }
        }
        Ok(())
    }

    pub fn parse_args(args: Vec<String>) -> Self {
        let args = args.split_at(2).1;
    
        if args.len() % 2 != 0 {
            println!("args error");
        }
        
        let mut i = 0;
    
        let mut config = Config::default();
    
        while i < args.len() {
            let key = args[i].clone();
            let value = args[i + 1].clone();
    
            if let Err(err) = config.set_param(key, value) {
                println!("{err}");
            }
    
            i += 2;
        }

        config
    }

    pub fn save(&self) {
        if let Err(err) = save_json("config.json", self) {
            match err {
                Error::CreateFile => {
                    println!("Create config file error");
                },
                Error::WriteFile => {
                    println!("Save config to file error");
                },
                _ => {},
            }
        }
    } 
}

fn save_json<J>(name: &str, json: &J) -> Result<(), Error>
    where
    J: Serialize
{
    let data = match serde_json::to_string(json) {
        Ok(res) => res,
        Err(err) => {
            return Err(Error::JsonToString);
        }
    };

    let mut file = match File::create(name) {
        Ok(file) => file,
        Err(err) => {
            return Err(Error::CreateFile);
        }
    };
    if let Err(err) = file.write_all(data.as_bytes()) {
        return Err(Error::WriteFile);
    }
    Ok(())
}

fn read_json<J>(name: &str) -> Result<J, Error>
    where
    J: DeserializeOwned
{
    let mut data: Vec<u8> = vec![];

    let mut file = match File::open(name) {
        Ok(file) => file,
        Err(err) => {
            return Err(Error::OpenFile);
        },
    };
    if let Err(err) = file.read_to_end(&mut data) {
        return Err(Error::ReadFile);
    }

    let data = data.as_slice();

    match serde_json::from_slice(data) {
        Err(err) => {
            return Err(Error::StringToJson);
        },
        Ok(res) => Ok(res),
    }
}
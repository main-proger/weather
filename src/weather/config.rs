use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::{fs::File, io::Write, io::Read};

use super::{info::TempType, provider::ProviderType};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub date: Option<String>,
    pub address: Option<String>,
    pub temp_type: Option<TempType>,
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
                if let None = config.temp_type {
                    config.temp_type = Some(TempType::Celsius);
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
                    temp_type: Some(TempType::Celsius),
                    provider: Some(ProviderType::OpenWeather),
                }
            },
        }
    }
}

impl Config {
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

pub fn save_json<J>(name: &str, json: &J) -> Result<(), Error>
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

pub fn read_json<J>(name: &str) -> Result<J, Error>
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
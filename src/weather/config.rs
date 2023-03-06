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
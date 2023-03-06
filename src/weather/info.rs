use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::utils::temp::{ftoc, ktoc, ftok, ctof, ktof};

pub trait WeatherInfo {
    fn temp(&self) -> Option<Temp>;
    fn humidity(&self) -> Option<f64>;
    fn description(&self) -> Option<String>;
    fn address(&self) -> Option<String>;
    fn print(&self);
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TempType {
    Fahrenheit,
    Kelvin,
    Celsius,
}

pub struct Temp {
    pub temp_min: f64,
    pub temp_max: f64,
    pub temp_type: TempType,
}


impl ToString for TempType {
    fn to_string(&self) -> String {
        match self {
            TempType::Fahrenheit => {
                String::from("°F")
            },
            TempType::Kelvin => {
                String::from("°K")
            },
            TempType::Celsius => {
                String::from("°C")
            },
        }
    }
}

impl ToString for Temp {
    fn to_string(&self) -> String {
        if self.temp_max == self.temp_min {
            format!("{:5} {}", (self.temp_max * 100f64).round() / 100f64, self.temp_type.to_string())
        } else {
            format!("[{:5} : {:5}] {}", (self.temp_min * 100f64).round() / 100f64, (self.temp_max * 100f64).round() / 100f64, self.temp_type.to_string())
        }
    }
}

impl Temp {
    pub fn to_celsius(&self) -> Temp {
        let celsius = match self.temp_type {
            TempType::Fahrenheit => (ftoc(self.temp_min), ftoc(self.temp_max)),
            TempType::Celsius => (self.temp_min, self.temp_max),
            TempType::Kelvin => (ktoc(self.temp_min), ktoc(self.temp_max)),
        };

        Temp {
            temp_min: celsius.0,
            temp_max: celsius.1,
            temp_type: TempType::Celsius,
        }
    }

    pub fn to_kelvin(&self) -> Temp {
        let kelvin = match self.temp_type {
            TempType::Fahrenheit => (ftok(self.temp_min), ftok(self.temp_max)),
            TempType::Celsius => (ftok(self.temp_min), ftok(self.temp_max)),
            TempType::Kelvin => (self.temp_min, self.temp_max),
        };

        Temp {
            temp_min: kelvin.0,
            temp_max: kelvin.1,
            temp_type: TempType::Kelvin,
        }
    }

    pub fn to_fahrenheit(&self) -> Temp {
        let fahrenheit = match self.temp_type {
            TempType::Fahrenheit => (self.temp_min, self.temp_max),
            TempType::Celsius => (ctof(self.temp_min), ctof(self.temp_max)),
            TempType::Kelvin => (ktof(self.temp_min), ktof(self.temp_max)),
        };

        Temp {
            temp_min: fahrenheit.0,
            temp_max: fahrenheit.1,
            temp_type: TempType::Fahrenheit,
        }
    }

    pub fn to_type(&self, temp_type: &TempType) -> Temp {
        match temp_type {
            TempType::Fahrenheit => self.to_fahrenheit(),
            TempType::Celsius => self.to_celsius(),
            TempType::Kelvin => self.to_kelvin(),
        }
    }
}

impl TempType {
    pub fn parse(str: &str) -> Result<TempType, ()> {
        match str {
            "F" => Ok(TempType::Fahrenheit),
            "C" => Ok(TempType::Celsius),
            "K" => Ok(TempType::Kelvin),
            _ => Err(()),
        }
    }
}
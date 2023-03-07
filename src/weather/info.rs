use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::utils::temp::{ftoc, ktoc, ftok, ctof, ktof};

pub trait WeatherInfo {
    fn temp(&self) -> Option<Temp>;
    fn feels_like(&self) -> Option<Temp>;
    
    fn humidity(&self) -> Option<f64>;
    fn pressure(&self) -> Option<f64>;

    fn wind_speed(&self) -> Option<Speed>;
    fn wind_deg(&self) -> Option<f64>;
    fn wind_gust(&self) -> Option<Speed>;

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
    pub temp: TempType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpeedType {
    Meter,
    Miles,
}

pub struct Speed {
    pub speed: f64,
    pub speed_type: SpeedType,
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
            format!("{:5} {}", (self.temp_max * 100f64).round() / 100f64, self.temp.to_string())
        } else {
            format!("[{:5} : {:5}] {}", (self.temp_min * 100f64).round() / 100f64, (self.temp_max * 100f64).round() / 100f64, self.temp.to_string())
        }
    }
}

impl ToString for SpeedType {
    fn to_string(&self) -> String {
        match self {
            SpeedType::Meter => {
                String::from("meter/sec")
            },
            SpeedType::Miles => {
                String::from("miles/sec")
            },
        }
    }
}

impl ToString for Speed {
    fn to_string(&self) -> String {
        format!("{} {}", (self.speed * 100000f64).round() / 100000f64, self.speed_type.to_string())
    }
}

impl Temp {
    pub fn to_celsius(&self) -> Temp {
        let celsius = match self.temp {
            TempType::Fahrenheit => (ftoc(self.temp_min), ftoc(self.temp_max)),
            TempType::Celsius => (self.temp_min, self.temp_max),
            TempType::Kelvin => (ktoc(self.temp_min), ktoc(self.temp_max)),
        };

        Temp {
            temp_min: celsius.0,
            temp_max: celsius.1,
            temp: TempType::Celsius,
        }
    }

    pub fn to_kelvin(&self) -> Temp {
        let kelvin = match self.temp {
            TempType::Fahrenheit => (ftok(self.temp_min), ftok(self.temp_max)),
            TempType::Celsius => (ftok(self.temp_min), ftok(self.temp_max)),
            TempType::Kelvin => (self.temp_min, self.temp_max),
        };

        Temp {
            temp_min: kelvin.0,
            temp_max: kelvin.1,
            temp: TempType::Kelvin,
        }
    }

    pub fn to_fahrenheit(&self) -> Temp {
        let fahrenheit = match self.temp {
            TempType::Fahrenheit => (self.temp_min, self.temp_max),
            TempType::Celsius => (ctof(self.temp_min), ctof(self.temp_max)),
            TempType::Kelvin => (ktof(self.temp_min), ktof(self.temp_max)),
        };

        Temp {
            temp_min: fahrenheit.0,
            temp_max: fahrenheit.1,
            temp: TempType::Fahrenheit,
        }
    }

    pub fn to_type(&self, temp: &TempType) -> Temp {
        match temp {
            TempType::Fahrenheit => self.to_fahrenheit(),
            TempType::Celsius => self.to_celsius(),
            TempType::Kelvin => self.to_kelvin(),
        }
    }
}

impl TempType {
    pub fn parse(str: &str) -> Option<TempType> {
        match str {
            "F" => Some(TempType::Fahrenheit),
            "C" => Some(TempType::Celsius),
            "K" => Some(TempType::Kelvin),
            _ => None,
        }
    }
}

impl Speed {
    pub fn to_meter(self) -> Self {
        match self.speed_type {
            SpeedType::Meter => self,
            SpeedType::Miles => Self {
                speed: self.speed * 1609.34f64,
                speed_type: SpeedType::Meter,
            },
        }
    }
    pub fn to_miles(self) -> Self {
        match self.speed_type {
            SpeedType::Miles => self,
            SpeedType::Meter => Self {
                speed: self.speed / 1609.34f64,
                speed_type: SpeedType::Miles,
            },
        }
    }
    pub fn to_type(self, speed_type: &SpeedType) -> Speed {
        match speed_type {
            SpeedType::Miles => self.to_miles(),
            SpeedType::Meter => self.to_meter(),
        }
    }
}

impl SpeedType {
    pub fn parse(str: &str) -> Option<Self> {
        match str {
            "meter" => Some(SpeedType::Meter),
            "miles" => Some(SpeedType::Miles),
            _ => None,
        }
    }
}
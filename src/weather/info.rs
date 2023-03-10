use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::utils::temp::{ctof, ftoc, ftok, ktoc, ktof};

pub trait WeatherInfo {
    fn temp(&self) -> Option<Temp>;
    fn feels_like(&self) -> Option<Temp>;

    fn humidity(&self) -> Option<f64>;
    fn pressure(&self) -> Option<f64>;

    fn wind_speed(&self) -> Option<Speed>;
    fn wind_deg(&self) -> Option<f64>;
    fn wind_gust(&self) -> Option<Speed>;

    fn description(&self) -> Option<String>;
    fn date(&self) -> Option<String>;
    fn address(&self) -> Option<String>;

    fn print(&self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    pub day: u64,
    pub hours: Option<u64>,
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
            TempType::Fahrenheit => String::from("°F"),
            TempType::Kelvin => String::from("°K"),
            TempType::Celsius => String::from("°C"),
        }
    }
}

impl ToString for Temp {
    fn to_string(&self) -> String {
        if self.temp_max == self.temp_min {
            format!(
                "{} {}",
                (self.temp_max * 100f64).round() / 100f64,
                self.temp_type.to_string()
            )
        } else {
            format!(
                "[{} : {}] {}",
                (self.temp_min * 100f64).round() / 100f64,
                (self.temp_max * 100f64).round() / 100f64,
                self.temp_type.to_string()
            )
        }
    }
}

impl ToString for SpeedType {
    fn to_string(&self) -> String {
        match self {
            SpeedType::Meter => String::from("meter/sec"),
            SpeedType::Miles => String::from("miles/hour"),
        }
    }
}

impl ToString for Speed {
    fn to_string(&self) -> String {
        format!(
            "{} {}",
            (self.speed * 100000f64).round() / 100000f64,
            self.speed_type.to_string()
        )
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        if self.hours.is_none() {
            format!("{} day", self.day)
        } else {
            format!("{} day, {} hour", self.day, self.hours.unwrap())
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

        Self {
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

        Self {
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

        Self {
            temp_min: fahrenheit.0,
            temp_max: fahrenheit.1,
            temp_type: TempType::Fahrenheit,
        }
    }

    pub fn convert_to(&self, temp: &TempType) -> Temp {
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
    pub fn to_meter(&self) -> Self {
        let speed = match self.speed_type {
            SpeedType::Meter => self.speed,
            SpeedType::Miles => self.speed / 3600f64 * 1609.34f64,
        };
        Self {
            speed,
            speed_type: SpeedType::Meter,
        }
    }
    pub fn to_miles(&self) -> Self {
        let speed = match self.speed_type {
            SpeedType::Miles => self.speed,
            SpeedType::Meter => self.speed * 3600f64 / 1609.34f64,
        };
        Self {
            speed,
            speed_type: SpeedType::Miles,
        }
    }
    pub fn convert_to(&self, speed_type: &SpeedType) -> Speed {
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

impl Date {
    pub fn parse(str: &str) -> Option<Self> {
        let str = str.replace(' ', "");
        let arr = str.split(',').collect::<Vec<&str>>();
        if arr.len() != 2 {
            None
        } else {
            let day = match arr[0].parse::<u64>() {
                Ok(res) => res,
                Err(_) => {
                    return None;
                }
            };
            let hours = match arr[1].parse::<u64>() {
                Ok(res) => res,
                Err(_) => {
                    return None;
                }
            };
            if hours >= 24 {
                return None;
            }
            Some(Self {
                day,
                hours: Some(hours),
            })
        }
    }
}

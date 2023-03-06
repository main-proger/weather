use crate::utils::temp::{ftoc, ktoc, ftok, ctof, ktof};

pub trait WeaterInfo {
    fn temp() -> Option<Temp>;
    fn humidity() -> Option<f64>;
    fn description() -> Option<String>;
    fn address() -> Option<String>;
    fn print();
}

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

pub struct Address {
    pub country: Option<String>,
    pub address: String,
}

pub struct Humidity(f64);



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
            format!("{} {}", self.temp_max, self.temp_type.to_string())
        } else {
            format!("[{} : {}] {}", self.temp_min, self.temp_max, self.temp_type.to_string())
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        if let Some(country) = self.country.as_ref() {
            format!("{}, {}", country, self.address)
        } else {
            format!("{}", self.address)
        }
    }
}

impl ToString for Humidity {
    fn to_string(&self) -> String {
        format!("{}%", self.0)
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
}
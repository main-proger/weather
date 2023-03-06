pub trait WeaterInfo {
    fn temp() -> Option<Temp>;
    fn humidity() -> Option<f64>;
    fn description() -> Option<String>;
    fn address() -> Option<String>;
    fn print();
}

pub enum TempType {
    Farengate,
    Kelvin,
    Celsius,
}

pub struct Temp {
    pub temp_max: f64,
    pub temp_min: f64,
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
            TempType::Farengate => {
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
            format!("{}-{} {}", self.temp_max, self.temp_min, self.temp_type.to_string())
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
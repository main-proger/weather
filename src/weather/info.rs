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
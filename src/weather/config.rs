use super::info::TempType;

struct Config {
    time: Option<i64>,
    address: bool,
    temp: bool,
    temp_type: TempType,
    humidity: bool,
    description: bool,
}
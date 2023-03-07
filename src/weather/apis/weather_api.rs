use crate::weather::{config::Config, info::Date};

static API_KEY: &str = "9c7e143cdf584874a59173938230703";
static API_CURRENT_URL: &str = "https://api.weatherapi.com/v1/current.xml?key={API_KEY}&q={CITY}&aqi=no";
static API_FORECAST_URL: &str = "https://api.weatherapi.com/v1/forecast.xml?key={API_KEY}&q={CITY}&days={DAYS}&aqi=no&alerts=no";

#[derive(Debug)]
enum ApiType {
    Current,
    Forecast,
}

pub struct WeatherApiProvider;
pub struct XmlWeatherInfo {
    config: Config,
    api_type: ApiType,
    json: String,
}

impl ApiType {
    fn from(date: &Date) -> Option<ApiType> {
        if date.day > 10 {
            None
        }else if date.day == 0 {
            Some(ApiType::Current)
        } else {
            Some(ApiType::Forecast)
        }
    }

    fn api_url(&self) -> &str {
        match self {
            ApiType::Current => API_CURRENT_URL,
            ApiType::Forecast => API_FORECAST_URL,
        }
    }

    fn get_url(&self, config: &Config) -> String {
        let url = self.api_url()
            .replace("{API_KEY}", API_KEY)
            .replace("{CITY}", config.address.as_ref().unwrap());
        match self {
            ApiType::Current => {
                url
            },
            ApiType::Forecast => {
                url.replace("{DAYS}", &config.date.as_ref().unwrap().day.to_string())
            },
        }
    }
}
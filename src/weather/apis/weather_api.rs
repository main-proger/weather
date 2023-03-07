use crate::weather::config::Config;

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
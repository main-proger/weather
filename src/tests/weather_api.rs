#[cfg(test)]
mod weather_api_provider {
    use crate::weather::apis::open_weather::OpenWeatherProvider;
    use crate::weather::config::Config;
    use crate::weather::info::Date;
    use crate::weather::info::SpeedType;
    use crate::weather::info::TempType;
    use crate::weather::info::WeatherInfo;
    use crate::weather::provider::Provider;
    use crate::weather::provider::ProviderType;

    #[test]
    fn get_current_info() {
        let config = Config {
            date: Some(Date {
                day: 0,
                hours: None,
            }),
            address: Some(String::from("Ukraine, Kyiv")),
            temp: Some(TempType::Celsius),
            speed: Some(SpeedType::Meter),
            provider: Some(ProviderType::OpenWeather),
        };
        let info = OpenWeatherProvider::get_info(config);
        assert!(info.is_some());
        let info = info.unwrap();
        assert!(info.temp().is_some());
        assert!(info.feels_like().is_some());
        assert!(info.humidity().is_some());
        assert!(info.pressure().is_some());
        assert!(info.wind_speed().is_some());
        assert!(info.wind_deg().is_some());
        assert!(info.wind_gust().is_some());
    }

    #[test]
    fn get_forecast_info() {
        let config = Config {
            date: Some(Date {
                day: 3,
                hours: Some(12),
            }),
            address: Some(String::from("Ukraine, Kyiv")),
            temp: Some(TempType::Celsius),
            speed: Some(SpeedType::Meter),
            provider: Some(ProviderType::OpenWeather),
        };
        let info = OpenWeatherProvider::get_info(config);
        assert!(info.is_some());
        let info = info.unwrap();
        assert!(info.temp().is_some());
        assert!(info.feels_like().is_some());
        assert!(info.humidity().is_some());
        assert!(info.pressure().is_some());
        assert!(info.wind_speed().is_some());
        assert!(info.wind_deg().is_some());
        assert!(info.wind_gust().is_some());
    }
}

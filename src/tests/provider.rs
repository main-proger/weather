#[cfg(test)]
mod provider {
    use crate::weather::provider::ProviderType;

    #[test]
    fn parse_provider_bad() {
        let provider = ProviderType::parse("bad");
        assert!(provider.is_none());
    }
    #[test]
    fn parse_provider_open_weather() {
        let provider = ProviderType::parse("OpenWeather");
        assert!(provider.is_some());
        assert!(match provider.unwrap() {
            ProviderType::OpenWeather => true,
            _ => false,
        });
    }
    #[test]
    fn parse_provider_weather_api() {
        let provider = ProviderType::parse("WeatherApi");
        assert!(provider.is_some());
        assert!(match provider.unwrap() {
            ProviderType::WeatherApi => true,
            _ => false,
        });
    }
}

#[cfg(test)]
mod config {
    use crate::weather::config::Config;
    use crate::weather::info::Date;
    use crate::weather::info::SpeedType;
    use crate::weather::info::TempType;
    use crate::weather::provider::ProviderType;

    fn save_default_config() {
        let save_config = Config {
            date: Some(Date::parse("1, 2").unwrap()),
            address: Some(String::from("test")),
            temp: Some(TempType::Kelvin),
            speed: Some(SpeedType::Miles),
            provider: Some(ProviderType::WeatherApi),
        };
        save_config.save();
    }

    #[test]
    fn save_and_load_config() {
        save_default_config();

        let load_config = Config::default();
        assert!(load_config.date.is_some());
        assert!(load_config.address.is_some());
        assert!(load_config.temp.is_some());
        assert!(load_config.speed.is_some());
        assert!(load_config.provider.is_some());

        let date = load_config.date.unwrap();
        assert_eq!(date.day, 1);
        assert!(date.hours.is_some());
        assert_eq!(date.hours.unwrap(), 2);

        assert_eq!(load_config.address.unwrap(), "test");

        assert!(match load_config.temp.unwrap() {
            TempType::Kelvin => true,
            _ => false,
        });
        assert!(match load_config.speed.unwrap() {
            SpeedType::Miles => true,
            _ => false,
        });
        assert!(match load_config.provider.unwrap() {
            ProviderType::WeatherApi => true,
            _ => false,
        });
    }

    #[test]
    fn from_args_bad() {
        save_default_config();
        Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-bad".to_string(),
            "bad".to_string(),
        ]);
        // in logs must be "Unknown argument '-bad'"
    }

    #[test]
    fn from_args_date_bad() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-date".to_string(),
            "bad".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert_eq!(date.day, 1);
    }
    #[test]
    fn from_args_date_now() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-date".to_string(),
            "now".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert_eq!(date.day, 0);
        assert!(date.hours.is_none());
    }
    #[test]
    fn from_args_date_format() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-date".to_string(),
            "2, 1".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert_eq!(date.day, 2);
        assert!(date.hours.is_some());
        assert_eq!(date.hours.unwrap(), 1);
    }

    #[test]
    fn from_args_day_bad() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-day".to_string(),
            "bad".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert_eq!(date.day, 1);
    }
    #[test]
    fn from_args_day() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-day".to_string(),
            "2".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert_eq!(date.day, 2);
    }

    #[test]
    fn from_args_hour_bad() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-hour".to_string(),
            "bad".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert!(date.hours.is_some());
        assert_eq!(date.hours.unwrap(), 2);
    }
    #[test]
    fn from_args_hour_boundary() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-hour".to_string(),
            "24".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert!(date.hours.is_some());
        assert_eq!(date.hours.unwrap(), 2);
    }
    #[test]
    fn from_args_hour() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-hour".to_string(),
            "1".to_string(),
        ]);
        assert!(config.date.is_some());
        let date = config.date.unwrap();
        assert!(date.hours.is_some());
        assert_eq!(date.hours.unwrap(), 1);
    }

    #[test]
    fn from_args_address() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-address".to_string(),
            "test2".to_string(),
        ]);
        assert!(config.address.is_some());
        assert_eq!(config.address.unwrap(), "test2");
    }

    #[test]
    fn from_args_temp_bad() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-temp".to_string(),
            "T".to_string(),
        ]);
        assert!(config.temp.is_some());
        assert!(match config.temp.unwrap() {
            TempType::Kelvin => true,
            _ => false,
        });
    }
    #[test]
    fn from_args_temp() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-temp".to_string(),
            "F".to_string(),
        ]);
        assert!(config.temp.is_some());
        assert!(match config.temp.unwrap() {
            TempType::Fahrenheit => true,
            _ => false,
        });
    }

    #[test]
    fn from_args_speed_bad() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-speed".to_string(),
            "test".to_string(),
        ]);
        assert!(config.speed.is_some());
        assert!(match config.speed.unwrap() {
            SpeedType::Miles => true,
            _ => false,
        });
    }
    #[test]
    fn from_args_speed() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-speed".to_string(),
            "meter".to_string(),
        ]);
        assert!(config.speed.is_some());
        assert!(match config.speed.unwrap() {
            SpeedType::Meter => true,
            _ => false,
        });
    }

    #[test]
    fn from_args_provider_bad() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-provider".to_string(),
            "Test".to_string(),
        ]);
        assert!(config.provider.is_some());
        assert!(match config.provider.unwrap() {
            ProviderType::WeatherApi => true,
            _ => false,
        });
    }
    #[test]
    fn from_args_provider() {
        save_default_config();
        let config = Config::parse_args(vec![
            "cmd".to_string(),
            "get".to_string(),
            "-provider".to_string(),
            "OpenWeather".to_string(),
        ]);
        assert!(config.provider.is_some());
        assert!(match config.provider.unwrap() {
            ProviderType::OpenWeather => true,
            _ => false,
        });
    }
}

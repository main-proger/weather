#[cfg(test)]
mod date {
    use crate::weather::info::Date;

    #[test]
    fn parse_date_empty() {
        let date = Date::parse("");
        assert!(date.is_none());
    }
    #[test]
    fn parse_date_no_number() {
        let date = Date::parse("a, b");
        assert!(date.is_none());
    }
    #[test]
    fn parse_date_bad_hour() {
        let date = Date::parse("1, 24");
        assert!(date.is_none());
    }
    #[test]
    fn parse_date_true() {
        let date = Date::parse("1, 23");
        assert!(date.is_some());
    }
    #[test]
    fn to_string_hour_now() {
        let date = Date {
            day: 1,
            hours: None,
        };
        assert_eq!(date.to_string(), "1 day");
    }
    #[test]
    fn to_string() {
        let date = Date {
            day: 1,
            hours: Some(1),
        };
        assert_eq!(date.to_string(), "1 day, 1 hour");
    }
}

#[cfg(test)]
mod temp {
    use crate::weather::info::Temp;
    use crate::weather::info::TempType;

    #[test]
    fn parse_temp_type_bad() {
        let temp_type = TempType::parse("bad");
        assert!(temp_type.is_none());
    }

    #[test]
    fn parse_temp_type_c() {
        let temp_type = TempType::parse("C");
        assert!(temp_type.is_some());
        assert!(match temp_type.unwrap() {
            TempType::Celsius => true,
            _ => false,
        });
    }
    #[test]
    fn parse_temp_type_f() {
        let temp_type = TempType::parse("F");
        assert!(temp_type.is_some());
        assert!(match temp_type.unwrap() {
            TempType::Fahrenheit => true,
            _ => false,
        });
    }
    #[test]
    fn parse_temp_type_k() {
        let temp_type = TempType::parse("K");
        assert!(temp_type.is_some());
        assert!(match temp_type.unwrap() {
            TempType::Kelvin => true,
            _ => false,
        });
    }

    #[test]
    fn convert_temp() {
        let types = vec!["C", "F", "K"];
        let results = vec![
            vec![1f64, 33.8f64, 255.92777777777775f64],
            vec![-17.22222222222222f64, 1f64, 255.92777777777775f64],
            vec![-272.15f64, -457.87f64, 1f64],
        ];

        for i in 0..types.len() {
            let from_temp = Temp {
                temp_min: 1f64,
                temp_max: 1f64,
                temp_type: TempType::parse(types[i]).unwrap(),
            };
            for j in 0..types.len() {
                let to_temp = from_temp.convert_to(&TempType::parse(types[j]).unwrap());
                assert_eq!(to_temp.temp_max, results[i][j])
            }
        }
    }

    #[test]
    fn to_strings() {
        assert_eq!(TempType::Celsius.to_string(), "°C");
        assert_eq!(TempType::Fahrenheit.to_string(), "°F");
        assert_eq!(TempType::Kelvin.to_string(), "°K");
    }

    #[test]
    fn to_string_max_min() {
        let temp = Temp {
            temp_min: 1.5f64,
            temp_max: 2.5f64,
            temp_type: TempType::Celsius,
        };
        assert_eq!(temp.to_string(), "[1.5 : 2.5] °C");
    }
    #[test]
    fn to_string_average() {
        let temp = Temp {
            temp_min: 1.5f64,
            temp_max: 1.5f64,
            temp_type: TempType::Celsius,
        };
        assert_eq!(temp.to_string(), "1.5 °C");
    }
}

#[cfg(test)]
mod speed {
    use crate::weather::info::Speed;
    use crate::weather::info::SpeedType;

    #[test]
    fn parse_speed_type_bad() {
        let speed_type = SpeedType::parse("bad");
        assert!(speed_type.is_none());
    }

    #[test]
    fn parse_speed_type_meter() {
        let speed_type = SpeedType::parse("meter");
        assert!(speed_type.is_some());
        assert!(match speed_type.unwrap() {
            SpeedType::Meter => true,
            _ => false,
        });
    }
    #[test]
    fn parse_speed_type_miles() {
        let speed_type = SpeedType::parse("miles");
        assert!(speed_type.is_some());
        assert!(match speed_type.unwrap() {
            SpeedType::Miles => true,
            _ => false,
        });
    }

    #[test]
    fn convert_speed() {
        let types = vec!["meter", "miles"];
        let results = vec![
            vec![1f64, 2.2369418519393043f64],
            vec![0.4470388888888889f64, 1f64],
        ];

        for i in 0..types.len() {
            let from_speed = Speed {
                speed: 1f64,
                speed_type: SpeedType::parse(types[i]).unwrap(),
            };
            for j in 0..types.len() {
                let to_speed = from_speed.convert_to(&SpeedType::parse(types[j]).unwrap());
                assert_eq!(to_speed.speed, results[i][j])
            }
        }
    }

    #[test]
    fn to_strings() {
        assert_eq!(SpeedType::Meter.to_string(), "meter/sec");
        assert_eq!(SpeedType::Miles.to_string(), "miles/hour");
    }

    #[test]
    fn to_string() {
        let speed = Speed {
            speed: 1.5f64,
            speed_type: SpeedType::Meter,
        };
        assert_eq!(speed.to_string(), "1.5 meter/sec");
    }
}

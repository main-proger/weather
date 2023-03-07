# weather

### Commands
Format:
```shell
weather <command> [<name> <param>]
```

help - view usage information
```shell
weather help
```

save - save default configuration
```shell
weather save -provider OpenWeather -temp C
weather save -address \"your address\" -speed meter
```

get - view weather
```shell
weather get -date now
weather get -provider WeatherApi -date \"1, 10\"
```

providers - view all providers
```shell
weather providers
```

### Configuration
provider - weather provider, default OpenWeather
```shell
weather [save, get] -provider [OpenWeather, WeatherApi]
weather save -provider WeatherApi
```

date - weather date, <day> - day for weather from current day, <hour> - hour of day, default now
```shell
weather get -date [now, "<day>, <hour>"]
weather get -date now
weather get -date "2, 16"
```

day - weather day, <day> - day for weather from current day, default now
```shell
weather get -day [now, <day>]
weather get -day now
weather get -day 2
```

hour - weather date, <hour> - hour of day, default now
```shell
weather get -hour [now, <hour>]
weather get -hour now
weather get -hour 19
```

address - weather address
```shell
weather [save, get] -address <address>
weather save -address "Country, City"
weather get -address "Country, City"
```

speed - wind speed format (meter/sec or miles/hour), default meter
```shell
weather [save, get] -speed [meter, miles]
weather save -speed miles
weather get -speed meter
```

temp - temperature format (Fahrenheit, Celsius, Kelvin), default Celsius
```shell
weather [save, get] -temp [C, F, K]
weather save -temp C
weather get -temp K
```

### Default configuration
if there is no parameter in the command, then this parameter is set by default.

Example:
```shell
# save default configuration
weather save -provider WeatherApi -temp K -address "Country, City"
# run command without provider params
weather get -date now -temp C
# ->
# this command run with default config
weather get -date now -temp C -provider WeatherApi -address "Country, City"
```

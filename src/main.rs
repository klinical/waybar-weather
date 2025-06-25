use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use std::{env, thread, time};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize)]
struct Current {
    temp_c: f64,
    temp_f: f64,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    text: WeatherCondition,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
enum WeatherCondition {
    #[serde(rename = "Sunny")]
    Sunny,
    #[serde(rename = "Partly cloudy")]
    Partly_cloudy,
    #[serde(rename = "Cloudy")]
    Cloudy,
    #[serde(rename = "Overcast")]
    Overcast,
    #[serde(rename = "Mist")]
    Mist,
    #[serde(rename = "Patchy rain possible")]
    Patchy_rain_possible,
    #[serde(rename = "Patchy snow possible")]
    Patchy_snow_possible,
    #[serde(rename = "Patchy sleet possible")]
    Patchy_sleet_possible,
    #[serde(rename = "Patchy freezing drizzle possible")]
    Patchy_freezing_drizzle_possible,
    #[serde(rename = "Thundery outbreaks possible")]
    Thundery_outbreaks_possible,
    #[serde(rename = "Blowing snow")]
    Blowing_snow,
    #[serde(rename = "Blizzard")]
    Blizzard,
    #[serde(rename = "Fog")]
    Fog,
    #[serde(rename = "Freezing fog")]
    Freezing_fog,
    #[serde(rename = "Patchy light drizzle")]
    Patchy_light_drizzle,
    #[serde(rename = "Light drizzle")]
    Light_drizzle,
    #[serde(rename = "Freezing drizzle")]
    Freezing_drizzle,
    #[serde(rename = "Heavy freezing drizzle")]
    Heavy_freezing_drizzle,
    #[serde(rename = "Patchy light rain")]
    Patchy_light_rain,
    #[serde(rename = "Light rain")]
    Light_rain,
    #[serde(rename = "Moderate rain at times")]
    Moderate_rain_at_times,
    #[serde(rename = "Moderate rain")]
    Moderate_rain,
    #[serde(rename = "Heavy rain at times")]
    Heavy_rain_at_times,
    #[serde(rename = "Heavy rain")]
    Heavy_rain,
    #[serde(rename = "Light freezing rain")]
    Light_freezing_rain,
    #[serde(rename = "Moderate or heavy freezing rain")]
    Moderate_or_heavy_freezing_rain,
    #[serde(rename = "Light sleet")]
    Light_sleet,
    #[serde(rename = "Moderate or heavy sleet")]
    Moderate_or_heavy_sleet,
    #[serde(rename = "Patchy light snow")]
    Patchy_light_snow,
    #[serde(rename = "Light snow")]
    Light_snow,
    #[serde(rename = "Patchy moderate snow")]
    Patchy_moderate_snow,
    #[serde(rename = "Moderate snow")]
    Moderate_snow,
    #[serde(rename = "Patchy heavy snow")]
    Patchy_heavy_snow,
    #[serde(rename = "Heavy snow")]
    Heavy_snow,
    #[serde(rename = "Ice pellets")]
    Ice_pellets,
    #[serde(rename = "Light rain shower")]
    Light_rain_shower,
    #[serde(rename = "Moderate or heavy rain shower")]
    Moderate_or_heavy_rain_shower,
    #[serde(rename = "Torrential rain shower")]
    Torrential_rain_shower,
    #[serde(rename = "Light sleet showers")]
    Light_sleet_showers,
    #[serde(rename = "Moderate or heavy sleet showers")]
    Moderate_or_heavy_sleet_showers,
    #[serde(rename = "Light snow showers")]
    Light_snow_showers,
    #[serde(rename = "Moderate or heavy snow showers")]
    Moderate_or_heavy_snow_showers,
    #[serde(rename = "Light showers of ice pellets")]
    Light_showers_of_ice_pellets,
    #[serde(rename = "Moderate or heavy showers of ice pellets")]
    Moderate_or_heavy_showers_of_ice_pellets,
    #[serde(rename = "Patchy light rain with thunder")]
    Patchy_light_rain_with_thunder,
    #[serde(rename = "Moderate or heavy rain with thunder")]
    Moderate_or_heavy_rain_with_thunder,
    #[serde(rename = "Patchy light snow with thunder")]
    Patchy_light_snow_with_thunder,
    #[serde(rename = "Moderate or heavy snow with thunder")]
    Moderate_or_heavy_snow_with_thunder,
}

impl Display for WeatherCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            WeatherCondition::Sunny => "",
            WeatherCondition::Partly_cloudy => "",
            WeatherCondition::Cloudy => "",
            WeatherCondition::Overcast => "",
            WeatherCondition::Mist => "󰖑",
            WeatherCondition::Patchy_rain_possible => "",
            WeatherCondition::Patchy_snow_possible => "",
            WeatherCondition::Patchy_sleet_possible => "",
            WeatherCondition::Patchy_freezing_drizzle_possible => "",
            WeatherCondition::Thundery_outbreaks_possible => "",
            WeatherCondition::Blowing_snow => "",
            WeatherCondition::Blizzard => "",
            WeatherCondition::Fog => "󰖑",
            WeatherCondition::Freezing_fog => "󰖑",
            WeatherCondition::Patchy_light_drizzle => "",
            WeatherCondition::Light_drizzle => "",
            WeatherCondition::Freezing_drizzle => "",
            WeatherCondition::Heavy_freezing_drizzle => "",
            WeatherCondition::Patchy_light_rain => "",
            WeatherCondition::Light_rain => "",
            WeatherCondition::Moderate_rain_at_times => "",
            WeatherCondition::Moderate_rain => "",
            WeatherCondition::Heavy_rain_at_times => "",
            WeatherCondition::Heavy_rain => "",
            WeatherCondition::Light_freezing_rain => "",
            WeatherCondition::Moderate_or_heavy_freezing_rain => "",
            WeatherCondition::Light_sleet => "",
            WeatherCondition::Moderate_or_heavy_sleet => "",
            WeatherCondition::Patchy_light_snow => "",
            WeatherCondition::Light_snow => "",
            WeatherCondition::Patchy_moderate_snow => "",
            WeatherCondition::Moderate_snow => "",
            WeatherCondition::Patchy_heavy_snow => "",
            WeatherCondition::Heavy_snow => "",
            WeatherCondition::Ice_pellets => "",
            WeatherCondition::Light_rain_shower => "",
            WeatherCondition::Moderate_or_heavy_rain_shower => "",
            WeatherCondition::Torrential_rain_shower => "",
            WeatherCondition::Light_sleet_showers => "",
            WeatherCondition::Moderate_or_heavy_sleet_showers => "",
            WeatherCondition::Light_snow_showers => "",
            WeatherCondition::Moderate_or_heavy_snow_showers => "",
            WeatherCondition::Light_showers_of_ice_pellets => "",
            WeatherCondition::Moderate_or_heavy_showers_of_ice_pellets => "",
            WeatherCondition::Patchy_light_rain_with_thunder => "",
            WeatherCondition::Moderate_or_heavy_rain_with_thunder => "",
            WeatherCondition::Patchy_light_snow_with_thunder => "",
            WeatherCondition::Moderate_or_heavy_snow_with_thunder => "",
        };
        write!(f, "{}", x.to_string())
    }
}

#[derive(Deserialize)]
struct WeatherResponse {
    current: Current,
}

#[derive(Debug)]
struct Config {
    key: String,
    zipcode: String,
    temp_unit: Unit,
}

#[derive(Debug)]
enum Unit {
    Celsius,
    Fahrenheit,
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" | "celsius" => Ok(Unit::Celsius),
            "f" | "fahrenheit" => Ok(Unit::Fahrenheit),
            _ => Err(()),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Unit::Celsius => "C",
            Unit::Fahrenheit => "F",
        };
        write!(f, "{}", x.to_string())
    }
}

#[derive(Serialize, Debug)]
struct WaybarPayload {
    text: String,
    class: String,
    alt: String,
    tooltip: String,
}

const URL: &str = "https://api.weatherapi.com/v1/current.json";

fn main() {
    let config = read_config();
    let client = reqwest::blocking::Client::new();
    loop {
        let request = client
            .get(URL)
            .query(&[("key", &config.key), ("q", &config.zipcode)])
            .build();

        match request {
            Ok(req) => match client.execute(req) {
                Ok(response) => process_response(&config, response),
                Err(e) => eprintln!("Error executing request: {:?}", e),
            },
            Err(e) => eprintln!("Error building request: {:?}", e),
        }
        thread::sleep(time::Duration::from_secs(60));
    }
}

fn process_response(config: &Config, response: Response) {
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<WeatherResponse>() {
            Ok(json) => output_waybar_json(config, json),
            Err(e) => eprintln!("Error parsing JSON: {:?}", e),
        },
        _ => panic!("{}", response.status()),
    }
}

fn output_waybar_json(config: &Config, weather_response: WeatherResponse) {
    let temp = match config.temp_unit {
        Unit::Celsius => weather_response.current.temp_c,
        Unit::Fahrenheit => weather_response.current.temp_f,
    };
    let text = format!("{}° {}", temp, config.temp_unit);
    let output = serde_json::to_string(&WaybarPayload {
        text: text.clone(),
        class: "weather".to_string(),
        alt: weather_response.current.condition.text.to_string(),
        tooltip: format!("Current temperature: {}", text),
    });
    match output {
        Err(e) => eprintln!("Error serializing JSON: {:?}", e),
        Ok(o) => println!("{}", o),
    }
}

fn read_config() -> Config {
    let args = env::args().collect::<Vec<String>>();
    let mut key: Option<String> = None;
    let mut zipcode: Option<String> = None;
    let mut temp_unit: Option<Unit> = None;

    for i in 0..args.len() {
        if i == 0 {
            continue;
        }
        if args[i] == "-key" && i + 1 < args.len() {
            key = Some(args[i + 1].clone());
        }
        else if args[i] == "-zip" && i + 1 < args.len() {
            zipcode = Some(args[i + 1].clone());
        }
        else if args[i] == "-unit" && i + 1 < args.len() {
            temp_unit = Some(Unit::from_str(&args[i + 1]).expect("REASON"));
        }
    }

    if args.len() != 7 || key.is_none() || zipcode.is_none() || temp_unit.is_none() {
        eprintln!("Usage: waybar-weather -key <API_KEY> -zip <ZIP_CODE> -unit <UNIT>");
    }

    Config {
        key: key.unwrap(),
        zipcode: zipcode.unwrap(),
        temp_unit: temp_unit.unwrap(),
    }
}

use reqwest::blocking::Response;
use serde::Deserialize;
use std::{env, thread, time};
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct Current {
    temp_c: f64,
    temp_f: f64,
}

#[derive(Deserialize, Debug)]
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

const URL: &str = "https://api.weatherapi.com/v1/current.json";

fn main() {
    let config = read_config();
    loop {
        let client = reqwest::blocking::Client::new();
        let request = client.get(URL).query(&[("key", &config.key), ("q", &config.zipcode)]).build().unwrap();
        let response: Response = client.execute(request).expect("REQUEST ERROR");

        match response.status() {
            reqwest::StatusCode::OK => {
                let json = response.json::<WeatherResponse>().expect("JSON ERROR");
                let output;
                match config.temp_unit {
                    Unit::Celsius => {
                        output = format!("{}° C", json.current.temp_c);
                    },
                    Unit::Fahrenheit => {
                        output = format!("{}° F", json.current.temp_f);
                    }
                }
                println!("{{ \"text\": \"{}\", \"alt\": \"NA\", \"tooltip\": \"tooltip\", \"class\": \"$class\", \"percentage\":\
                 100 }}", output);
            },
            _ => panic!("{}", response.status()),
        }

        thread::sleep(time::Duration::from_secs(60));
    }
}

fn read_config() -> Config {
    // -key etc -zip etc -unit etc
    let args = env::args().collect::<Vec<String>>();
    let mut key: Option<String> = None;
    let mut zipcode: Option<String> = None;
    let mut temp_unit: Option<Unit> = None;

    for i in 0..args.len() {
        if i == 0 {
            continue; // Skip the first argument which is the program name
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

use std::time::{Duration, UNIX_EPOCH};

use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

/// Nerd Font representation of OpenWeatherMap icons.
const CLEAR_DAY: &str = "";
const CLEAR_NIGHT: &str = "";
const FEW_CLOUDS_DAY: &str = "";
const FEW_CLOUDS_NIGHT: &str = "";
const SCATTERED_CLOUDS: &str = "摒";
const BROKEN_CLOUDS: &str = "";
const SHOWER_RAIN: &str = "";
const RAIN: &str = "";
const THUNDERSTORM: &str = "";
const SNOW: &str = "";
const MIST: &str = "";

#[derive(Debug)]
pub struct Location {
    pub city: String,
    pub state: String,
    pub country: String,
}

impl Location {
    pub fn new(city: &str, state: &str, country: &str) -> Self {
        Self {
            city: city.to_string(),
            state: state.to_string(),
            country: country.to_string(),
        }
    }

    // Returns a string of the location in the format "city,state,country",
    // unless state is empty, in which case it returns "city,country".
    pub fn to_string(&self) -> String {
        if self.state.is_empty() {
            format!("{},{}", self.city, self.country)
        } else {
            format!("{},{},{}", self.city, self.state, self.country)
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub enum Units {
    #[default]
    Metric,
    Imperial,
}

impl Units {
    fn as_str(&self) -> &'static str {
        match self {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    }
}

/// Data structures from the OpenWeatherMap API. Not everything is used, but
/// it's all here should it be needed.
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    /// City geo location, longitude
    lon: f64,
    /// City geo location, latitude
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    /// Weather condition id
    id: u64,
    /// Group of weather parameters (Rain, Snow, Extreme, etc.)
    main: String,
    /// Weather condition within the group
    description: String,
    /// Weather icon id
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    /// Temperature, Kelvin.
    temp: f64,
    /// Temperature accounting for human perception.
    feels_like: f64,
    /// Atmospheric pressure, hPa
    pressure: u64,
    /// Humidity, %
    humidity: u64,
    /// Minimum temperature at the moment, Kelvin
    temp_min: f64,
    /// Maximum temperature at the moment, Kelvin
    temp_max: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    /// Wind speed, m/s
    speed: f64,
    /// Wind direction, degrees (meteorological)
    deg: u16,
    /// Wind gust, m/s
    gust: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    /// Cloudiness, %
    all: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    #[serde(rename = "type")]
    /// Internal parameter
    type_: i64,
    /// Internal parameter
    id: i64,
    /// Internal parameter
    message: Option<String>,
    /// Country code (GB, JP etc.)
    country: String,
    /// Sunrise time, unix, UTC
    sunrise: u64,
    /// Sunset time, unix, UTC
    sunset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    coord: Option<Coord>,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    /// Visibility, meter, maximum is 10km
    visibility: u64,
    wind: Wind,
    clouds: Clouds,
    /// Time of data calculation, unix, UTC
    dt: i64,
    sys: Sys,
    /// Shift in seconds from UTC
    timezone: i64,
    /// City ID
    id: u64,
    /// City name
    name: String,
    /// Internal parameter
    cod: u64,
}

impl CurrentWeather {
    /// Returns supported weather data. Modify this if you need more data types.
    pub fn get(&self, item: &str) -> String {
        match item {
            "icon" => match_icon(&self.weather[0].icon).to_string(),
            "temp" => format!("{}°", self.main.temp.round()),
            "feels_like" => format!("{}°", self.main.feels_like.round()),
            "humidity" => format!("{}%", self.main.humidity),
            "description" => self.weather[0].description.to_string(),
            "time" => epoch_to_time(self.dt + self.timezone),
            _ => format!("('{}?')", item),
        }
    }
}

/// Convert OpenWeatherMap icon id to Nerd Font icon.
fn match_icon<S: AsRef<str>>(code: S) -> &'static str {
    match code.as_ref() {
        "01d" => CLEAR_DAY,
        "01n" => CLEAR_NIGHT,
        "02d" => FEW_CLOUDS_DAY,
        "02n" => FEW_CLOUDS_NIGHT,
        "03d" | "03n" => SCATTERED_CLOUDS,
        "04d" | "04n" => BROKEN_CLOUDS,
        "09d" | "09n" => SHOWER_RAIN,
        "10d" | "10n" => RAIN,
        "11d" | "11n" => THUNDERSTORM,
        "13d" | "13n" => SNOW,
        "50d" | "50n" => MIST,
        _ => "?",
    }
}

/// Fetches the current weather for the given location.
pub fn get(location: Location, units: Units, key: &str) -> Result<CurrentWeather> {
    let mut url = Url::parse("http://api.openweathermap.org/data/2.5/weather")?;
    url.query_pairs_mut()
       .append_pair("q", location.to_string().as_str());
    url.query_pairs_mut().append_pair("units", units.as_str());
    url.query_pairs_mut().append_pair("appid", key);

    let body: String = reqwest::blocking::get(url.as_str())?.text()?;
    let result: CurrentWeather = serde_json::from_str(&body)?;

    Ok(result)
}

/// Converts epoch time to a human-readable time.
#[must_use]
fn epoch_to_time(epoch: i64) -> String {
    let st = UNIX_EPOCH + Duration::from_secs(epoch.try_into().unwrap());
    let dt = DateTime::<Utc>::from(st);
    dt.format("%H:%M:%S").to_string()
}

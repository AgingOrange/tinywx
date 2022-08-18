use anyhow::Result;
use clap::Arg;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
struct Config {
    city: String,
    #[serde(default)]
    state: String,
    country: String,
    api_key: String,
    #[serde(default)]
    imperial: bool,
    #[serde(default)]
    data: Vec<String>,
}

fn main() {
    match app() {
        Ok(x) => println!("{}", x),
        Err(e) => eprintln!("{}", e),
    }
}

fn app() -> Result<String> {
    let matches = clap::App::new("tinywx")
        .version("0.1.0")
        .about("Fetch current weather from OpenWeatherMap.")
        .arg(
            Arg::new("city")
                .short('c')
                .long("city")
                .value_name("CITY")
                .required(true)
                .help("City name (enclosed within quotes if it contains spaces)")
        )
        .arg(
            Arg::new("state")
                .short('s')
                .long("state")
                .value_name("STATE")
                .required(false)
                .help("State abbreviation")
        )
        .arg(
            Arg::new("country")
                .short('C')
                .long("country")
                .value_name("COUNTRY_CODE")
                .required(true)
                .help("Country code")
        )
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .value_name("WX_DATA")
                .required(true)
                .multiple_values(true)
                .possible_values(&["icon", "temp", "feels_like", "description", "humidity"])
                .help("Weather data to display"),
        )
        .arg(
            Arg::new("imperial")
                .short('i')
                .long("imperial")
                .required(false)
                .help("Display imperial units instead of metric"),
        )
        .arg(
            Arg::new("api_key")
                .short('k')
                .long("api-key")
                .value_name("API_KEY")
                .required(true)
                .takes_value(true)
                .help("OpenWeatherMap API key"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .required(false)
                .help("Path to TOML file to read configuration from")
                .long_help(
                    "Path to TOML file to read configuration from. Note that \
                    arguments can be read from the command line \
                    or from a configuration file, but not both at \
                    the same time."
                )
                .conflicts_with_all(&["city", "state", "country", "data", "imperial", "api_key"]),
        )
        .get_matches();

    // If "file" was supplied, read configuration from it.
    // Else, process the command line arguments.
    let mut cfg = Config::default();

    #[allow(clippy::unwrap_used)]
    // We can use unwrap() here because Clap ensures that values are set.
    if let Some(filename) = matches.get_one::<String>("file") {
        cfg = toml_from_file(Path::new(filename))?;
    } else {
        cfg.city = matches.value_of("city").unwrap().to_string();
        cfg.state = matches.value_of("state").unwrap_or("").to_string();
        cfg.country = matches.value_of("country").unwrap().to_string();

        cfg.imperial = matches.is_present("imperial");

        cfg.api_key = matches.value_of("api_key").unwrap().to_string();

        cfg.data = matches
            .values_of("data")
            .unwrap()
            .map(ToString::to_string)
            .collect();
    }

    // Get the current weather from OpenWeatherMap.
    let location = wx::Location::new(&cfg.city, &cfg.state, &cfg.country);
    let units = if cfg.imperial {
        wx::Units::Imperial
    } else {
        wx::Units::Metric
    };
    let current_weather = wx::get(location, units, &cfg.api_key)?;

    // Return requested weather data as one string.
    Ok(cfg
        .data
        .iter()
        .map(|x| current_weather.get(x))
        .collect::<Vec<String>>()
        .join(" "))
}

/// Read contents of toml file into Config struct.
fn toml_from_file(path: impl AsRef<Path>) -> Result<Config> {
    let contents = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&contents)?;
    Ok(cfg)
}

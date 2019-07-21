use crate::db::Db;
use crate::measurement::Measurement;
use crate::services::Service;
use crate::value::{PointOfTheCompass, Value};
use chrono::{DateTime, Local};
use clap::crate_version;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::error::Error;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Buienradar JSON feed URL.
const URL: &str = "https://json.buienradar.nl/";
const REFRESH_PERIOD: Duration = Duration::from_millis(60000);
const USER_AGENT: &str = concat!(
    "My IoT / ",
    crate_version!(),
    " (Rust; https://github.com/eigenein/my-iot-rs)"
);

#[derive(Deserialize, Debug)]
pub struct BuienradarSettings {
    /// Station ID. Find a one [here](https://json.buienradar.nl/).
    station_id: u32,
}

#[derive(Debug)]
pub struct Buienradar {
    station_id: u32,
    client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
pub struct BuienradarFeed {
    actual: BuienradarFeedActual,
}

#[derive(Deserialize, Debug)]
pub struct BuienradarFeedActual {
    #[serde(rename = "stationmeasurements")]
    station_measurements: Vec<BuienradarStationMeasurement>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuienradarStationMeasurement {
    #[serde(rename = "stationid")]
    station_id: u32,

    #[serde(rename = "stationname")]
    name: String,

    temperature: Option<f64>,

    #[serde(rename = "groundtemperature")]
    ground_temperature: Option<f64>,

    #[serde(rename = "feeltemperature")]
    feel_temperature: Option<f64>,

    #[serde(rename = "windspeedBft")]
    wind_speed_bft: Option<u32>,

    #[serde(with = "date_format")]
    timestamp: DateTime<Local>,

    #[serde(default, rename = "winddirection", with = "wind_direction")]
    wind_direction: Option<PointOfTheCompass>,
}

impl Buienradar {
    pub fn new(settings: &BuienradarSettings) -> Buienradar {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, HeaderValue::from_static(USER_AGENT));
        Buienradar {
            station_id: settings.station_id,
            client: reqwest::Client::builder()
                .gzip(true)
                .timeout(Duration::from_secs(10))
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    /// Fetch measurement for the configured station.
    fn fetch(&self) -> Result<BuienradarStationMeasurement, Box<Error>> {
        let body = self.client.get(URL).send()?.text()?;
        let feed: BuienradarFeed = serde_json::from_str(&body)?;
        let measurement = feed
            .actual
            .station_measurements
            .iter()
            .find(|measurement| measurement.station_id == self.station_id)
            .ok_or_else(|| format!("station {} is not found", self.station_id))?
            .clone();
        Ok(measurement)
    }

    /// Sends out measurements based on Buienradar station measurement.
    fn send_measurements(&self, measurement: BuienradarStationMeasurement, tx: &Sender<Measurement>) {
        self.send(
            &tx,
            vec![Measurement::new(
                format!("buienradar:{}:name", self.station_id),
                Value::Text(measurement.name.clone()),
                Some(measurement.timestamp),
            )],
        );
        if let Some(degrees) = measurement.temperature {
            tx.send(Measurement::new(
                format!("buienradar:{}:temperature", self.station_id),
                Value::Celsius(degrees),
                Some(measurement.timestamp),
            ))
            .unwrap();
        }
        if let Some(degrees) = measurement.ground_temperature {
            tx.send(Measurement::new(
                format!("buienradar:{}:ground_temperature", self.station_id),
                Value::Celsius(degrees),
                Some(measurement.timestamp),
            ))
            .unwrap();
        }
        if let Some(degrees) = measurement.feel_temperature {
            tx.send(Measurement::new(
                format!("buienradar:{}:feel_temperature", self.station_id),
                Value::Celsius(degrees),
                Some(measurement.timestamp),
            ))
            .unwrap();
        }
        if let Some(bft) = measurement.wind_speed_bft {
            tx.send(Measurement::new(
                format!("buienradar:{}:wind_speed_bft", self.station_id),
                Value::Bft(bft),
                Some(measurement.timestamp),
            ))
            .unwrap();
        }
        if let Some(point) = measurement.wind_direction {
            tx.send(Measurement::new(
                format!("buienradar:{}:wind_direction", self.station_id),
                Value::WindDirection(point),
                Some(measurement.timestamp),
            ))
            .unwrap();
        }
    }
}

impl Service for Buienradar {
    fn run(&mut self, _db: Arc<Mutex<Db>>, tx: Sender<Measurement>) {
        loop {
            match self.fetch() {
                Ok(measurement) => self.send_measurements(measurement, &tx),
                Err(error) => {
                    log::error!("Buienradar has failed: {}", error);
                }
            }
            thread::sleep(REFRESH_PERIOD);
        }
    }
}

/// Implements [custom date/time format](https://serde.rs/custom-date-format.html) with Amsterdam timezone.
mod date_format {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use chrono_tz::Europe::Amsterdam;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Local>, D::Error> {
        let string = String::deserialize(deserializer)?;
        let datetime = NaiveDateTime::parse_from_str(&string, FORMAT).unwrap();
        Ok(Amsterdam.from_local_datetime(&datetime).unwrap().with_timezone(&Local))
    }
}

/// Translates Dutch wind direction acronyms.
mod wind_direction {
    use crate::value::PointOfTheCompass;
    use serde::de::Error;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<PointOfTheCompass>, D::Error> {
        match String::deserialize(deserializer)?.as_ref() {
            "N" => Ok(Some(PointOfTheCompass::North)),
            "NNO" => Ok(Some(PointOfTheCompass::NorthNortheast)),
            "NO" => Ok(Some(PointOfTheCompass::Northeast)),
            "ONO" => Ok(Some(PointOfTheCompass::EastNortheast)),
            "O" => Ok(Some(PointOfTheCompass::East)),
            "OZO" => Ok(Some(PointOfTheCompass::EastSoutheast)),
            "ZO" => Ok(Some(PointOfTheCompass::Southeast)),
            "ZZO" => Ok(Some(PointOfTheCompass::SouthSoutheast)),
            "Z" => Ok(Some(PointOfTheCompass::South)),
            "ZZW" => Ok(Some(PointOfTheCompass::SouthSouthwest)),
            "ZW" => Ok(Some(PointOfTheCompass::Southwest)),
            "WZW" => Ok(Some(PointOfTheCompass::WestSouthwest)),
            "W" => Ok(Some(PointOfTheCompass::West)),
            "WNW" => Ok(Some(PointOfTheCompass::WestNorthwest)),
            "NW" => Ok(Some(PointOfTheCompass::Northwest)),
            "NNW" => Ok(Some(PointOfTheCompass::NorthNorthwest)),
            value => Err(Error::custom(format!("could not translate wind direction: {}", value))),
        }
    }
}
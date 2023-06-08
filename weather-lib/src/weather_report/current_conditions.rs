use std::str::FromStr;

use color_eyre::eyre::{eyre, Report as EyreReport, Result};
use schemars::JsonSchema;
use serde::Serialize;

use crate::environment_canada::weather_feed::Entry;

use super::{parsers, LocalDateTime};

/// Current weather conditions
#[derive(Debug, Serialize, JsonSchema)]
pub struct CurrentConditions {
    /// [Relative humidity](https://climate.weather.gc.ca/glossary_e.html#r_humidity) as a percentage
    pub humidity_pct: Option<u8>,
    /// Temperature in degrees Celsius
    pub temperature_c: Option<f32>,
    /// Time and place where conditions were observed
    pub observed_at: ObservedAt,
    /// [Dewpoint](https://climate.weather.gc.ca/glossary_e.html#dewPnt) in degrees Celsius
    pub dewpoint_c: Option<f32>,
    /// [Atmospheric pressure](https://climate.weather.gc.ca/glossary_e.html#AB_pressure)
    pub pressure: Option<Pressure>,
    /// Wind speed and direction
    pub wind: Option<Wind>,
    /// [Air quality health index](https://www.canada.ca/en/environment-climate-change/services/air-quality-health-index/about.html)
    pub aqhi: Option<u8>,
    /// [Visibility](https://climate.weather.gc.ca/glossary_e.html#visibility) in kilometers
    pub visibility_km: Option<f32>,
    /// Description of current conditions (e.g. "Light Rainshower")
    pub condition: Option<String>,
    /// [Humidex](https://climate.weather.gc.ca/glossary_e.html#humidex) in degrees Celsius
    pub humidex_c: Option<f32>,
    /// [Wind chill](https://climate.weather.gc.ca/glossary_e.html#windChill) in degrees Celsius
    pub wind_chill_c: Option<f32>,
}

impl CurrentConditions {
    pub fn from_entry(entry: &Entry) -> Result<Self> {
        let doc = scraper::Html::parse_fragment(&entry.summary.content);
        let selector = scraper::Selector::parse("b").unwrap();
        let elements = doc.select(&selector);
        let mut builder = Builder::default();

        for b in elements {
            let key = b
                .text()
                .collect::<String>()
                .trim_end_matches(':')
                .to_string();

            let Some(value) = b.next_sibling().and_then(|e| e.value().as_text()).map(|v| v.trim_start()) else {
                continue;
            };

            match key.as_str() {
                "Humidity" => builder.set_humidity(value)?,
                "Temperature" => builder.set_temperature(value)?,
                "Observed at" => builder.set_observed_at(value)?,
                "Dewpoint" => builder.set_dewpoint(value)?,
                "Pressure" | "Pressure / Tendency" => builder.set_pressure(value)?,
                "Wind" => builder.set_wind(value)?,
                "Air Quality Health Index" => builder.set_aqhi(value)?,
                "Visibility" => builder.set_visibility(value)?,
                "Condition" => builder.set_condition(value)?,
                "Humidex" => builder.set_humidex(value)?,
                "Wind Chill" => builder.set_wind_chill(value)?,
                key => {
                    eprintln!("unexpected key {key:?} in entry {:?}", &entry.title);
                }
            }
        }

        builder.build()
    }
}

/// Time and place where weather conditions were observed
#[derive(Debug, PartialEq, Serialize, JsonSchema)]
pub struct ObservedAt {
    pub location: String,
    pub datetime: LocalDateTime,
}

/// [Atmospheric pressure](https://climate.weather.gc.ca/glossary_e.html#AB_pressure)
#[derive(Debug, PartialEq, Serialize, JsonSchema)]
pub struct Pressure {
    pub kpa: f32,
    pub tendency: Option<PressureTendency>,
}

/// Wind conditions
#[derive(Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Wind {
    pub speed: WindSpeed,
    pub gust_kph: Option<u8>,
    pub direction: Option<CardinalDirection>,
}

/// Wind speed
#[derive(Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WindSpeed {
    /// Little to no wind
    Calm,
    /// Wind speed in kilometers per hour
    Kph(u8),
}

#[derive(Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PressureTendency {
    Falling,
    Rising,
    Steady,
}

#[derive(Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub enum CardinalDirection {
    N,
    NNE,
    NE,
    ENE,
    E,
    ESE,
    SE,
    SSE,
    S,
    SSW,
    SW,
    WSW,
    W,
    WNW,
    NW,
    NNW,
}

impl FromStr for CardinalDirection {
    type Err = EyreReport;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "N" => Ok(CardinalDirection::N),
            "NNE" => Ok(CardinalDirection::NNE),
            "NE" => Ok(CardinalDirection::NE),
            "ENE" => Ok(CardinalDirection::ENE),
            "E" => Ok(CardinalDirection::E),
            "ESE" => Ok(CardinalDirection::ESE),
            "SE" => Ok(CardinalDirection::SE),
            "SSE" => Ok(CardinalDirection::SSE),
            "S" => Ok(CardinalDirection::S),
            "SSW" => Ok(CardinalDirection::SSW),
            "SW" => Ok(CardinalDirection::SW),
            "WSW" => Ok(CardinalDirection::WSW),
            "W" => Ok(CardinalDirection::W),
            "WNW" => Ok(CardinalDirection::WNW),
            "NW" => Ok(CardinalDirection::NW),
            "NNW" => Ok(CardinalDirection::NNW),
            _ => Err(eyre!("invalid cardinal direction: {s:?}")),
        }
    }
}

#[derive(Debug, Default)]
struct Builder {
    humidity_pct: Option<u8>,
    temperature_c: Option<f32>,
    observed_at: Option<ObservedAt>,
    dewpoint_c: Option<f32>,
    pressure: Option<Pressure>,
    wind: Option<Wind>,
    aqhi: Option<u8>,
    visibility_km: Option<f32>,
    condition: Option<String>,
    humidex_c: Option<f32>,
    wind_chill_c: Option<f32>,
}

impl Builder {
    fn build(self) -> Result<CurrentConditions> {
        Ok(CurrentConditions {
            humidity_pct: self.humidity_pct,
            temperature_c: self.temperature_c,
            observed_at: self
                .observed_at
                .ok_or_else(|| eyre!("missing observed at"))?,
            dewpoint_c: self.dewpoint_c,
            pressure: self.pressure,
            wind: self.wind,
            aqhi: self.aqhi,
            visibility_km: self.visibility_km,
            condition: self.condition,
            humidex_c: self.humidex_c,
            wind_chill_c: self.wind_chill_c,
        })
    }

    fn set_humidity(&mut self, value: &str) -> Result<()> {
        let (_, humidity) = parsers::humidity(value)?;
        self.humidity_pct.replace(humidity);
        Ok(())
    }

    fn set_temperature(&mut self, value: &str) -> Result<()> {
        let (_, temperature) = parsers::temperature(value)?;
        self.temperature_c.replace(temperature);
        Ok(())
    }

    fn set_observed_at(&mut self, value: &str) -> Result<()> {
        let (_, observed_at) = parsers::observed_at(value)?;
        self.observed_at.replace(observed_at);
        Ok(())
    }

    fn set_dewpoint(&mut self, value: &str) -> Result<()> {
        let (_, temperature) = parsers::temperature(value)?;
        self.dewpoint_c.replace(temperature);
        Ok(())
    }

    fn set_pressure(&mut self, value: &str) -> Result<()> {
        let (_, pressure) = parsers::pressure::parse(value)?;
        self.pressure.replace(pressure);
        Ok(())
    }

    fn set_wind(&mut self, value: &str) -> Result<()> {
        let (_, wind) = parsers::wind::parse(value)?;
        self.wind.replace(wind);
        Ok(())
    }

    fn set_aqhi(&mut self, value: &str) -> Result<()> {
        let (_, aqhi) = parsers::aqhi(value)?;
        self.aqhi = aqhi;
        Ok(())
    }

    fn set_visibility(&mut self, value: &str) -> Result<()> {
        let (_, visibility) = parsers::visibility(value)?;
        self.visibility_km.replace(visibility);
        Ok(())
    }

    fn set_condition(&mut self, value: &str) -> Result<()> {
        self.condition.replace(value.to_string());
        Ok(())
    }

    fn set_humidex(&mut self, value: &str) -> Result<()> {
        let (_, humidex) = parsers::unitless_temperature(value)?;
        self.humidex_c.replace(humidex);
        Ok(())
    }

    fn set_wind_chill(&mut self, value: &str) -> Result<()> {
        let (_, wind_chill) = parsers::unitless_temperature(value)?;
        self.wind_chill_c.replace(wind_chill);
        Ok(())
    }
}

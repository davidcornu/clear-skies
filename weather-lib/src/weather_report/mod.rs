pub mod current_conditions;
pub mod forecasts;
pub mod parsers;
pub mod special_weather_statements;

use chrono::{DateTime, FixedOffset, Utc};
use color_eyre::eyre::{eyre, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::environment_canada::weather_feed::WeatherFeed;
use current_conditions::CurrentConditions;
use forecasts::{Builder, Day};
use special_weather_statements::SpecialWeatherStatement;

#[derive(Debug, Serialize, JsonSchema)]
pub struct WeatherReport {
    pub title: String,
    pub updated: DateTime<Utc>,
    pub url: String,
    pub current_conditions: Option<CurrentConditions>,
    pub special_weather_statements: Vec<SpecialWeatherStatement>,
    pub weather_forecasts: Vec<Day>,
}

impl WeatherReport {
    pub fn from_weather_feed(feed: WeatherFeed, tz: CanadaTz) -> Result<WeatherReport> {
        let mut current_conditions = None;
        let mut special_weather_statements = Vec::new();
        let mut forecasts_builder = Builder::new(feed.updated, &tz.chrono());

        for entry in feed.entries {
            match entry.category.term.as_str() {
                "Weather Forecasts" => {
                    forecasts_builder.add(&entry)?;
                }
                "Current Conditions" => {
                    if current_conditions.is_some() {
                        return Err(eyre!(
                            "feed contains multiple entries for \"Current Conditions\""
                        ));
                    } else {
                        current_conditions.replace(CurrentConditions::from_entry(&entry)?);
                    }
                }
                "Warnings and Watches" => {
                    if !entry.title.starts_with("No watches or warnings in effect") {
                        special_weather_statements
                            .push(SpecialWeatherStatement::from_entry(&entry)?);
                    }
                }
                "Notice" => {
                    // Ignore these entries
                }
                term => {
                    return Err(eyre!("feed contains entry with unknown category: {term:?}"));
                }
            }
        }

        let url = feed
            .links
            .iter()
            .find_map(|l| (l.rel.as_deref() == Some("related")).then(|| l.href.clone()))
            .ok_or_else(|| eyre!("missing alternate link"))?;

        Ok(WeatherReport {
            title: feed.title,
            updated: feed.updated,
            url,
            current_conditions,
            special_weather_statements,
            weather_forecasts: forecasts_builder.into_inner(),
        })
    }

    pub fn detected_timezone(&self) -> Option<CanadaTz> {
        self.current_conditions
            .as_ref()
            .map(|cc| cc.observed_at.datetime.tz)
    }
}

/// A timestamp with an associated timezone
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct LocalDateTime {
    /// Local ISO 8601 timestamp with a fixed UTC offset
    ts: DateTime<FixedOffset>,
    /// The timezone name
    tz: CanadaTz,
}

/// Canadian timezone (https://www.timeanddate.com/time/zone/canada)
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy, PartialEq, Eq)]
pub enum CanadaTz {
    #[serde(rename = "Canada/Atlantic")]
    Atlantic,
    #[serde(rename = "Canada/Central")]
    Central,
    #[serde(rename = "Canada/Eastern")]
    Eastern,
    #[serde(rename = "Canada/Mountain")]
    Mountain,
    #[serde(rename = "Canada/Newfoundland")]
    Newfoundland,
    #[serde(rename = "Canada/Pacific")]
    Pacific,
}

impl CanadaTz {
    fn chrono(&self) -> chrono_tz::Tz {
        match self {
            CanadaTz::Atlantic => chrono_tz::Canada::Atlantic,
            CanadaTz::Central => chrono_tz::Canada::Central,
            CanadaTz::Eastern => chrono_tz::Canada::Eastern,
            CanadaTz::Mountain => chrono_tz::Canada::Mountain,
            CanadaTz::Newfoundland => chrono_tz::Canada::Newfoundland,
            CanadaTz::Pacific => chrono_tz::Canada::Pacific,
        }
    }
}

use chrono::{DateTime, Datelike, Days, NaiveDate, TimeZone, Utc, Weekday};
use color_eyre::eyre::{eyre, Result};
use schemars::JsonSchema;
use serde::Serialize;

use crate::environment_canada::weather_feed::Entry;

use super::{parsers, LocalDateTime};

pub struct Builder<Tz: TimeZone> {
    forecasts: Vec<Day>,
    local_time: DateTime<Tz>,
}

impl<Tz: TimeZone> Builder<Tz> {
    pub fn new(feed_updated: DateTime<Utc>, local_tz: &Tz) -> Self {
        Self {
            forecasts: Vec::with_capacity(14),
            local_time: feed_updated.with_timezone(local_tz),
        }
    }

    pub fn add(&mut self, entry: &Entry) -> Result<()> {
        let (forecast, period) = Forecast::from_entry(entry)?;

        if period.is_night {
            match self.forecasts.last_mut() {
                // The last forecast should be for the same day
                Some(last_day) => {
                    if period.weekday != last_day.date.weekday() {
                        return Err(eyre!("entry weekday mismatch: {:?}", &entry.title));
                    }

                    last_day.make_detailed(forecast)?;
                }
                // Feed was retrieved during the day, so only the night forecast is available
                None => {
                    self.forecasts.push(Day {
                        date: self.local_time.date_naive(),
                        content: Content::Night(forecast),
                    });

                    self.local_time = self.local_time.clone() + Days::new(1);
                }
            }
        } else {
            if period.weekday != self.local_time.weekday() {
                return Err(eyre!(
                    "expected {} for entry: {:?}",
                    self.local_time.weekday(),
                    &entry.title
                ));
            }

            self.forecasts.push(Day {
                date: self.local_time.date_naive(),
                content: Content::Abridged(forecast),
            });

            self.local_time = self.local_time.clone() + Days::new(1);
        }

        Ok(())
    }

    pub fn into_inner(self) -> Vec<Day> {
        self.forecasts
    }
}

/// Weather forecast(s) for a single calendar day
#[derive(Debug, Serialize, JsonSchema)]
pub struct Day {
    pub date: NaiveDate,
    pub content: Content,
}

impl Day {
    fn make_detailed(&mut self, night: Forecast) -> Result<()> {
        let Content::Abridged(ref day) = self.content else {
            return Err(eyre!("forecast is already detailed"))
        };

        self.content = Content::Detailed {
            day: day.clone(),
            night,
        };

        Ok(())
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Content {
    /// Evening forecast for the current day
    Night(Forecast),
    /// Separate day and night forecasts (usually issued for the next 5 days)
    Detailed { day: Forecast, night: Forecast },
    /// Single forecast covering the entire day (usually issued beyond 5 days out)
    Abridged(Forecast),
}

impl Content {
    pub fn forecasts(&self) -> Vec<&Forecast> {
        match self {
            Content::Night(forecast) => vec![forecast],
            Content::Detailed { day, night } => vec![day, night],
            Content::Abridged(forecast) => vec![forecast],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Forecast {
    /// A short description of forecasted weather conditions (e.g. "Chance of showers")
    pub conditions: String,
    /// The [probability of precipitation](https://www.canada.ca/en/environment-climate-change/services/sky-watchers/glossary.html#wsDTE9CAF366) as a percentage
    pub probability_of_precipitation: Option<u8>,
    /// The forecasted temperature
    pub temperature: Temperature,
    /// A longer description of forecasted weather conditions (e.g. "Cloudy with 70 percent chance of showers. Low 17.")
    pub summary: String,
    /// When the forecast was issued
    pub issued_at: LocalDateTime,
}

struct ForecastPeriod {
    weekday: Weekday,
    is_night: bool,
}

impl Forecast {
    fn from_entry(entry: &Entry) -> Result<(Self, ForecastPeriod)> {
        let (_, parsed_title) = parsers::title::parse(&entry.title)?;

        let summary_html = scraper::Html::parse_fragment(&entry.summary.content);
        let full_summary = summary_html.root_element().text().collect::<String>();
        let (_, (summary, issued_at)) = parsers::forecast_summary(&full_summary)?;

        Ok((
            Forecast {
                conditions: parsed_title.conditions,
                probability_of_precipitation: parsed_title.probability_of_precipitation,
                temperature: parsed_title.temperature,
                summary: summary.to_string(),
                issued_at,
            },
            ForecastPeriod {
                weekday: parsed_title.weekday,
                is_night: parsed_title.is_night,
            },
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct Temperature {
    pub degrees_c: f32,
    pub trend: TemperatureTrend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureTrend {
    High,
    Low,
    Steady,
}

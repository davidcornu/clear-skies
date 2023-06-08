use color_eyre::eyre::{eyre, Result};
use schemars::JsonSchema;
use serde::Serialize;

use super::{parsers, LocalDateTime};
use crate::environment_canada::weather_feed::Entry;

#[derive(Debug, Serialize, JsonSchema)]
pub struct SpecialWeatherStatement {
    pub title: String,
    pub summary: String,
    pub issued_at: LocalDateTime,
    pub url: String,
}

impl SpecialWeatherStatement {
    pub fn from_entry(entry: &Entry) -> Result<Self> {
        let summary_html = scraper::Html::parse_fragment(&entry.summary.content);
        let full_summary = summary_html.root_element().text().collect::<String>();
        let (_, (summary, issued_at)) = parsers::special_weather_statement_summary(&full_summary)?;

        Ok(SpecialWeatherStatement {
            title: entry.title.clone(),
            summary: summary.to_string(),
            issued_at,
            url: entry
                .links
                .first()
                .map(|l| l.href.clone())
                .ok_or_else(|| eyre!("missing link"))?,
        })
    }
}

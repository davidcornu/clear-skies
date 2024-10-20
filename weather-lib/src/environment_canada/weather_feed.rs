use std::io::BufRead;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherFeed {
    pub title: String,
    pub updated: DateTime<Utc>,
    #[serde(rename = "link")]
    pub links: Vec<Link>,
    #[serde(rename = "entry", default)]
    pub entries: Vec<Entry>,
}

impl WeatherFeed {
    pub fn from_xml_reader<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        quick_xml::de::from_reader(reader).map_err(Into::into)
    }

    pub fn from_xml_str(s: &str) -> Result<Self> {
        quick_xml::de::from_str(s).map_err(Into::into)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "@rel")]
    pub rel: Option<String>,
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@type")]
    pub r#type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub summary: Summary,
    #[serde(rename = "link")]
    pub links: Vec<Link>,
    pub category: Category,
    pub updated: String,
    // Environment Canada recently started adding a "Notice"
    // entry without a `<published>` tag.
    pub published: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Summary {
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    #[serde(rename = "@term")]
    pub term: String,
}

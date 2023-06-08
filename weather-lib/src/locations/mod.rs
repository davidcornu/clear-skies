use std::{collections::BTreeMap, fmt, sync::OnceLock};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::weather_report::CanadaTz;

pub mod data;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub enum ProvinceOrTerritory {
    #[serde(rename = "ab", alias = "AB")]
    Alberta,
    #[serde(rename = "bc", alias = "BC")]
    BritishColumbia,
    #[serde(rename = "mb", alias = "MB")]
    Manitoba,
    #[serde(rename = "nb", alias = "NB")]
    NewBrunswick,
    #[serde(rename = "nl", alias = "NL")]
    NewfoundlandAndLabrador,
    #[serde(rename = "nt", alias = "NT")]
    NorthwestTerritories,
    #[serde(rename = "ns", alias = "NS")]
    NovaScotia,
    #[serde(rename = "nu", alias = "NU")]
    Nunavut,
    #[serde(rename = "on", alias = "ON")]
    Ontario,
    #[serde(rename = "pe", alias = "PE")]
    PrinceEdwardIsland,
    #[serde(rename = "qc", alias = "QC")]
    Quebec,
    #[serde(rename = "sk", alias = "SK")]
    Saskatchewan,
    #[serde(rename = "yt", alias = "YT")]
    Yukon,
}

impl ProvinceOrTerritory {
    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "ab" | "AB" => Some(Self::Alberta),
            "bc" | "BC" => Some(Self::BritishColumbia),
            "mb" | "MB" => Some(Self::Manitoba),
            "nb" | "NB" => Some(Self::NewBrunswick),
            "nl" | "NL" => Some(Self::NewfoundlandAndLabrador),
            "nt" | "NT" => Some(Self::NorthwestTerritories),
            "ns" | "NS" => Some(Self::NovaScotia),
            "nu" | "NU" => Some(Self::Nunavut),
            "on" | "ON" => Some(Self::Ontario),
            "pe" | "PE" => Some(Self::PrinceEdwardIsland),
            "qc" | "QC" => Some(Self::Quebec),
            "sk" | "SK" => Some(Self::Saskatchewan),
            "yt" | "YT" => Some(Self::Yukon),
            _ => None,
        }
    }

    pub fn to_abbr(&self) -> &'static str {
        match self {
            ProvinceOrTerritory::Alberta => "ab",
            ProvinceOrTerritory::BritishColumbia => "bc",
            ProvinceOrTerritory::Manitoba => "mb",
            ProvinceOrTerritory::NewBrunswick => "nb",
            ProvinceOrTerritory::NewfoundlandAndLabrador => "nl",
            ProvinceOrTerritory::NorthwestTerritories => "nt",
            ProvinceOrTerritory::NovaScotia => "ns",
            ProvinceOrTerritory::Nunavut => "nu",
            ProvinceOrTerritory::Ontario => "on",
            ProvinceOrTerritory::PrinceEdwardIsland => "pe",
            ProvinceOrTerritory::Quebec => "qc",
            ProvinceOrTerritory::Saskatchewan => "sk",
            ProvinceOrTerritory::Yukon => "yt",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ProvinceOrTerritory::Alberta => "Alberta",
            ProvinceOrTerritory::BritishColumbia => "British Columbia",
            ProvinceOrTerritory::Manitoba => "Manitoba",
            ProvinceOrTerritory::NewBrunswick => "New Brunswick",
            ProvinceOrTerritory::NewfoundlandAndLabrador => "Newfoundland and Labrador",
            ProvinceOrTerritory::NorthwestTerritories => "Northwest Territories",
            ProvinceOrTerritory::NovaScotia => "Nova Scotia",
            ProvinceOrTerritory::Nunavut => "Nunavut",
            ProvinceOrTerritory::Ontario => "Ontario",
            ProvinceOrTerritory::PrinceEdwardIsland => "Prince Edward Island",
            ProvinceOrTerritory::Quebec => "Quebec",
            ProvinceOrTerritory::Saskatchewan => "Saskatchewan",
            ProvinceOrTerritory::Yukon => "Yukon",
        }
    }
}

impl fmt::Display for ProvinceOrTerritory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug)]
pub struct OwnedLocation {
    pub name: String,
    pub province_or_territory: ProvinceOrTerritory,
    pub slug: String,
    pub tz: CanadaTz,
    pub feed_url: String,
    pub html_url: String,
}

#[derive(Debug)]
pub struct Location {
    pub name: &'static str,
    pub province_or_territory: ProvinceOrTerritory,
    pub slug: &'static str,
    pub tz: CanadaTz,
    pub feed_url: &'static str,
    pub html_url: &'static str,
}

static LOCATION_INDEX: OnceLock<BTreeMap<(ProvinceOrTerritory, &'static str), &'static Location>> =
    OnceLock::new();

pub fn location_index() -> &'static BTreeMap<(ProvinceOrTerritory, &'static str), &'static Location>
{
    LOCATION_INDEX.get_or_init(|| {
        let mut map = BTreeMap::new();
        for location in &data::LOCATIONS {
            map.insert((location.province_or_territory, location.slug), location);
        }
        map
    })
}

impl Location {
    pub fn lookup(province_or_territory: ProvinceOrTerritory, slug: &str) -> Option<&'static Self> {
        location_index()
            .get(&(province_or_territory, slug))
            .copied()
    }
}

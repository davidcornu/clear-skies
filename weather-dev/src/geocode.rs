use std::{collections::HashMap, sync::OnceLock};

use color_eyre::eyre::{eyre, Result};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};

pub struct Geocoder {
    mapbox_access_token: String,
    http_client: reqwest::Client,
}

static CACHE: OnceLock<HashMap<&'static str, LonLat>> = OnceLock::new();

pub fn cache() -> &'static HashMap<&'static str, LonLat> {
    CACHE.get_or_init(|| serde_json::from_str(include_str!("../data/coordinates.json")).unwrap())
}

impl Geocoder {
    const BASE_URL: &'static str = "https://api.mapbox.com/geocoding/v5";

    pub fn new<S: ToString>(mapbox_access_token: S) -> Self {
        Self {
            mapbox_access_token: mapbox_access_token.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn lookup(&self, search_text: &str) -> Result<GeocoderResponse> {
        let mut url = Url::parse(Self::BASE_URL).unwrap();

        url.path_segments_mut()
            .unwrap()
            .extend(&["mapbox.places", &format!("{search_text}.json")]);

        url.query_pairs_mut().extend_pairs(&[
            ("country", "ca"),
            ("fuzzyMatch", "false"),
            ("language", "en"),
            ("access_token", &self.mapbox_access_token),
        ]);

        let response = self.http_client.get(url).send().await?;

        if response.status() != StatusCode::OK {
            let status = response.status();
            let body = response.text().await?;

            return Err(eyre!(
                "Mapbox API returned status code {status} with body {body}"
            ));
        }

        let body: GeocoderResponse = response.json().await?;

        Ok(body)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeocoderResponse {
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feature {
    pub place_name: String,
    pub center: LonLat,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct LonLat(pub f64, pub f64);

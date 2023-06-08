use std::sync::Arc;

use color_eyre::eyre::{eyre, Result};
use moka::future::Cache;
use reqwest::{Client as ReqwestClient, Method, StatusCode, Url};
use weather_lib::{
    environment_canada::weather_feed::WeatherFeed, locations::Location, weather_report::Report,
};

pub struct Client {
    cache: Cache<String, Arc<Report>>,
    http_client: ReqwestClient,
}

impl Client {
    // There are just over 800 locations supported by Environment Canada
    const CACHE_SIZE: u64 = 1000;

    const USER_AGENT: &str = concat!(
        "weather-sync v",
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("CARGO_PKG_HOMEPAGE"),
        ")"
    );

    pub fn new() -> Self {
        let http_client = ReqwestClient::builder()
            .user_agent(Self::USER_AGENT)
            .build()
            .unwrap();

        let cache = Cache::new(Self::CACHE_SIZE);

        Self { cache, http_client }
    }

    pub async fn fetch_report(&self, location: &Location) -> Result<Arc<Report>> {
        let feed_url = location.feed_url.parse::<Url>()?;
        let cache_key = feed_url.to_string();

        if let Some(cached_report) = self.cache.get(&cache_key) {
            // TODO - check if cache entry is stale
            return Ok(cached_report);
        }

        let res = self
            .http_client
            .request(Method::GET, feed_url)
            .send()
            .await?;

        if res.status() != StatusCode::OK {
            // TODO - cache error responses so we don't overload the server
            return Err(eyre!("received non-200 status code: {}", res.status()));
        }

        let body = res.text().await?;
        let feed = WeatherFeed::from_xml_str(&body)?;
        let report = Arc::new(Report::from_weather_feed(feed, location.tz)?);

        let fresh_report = Arc::clone(&report);

        self.cache.insert(cache_key, report).await;

        Ok(fresh_report)
    }
}

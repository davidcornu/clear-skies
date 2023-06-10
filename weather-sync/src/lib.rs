use std::{sync::Arc, time::SystemTime};

use color_eyre::eyre::{eyre, Result};
use http::request::Parts;
use http_cache_semantics::{AfterResponse, BeforeRequest, CacheOptions, CachePolicy};
use moka::future::Cache;
use reqwest::{Client as ReqwestClient, Method, Request, StatusCode, Url};
use tokio::sync::Semaphore;
use weather_lib::{
    environment_canada::weather_feed::WeatherFeed, locations::Location, weather_report::Report,
};

pub struct Client {
    cache: Cache<String, CacheEntry>,
    http_client: ReqwestClient,
    http_cache_options: CacheOptions,
    request_semaphore: Semaphore,
}

pub struct ClientOptions {
    pub cache_size: u64,
    pub request_concurrency: usize,
}

impl ClientOptions {
    // There are just over 800 locations supported by Environment Canada
    const DEFAULT_CACHE_SIZE: u64 = 1000;
    const DEFAULT_REQUEST_CONCURRENCY: usize = 4;
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            cache_size: Self::DEFAULT_CACHE_SIZE,
            request_concurrency: Self::DEFAULT_REQUEST_CONCURRENCY,
        }
    }
}

impl Client {
    const USER_AGENT: &str = concat!(
        "weather-sync v",
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("CARGO_PKG_HOMEPAGE"),
        ")"
    );

    pub fn new(options: ClientOptions) -> Self {
        let http_client = ReqwestClient::builder()
            .user_agent(Self::USER_AGENT)
            .build()
            .unwrap();

        let cache = Cache::new(options.cache_size);

        Self {
            cache,
            http_client,
            http_cache_options: Default::default(),
            request_semaphore: Semaphore::new(options.request_concurrency),
        }
    }

    // TODO - needs a locking mechanism to prevent multiple requests for the same location
    pub async fn fetch_report(&self, location: &Location) -> Result<Arc<Report>> {
        let feed_url = location.feed_url.parse::<Url>()?;
        let cache_key = feed_url.to_string();
        let time_before_request = SystemTime::now();
        let mut request = Request::new(Method::GET, feed_url);

        let mut cached_entry = self.cache.get(&cache_key);

        if let Some(entry) = &cached_entry {
            match entry
                .cache_policy
                .before_request(&request, time_before_request)
            {
                BeforeRequest::Fresh(_) => return Ok(cached_entry.take().unwrap().report),
                BeforeRequest::Stale { request: parts, .. } => {
                    // issue a conditional request based on cached data
                    request = request_from_parts(parts);
                }
            }
        }

        let response = {
            let _permit = self.request_semaphore.acquire().await?;
            self.http_client
                .execute(request.try_clone().unwrap())
                .await?
        };

        let time_after_response = SystemTime::now();

        if !matches!(response.status(), StatusCode::OK | StatusCode::NOT_MODIFIED) {
            // TODO - cache error responses so we don't overload the server
            return Err(eyre!(
                "received unexpected status code: {}",
                response.status()
            ));
        }

        let cache_policy = match &mut cached_entry {
            Some(entry) => {
                match entry
                    .cache_policy
                    .after_response(&request, &response, time_after_response)
                {
                    AfterResponse::NotModified(updated_cache_policy, _) => {
                        let report = Arc::clone(&entry.report);

                        self.cache
                            .insert(
                                cache_key,
                                CacheEntry {
                                    cache_policy: Arc::new(updated_cache_policy),
                                    report: Arc::clone(&report),
                                },
                            )
                            .await;

                        return Ok(report);
                    }
                    AfterResponse::Modified(updated_cache_policy, _) => updated_cache_policy,
                }
            }
            None => CachePolicy::new_options(
                &request,
                &response,
                time_after_response,
                self.http_cache_options,
            ),
        };

        let body = response.text().await?;
        let feed = WeatherFeed::from_xml_str(&body)?;
        let report = Arc::new(Report::from_weather_feed(feed, location.tz)?);

        self.cache
            .insert(
                cache_key,
                CacheEntry {
                    cache_policy: Arc::new(cache_policy),
                    report: Arc::clone(&report),
                },
            )
            .await;

        Ok(report)
    }
}

#[derive(Clone)]
struct CacheEntry {
    cache_policy: Arc<CachePolicy>,
    report: Arc<Report>,
}

fn request_from_parts(parts: Parts) -> Request {
    let Parts {
        method,
        uri,
        headers,
        ..
    } = parts;

    let url = uri.to_string().parse().unwrap();

    let mut req = Request::new(method, url);
    *req.headers_mut() = headers;

    req
}

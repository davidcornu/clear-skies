use std::path::Path;

use color_eyre::eyre::Result;
use reqwest::{Client, Url};
use scraper::{Html, Selector};

const PROVINCE_AND_TERRITORY_CODES: [&str; 13] = [
    "AB", "BC", "MB", "NB", "NL", "NS", "NT", "NU", "ON", "PE", "QC", "SK", "YT",
];

const BASE_URL: &str = "https://weather.gc.ca/forecast/canada/index_e.html";
const RSS_BASE_URL: &str = "https://weather.gc.ca/rss/city/";

#[tokio::main]
pub async fn run(cache_dir: &Path) -> Result<()> {
    let client = Client::new();
    let selector = Selector::parse("a[href^='/city/pages/']").expect("always valid");
    let base_url = Url::parse(BASE_URL).expect("always valid");
    let rss_base_url = Url::parse(RSS_BASE_URL).expect("always valid");

    let mut test_feeds_path = cache_dir.to_owned();
    test_feeds_path.push("test_feeds");
    std::fs::create_dir_all(test_feeds_path)?;

    for id in PROVINCE_AND_TERRITORY_CODES {
        let mut url = base_url.clone();
        url.query_pairs_mut().append_pair("id", id);

        let response = client.get(url).send().await?;
        let body = response.text().await?;

        let document = Html::parse_document(&body);

        for element in document.select(&selector) {
            let Some(href) = element.value().attr("href") else {
                continue;
            };

            let Some(code) = href.split('/').find_map(|part| {
                if part.ends_with(".html") {
                    part.split_once('_').map(|(code, _)| code)
                } else {
                    None
                }
            }) else {
                continue;
            };

            let rss_url = rss_base_url
                .join(&format!("{code}_e.xml"))
                .expect("always valid");

            let mut path = cache_dir.to_owned();
            path.push("test_feeds");
            path.push(format!("{code}.xml"));

            eprintln!("fetching {rss_url}");

            let rss_response = client.get(rss_url).send().await?;
            let rss_text = rss_response.text().await?;
            tokio::fs::write(path, rss_text).await?;
        }
    }

    Ok(())
}

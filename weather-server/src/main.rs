mod location_search;

use std::{net::SocketAddrV4, ops::Bound, sync::Arc};

use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use dropshot::{
    endpoint, ApiDescription, ConfigDropshot, ConfigLogging, EmptyScanParams, HttpError,
    HttpResponseOk, HttpServerStarter, PaginationParams, Path, Query, RequestContext, ResultsPage,
    WhichPage,
};
use http::Response;
use hyper::Body;
use location_search::LocationSearch;
use rust_embed::RustEmbed;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use weather_lib::{
    locations::{location_index, Coordinates, ProvinceOrTerritory},
    weather_report::{CanadaTz, WeatherReport},
};
use weather_sync::Client as SyncClient;

#[derive(Parser)]
struct Cli {
    #[arg(long, env = "BIND_ADDR", default_value = "127.0.0.1:1971")]
    bind_addr: SocketAddrV4,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    run_server(cli.bind_addr)?;

    Ok(())
}

struct State {
    open_api_definition: serde_json::Value,
    sync_client: SyncClient,
    location_search: OnceCell<LocationSearch>,
}

impl State {
    pub async fn location_search(&self) -> Result<&LocationSearch> {
        self.location_search
            .get_or_try_init(|| async {
                tokio::task::spawn_blocking(|| {
                    let location_search = LocationSearch::new()?;
                    location_search.index_locations(
                        weather_lib::locations::data::LOCATIONS.iter().enumerate(),
                    )?;

                    Ok(location_search)
                })
                .await?
            })
            .await
    }
}

#[tokio::main]
async fn run_server(bind_addr: SocketAddrV4) -> Result<()> {
    let log = ConfigLogging::StderrTerminal {
        level: dropshot::ConfigLoggingLevel::Info,
    }
    .to_logger("weather-server")?;

    let mut api = ApiDescription::new();

    api.register(index).unwrap();
    api.register(openapi_schema).unwrap();
    api.register(swagger_ui).unwrap();
    api.register(locations).unwrap();
    api.register(locations_search).unwrap();
    api.register(weather).unwrap();

    let state = Arc::new(State {
        open_api_definition: api
            .openapi("Clear Skies", env!("CARGO_PKG_VERSION"))
            .json()
            .map_err(|e| eyre!("failed to generate openapi spec: {e:?}"))?,
        sync_client: SyncClient::new(Default::default()),
        location_search: OnceCell::new(),
    });

    let server_config = ConfigDropshot {
        bind_address: bind_addr.into(),
        request_body_max_bytes: 1024,
        tls: None,
    };

    let server = HttpServerStarter::new(&server_config, api, state, &log)
        .map_err(|e| eyre!("failed to start server: {e:?}"))?
        .start();

    server.await.map_err(|e| eyre!("server failed: {e:?}"))?;

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "src/html"]
struct StaticAsset;

fn static_response(file_path: &str) -> Result<Response<Body>, HttpError> {
    let file = StaticAsset::get(file_path)
        .ok_or_else(|| HttpError::for_not_found(None, format!("failed to load {file_path:?}")))?;

    let response = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(file.data))
        .unwrap();

    Ok(response)
}

#[endpoint(
    method = GET,
    path = "/",
    unpublished = true,
)]
async fn index(_rqctx: RequestContext<Arc<State>>) -> Result<Response<Body>, HttpError> {
    static_response("index.html")
}

/// Returns the OpenAPI v3.0.3 specification for this server.
#[endpoint(
    method = GET,
    path = "/openapi.json",
    tags = ["documentation"],
)]
async fn openapi_schema(
    rqctx: RequestContext<Arc<State>>,
) -> Result<HttpResponseOk<serde_json::Value>, HttpError> {
    Ok(HttpResponseOk(rqctx.context().open_api_definition.clone()))
}

/// Renders Swagger UI for this server's OpenAPI specification.
#[endpoint(
    method = GET,
    path = "/swagger-ui",
    tags = ["documentation"],
    unpublished = true
)]
async fn swagger_ui(_rqctx: RequestContext<Arc<State>>) -> Result<Response<Body>, HttpError> {
    static_response("swagger-ui.html")
}

/// A location for which weather data is available
#[derive(Serialize, JsonSchema)]
struct Location {
    /// The location name as listed by Environment Canada
    name: &'static str,
    /// The province or territory where the location is
    province_or_territory: ProvinceOrTerritory,
    /// The URL [slug](https://developer.mozilla.org/en-US/docs/Glossary/Slug) for this location.
    /// Location slugs are unique by province/territory.
    slug: &'static str,
    /// The location's local timezone
    tz: CanadaTz,
    /// The path to retrieve the weather for this location
    path: String,
    /// The URL for the Environment Canada RSS feed for this location
    feed_url: &'static str,
    /// The Environment Canada web page for this location
    html_url: &'static str,
    /// The latitude and longitude for the center of this location
    coordinates: Coordinates,
    #[serde(skip)]
    cursor: (ProvinceOrTerritory, &'static str),
}

impl Location {
    fn from_data(d: &weather_lib::locations::Location) -> Self {
        Self {
            name: d.name,
            province_or_territory: d.province_or_territory,
            slug: d.slug,
            tz: d.tz,
            path: format!("/weather/{}/{}", d.province_or_territory.to_abbr(), d.slug),
            feed_url: d.feed_url,
            html_url: d.html_url,
            coordinates: d.coordinates,
            cursor: (d.province_or_territory, d.slug),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LocationPage {
    province_or_territory: ProvinceOrTerritory,
    slug: String,
}

/// Listing of all available locations
#[endpoint(
    method = GET,
    path = "/locations",
    tags = ["locations"]
)]
async fn locations(
    rqctx: RequestContext<Arc<State>>,
    query: Query<PaginationParams<EmptyScanParams, LocationPage>>,
) -> Result<HttpResponseOk<ResultsPage<Location>>, HttpError> {
    let pag_params = query.into_inner();
    let limit = rqctx.page_limit(&pag_params)?.get() as usize;

    let locations = match &pag_params.page {
        WhichPage::First(_) => location_index()
            .iter()
            .take(limit)
            .map(|(_, l)| Location::from_data(l))
            .collect(),
        WhichPage::Next(LocationPage {
            province_or_territory,
            slug,
        }) => location_index()
            .range((
                Bound::Excluded((*province_or_territory, slug.as_str())),
                Bound::Unbounded,
            ))
            .take(limit)
            .map(|(_, l)| Location::from_data(l))
            .collect(),
    };

    Ok(HttpResponseOk(ResultsPage::new(
        locations,
        &EmptyScanParams {},
        |last_item, _scan_params| LocationPage {
            province_or_territory: last_item.cursor.0,
            slug: last_item.cursor.1.to_string(),
        },
    )?))
}

#[derive(Deserialize, JsonSchema)]
struct LocationsSearchQuery {
    /// Search query (e.g. "montreal")
    q: String,
}

/// Fuzzy search all available locations
#[endpoint(
    method = GET,
    path = "/locations/search",
    tags = ["locations"]
)]
async fn locations_search(
    rqctx: RequestContext<Arc<State>>,
    query: Query<LocationsSearchQuery>,
) -> Result<HttpResponseOk<Vec<Location>>, HttpError> {
    let state = rqctx.context();
    let search = state.location_search().await.map_err(|err| {
        HttpError::for_internal_error(format!("failed to initialize search index: {err:#?}"))
    })?;

    let search_text = query.into_inner().q;
    let results = search
        .query(&search_text)
        .map_err(|err| {
            HttpError::for_internal_error(format!(
                "failed to execute query {search_text:?}: {err:#?}"
            ))
        })?
        .into_iter()
        .map(|idx| Location::from_data(&weather_lib::locations::data::LOCATIONS[idx]))
        .collect::<Vec<_>>();

    Ok(HttpResponseOk(results))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct WeatherPath {
    province_or_territory: ProvinceOrTerritory,
    slug: String,
}

#[endpoint(
    method = GET,
    path = "/weather/{province_or_territory}/{slug}",
    tags = ["weather"]
)]
async fn weather(
    rqctx: RequestContext<Arc<State>>,
    path: Path<WeatherPath>,
) -> Result<HttpResponseOk<Arc<WeatherReport>>, HttpError> {
    let WeatherPath {
        province_or_territory,
        slug,
    } = path.into_inner();

    let location = weather_lib::locations::Location::lookup(province_or_territory, &slug)
        .ok_or_else(|| HttpError::for_not_found(None, "location lookup failed".to_string()))?;

    let state = rqctx.context();

    let report = state
        .sync_client
        .fetch_report(location)
        .await
        .map_err(|e| {
            HttpError::for_internal_error(format!("error while fetching report: {e:#?}"))
        })?;

    Ok(HttpResponseOk(report))
}

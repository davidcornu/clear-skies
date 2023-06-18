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
use indoc::indoc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
}

#[tokio::main]
async fn run_server(bind_addr: SocketAddrV4) -> Result<()> {
    let log = ConfigLogging::StderrTerminal {
        level: dropshot::ConfigLoggingLevel::Info,
    }
    .to_logger("weather-server")?;

    let mut api = ApiDescription::new();

    api.register(openapi_schema).unwrap();
    api.register(swagger_ui).unwrap();
    api.register(locations).unwrap();
    api.register(weather).unwrap();

    let state = Arc::new(State {
        open_api_definition: api
            .openapi("weather-server", "1.0.0")
            .json()
            .map_err(|e| eyre!("failed to generate openapi spec: {e:?}"))?,
        sync_client: SyncClient::new(Default::default()),
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

/// Returns the OpenAPI v3.0.3 specification for this server.
#[endpoint {
    method = GET,
    path = "/openapi.json",
    tags = ["documentation"],
}]
async fn openapi_schema(
    rqctx: RequestContext<Arc<State>>,
) -> Result<HttpResponseOk<serde_json::Value>, HttpError> {
    Ok(HttpResponseOk(rqctx.context().open_api_definition.clone()))
}

/// Renders Swagger UI for this server's OpenAPI specification.
#[endpoint {
    method = GET,
    path = "/swagger-ui",
    tags = ["documentation"],
}]
async fn swagger_ui(_rqctx: RequestContext<Arc<State>>) -> Result<Response<Body>, HttpError> {
    let body = indoc! {r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Weather Server | Swagger UI</title>
            <link
                rel="stylesheet"
                href="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/4.18.3/swagger-ui.min.css"
                integrity="sha512-oSy4rNbpqDUaoXIoFxhwKj/LFbkzUzo/WWJrn9RcIFLr4wm30upZ8r1OmhSRVvndMFZ2bhHEc2HklxjlS/aOyQ=="
                crossorigin="anonymous"
                referrerpolicy="no-referrer"
            />
        </head>
        <body>
            <div id="swagger-ui"></div>
            <script
                src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/4.18.3/swagger-ui-bundle.min.js"
                integrity="sha512-PijkKcRp7VDW1K2S8nNgljcNRrEQmazUc8sPiVRMciEuNzJzz2KeKb2Cjz/HdjZrKwmEYEyhOFZlOi0xzqWdqg=="
                crossorigin="anonymous"
                referrerpolicy="no-referrer"
            ></script>
            <script
                src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/4.18.3/swagger-ui-standalone-preset.min.js"
                integrity="sha512-dQDXlqaJDnXhlgBLVyoNPZeVC7dvQ1ylq/F9DRHbvJ4WMFVD1GtB7T8QmnWdhFy2sGJU2xxSsoPIEx85gnAXnw=="
                crossorigin="anonymous"
                referrerpolicy="no-referrer"
            ></script>
            <script>
                window.onload = () => {
                    window.ui = SwaggerUIBundle({
                        url: '/openapi.json',
                        dom_id: '#swagger-ui',
                        presets: [
                            SwaggerUIBundle.presets.apis,
                            SwaggerUIStandalonePreset
                        ],
                        layout: "StandaloneLayout",
                        deepLinking: true,
                    });
                };
            </script>
        </body>
        </html>
    "#};

    let response = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(body))
        .unwrap();

    Ok(response)
}

/// A location for which weather data is available
#[derive(Serialize, JsonSchema)]
struct Location {
    /// The location name as listed by Environment Canada
    name: &'static str,
    /// The province or territory where the location is
    province_or_territory: &'static str,
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
            province_or_territory: d.province_or_territory.name(),
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
#[endpoint(method = GET, path = "/locations")]
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

#[derive(Deserialize, JsonSchema, Debug)]
struct WeatherPath {
    province_or_territory: ProvinceOrTerritory,
    slug: String,
}

#[endpoint(method = GET, path = "/weather/{province_or_territory}/{slug}")]
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
        .fetch_report(&location)
        .await
        .map_err(|e| {
            HttpError::for_internal_error(format!("error while fetching report: {e:#?}"))
        })?;

    Ok(HttpResponseOk(report))
}

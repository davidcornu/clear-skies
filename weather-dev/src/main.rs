mod build_mapping;
mod fetch_feeds;
mod geocode;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    /// Download all available feeds from Environment Canada and store them in the cache directory.
    CacheFeeds,
    /// Use the cached feeds to generate `weather_lib::locations::data`
    BuildMapping {
        #[arg(long, env = "MAPBOX_ACCESS_TOKEN")]
        mapbox_access_token: Option<String>,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, env = "CACHE_DIR")]
    cache_dir: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Command::CacheFeeds => fetch_feeds::run(&cli.cache_dir)?,
        Command::BuildMapping {
            mapbox_access_token,
        } => build_mapping::run(&cli.cache_dir, mapbox_access_token.as_deref())?,
    }

    Ok(())
}

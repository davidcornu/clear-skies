mod build_mapping;
mod fetch_feeds;
mod geocode;
mod extract_conditions;

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
    /// Use the cached feeds to print out all the weather conditions
    ExtractConditions
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, env = "CACHE_DIR")]
    cache_dir: PathBuf,
}

impl Cli {
    fn absolute_cache_dir(&self) -> Result<PathBuf> {
        let cwd = std::env::current_dir()?;
        Ok(cwd.join(&self.cache_dir).canonicalize()?)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let cache_dir = cli.absolute_cache_dir()?;

    eprintln!("{}", cache_dir.display());

    match cli.command {
        Command::CacheFeeds => fetch_feeds::run(&cache_dir)?,
        Command::BuildMapping {
            mapbox_access_token,
        } => build_mapping::run(&cache_dir, mapbox_access_token.as_deref())?,
        Command::ExtractConditions => {
            extract_conditions::run(&cache_dir)?;
        }
    }

    Ok(())
}

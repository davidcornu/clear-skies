mod build_mapping;
mod fetch_feeds;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    /// Download all available feeds from Environment Canada and store them in the cache directory.
    CacheFeeds,
    /// Use the cached feeds to generate `weather_lib::locations::data`
    BuildMapping,
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
        Command::BuildMapping => build_mapping::run(&cli.cache_dir)?,
    }

    // for file_result in std::fs::read_dir("../cached_feeds")? {
    //     let file = file_result?;
    //     let mut reader = BufReader::new(File::open(file.path())?);

    //     let report = match WeatherFeed::from_xml_reader(&mut reader)
    //         .and_then(Report::from_weather_feed)
    //         .wrap_err_with(|| format!("{:?}", file.path()))
    //     {
    //         Ok(report) => report,
    //         Err(err) => {
    //             dbg!(err);
    //             continue;
    //         }
    //     };

    //     // if !report.special_weather_statements.is_empty() {
    //     //     println!("{:#?}", &report.special_weather_statements);
    //     // }

    //     println!("{:?}", file.path());
    //     println!("{:#?}", report);
    // }

    Ok(())
}

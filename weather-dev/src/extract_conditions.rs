use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use weather_lib::environment_canada::weather_feed::WeatherFeed;
use weather_lib::weather_report::{CanadaTz, WeatherReport};
use color_eyre::eyre::Result;

pub fn run(cache_dir: &Path) -> Result<()> {
    let mut distinct = HashSet::new();

    for file_result in std::fs::read_dir(cache_dir)? {
        let file = file_result?;
        dbg!(file.path());
        let mut reader = BufReader::new(File::open(file.path())?);
        let feed = WeatherFeed::from_xml_reader(&mut reader)?;
        let report = WeatherReport::from_weather_feed(feed, CanadaTz::Eastern)?;

        if let Some(current) = report.current_conditions.and_then(|c| c.condition) {
            distinct.insert(current);
        }

        for day in report.weather_forecasts {
            for forecast in day.forecast.forecasts() {
                distinct.insert(forecast.condition.clone());
            }
        }
    }

    for condition in distinct {
        println!("{condition}");
    }

    Ok(())
}
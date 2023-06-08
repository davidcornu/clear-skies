use std::{
    fs::{self, File},
    io::{BufReader, Write},
    panic,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use color_eyre::eyre::{eyre, Context, Result};
use deunicode::deunicode;
use indoc::{formatdoc, writedoc};
use slugs::register_slug;
use weather_lib::{
    environment_canada::weather_feed::WeatherFeed,
    locations::{OwnedLocation, ProvinceOrTerritory},
    weather_report::{CanadaTz, Report},
};

mod slugs {
    use color_eyre::eyre::{eyre, Result};
    use std::{
        collections::{HashMap, HashSet},
        sync::{Mutex, OnceLock},
    };

    use weather_lib::locations::ProvinceOrTerritory;

    static SLUGS: OnceLock<Mutex<HashMap<ProvinceOrTerritory, HashSet<String>>>> = OnceLock::new();

    fn slugs() -> &'static Mutex<HashMap<ProvinceOrTerritory, HashSet<String>>> {
        SLUGS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    pub fn register_slug(province_code: ProvinceOrTerritory, slug: &str) -> Result<()> {
        let mut map_guard = slugs().lock().unwrap();

        let province_set = map_guard.entry(province_code).or_default();

        if province_set.contains(slug) {
            return Err(eyre!("slug already exists: {province_code:?} {slug:?}"));
        }

        province_set.insert(slug.to_string());

        Ok(())
    }
}

// The following locations do not provide current conditions which means
// we can't infer a local timezone as forecasts aren't necessarily issued
// from the same timezone.
static MISSING_TIMEZONES: [(ProvinceOrTerritory, &str, CanadaTz); 18] = [
    (
        ProvinceOrTerritory::Alberta,
        "Vegreville",
        CanadaTz::Mountain,
    ),
    (
        ProvinceOrTerritory::BritishColumbia,
        "Dome Creek",
        CanadaTz::Pacific,
    ),
    (
        ProvinceOrTerritory::BritishColumbia,
        "Invermere",
        CanadaTz::Mountain,
    ),
    (
        ProvinceOrTerritory::BritishColumbia,
        "Liard River",
        CanadaTz::Mountain,
    ),
    (
        ProvinceOrTerritory::BritishColumbia,
        "McBride",
        CanadaTz::Pacific,
    ),
    (
        ProvinceOrTerritory::BritishColumbia,
        "Valemount",
        CanadaTz::Pacific,
    ),
    (
        ProvinceOrTerritory::Manitoba,
        "Little Grand Rapids",
        CanadaTz::Central,
    ),
    (
        ProvinceOrTerritory::Manitoba,
        "Oxford House",
        CanadaTz::Central,
    ),
    (
        ProvinceOrTerritory::Manitoba,
        "Shamattawa",
        CanadaTz::Central,
    ),
    (
        ProvinceOrTerritory::Manitoba,
        "York Factory",
        CanadaTz::Central,
    ),
    (
        ProvinceOrTerritory::NewfoundlandAndLabrador,
        "Rigolet",
        CanadaTz::Atlantic,
    ),
    (
        ProvinceOrTerritory::NorthwestTerritories,
        "Ekati (Lac de Gras)",
        CanadaTz::Mountain,
    ),
    (ProvinceOrTerritory::Ontario, "Gogama", CanadaTz::Eastern),
    (ProvinceOrTerritory::Ontario, "Killarney", CanadaTz::Eastern),
    (
        ProvinceOrTerritory::Quebec,
        "Manicouagan",
        CanadaTz::Eastern,
    ),
    (
        ProvinceOrTerritory::Quebec,
        "Murdochville",
        CanadaTz::Eastern,
    ),
    (ProvinceOrTerritory::Quebec, "Parent", CanadaTz::Eastern),
    (
        ProvinceOrTerritory::Yukon,
        "Dempster (Highway)",
        CanadaTz::Mountain,
    ),
];

fn git_project_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(eyre!("git command failed: {:?}", output.stderr));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let path = stdout
        .lines()
        .next()
        .ok_or_else(|| eyre!("expected git stdout to contain at least one line"))?;

    Ok(PathBuf::from(path))
}

pub fn run(cache_dir: &Path) -> Result<()> {
    let mut locations = Vec::new();

    for file_result in std::fs::read_dir(cache_dir)? {
        let file = file_result?;
        match handle_file(&file.path()).wrap_err_with(|| format!("{:?}", file.path())) {
            Ok(location) => {
                locations.push(location);
            }
            Err(err) => {
                eprintln!("{:#?}", err);
            }
        }
    }

    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut stdin = rustfmt.stdin.take().expect("failed to get stdin");
    let writer_thread = thread::spawn(move || write_data_file(&mut stdin, locations));

    let output = rustfmt.wait_with_output()?;

    match writer_thread.join() {
        Ok(result) => result?,
        Err(panic_payload) => {
            panic::resume_unwind(panic_payload);
        }
    }

    if !output.status.success() {
        return Err(eyre!("rustfmt failed"));
    }

    let root = git_project_root()?;
    let source_file = root
        .join("weather-lib")
        .join("src")
        .join("locations")
        .join("data.rs");

    eprintln!("writing to {:?}", source_file);

    fs::write(source_file, output.stdout)?;

    Ok(())
}

fn write_data_file<W>(mut w: W, mut locations: Vec<OwnedLocation>) -> Result<()>
where
    W: Write,
{
    locations.sort_by_cached_key(|l| (l.province_or_territory, l.name.clone()));

    writedoc!(
        w,
        r#"
            //! In-memory listing of all locations provided by Environment Canada.
            //! This file is automatically generated using `weather-dev build-mapping`.
            //! DO NOT EDIT MANUALLY.

            use super::{{Location, ProvinceOrTerritory}};
            use crate::weather_report::CanadaTz;

            pub static LOCATIONS: [Location;{}] = [
       "#,
        locations.len()
    )?;

    for location in &locations {
        writeln!(w, "{},", location_literal(location))?;
    }

    writeln!(w, "];\n")?;

    Ok(())
}

fn location_literal(location: &OwnedLocation) -> String {
    let OwnedLocation {
        province_or_territory,
        slug,
        name,
        tz,
        feed_url,
        html_url,
    } = location;

    formatdoc! {r#"
        Location {{
            name: {name:?},
            province_or_territory: ProvinceOrTerritory::{province_or_territory:?},
            slug: {slug:?},
            tz: CanadaTz::{tz:?},
            feed_url: {feed_url:?},
            html_url: {html_url:?},
        }}
    "#}
}

fn handle_file(path: &Path) -> Result<OwnedLocation> {
    let mut reader = BufReader::new(File::open(path)?);

    let feed = WeatherFeed::from_xml_reader(&mut reader)?;

    let feed_url = feed
        .links
        .iter()
        .find_map(|l| (l.rel.as_deref() == Some("self")).then(|| l.href.clone()))
        .ok_or_else(|| eyre!("could not find self link"))?;

    let html_url = feed
        .links
        .iter()
        .find_map(|l| (l.rel.as_deref() == Some("related")).then(|| l.href.clone()))
        .ok_or_else(|| eyre!("could not find related link"))?;

    let province_or_territory = path
        .file_stem()
        .and_then(|os_str| os_str.to_str())
        .and_then(|s| s.split_once('-'))
        .and_then(|(abbr, _)| ProvinceOrTerritory::from_abbr(abbr))
        .ok_or_else(|| eyre!("failed to extract province code and id from file name"))?;

    let name = feed
        .title
        .trim_end_matches(" - Weather - Environment Canada")
        .to_string();

    let slug = slug(&name);
    register_slug(province_or_territory, &slug)?;

    // Note that dates in this report with be invalid as using Canada/Eastern
    // as a placeholder. This is fine as we aren't actually using the forecasts.
    let report = Report::from_weather_feed(feed, CanadaTz::Eastern)?;

    let tz = match report.detected_timezone() {
        Some(tz) => tz,
        None => {
            let fallback =
                MISSING_TIMEZONES
                    .iter()
                    .find_map(|(candidate_province, candidate_name, tz)| {
                        (province_or_territory == *candidate_province && &name == candidate_name)
                            .then(|| *tz)
                    });

            fallback.ok_or_else(|| {
                eyre!("could not find time zone for {name} ({province_or_territory:?}) {html_url}")
            })?
        }
    };

    Ok(OwnedLocation {
        name,
        province_or_territory,
        slug,
        tz,
        feed_url,
        html_url,
    })
}

fn adjust_name(input: &str) -> String {
    let Some((main, mut alternate)) = input.split_once(" (") else {
        return input.to_string();
    };

    alternate = alternate.trim_end_matches(")");

    match alternate {
        "University of" | "Réserve faunique" | "Parc national" | "Réservoir" => {
            format!("{alternate} {main}")
        }
        "Sanctuary" | "Provincial Park" | "National Park" | "Highway" => {
            format!("{main} {alternate}")
        }
        _ => {
            format!("{main} {alternate}")
        }
    }
}

fn slug(input: &str) -> String {
    let adjusted = adjust_name(input);
    let ascii = deunicode(&adjusted);
    let mut slug = String::with_capacity(ascii.len());

    for c in ascii.chars() {
        if c.is_ascii_alphabetic() {
            slug.push(c.to_ascii_lowercase())
        } else {
            if !slug.ends_with('-') {
                slug.push('-');
            }
        }
    }

    slug
}

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    combinator::{map, opt},
    error::Error as NomError,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub enum Condition {
    Blizzard,
    BlowingSnow,
    Clear,
    Cloudy,
    Drizzle,
    Flurries,
    Fog,
    FreezingDrizzle,
    FreezingFog,
    FreezingRain,
    Haze,
    IceCrystals,
    IcePellets,
    Mist,
    Overcast,
    Rain,
    Snow,
    SnowSqualls,
    Sunny,
    Other(String),
}

pub fn conditions(input: &str) -> Result<(&str, Vec<Condition>), NomError<String>> {
    separated_list1(
        conjunction,
        delimited(
            tuple((
                opt(terminated(prefix_frequency, tag(" "))),
                opt(terminated(qualifier, tag(" "))),
            )),
            alt((
                map(tag_no_case("blizzard"), |_| Condition::Blizzard),
                map(
                    terminated(tag_no_case("clear"), opt(tag_no_case("ing"))),
                    |_| Condition::Clear,
                ),
                map(
                    terminated(
                        tag_no_case("cloud"),
                        opt(alt((
                            tag_no_case("s"),
                            tag_no_case("y"),
                            tag_no_case("iness"),
                        ))),
                    ),
                    |_| Condition::Cloudy,
                ),
                map(tag_no_case("drizzle"), |_| Condition::Drizzle),
                map(tag_no_case("freezing drizzle"), |_| {
                    Condition::FreezingDrizzle
                }),
                map(tag_no_case("flurries"), |_| Condition::Flurries),
                map(tag_no_case("fog"), |_| Condition::Fog),
                map(tag_no_case("freezing fog"), |_| Condition::FreezingFog),
                map(tag_no_case("haze"), |_| Condition::Haze),
                map(tag_no_case("ice crystals"), |_| Condition::IceCrystals),
                map(tag_no_case("ice pellets"), |_| Condition::IcePellets),
                map(tag_no_case("mist"), |_| Condition::Mist),
                map(tag_no_case("overcast"), |_| Condition::Overcast),
                map(
                    alt((
                        terminated(
                            tag_no_case("rain"),
                            opt(alt((tag_no_case(" showers"), tag_no_case("shower")))),
                        ),
                        tag_no_case("showers"),
                    )),
                    |_| Condition::Rain,
                ),
                map(tag_no_case("freezing rain"), |_| Condition::FreezingRain),
                map(
                    terminated(
                        tag_no_case("snow "),
                        alt((tag_no_case("squalls"), tag_no_case("sqalls"))),
                    ),
                    |_| Condition::SnowSqualls,
                ),
                map(
                    terminated(tag_no_case("snow"), opt(tag_no_case("shower"))),
                    |_| Condition::Snow,
                ),
                map(tag_no_case("blowing snow"), |_| Condition::BlowingSnow),
                map(
                    terminated(tag_no_case("sun"), opt(tag_no_case("ny"))),
                    |_| Condition::Sunny,
                ),
            )),
            opt(preceded(tag(" "), suffix_frequency)),
        ),
    )(input)
    .map_err(|e| e.to_owned())
    .finish()
}

fn conjunction(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case(" and "),
        tag_no_case(" or "),
        tag_no_case(" mixed with "),
    ))(input)
}

fn prefix_frequency(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("a few"),
        tag_no_case("chance of"),
        tag_no_case("mainly"),
        tag_no_case("mostly"),
        tag_no_case("partly"),
        tag_no_case("periods of"),
        tag_no_case("increasing"),
        tag_no_case("a mix of"),
    ))(input)
}

fn suffix_frequency(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("periods"),
        tag_no_case("at times heavy"),
        tag_no_case("patches"),
    ))(input)
}

fn qualifier(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("light"),
        tag_no_case("heavy"),
        tag_no_case("wet"),
        tag_no_case("drifting"),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::eyre;

    #[test]
    fn test_conditions() {
        // Obtained by running `weather-dev`'s `cache-feeds` and `extract-conditions` commands
        let cases = [
            "A few clouds",
            "A few flurries or rain showers",
            "A few flurries",
            "A few showers",
            "A mix of sun and cloud",
            "Blizzard",
            "Blowing snow",
            "Chance of drizzle or rain",
            "Chance of drizzle",
            "Chance of flurries or rain showers",
            "Chance of flurries",
            "Chance of light snow",
            "Chance of rain showers or flurries",
            "Chance of rain showers or wet flurries",
            "Chance of rain",
            "Chance of showers or drizzle",
            "Chance of showers",
            "Chance of snow sqalls",
            "Chance of snow",
            "Clear",
            "Clearing",
            "Cloudy periods",
            "Cloudy",
            "Drifting Snow",
            "Drizzle mixed with freezing drizzle",
            "Drizzle",
            "Flurries or rain showers",
            "Flurries",
            "Fog Patches",
            "Fog",
            "Freezing Fog",
            "Freezing rain or rain",
            "Freezing rain or snow",
            "Haze",
            "Heavy Snow",
            "Ice Crystals",
            "Increasing cloudiness",
            "Light Drizzle",
            "Light Freezing Drizzle",
            "Light Rain and Fog",
            "Light Rain",
            "Light Rainshower",
            "Light Snow and Blowing Snow",
            "Light snow mixed with rain",
            "Light snow",
            "Light Snow",
            "Light Snowshower",
            "Mainly Clear",
            "Mainly cloudy",
            "Mainly sunny",
            "Mainly Sunny",
            "Mist",
            "Mostly Cloudy",
            "Overcast",
            "Partly cloudy",
            "Partly Cloudy",
            "Periods of drizzle or rain",
            "Periods of drizzle",
            "Periods of light snow mixed with freezing drizzle",
            "Periods of light snow",
            "Periods of rain mixed with snow",
            "Periods of rain or drizzle",
            "Periods of rain or freezing rain",
            "Periods of rain or snow",
            "Periods of rain",
            "Periods of snow and blowing snow",
            "Periods of snow mixed with rain",
            "Periods of snow or rain",
            "Periods of snow",
            "Periods of wet snow mixed with rain",
            "Rain at times heavy",
            "Rain mixed with snow",
            "Rain or drizzle",
            "Rain or snow",
            "Rain showers or flurries",
            "Rain",
            "Showers or drizzle",
            "Showers",
            "Snow and blowing snow",
            "Snow and Blowing Snow",
            "Snow mixed with rain",
            "Snow or rain",
            "Snow squalls",
            "Snow",
            "Sunny",
        ];

        let mappings = cases
            .iter()
            .map(|input| {
                conditions(input)
                    .map_err(Into::into)
                    .and_then(|(remainder, parsed)| {
                        if remainder.is_empty() {
                            Ok((input, parsed))
                        } else {
                            Err(eyre!(
                                "Expected parser to consume all of {input:?} but got {remainder:?}"
                            ))
                        }
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        insta::assert_debug_snapshot!(mappings);
    }
}

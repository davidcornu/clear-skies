use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Condition {
    Blizzard,
    Clear,
    Cloudy,
    Drizzle,
    FreezingDrizzle,
    Flurries,
    Fog,
    FreezingFog,
    Haze,
    IceCrystals,
    IcePellets,
    Mist,
    Overcast,
    Rain,
    FreezingRain,
    Snow,
    SnowSqualls,
    BlowingSnow,
    Sunny,
    Other(String),
}

fn conditions(input: &str) -> IResult<&str, Vec<Condition>> {
    separated_list1(
        alt((
            tag_no_case(" and "),
            tag_no_case(" or "),
            tag_no_case(" mixed with "),
        )),
        preceded(
            tuple((
                opt(terminated(
                    alt((
                        tag_no_case("a few"),
                        tag_no_case("chance of"),
                        tag_no_case("mainly"),
                        tag_no_case("mostly"),
                        tag_no_case("partly"),
                        tag_no_case("periods of"),
                        tag_no_case("increasing"),
                        tag_no_case("a mix of"),
                    )),
                    tag(" "),
                )),
                opt(terminated(
                    alt((
                        tag_no_case("light"),
                        tag_no_case("wet"),
                        tag_no_case("drifting"),
                    )),
                    tag(" "),
                )),
            )),
            alt((
                map(tag_no_case("blizzard"), |_| Condition::Blizzard),
                map(tag_no_case("clear"), |_| Condition::Clear),
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
                map(tag_no_case("snow"), |_| Condition::Snow),
                map(tag_no_case("blowing snow"), |_| Condition::BlowingSnow),
                map(
                    terminated(tag_no_case("sun"), opt(tag_no_case("ny"))),
                    |_| Condition::Sunny,
                ),
            )),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditions() {
        let cases = [
            "A few clouds",
            "A few flurries",
            "A mix of sun and cloud",
            "Blizzard",
            "Blowing snow",
            "Chance of drizzle",
            "Chance of flurries",
            "Chance of flurries or rain showers",
            "Chance of light snow",
            "Chance of rain",
            "Chance of rain showers or flurries",
            "Chance of rain showers or wet flurries",
            "Chance of showers",
            "Chance of showers or drizzle",
            "Chance of snow",
            "Chance of snow sqalls",
            "Clear",
            "Clearing",
            "Cloudy",
            "Cloudy periods",
            "Drifting Snow",
            "Drizzle",
            "Drizzle mixed with freezing drizzle",
            "Flurries",
            "Flurries or rain showers",
            "Fog",
            "Freezing Fog",
            "Freezing rain or rain",
            "Freezing rain or snow",
            "Haze",
            "Ice Crystals",
            "Increasing cloudiness",
            "Light Drizzle",
            "Light Rain",
            "Light Rain and Fog",
            "Light Rainshower",
            "Light Snow",
            "Light Snowshower",
            "Light snow",
            "Mainly Clear",
            "Mainly Sunny",
            "Mainly cloudy",
            "Mainly sunny",
            "Mist",
            "Mostly Cloudy",
            "Overcast",
            "Partly Cloudy",
            "Partly cloudy",
            "Periods of drizzle",
            "Periods of drizzle or rain",
            "Periods of light snow",
            "Periods of light snow mixed with freezing drizzle",
            "Periods of rain",
            "Periods of rain mixed with snow",
            "Periods of rain or drizzle",
            "Periods of rain or freezing rain",
            "Periods of rain or snow",
            "Periods of snow",
            "Periods of snow and blowing snow",
            "Periods of snow or rain",
            "Periods of wet snow mixed with rain",
            "Rain",
            "Rain at times heavy",
            "Rain mixed with snow",
            "Rain or drizzle",
            "Rain or snow",
            "Rain showers or flurries",
            "Showers",
            "Showers or drizzle",
            "Snow",
            "Snow and blowing snow",
            "Snow at times heavy",
            "Snow mixed with ice pellets",
            "Snow mixed with rain",
            "Snow or rain",
            "Snow squalls",
            "Sunny",
            "Wet snow mixed with rain",
        ];

        let mappings = cases
            .iter()
            .map(|input| conditions(input).map(|(_, parsed)| (input, parsed)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        insta::assert_debug_snapshot!(mappings);
    }
}
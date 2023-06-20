use chrono::Weekday;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1},
    character::complete::{char, u8},
    combinator::{map, opt},
    error::Error as NomError,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Finish, IResult,
};

use super::common::{f32, weekday};
use crate::weather_report::forecasts::{Temperature, TemperatureTrend};

fn temperature_trend(input: &str) -> IResult<&str, TemperatureTrend> {
    alt((
        map(tag("High"), |_| TemperatureTrend::High),
        map(tag("Low"), |_| TemperatureTrend::Low),
        map(tag("Temperature steady near"), |_| TemperatureTrend::Steady),
        map(tag("Temperature falling to"), |_| TemperatureTrend::Low),
    ))(input)
}

fn temperature_degrees(input: &str) -> IResult<&str, f32> {
    map(
        alt((
            pair(map(tag("plus "), |_| true), f32),
            pair(map(tag("minus "), |_| false), f32),
            map(tag("zero"), |_| (true, 0.0)),
            map(f32, |num| (true, num)),
        )),
        |(is_positive, num)| {
            if is_positive {
                num
            } else {
                -num
            }
        },
    )(input)
}

fn temperature(input: &str) -> IResult<&str, Temperature> {
    map(
        pair(temperature_trend, preceded(char(' '), temperature_degrees)),
        |(trend, degrees_c)| Temperature { trend, degrees_c },
    )(input)
}

fn weekday_with_night(input: &str) -> IResult<&str, (Weekday, bool)> {
    pair(weekday, map(opt(tag(" night")), |night| night.is_some()))(input)
}

fn probability_of_precipitation(input: &str) -> IResult<&str, u8> {
    delimited(tag("POP "), u8, char('%'))(input)
}

#[derive(Debug, PartialEq)]
pub struct Parsed {
    pub weekday: Weekday,
    pub is_night: bool,
    pub condition: String,
    pub temperature: Temperature,
    pub probability_of_precipitation: Option<u8>,
}

pub fn parse(input: &str) -> Result<(&str, Parsed), NomError<String>> {
    map(
        tuple((
            terminated(weekday_with_night, char(':')),
            delimited(char(' '), take_until1("."), char('.')),
            delimited(
                char(' '),
                temperature,
                pair(take_until("."), char('.')), // Skip additional temperature information like frost
            ),
            opt(preceded(char(' '), probability_of_precipitation)),
        )),
        |((weekday, is_night), condition, temperature, pop)| Parsed {
            weekday,
            is_night,
            condition: condition.to_string(),
            temperature,
            probability_of_precipitation: pop,
        },
    )(input)
    .map_err(|e| e.to_owned())
    .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekday_with_night() {
        assert_eq!(
            Ok(("", (Weekday::Mon, false))),
            weekday_with_night("Monday")
        );
        assert_eq!(
            Ok(("", (Weekday::Fri, true))),
            weekday_with_night("Friday night")
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("Sunday night: Chance of showers. High 19. POP 60%"),
            Ok((
                "",
                Parsed {
                    weekday: Weekday::Sun,
                    is_night: true,
                    condition: "Chance of showers".to_string(),
                    temperature: Temperature {
                        trend: TemperatureTrend::High,
                        degrees_c: 19.0
                    },
                    probability_of_precipitation: Some(60)
                }
            ))
        );

        assert_eq!(
            parse("Thursday night: Clear. Low plus 1 with patchy frost."),
            Ok((
                "",
                Parsed {
                    weekday: Weekday::Thu,
                    is_night: true,
                    condition: "Clear".to_string(),
                    temperature: Temperature {
                        trend: TemperatureTrend::Low,
                        degrees_c: 1.0
                    },
                    probability_of_precipitation: None
                }
            ))
        )
    }
}

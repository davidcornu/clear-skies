mod common;
pub mod pressure;
pub mod title;
pub mod wind;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, u8},
    combinator::map,
    error::Error as NomError,
    multi::many_till,
    sequence::{pair, preceded, terminated},
    Finish, IResult,
};

use common::f32;

use self::common::local_datetime;

use super::{current_conditions::ObservedAt, LocalDateTime};

pub fn temperature(input: &str) -> Result<(&str, f32), NomError<String>> {
    terminated(f32, tag("°C"))(input)
        .map_err(|e| e.to_owned())
        .finish()
}

pub fn aqhi(input: &str) -> Result<(&str, Option<u8>), NomError<String>> {
    let res: IResult<&str, Option<u8>> = alt((map(tag("n/a"), |_| None), map(u8, Some)))(input);
    res.map_err(|e| e.to_owned()).finish()
}

pub fn visibility(input: &str) -> Result<(&str, f32), NomError<String>> {
    terminated(f32, tag(" km"))(input)
        .map_err(|e| e.to_owned())
        .finish()
}

pub fn humidity(input: &str) -> Result<(&str, u8), NomError<String>> {
    let res: IResult<&str, u8> = terminated(u8, tag(" %"))(input);
    res.map_err(|e| e.to_owned()).finish()
}

pub fn unitless_temperature(input: &str) -> Result<(&str, f32), NomError<String>> {
    f32(input).map_err(|e| e.to_owned()).finish()
}

// Paulatuk Airport 9:00 AM MDT Thursday 25 May 2023
pub fn observed_at(input: &str) -> Result<(&str, ObservedAt), NomError<String>> {
    map(
        many_till(anychar, preceded(char(' '), local_datetime)),
        |(location_chars, local_datetime)| {
            let location = location_chars.into_iter().collect::<String>();

            ObservedAt {
                location,
                datetime: local_datetime,
            }
        },
    )(input)
    .map_err(|e| e.to_owned())
    .finish()
}

pub fn special_weather_statement_summary(
    input: &str,
) -> Result<(&str, (&str, LocalDateTime)), NomError<String>> {
    let delimiter = " Issued: ";

    pair(
        take_until(delimiter),
        preceded(tag(delimiter), local_datetime),
    )(input)
    .map_err(|e| e.to_owned())
    .finish()
}

// A mix of sun and cloud. Wind southwest 30 km/h gusting to 60. High 21. UV index 7 or high. Forecast issued 11:30 AM EDT Monday 15 May 2023
pub fn forecast_summary(input: &str) -> Result<(&str, (&str, LocalDateTime)), NomError<String>> {
    let delimiter = " Forecast issued ";

    pair(
        take_until(delimiter),
        preceded(tag(delimiter), local_datetime),
    )(input)
    .map_err(|e| e.to_owned())
    .finish()
}

#[cfg(test)]
mod tests {
    use chrono::{FixedOffset, TimeZone};

    use crate::weather_report::CanadaTz;

    use super::*;

    #[test]
    fn test_temperature() {
        assert_eq!(temperature("12.3°C"), Ok(("", 12.3)));
        assert_eq!(temperature("-12.3°C"), Ok(("", -12.3)));
        assert_eq!(temperature("-12°C"), Ok(("", -12.0)));
        assert_eq!(temperature("14.7°C"), Ok(("", 14.7)));
    }

    #[test]
    fn test_aqhi() {
        assert_eq!(Ok(("", None)), aqhi("n/a"));
        assert_eq!(Ok(("", Some(1))), aqhi("1"));
    }

    #[test]
    fn test_visibility() {
        assert_eq!(Ok(("", 24.1)), visibility("24.1 km"));
        assert_eq!(Ok(("", 1.0)), visibility("1 km"));
    }

    #[test]
    fn test_observed_at() {
        assert_eq!(
            observed_at("Paulatuk Airport 9:00 AM MDT Thursday 25 May 2023"),
            Ok((
                "",
                ObservedAt {
                    location: "Paulatuk Airport".to_string(),
                    datetime: LocalDateTime {
                        ts: FixedOffset::west_opt(6 * 3600)
                            .unwrap()
                            .with_ymd_and_hms(2023, 5, 25, 9, 0, 0)
                            .unwrap(),
                        tz: CanadaTz::Mountain
                    }
                }
            ))
        );

        assert_eq!(
            observed_at("Montréal-Trudeau Int'l Airport 9:00 PM EDT Thursday 25 May 2023"),
            Ok((
                "",
                ObservedAt {
                    location: "Montréal-Trudeau Int'l Airport".to_string(),
                    datetime: LocalDateTime {
                        ts: FixedOffset::west_opt(4 * 3600)
                            .unwrap()
                            .with_ymd_and_hms(2023, 5, 25, 21, 0, 0)
                            .unwrap(),
                        tz: CanadaTz::Eastern
                    }
                }
            ))
        );

        assert_eq!(
            observed_at("Sainte-Anne-de-Bellevue 12:00 AM EDT Thursday 25 May 2023"),
            Ok((
                "",
                ObservedAt {
                    location: "Sainte-Anne-de-Bellevue".to_string(),
                    datetime: LocalDateTime {
                        ts: FixedOffset::west_opt(4 * 3600)
                            .unwrap()
                            .with_ymd_and_hms(2023, 5, 25, 0, 0, 0)
                            .unwrap(),
                        tz: CanadaTz::Eastern
                    }
                }
            ))
        );

        assert_eq!(
            observed_at("Bella Coola Airport 12:00 PM PDT Thursday 25 May 2023"),
            Ok((
                "",
                ObservedAt {
                    location: "Bella Coola Airport".to_string(),
                    datetime: LocalDateTime {
                        ts: FixedOffset::west_opt(7 * 3600)
                            .unwrap()
                            .with_ymd_and_hms(2023, 5, 25, 12, 0, 0)
                            .unwrap(),
                        tz: CanadaTz::Pacific
                    }
                }
            ))
        )
    }

    #[test]
    fn test_special_weather_statement_summary() {
        assert_eq!(
            special_weather_statement_summary("Persons in or near this area should be on the lookout for adverse weather conditions and take necessary safety precautions. Issued: 12:00 noon EST Saturday 9 December 2023"),
            Ok((
               "",
               (
                   "Persons in or near this area should be on the lookout for adverse weather conditions and take necessary safety precautions.",
                    LocalDateTime {
                        ts: FixedOffset::west_opt(5 * 3600)
                            .unwrap()
                            .with_ymd_and_hms(2023, 12, 9, 12, 0, 0)
                            .unwrap(),
                        tz: CanadaTz::Eastern
                    }
               )
            ))
        );
    }
}

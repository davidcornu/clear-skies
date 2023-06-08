use crate::weather_report::current_conditions::{Pressure, PressureTendency};

use super::common::f32;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    error::Error as NomError,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

fn kpa(input: &str) -> IResult<&str, f32> {
    terminated(f32, tag(" kPa"))(input)
}

fn tendency(input: &str) -> IResult<&str, PressureTendency> {
    alt((
        map(tag("falling"), |_| PressureTendency::Falling),
        map(tag("rising"), |_| PressureTendency::Rising),
        map(tag("steady"), |_| PressureTendency::Steady),
    ))(input)
}

pub fn parse(input: &str) -> Result<(&str, Pressure), NomError<String>> {
    map(
        tuple((kpa, opt(preceded(char(' '), tendency)))),
        |(kpa, tendency)| Pressure { kpa, tendency },
    )(input)
    .map_err(|e| e.to_owned())
    .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("102.4 kPa falling"),
            Ok((
                "",
                Pressure {
                    kpa: 102.4,
                    tendency: Some(PressureTendency::Falling)
                }
            ))
        );

        assert_eq!(
            parse("102.8 kPa rising"),
            Ok((
                "",
                Pressure {
                    kpa: 102.8,
                    tendency: Some(PressureTendency::Rising)
                }
            ))
        );

        assert_eq!(
            parse("103.3 kPa"),
            Ok((
                "",
                Pressure {
                    kpa: 103.3,
                    tendency: None
                }
            ))
        );

        assert_eq!(
            parse("101 kPa steady"),
            Ok((
                "",
                Pressure {
                    kpa: 101.0,
                    tendency: Some(PressureTendency::Steady)
                }
            ))
        )
    }
}

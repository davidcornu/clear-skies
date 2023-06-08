use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, u8},
    combinator::{map, map_res, opt},
    error::Error as NomError,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

use crate::weather_report::current_conditions::{CardinalDirection, Wind, WindSpeed};

fn cardinal_direction(input: &str) -> IResult<&str, CardinalDirection> {
    map_res(
        take_while_m_n(1, 3, |c: char| matches!(c, 'N' | 'S' | 'E' | 'W')),
        |s: &str| s.parse::<CardinalDirection>(),
    )(input)
}

fn kph(input: &str) -> IResult<&str, u8> {
    terminated(u8, tag(" km/h"))(input)
}

pub fn parse(input: &str) -> Result<(&str, Wind), NomError<String>> {
    map(
        tuple((
            opt(terminated(cardinal_direction, char(' '))),
            alt((
                map(kph, WindSpeed::Kph),
                map(tag("calm km/h"), |_| WindSpeed::Calm),
            )),
            opt(preceded(tag(" gust "), kph)),
        )),
        |(direction, speed, gust_kph)| Wind {
            direction,
            speed,
            gust_kph,
        },
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
            parse("NNE 20 km/h gust 40 km/h"),
            Ok((
                "",
                Wind {
                    direction: Some(CardinalDirection::NNE),
                    speed: WindSpeed::Kph(20),
                    gust_kph: Some(40)
                }
            ))
        );

        assert_eq!(
            parse("18 km/h gust 30 km/h"),
            Ok((
                "",
                Wind {
                    direction: None,
                    speed: WindSpeed::Kph(18),
                    gust_kph: Some(30)
                }
            ))
        );

        assert_eq!(
            parse("SSE 27 km/h"),
            Ok((
                "",
                Wind {
                    direction: Some(CardinalDirection::SSE),
                    speed: WindSpeed::Kph(27),
                    gust_kph: None
                }
            ))
        );

        assert_eq!(
            parse("13 km/h"),
            Ok((
                "",
                Wind {
                    direction: None,
                    speed: WindSpeed::Kph(13),
                    gust_kph: None
                }
            ))
        );

        assert_eq!(
            parse("NNE calm km/h"),
            Ok((
                "",
                Wind {
                    direction: Some(CardinalDirection::NNE),
                    speed: WindSpeed::Calm,
                    gust_kph: None
                }
            ))
        )
    }
}

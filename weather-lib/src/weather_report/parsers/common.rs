use chrono::{Month, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, i32, u32},
    combinator::{map, map_opt, map_res, opt, recognize},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use crate::weather_report::{CanadaTz, LocalDateTime};

pub fn f32(input: &str) -> IResult<&str, f32> {
    let with_seperator = recognize(tuple((digit1, preceded(char('.'), digit1))));
    map_res(
        recognize(pair(opt(char('-')), alt((with_seperator, digit1)))),
        |s: &str| s.parse::<f32>(),
    )(input)
}

pub fn weekday(input: &str) -> IResult<&str, Weekday> {
    alt((
        map(tag("Monday"), |_| Weekday::Mon),
        map(tag("Tuesday"), |_| Weekday::Tue),
        map(tag("Wednesday"), |_| Weekday::Wed),
        map(tag("Thursday"), |_| Weekday::Thu),
        map(tag("Friday"), |_| Weekday::Fri),
        map(tag("Saturday"), |_| Weekday::Sat),
        map(tag("Sunday"), |_| Weekday::Sun),
    ))(input)
}

fn month(input: &str) -> IResult<&str, Month> {
    alt((
        map(tag("January"), |_| Month::January),
        map(tag("February"), |_| Month::February),
        map(tag("March"), |_| Month::March),
        map(tag("April"), |_| Month::April),
        map(tag("May"), |_| Month::May),
        map(tag("June"), |_| Month::June),
        map(tag("July"), |_| Month::July),
        map(tag("August"), |_| Month::August),
        map(tag("September"), |_| Month::September),
        map(tag("October"), |_| Month::October),
        map(tag("November"), |_| Month::November),
        map(tag("December"), |_| Month::December),
    ))(input)
}

fn abbreviated_timezone(input: &str) -> IResult<&str, CanadaTz> {
    terminated(
        alt((
            map(char('A'), |_| CanadaTz::Atlantic),
            map(char('C'), |_| CanadaTz::Central),
            map(char('E'), |_| CanadaTz::Eastern),
            map(char('M'), |_| CanadaTz::Mountain),
            map(char('N'), |_| CanadaTz::Newfoundland),
            map(char('P'), |_| CanadaTz::Pacific),
        )),
        pair(alt((char('D'), char('S'))), char('T')),
    )(input)
}

enum PeriodOfDay {
    AM,
    PM,
}

fn am_pm(input: &str) -> IResult<&str, PeriodOfDay> {
    alt((
        map(tag("AM"), |_| PeriodOfDay::AM),
        map(tag("a.m."), |_| PeriodOfDay::AM),
        map(tag("PM"), |_| PeriodOfDay::PM),
        map(tag("p.m."), |_| PeriodOfDay::PM),
        map(tag("noon"), |_| PeriodOfDay::PM),
    ))(input)
}

fn time(input: &str) -> IResult<&str, NaiveTime> {
    map_opt(
        tuple((u32, preceded(char(':'), u32), preceded(char(' '), am_pm))),
        |(hours, minutes, am_pm)| {
            let adjusted_hours = match (hours, am_pm) {
                (12, PeriodOfDay::AM) => 0,
                (12, PeriodOfDay::PM) => 12,
                (hours, PeriodOfDay::AM) => hours,
                (hours, PeriodOfDay::PM) => hours + 12,
            };
            NaiveTime::from_hms_opt(adjusted_hours, minutes, 0)
        },
    )(input)
}

fn date(input: &str) -> IResult<&str, NaiveDate> {
    map_opt(
        tuple((u32, preceded(char(' '), month), preceded(char(' '), i32))),
        |(day, month, year)| NaiveDate::from_ymd_opt(year, month.number_from_month(), day),
    )(input)
}

pub fn local_datetime(input: &str) -> IResult<&str, LocalDateTime> {
    map_opt(
        tuple((
            time,
            preceded(char(' '), abbreviated_timezone),
            preceded(char(' '), weekday),
            preceded(char(' '), date),
        )),
        |(time, tz, _weekday, date)| {
            let local = NaiveDateTime::new(date, time);
            tz.chrono()
                .from_local_datetime(&local)
                .single()
                .map(|dt| LocalDateTime {
                    ts: dt.fixed_offset(),
                    tz,
                })
        },
    )(input)
}

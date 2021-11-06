use anyhow::{anyhow, bail, Error, Result};
use std::default::Default;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::ops::Not as _;
use std::process::exit;
use std::str::FromStr;

const EXIT_FAILURE: i32 = 1;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        exit(EXIT_FAILURE);
    }
}

fn run() -> Result<()> {
    let input = env::args().skip(1).collect::<Vec<_>>();

    if input.is_empty() {
        bail!("missing input");
    }

    let result = parse_and_solve(&input)?;
    println!("{}", result);
    Ok(())
}

fn parse_and_solve(input: &[String]) -> Result<Duration> {
    let equation = parse_equation(input)?;
    solve_equation(&equation)
}

fn parse_equation(input: &[String]) -> Result<Equation> {
    let mut parts = Vec::new();
    let mut so_far = String::new();

    for part in input {
        let part = part.trim();

        if is_op(first_char(part)?) {
            parts.push(Phrase::try_from(so_far.trim().to_owned())?);
            parts.push(Phrase::try_from(part.to_owned())?);
            so_far = String::default();
        } else {
            so_far.push_str(&format!(" {}", part));
        }
    }

    parts.push(Phrase::try_from(so_far.trim().to_owned())?);
    Ok(parts)
}

fn solve_equation(equation: &[Phrase]) -> Result<Duration> {
    let mut result = 0;
    let mut next_op = Op::Plus;

    for part in equation {
        match part {
            Phrase::Seconds(u) => match next_op {
                Op::Plus => result += u,
                Op::Minus => result -= u,
            },
            Phrase::Plus => next_op = Op::Plus,
            Phrase::Minus => next_op = Op::Minus,
        }
    }

    Ok(Seconds(result).into())
}

fn parse_duration(s: &str, unit: &str, suffix: &str) -> Result<u64> {
    if s.ends_with(suffix).not() {
        bail!("{} not ending in '{}'", unit, suffix);
    }

    s.split(suffix)
        .next()
        .ok_or_else(|| anyhow!("no number for {}", unit))?
        .parse::<u64>()
        .map_err(Into::into)
}

fn is_op(c: char) -> bool {
    matches!(c, '+' | '-')
}

fn first_char(s: &str) -> Result<char> {
    s.chars()
        .next()
        .ok_or_else(|| anyhow!("no char in substring"))
}

#[derive(Debug)]
enum Phrase {
    Seconds(u64),
    Plus,
    Minus,
}

type Equation = Vec<Phrase>;

impl TryFrom<String> for Phrase {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        match s.as_ref() {
            "+" => Ok(Phrase::Plus),
            "-" => Ok(Phrase::Minus),
            s => Ok(Phrase::Seconds(Duration::from_str(s)?.as_seconds().0)),
        }
    }
}

enum Op {
    Plus,
    Minus,
}

trait Conversion<Unit, To> {
    fn conversion_factor() -> Unit;
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Duration {
    weeks: Weeks,
    days: Days,
    hours: Hours,
    minutes: Minutes,
    seconds: Seconds,
}

impl Duration {
    fn as_seconds(&self) -> Seconds {
        (*self).into()
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if matches!(self.weeks, Weeks(0)).not() {
            write!(f, "{} ", self.weeks)?;
        }

        if matches!(self.days, Days(0)).not() {
            write!(f, "{} ", self.days)?;
        }

        if matches!(self.hours, Hours(0)).not() {
            write!(f, "{} ", self.hours)?;
        }

        if matches!(self.minutes, Minutes(0)).not() {
            write!(f, "{} ", self.minutes)?;
        }

        if matches!(self.seconds, Seconds(0)).not() {
            write!(f, "{} ", self.seconds)?;
        }

        Ok(())
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut weeks = None;
        let mut days = None;
        let mut hours = None;
        let mut minutes = None;
        let mut seconds = None;

        for part in s.split(' ') {
            if part.ends_with('w') {
                if weeks.is_none() {
                    weeks = Some(Weeks::from_str(part)?);
                } else {
                    bail!("can't set weeks twice");
                }
            } else if part.ends_with('d') {
                if days.is_none() {
                    days = Some(Days::from_str(part)?);
                } else {
                    bail!("can't set days twice");
                }
            } else if part.ends_with('h') {
                if hours.is_none() {
                    hours = Some(Hours::from_str(part)?);
                } else {
                    bail!("can't set hours twice");
                }
            } else if part.ends_with('m') {
                if minutes.is_none() {
                    minutes = Some(Minutes::from_str(part)?);
                } else {
                    bail!("can't set minutes twice");
                }
            } else if part.ends_with('s') {
                if seconds.is_none() {
                    seconds = Some(Seconds::from_str(part)?);
                } else {
                    bail!("can't set seconds twice");
                }
            } else {
                let last = part
                    .chars()
                    .last()
                    .ok_or_else(|| anyhow!("missing suffix"))?;
                bail!("unknown duration suffix '{}'", last)
            }
        }

        Ok(Duration {
            weeks: weeks.unwrap_or_default(),
            days: days.unwrap_or_default(),
            hours: hours.unwrap_or_default(),
            minutes: minutes.unwrap_or_default(),
            seconds: seconds.unwrap_or_default(),
        })
    }
}

impl From<Seconds> for Duration {
    fn from(s: Seconds) -> Duration {
        let mut s = s.0;

        let weeks = Weeks(s / Weeks::conversion_factor());
        s %= Weeks::conversion_factor();

        let days = Days(s / Days::conversion_factor());
        s %= Days::conversion_factor();

        let hours = Hours(s / Hours::conversion_factor());
        s %= Hours::conversion_factor();

        let minutes = Minutes(s / Minutes::conversion_factor());
        s %= Minutes::conversion_factor();

        let seconds = Seconds(s / Seconds::conversion_factor());
        s %= Seconds::conversion_factor();

        assert_eq!(s, 0);

        Duration {
            weeks,
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Weeks(u64);

impl Display for Weeks {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}w", self.0)
    }
}

impl FromStr for Weeks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Weeks(parse_duration(s, "weeks", "w")?))
    }
}

impl Conversion<u64, Seconds> for Weeks {
    fn conversion_factor() -> u64 {
        Days::conversion_factor() * 7
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Days(u64);

impl Display for Days {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}d", self.0)
    }
}

impl FromStr for Days {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Days(parse_duration(s, "days", "d")?))
    }
}

impl Conversion<u64, Seconds> for Days {
    fn conversion_factor() -> u64 {
        Hours::conversion_factor() * 24
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Hours(u64);

impl Display for Hours {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}h", self.0)
    }
}

impl FromStr for Hours {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Hours(parse_duration(s, "hours", "h")?))
    }
}

impl Conversion<u64, Seconds> for Hours {
    fn conversion_factor() -> u64 {
        Minutes::conversion_factor() * 60
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Minutes(u64);

impl FromStr for Minutes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Minutes(parse_duration(s, "minutes", "m")?))
    }
}

impl Display for Minutes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl Conversion<u64, Seconds> for Minutes {
    fn conversion_factor() -> u64 {
        Seconds::conversion_factor() * 60
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Seconds(u64);

impl Display for Seconds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl FromStr for Seconds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Seconds(parse_duration(s, "seconds", "s")?))
    }
}

impl Conversion<u64, Seconds> for Seconds {
    fn conversion_factor() -> u64 {
        1
    }
}

impl From<Duration> for Seconds {
    fn from(d: Duration) -> Seconds {
        let mut s = 0;

        s += d.weeks.0 * Weeks::conversion_factor();
        s += d.days.0 * Days::conversion_factor();
        s += d.hours.0 * Hours::conversion_factor();
        s += d.minutes.0 * Minutes::conversion_factor();
        s += d.seconds.0 * Seconds::conversion_factor();

        Seconds(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_equality() {
        let duration_1 = Duration {
            hours: Hours(5),
            ..Default::default()
        };

        let duration_2: Duration = duration_1.as_seconds().into();

        assert_eq!(duration_1, duration_2);
    }

    #[test]
    fn minutes_to_seconds() {
        let duration = Duration {
            minutes: Minutes(1),
            ..Default::default()
        };

        let expected_seconds = Seconds(60);

        assert_eq!(expected_seconds, duration.as_seconds());
    }

    #[test]
    fn hours_to_seconds() {
        let duration = Duration {
            hours: Hours(1),
            ..Default::default()
        };

        let expected_seconds = Seconds(60 * 60);

        assert_eq!(expected_seconds, duration.as_seconds());
    }

    #[test]
    fn days_to_seconds() {
        let duration = Duration {
            days: Days(1),
            ..Default::default()
        };

        let expected_seconds = Seconds(60 * 60 * 24);

        assert_eq!(expected_seconds, duration.as_seconds());
    }

    #[test]
    fn weeks_to_seconds() {
        let duration = Duration {
            weeks: Weeks(1),
            ..Default::default()
        };

        let expected_seconds = Seconds(60 * 60 * 24 * 7);

        assert_eq!(expected_seconds, duration.as_seconds());
    }

    #[test]
    fn seconds_to_minutes() {
        let seconds = Seconds(60);

        let expected_duration = Duration {
            minutes: Minutes(1),
            ..Default::default()
        };

        assert_eq!(expected_duration, seconds.into());
    }

    #[test]
    fn seconds_to_hours() {
        let seconds = Seconds(60 * 60);

        let expected_duration = Duration {
            hours: Hours(1),
            ..Default::default()
        };

        assert_eq!(expected_duration, seconds.into());
    }

    #[test]
    fn seconds_to_days() {
        let seconds = Seconds(60 * 60 * 24);

        let expected_duration = Duration {
            days: Days(1),
            ..Default::default()
        };

        assert_eq!(expected_duration, seconds.into());
    }

    #[test]
    fn seconds_to_weeks() {
        let seconds = Seconds(60 * 60 * 24 * 7);

        let expected_duration = Duration {
            weeks: Weeks(1),
            ..Default::default()
        };

        assert_eq!(expected_duration, seconds.into());
    }

    #[test]
    fn double_subtraction() {
        let input = vec!["6d", "-", "1d", "-", "2d"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();

        let result = parse_and_solve(&input).unwrap();
        let expected_result = Duration::from_str("3d").unwrap();

        assert_eq!(expected_result, result);
    }
}

use std::default::Default;
use std::fmt::{self, Display, Formatter};
use std::ops::Not as _;
use std::str::FromStr;
use anyhow::{anyhow, bail, Error, Result};
use std::env;
use std::process::exit;

const EXIT_FAILURE: i32 = 1;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Weeks(u64);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Days(u64);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Hours(u64);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Minutes(u64);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Seconds(u64);

impl Display for Weeks {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}w", self.0)
    }
}

impl Display for Days {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}d", self.0)
    }
}

impl Display for Hours {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}h", self.0)
    }
}

impl Display for Minutes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl Display for Seconds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}s", self.0)
    }
}

fn parse_duration(s: &str, unit: &str, suffix: &str) -> Result<u64> {
    if s.ends_with(suffix).not() {
        bail!("{} not ending in '{}'", unit, suffix);
    }

    s
        .split(suffix)
        .next()
        .ok_or_else(|| anyhow!("no number for {}", unit))?
        .parse::<u64>()
        .map_err(Into::into)
}

impl FromStr for Weeks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Weeks(parse_duration(s, "weeks", "w")?))
    }
}

impl FromStr for Days {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Days(parse_duration(s, "days", "d")?))
    }
}

impl FromStr for Hours {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Hours(parse_duration(s, "hours", "h")?))
    }
}

impl FromStr for Minutes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Minutes(parse_duration(s, "minutes", "m")?))
    }
}

impl FromStr for Seconds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Seconds(parse_duration(s, "seconds", "s")?))
    }
}

trait Conversion<Unit, To> {
    fn conversion_factor() -> Unit;
}

impl Conversion<u64, Seconds> for Seconds {
    fn conversion_factor() -> u64 {
        1
    }
}

impl Conversion<u64, Seconds> for Minutes {
    fn conversion_factor() -> u64 {
        Seconds::conversion_factor() * 60
    }
}

impl Conversion<u64, Seconds> for Hours {
    fn conversion_factor() -> u64 {
        Minutes::conversion_factor() * 60
    }
}

impl Conversion<u64, Seconds> for Days {
    fn conversion_factor() -> u64 {
        Hours::conversion_factor() * 24
    }
}

impl Conversion<u64, Seconds> for Weeks {
    fn conversion_factor() -> u64 {
        Days::conversion_factor() * 7
    }
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
        self.clone().into()
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

impl From<Seconds> for Duration {
    fn from(s: Seconds) -> Duration {
        let mut s = s.0;

        let weeks = Weeks(s / Weeks::conversion_factor());
        s -= s / Weeks::conversion_factor();

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

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut weeks = None;
        let mut days = None;
        let mut hours = None;
        let mut minutes = None;
        let mut seconds = None;

        for part in s.split(" ") {
            if part.ends_with("w") {
                if weeks.is_none() {
                    weeks = Some(Weeks::from_str(part)?);
                } else {
                    bail!("can't set weeks twice");
                }
            }

            if part.ends_with("d") {
                if days.is_none() {
                    days = Some(Days::from_str(part)?);
                } else {
                    bail!("can't set days twice");
                }
            }

            if part.ends_with("h") {
                if hours.is_none() {
                    hours = Some(Hours::from_str(part)?);
                } else {
                    bail!("can't set hours twice");
                }
            }

            if part.ends_with("m") {
                if minutes.is_none() {
                    minutes = Some(Minutes::from_str(part)?);
                } else {
                    bail!("can't set minutes twice");
                }
            }

            if part.ends_with("s") {
                if seconds.is_none() {
                    seconds = Some(Seconds::from_str(part)?);
                } else {
                    bail!("can't set seconds twice");
                }
            }
        }

        Ok(
            Duration {
                weeks: weeks.unwrap_or_default(),
                days: days.unwrap_or_default(),
                hours: hours.unwrap_or_default(),
                minutes: minutes.unwrap_or_default(),
                seconds: seconds.unwrap_or_default(),
            }
        )
    }
}

fn run() -> Result<()> {
    let args = env::args();
    let d_str = args.skip(1).next().ok_or_else(|| anyhow!("no equation provided"))?;
    let duration = Duration::from_str(&d_str)?;
    println!("{}", duration);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        exit(EXIT_FAILURE);
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
}

use std::default::Default;
use std::fmt::{self, Display, Formatter};
use std::ops::Not as _;

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

fn main() {
    let duration = Duration {
        weeks: Weeks(2),
        days: Days(1),
        hours: Hours(5),
        minutes: Minutes(35),
        seconds: Seconds(12),
    };

    println!("{}", duration);
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

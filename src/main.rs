use std::default::Default;

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

fn main() {}

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

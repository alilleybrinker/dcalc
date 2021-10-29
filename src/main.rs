use std::default::Default;

#[derive(Default, Debug)]
struct Weeks(u64);

#[derive(Default, Debug)]
struct Days(u64);

#[derive(Default, Debug)]
struct Hours(u64);

#[derive(Default, Debug)]
struct Minutes(u64);

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
struct Duration {
    weeks: Weeks,
    days: Days,
    hours: Hours,
    minutes: Minutes,
    seconds: Seconds,
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
        let mut weeks = Default::default();
        let mut days = Default::default();
        let mut hours = Default::default();
        let mut minutes = Default::default();
        let mut seconds = Default::default();

        loop {
            weeks = Weeks(s / Weeks::conversion_factor());
            s -= s / Weeks::conversion_factor();

            if s == 0 {
                break;
            }

            days = Days(s / Days::conversion_factor());
            s -= s / Days::conversion_factor();

            if s == 0 {
                break;
            }

            hours = Hours(s / Hours::conversion_factor());
            s -= s / Hours::conversion_factor();

            if s == 0 {
                break;
            }

            minutes = Minutes(s / Minutes::conversion_factor());
            s -= s / Minutes::conversion_factor();

            if s == 0 {
                break;
            }

            seconds = Seconds(s / Seconds::conversion_factor());
            s -= s / Seconds::conversion_factor();

            break;
        }

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
        hours: Hours(5),
        ..Default::default()
    };

    println!("duration 1: {:?}", duration);

    let seconds: Seconds = duration.into();

    println!("seconds: {}", seconds.0);

    let duration: Duration = seconds.into();

    println!("duration 2: {:?}", duration);
}

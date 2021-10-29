use std::default::Default;

#[derive(Default)]
struct Weeks(u64);

#[derive(Default)]
struct Days(u64);

#[derive(Default)]
struct Hours(u64);

#[derive(Default)]
struct Minutes(u64);

#[derive(Default)]
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

#[derive(Default)]
struct Duration {
    weeks: Weeks,
    days: Days,
    hours: Hours,
    minutes: Minutes,
    seconds: Seconds,
}

impl Into<Seconds> for Duration {
    fn into(self) -> Seconds {
        let mut s = 0;

        s += self.weeks.0 * Weeks::conversion_factor();
        s += self.days.0 * Days::conversion_factor();
        s += self.hours.0 * Hours::conversion_factor();
        s += self.minutes.0 * Minutes::conversion_factor();
        s += self.seconds.0 * Seconds::conversion_factor();

        Seconds(s)
    }
}

fn main() {
    let duration = Duration {
        hours: Hours(5),
        ..Default::default()
    };

    let seconds: Seconds = duration.into();

    println!("seconds: {}", seconds.0);
}

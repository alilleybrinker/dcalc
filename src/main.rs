use uom::si::time::*;

struct Duration {
    days: Time<u64, day>,
    hours: Time<u64, hour>,
    minutes: Time<u64, minute>,
    seconds: Time<u64, second>,
}

impl Into<second> for Duration {
    fn into(self) -> second {
        let mut seconds = Time::new::<second>(0);

        seconds += self.days;
        seconds += self.hours;
        seconds += self.minutes;
        seconds += self.seconds;

        seconds
    }
}

fn main() {
    println!("Hello, world!");
}

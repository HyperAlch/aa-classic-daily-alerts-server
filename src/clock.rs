use chrono::{Duration, NaiveTime, Timelike, Utc};
use chrono_tz::Tz;
use std::format;

pub struct GameTime {
    time: NaiveTime,
}

pub struct ServerTime {
    time: NaiveTime,
}

impl ServerTime {
    pub fn new(time_zone: &Tz) -> Self {
        let now = Utc::now().with_timezone(time_zone);
        Self {
            time: NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap(),
        }
    }

    pub fn get_naive(&self) -> NaiveTime {
        self.time
    }
}

impl GameTime {
    pub fn new() -> Self {
        let now = Utc::now();

        let real_hours = now.hour();
        let real_minutes = now.minute();
        let real_seconds = now.second();

        let mut total_seconds = real_seconds;
        total_seconds += real_minutes * 60;
        total_seconds += real_hours * 60 * 60;

        let total_in_game_minutes = total_seconds / 10;

        let game_minutes = total_in_game_minutes % 60;
        let game_hours = total_in_game_minutes / 60;
        let game_hours = game_hours % 24;

        let time = NaiveTime::from_hms_opt(game_hours, game_minutes, 0).unwrap();

        Self { time }
    }

    pub fn offset(&mut self, amount: i64) {
        if amount > 0 {
            let result = self.time.overflowing_add_signed(Duration::hours(amount));
            self.time = result.0;
        }
        if amount < 0 {
            let amount = i64::abs(amount);
            let result = self.time.overflowing_sub_signed(Duration::hours(amount));
            self.time = result.0;
        }
    }

    pub fn get_naive(&self) -> NaiveTime {
        self.time
    }
}

impl std::fmt::Display for ServerTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time)
    }
}

impl std::fmt::Display for GameTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{}", self.time);
        let output = &output[0..5];
        write!(f, "{}", output)
    }
}

pub struct BasicTime {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl BasicTime {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
    pub fn get_naive(&self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.hours, self.minutes, self.seconds).unwrap()
    }
}

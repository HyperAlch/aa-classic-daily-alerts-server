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
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            time: NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap(),
        }
    }

    pub fn get_naive(&self) -> NaiveTime {
        self.time
    }

    pub fn with_tz(tz: Tz) -> Self {
        let now = Utc::now().with_timezone(&tz);
        Self {
            time: NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap(),
        }
    }
}

impl From<GameTime> for ServerTime {
    fn from(value: GameTime) -> Self {
        let game_time = value.get_naive();
        let game_hours = game_time.hour();
        let mut game_minutes = game_time.minute();

        game_minutes += game_hours * 60;

        let real_seconds = game_minutes * 10;

        Self {
            time: NaiveTime::from_num_seconds_from_midnight_opt(real_seconds, 0).unwrap(),
        }
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

    pub fn offset(&mut self, hours: i64, minutes: i64) {
        if hours > 0 {
            let result = self.time.overflowing_add_signed(Duration::hours(hours));
            self.time = result.0;
        }
        if hours < 0 {
            let hours = i64::abs(hours);
            let result = self.time.overflowing_sub_signed(Duration::hours(hours));
            self.time = result.0;
        }
        if minutes > 0 {
            let result = self.time.overflowing_add_signed(Duration::minutes(minutes));
            self.time = result.0;
        }
        if minutes < 0 {
            let minutes = i64::abs(minutes);
            let result = self.time.overflowing_sub_signed(Duration::minutes(minutes));
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

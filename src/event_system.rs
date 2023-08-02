use chrono::NaiveTime;
use chrono_tz::US::Eastern as my_tz;
use std::println;

use crate::{
    clock::{BasicTime, GameTime, ServerTime},
    OFFSET,
};

#[derive(PartialEq)]
pub struct GameEvent {
    pub name: EventName,
    pub times: Vec<NaiveTime>,
    pub time_type: TimeType,
}

impl std::fmt::Display for GameEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct GameEvents(pub Vec<GameEvent>);

impl GameEvent {
    pub fn new(name: EventName, times: Vec<BasicTime>, time_type: TimeType) -> Self {
        let mut converted_times: Vec<NaiveTime> = vec![];
        for time in times {
            converted_times.push(time.get_naive())
        }

        let times = converted_times;

        Self {
            name,
            times,
            time_type,
        }
    }
}

#[derive(PartialEq)]
pub enum TimeType {
    ServerTime,
    GameTime,
}

impl std::fmt::Display for TimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GameTime => write!(f, "Game Time"),
            Self::ServerTime => write!(f, "Server Time"),
        }
    }
}

#[derive(PartialEq)]
pub enum EventName {
    CrimsonRift,
    GrimghastRift,
    AbyssalAttack,
    LuscaAwakening,
    FreedichGoldTrader,
    OcleeraRift,
}

impl std::fmt::Display for EventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrimsonRift => write!(f, "Crimson Rift"),
            Self::GrimghastRift => write!(f, "Grimghast Rift"),
            Self::AbyssalAttack => write!(f, "Abyssal Attack"),
            Self::LuscaAwakening => write!(f, "Lusca Awakening"),
            Self::FreedichGoldTrader => write!(f, "Freedich Gold Trader"),
            Self::OcleeraRift => write!(f, "Ocleera Rift"),
        }
    }
}

impl EventName {}

impl GameEvent {
    pub fn check(&self) -> (bool, String) {
        println!("Checking: {}", self.name);
        let time_export = "".to_string();
        match self.time_type {
            TimeType::GameTime => {
                let mut game_time = GameTime::new();
                game_time.offset(OFFSET);
                let time = game_time.get_naive();

                let event_times = self.times.iter().find(|x| **x == time);
                if event_times.is_some() {
                    let time_export = event_times.unwrap().to_string();
                    (true, time_export)
                } else {
                    (false, time_export)
                }
            }
            TimeType::ServerTime => {
                let server_time = ServerTime::new(&my_tz);
                let time = server_time.get_naive();

                let event_times = self.times.iter().find(|x| **x == time);
                if event_times.is_some() {
                    let time_export = event_times.unwrap().to_string();
                    (true, time_export)
                } else {
                    (false, time_export)
                }
            }
        }
    }
}

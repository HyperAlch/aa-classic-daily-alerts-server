use chrono::NaiveTime;
use chrono_tz::US::Eastern as my_tz;
use std::println;

use crate::{
    clock::{BasicTime, GameTime, ServerTime},
    OFFSET,
};

pub struct GameEvent {
    pub name: EventName,
    times: Vec<NaiveTime>,
    time_type: TimeType,
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

pub enum TimeType {
    ServerTime,
    GameTime,
}

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

impl GameEvent {
    pub fn check(&self) -> bool {
        println!("Checking: {}", self.name);
        match self.time_type {
            TimeType::GameTime => {
                let mut game_time = GameTime::new();
                game_time.offset(OFFSET);
                let time = game_time.get_naive();

                let event_times = self.times.iter().find(|x| **x == time);
                if event_times.is_some() {
                    true
                } else {
                    false
                }
            }
            TimeType::ServerTime => {
                let server_time = ServerTime::new(&my_tz);
                let time = server_time.get_naive();

                let event_times = self.times.iter().find(|x| **x == time);
                if event_times.is_some() {
                    true
                } else {
                    false
                }
            }
        }
    }
}

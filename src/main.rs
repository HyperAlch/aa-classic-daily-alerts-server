use chrono::{Timelike, Utc};
use chrono_tz::US::Eastern as my_tz;
use clock::BasicTime;
use event_system::{EventName, GameEvent, GameEvents, TimeType};
use std::println;
use tokio::time::sleep;

use crate::clock::{GameTime, ServerTime};

const OFFSET: i64 = -14;
mod clock;
mod event_system;

#[tokio::main]
async fn main() {
    loop {
        tick_clock();
        let game_events = GameEvents(vec![
            GameEvent::new(
                EventName::CrimsonRift,
                vec![BasicTime::new(12, 0, 0)],
                TimeType::GameTime,
            ),
            GameEvent::new(
                EventName::GrimghastRift,
                vec![BasicTime::new(0, 0, 0)],
                TimeType::GameTime,
            ),
            GameEvent::new(
                EventName::AbyssalAttack,
                vec![BasicTime::new(17, 30, 0)],
                TimeType::ServerTime,
            ),
            GameEvent::new(
                EventName::LuscaAwakening,
                vec![BasicTime::new(18, 0, 0)],
                TimeType::ServerTime,
            ),
            GameEvent::new(
                EventName::FreedichGoldTrader,
                vec![BasicTime::new(10, 30, 0), BasicTime::new(18, 30, 0)],
                TimeType::ServerTime,
            ),
            GameEvent::new(
                EventName::OcleeraRift,
                vec![BasicTime::new(10, 0, 0), BasicTime::new(23, 0, 0)],
                TimeType::ServerTime,
            ),
        ]);

        for game_event in game_events.0 {
            if game_event.check() {
                println!("{} was detected", game_event.name);
            }
        }
        sleep(std::time::Duration::from_secs(1)).await;
    }
}

fn tick_clock() {
    let now = Utc::now();

    println!("Utc Time: {}:{}:{}", now.hour(), now.minute(), now.second());
    let mut game_time = GameTime::new();
    game_time.offset(OFFSET);

    let server_time = ServerTime::new(&my_tz);
    println!("Server Time: {}", server_time);
    println!("Game Time: {}\n\n", game_time);
}

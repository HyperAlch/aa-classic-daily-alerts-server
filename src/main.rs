use chrono::{Timelike, Utc};
use chrono_tz::US::Eastern as my_tz;
use clock::BasicTime;
use event_system::{EventName, GameEvent, GameEvents, TimeType};
use queue_system::EventQueue;

use std::{println, vec};
use tokio::time::sleep;

use crate::clock::{GameTime, ServerTime};

const OFFSET: i64 = -14;
mod clock;
mod event_system;
mod queue_system;

#[tokio::main]
async fn main() {
    let mut event_queue = EventQueue::new();
    loop {
        println!("\n\n");
        tick_clock();

        // Times are NOT accurate and are currently hardcoded here
        // in order to easily change them for testing.
        let game_events = GameEvents(vec![
            GameEvent::new(
                EventName::CrimsonRift,
                vec![BasicTime::new(7, 3, 0)],
                TimeType::GameTime,
            ),
            GameEvent::new(
                EventName::GrimghastRift,
                vec![BasicTime::new(7, 4, 0)],
                TimeType::GameTime,
            ),
            GameEvent::new(
                EventName::AbyssalAttack,
                vec![BasicTime::new(19, 29, 0)],
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
            let out = game_event.check();
            if out.0 {
                event_queue.add(game_event, out.1);
            }
        }

        if event_queue.is_ready() {
            let e = event_queue.pop();
            tokio::spawn(async move {
                if let Some(e) = e {
                    println!(
                        "\n\n[{} was detected at {} {}]\n\n",
                        e.name, e.time, e.time_type
                    );
                }
            });
            event_queue.clear();
        }

        event_queue.tick();
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
    println!("Game Time: {}", game_time);
}

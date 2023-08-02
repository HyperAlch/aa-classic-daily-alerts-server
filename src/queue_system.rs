use std::println;

use crate::event_system::GameEvent;

pub struct EventQueue(Vec<QueueEvent>, u8);

pub struct QueueEvent {
    pub name: String,
    pub time: String,
    pub time_type: String,
}

impl EventQueue {
    pub fn new() -> Self {
        Self(vec![], 0)
    }

    pub fn tick(&mut self) {
        if self.0.is_empty() {
            self.clear();
        } else {
            self.1 += 1;
            println!("Tick: {}", self.1);
        }
    }

    pub fn clear(&mut self) {
        self.1 = 0;
    }

    pub fn is_ready(&self) -> bool {
        if self.1 == 25 {
            true
        } else {
            false
        }
    }

    pub fn add(&mut self, game_event: GameEvent, time: String) {
        let result = self
            .0
            .iter()
            .find(|x| x.name == game_event.name.to_string());
        if result.is_none() {
            self.0.push(QueueEvent {
                name: game_event.name.to_string(),
                time,
                time_type: game_event.time_type.to_string(),
            })
        }
    }

    pub fn pop(&mut self) -> Option<QueueEvent> {
        self.0.pop()
    }
}

use crate::clock::{GameTime, ServerTime};
use chrono::{Duration, Timelike, Utc};
use chrono_tz::US::Eastern as my_tz;
use clock::BasicTime;
use event_system::{EventName, GameEvent, GameEvents, TimeType};
use queue_system::EventQueue;
use serenity::model::webhook::Webhook;
use serenity::{http::Http, model::prelude::Embed};
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;
use std::env;
use std::{println, vec};
use tokio::time::sleep;

const OFFSET: i64 = 0;
mod clock;
mod event_system;
mod queue_system;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {
        let mut conn = SqliteConnection::connect(&env::var("DATABASE_URL").unwrap()).await?;
        sqlx::migrate!("./migrations").run(&mut conn).await?;

        let recs = sqlx::query!(
            r#"
                SELECT *
                FROM webhook
            "#
        )
        .fetch_all(&mut conn)
        .await
        .unwrap();

        for rec in recs {
            println!("Guild ID: {}, Webhook: {}\n", rec.guild_id, rec.hook_url);
        }
    }

    tokio::spawn(async move {
        println!("Now listening on localhost:3000");
    });

    let mut event_queue = EventQueue::new();
    loop {
        break;
        println!("\n\n");
        tick_clock();

        // Times are NOT accurate and are currently hardcoded here
        // in order to easily change them for testing.
        let game_events = GameEvents(vec![
            GameEvent::new(
                EventName::CrimsonRift,
                vec![BasicTime::new(13, 0, 0)],
                TimeType::GameTime,
            ),
            GameEvent::new(
                EventName::GrimghastRift,
                vec![BasicTime::new(1, 0, 0)],
                TimeType::GameTime,
            ),
            GameEvent::new(
                EventName::AbyssalAttack,
                vec![BasicTime::new(4, 18, 0)],
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

        let time_ahead = 2;

        for game_event in game_events.0 {
            let out = game_event.check(Duration::minutes(time_ahead));
            if out.0 {
                event_queue.add(game_event, out.1);
            }
        }

        if event_queue.is_ready() {
            let e = event_queue.pop();
            tokio::spawn(async move {
                if let Some(event) = e {
                    println!(
                        "\n\n[{} was detected at {} {}]\n\n",
                        event.name, event.time, event.time_type
                    );
                    let http = Http::new("");
                    let webhook = Webhook::from_url(&http, "https://discord.com/api/webhooks/1137894015702945862/MgGmMvnVe8sUOkQOR-8QBdQ3mlwHj_NLa632jS4EQAjpvbChqqB9gsliutzV009iWpWk").await.expect("Webhook invalid!");
                    let embed = Embed::fake(|e| {
                        e.title(event.name)
                            .description(format!("Event starts in ~{} minutes", time_ahead))
                            .field(format!("[{}]", event.time_type), &event.time[0..5], false)
                            .field(
                                "[Local Time]",
                                format!("<t:{}>", event.time.split("|").collect::<Vec<&str>>()[1]),
                                false,
                            )
                    });
                    webhook
                        .execute(&http, false, |w| {
                            w.content("@everyone")
                                .embeds(vec![embed])
                                .username("AA Classic Daily Alerts")
                        })
                        .await
                        .expect("Could not execute webhook.");
                }
            });
            event_queue.clear();
        }

        event_queue.tick();
        sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}

fn tick_clock() {
    let now = Utc::now();

    println!("Utc Time: {}:{}:{}", now.hour(), now.minute(), now.second());
    let mut game_time = GameTime::new();
    game_time.offset(OFFSET, 5);

    let server_time = ServerTime::new();
    let server_time_eastern = ServerTime::with_tz(my_tz);
    println!("Server Time Eastern: {}", server_time_eastern);
    println!("Server Time Raw: {}", server_time);
    println!("[Game Time: {}]", game_time);

    game_time.offset(-OFFSET, 0);
    println!(
        "Server Time From Game Time: {}",
        ServerTime::from(game_time)
    )
}

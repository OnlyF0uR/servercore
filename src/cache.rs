use std::sync::Arc;

use dashmap::DashMap;
use lazy_static::lazy_static;
use pumpkin::entity::player::Player;

use crate::{config::get_config, db::get_db, utils::current_sec};

#[derive(Clone, Debug, sqlx::FromRow)]
struct DBPlayer {
    nickname: String,
    playtime: i64,
    balance: f64,
}

#[derive(Clone, Debug)]
struct CachePlayer {
    nickname: String,
    playtime: i64,
    join_time: i64,
    balance: f64,
}

lazy_static! {
    // User UUID -> Playtime at join
    static ref PLAYER_CACHE: DashMap<String, CachePlayer> = DashMap::new();
}

// Get the amount of seconds the user has been online
// on the server
pub fn get_playtime(player_uuid: &str) -> i64 {
    // Get the previous play time
    let old_pt = PLAYER_CACHE.get(player_uuid).map_or(0, |v| v.playtime);

    // Now we need to calculate how much additional time the user spent on the server
    let jt = PLAYER_CACHE.get(player_uuid).map_or(0, |v| v.join_time);
    let ct = current_sec();

    let diff = ct - jt;

    // Return the sum of the old playtime and the time spent on the server
    old_pt + diff
}

pub fn get_playtime_display(player_uuid: &str) -> String {
    // This is the amount of seconds of playtme
    let pt = get_playtime(player_uuid);

    // We need days, hours, minutes, and seconds
    let days = pt / 86400;
    let hours = (pt % 86400) / 3600;
    let minutes = (pt % 3600) / 60;
    let seconds = pt % 60;

    // We need to format the string but make sure the plural is correct too
    format!(
        "{} day{}, {} hour{}, {} minute{}, {} second{}",
        days,
        if days == 1 { "" } else { "s" },
        hours,
        if hours == 1 { "" } else { "s" },
        minutes,
        if minutes == 1 { "" } else { "s" },
        seconds,
        if seconds == 1 { "" } else { "s" }
    )
}

pub fn get_nickname(player_uuid: &str) -> String {
    // Get the nickname
    PLAYER_CACHE
        .get(player_uuid)
        .map_or("Unknown".to_string(), |v| v.nickname.clone())
}

pub fn update_nickname(player_uuid: &str, nickname: &str) {
    let mut old_player = PLAYER_CACHE.get_mut(player_uuid).unwrap();
    old_player.nickname = nickname.to_string();
}

pub fn get_balance(player_uuid: &str) -> f64 {
    PLAYER_CACHE.get(player_uuid).map_or(0.0, |v| v.balance)
}

pub fn update_balance(player_uuid: &str, balance: f64) {
    let mut old_player = PLAYER_CACHE.get_mut(player_uuid).unwrap();
    old_player.balance = balance;
}

pub async fn load_player(
    player: &Arc<Player>,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let db = get_db().await;

    let uuid_s = player.gameprofile.id.to_string();
    let nickname = player.gameprofile.name.clone();

    let mut new_player = false;

    let db_player = match sqlx::query_as::<_, DBPlayer>(
        "SELECT nickname, playtime FROM players WHERE uuid = $1",
    )
    .bind(player.gameprofile.id.to_string())
    .fetch_one(&db.pool)
    .await
    {
        Ok(player) => player,
        Err(e) => {
            // Player does not yet exist, create them
            if let sqlx::Error::RowNotFound = e {
                sqlx::query("INSERT INTO players (uuid, nickname) VALUES ($1, $2)")
                    .bind(&uuid_s)
                    .bind(&nickname)
                    .execute(&db.pool)
                    .await
                    .unwrap();

                log::info!("Created new user: {}", uuid_s);
                new_player = true;

                DBPlayer {
                    nickname,
                    playtime: 0,
                    balance: get_config().await.value.eco_starting_balance,
                }
            } else {
                return Err(e.into());
            }
        }
    };

    let cache_player = CachePlayer {
        nickname: db_player.nickname.clone(),
        playtime: db_player.playtime,
        join_time: current_sec(),
        balance: db_player.balance,
    };

    PLAYER_CACHE.insert(uuid_s.to_string(), cache_player);
    Ok(new_player)
}

pub async fn resolve_player(
    player_uuid: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Playtime is not updated by some function so we,
    // will get the "updated" playtime here, which is the playtime at join
    // plus the passed time since join
    let pt = get_playtime(player_uuid);

    let db = get_db().await;
    sqlx::query("UPDATE players SET nickname = $1, playtime = $2, balance = $3 WHERE uuid = $4")
        .bind(&get_nickname(player_uuid))
        .bind(pt)
        .bind(&get_balance(player_uuid))
        .bind(player_uuid)
        .execute(&db.pool)
        .await?;

    PLAYER_CACHE.remove(player_uuid);
    Ok(())
}

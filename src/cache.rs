use std::sync::Arc;

use dashmap::DashMap;
use lazy_static::lazy_static;
use pumpkin::entity::player::Player;

use crate::db::get_db;

#[derive(Clone, Debug, sqlx::FromRow)]
struct CachePlayer {
    nickname: String,
    playtime: i64,
}

lazy_static! {
    // User UUID -> Playtime at join
    static ref PLAYER_CACHE: DashMap<String, CachePlayer> = DashMap::new();
}

pub fn get_playtime(player_uuid: &str) -> i64 {
    // Get the playtime at join
    let jt = PLAYER_CACHE.get(player_uuid).map_or(0, |v| v.playtime);

    // Get current time in ms from std
    let ct = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Add the dif to the original playtime
    jt + ct
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
    PLAYER_CACHE.insert(
        player_uuid.to_string(),
        CachePlayer {
            nickname: nickname.to_string(),
            playtime: get_playtime(player_uuid),
        },
    );
}

pub async fn load_player(
    player: &Arc<Player>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = get_db().await;

    let uuid_s = player.gameprofile.id.to_string();
    let nickname = player.gameprofile.name.clone();

    let db_player = match sqlx::query_as::<_, CachePlayer>(
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

                CachePlayer {
                    nickname,
                    playtime: 0,
                }
            } else {
                return Err(e.into());
            }
        }
    };

    PLAYER_CACHE.insert(uuid_s.to_string(), db_player);
    Ok(())
}

pub async fn resolve_player(
    player_uuid: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Playtime is not updated by some function so we,
    // will get the "updated" playtime here, which is the playtime at join
    // plus the passed time since join
    let pt = get_playtime(player_uuid);

    let db = get_db().await;
    sqlx::query("UPDATE players SET nickname = $1, playtime = $2 WHERE uuid = $3")
        .bind(&get_nickname(player_uuid))
        .bind(pt)
        .bind(player_uuid)
        .execute(&db.pool)
        .await?;

    PLAYER_CACHE.remove(player_uuid);
    Ok(())
}

use crate::{db::get_db, utils::neutral_colour, GLOBAL_RUNTIME};
use async_trait::async_trait;
use pumpkin::plugin::{
    player::{join::PlayerJoinEventImpl, PlayerEvent, PlayerJoinEvent},
    EventHandler,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::{color::NamedColor, TextComponent};

pub struct JoinHandler;

#[derive(Clone, Debug, sqlx::FromRow)]
struct JoinGetUser {
    nickname: String,
}

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEventImpl> for JoinHandler {
    async fn handle_blocking(&self, event: &mut PlayerJoinEventImpl) {
        event.set_join_message(
            TextComponent::text(format!("Welcome, {}!", event.get_player().gameprofile.name))
                .color_named(NamedColor::Green),
        );

        // Check if player exists
        let db = get_db().await;
        let db_player =
            match sqlx::query_as::<_, JoinGetUser>("SELECT nickname FROM players WHERE uuid = $1")
                .bind(event.get_player().gameprofile.id.to_string())
                .fetch_one(&db.pool)
                .await
            {
                Ok(player) => player,
                Err(e) => {
                    if let sqlx::Error::RowNotFound = e {
                        sqlx::query("INSERT INTO players (uuid, nickname) VALUES ($1, $2)")
                            .bind(event.get_player().gameprofile.id.to_string())
                            .bind(event.get_player().gameprofile.name.clone())
                            .execute(&db.pool)
                            .await
                            .unwrap();

                        log::info!("Created new user: {}", event.get_player().gameprofile.name);

                        JoinGetUser {
                            nickname: event.get_player().gameprofile.name.clone(),
                        }
                    } else {
                        panic!("Failed to get player: {}", e);
                    }
                }
            };

        event.set_join_message(
            TextComponent::text(format!("Welcome, {}!", db_player.nickname))
                .color_rgb(neutral_colour()),
        );
    }
}

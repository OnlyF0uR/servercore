use crate::{
    cache::{get_nickname, load_player},
    utils::neutral_colour,
};
use async_trait::async_trait;
use pumpkin::{
    plugin::{
        player::{player_join::PlayerJoinEvent, PlayerEvent},
        EventHandler,
    },
    server::Server,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;
use std::sync::Arc;

pub struct JoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerJoinEvent) {
        let np = match load_player(&event.get_player()).await {
            Ok(np) => np,
            Err(err) => {
                log::error!("Could not load player data: {}", err);

                event
                    .get_player()
                    .kick(
                        pumpkin::net::DisconnectReason::Kicked,
                        TextComponent::text("Could not load player data.")
                            .color_rgb(neutral_colour()),
                    )
                    .await;

                false
            }
        };

        if np {
            // Teleport player to spawn
            event.join_message = TextComponent::text(format!(
                "Welcome, {}!",
                get_nickname(&event.get_player().gameprofile.id.to_string(),)
            ))
            .color_rgb(neutral_colour());
        } else {
            event.join_message = TextComponent::text(format!(
                "Welcome back, {}!",
                get_nickname(&event.get_player().gameprofile.id.to_string(),)
            ))
            .color_rgb(neutral_colour());
        }
    }
}

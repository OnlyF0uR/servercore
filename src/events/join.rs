use crate::{
    cache::{get_nickname, load_player},
    utils::neutral_colour,
};
use async_trait::async_trait;
use pumpkin::plugin::{
    player::{PlayerEvent, PlayerJoinEvent},
    EventHandler,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;

pub struct JoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    async fn handle_blocking(&self, event: &mut PlayerJoinEvent) {
        let np = match load_player(&event.get_player()).await {
            Ok(np) => np,
            Err(e) => panic!("Failed to load player: {}", e),
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

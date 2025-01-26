use crate::{
    cache::{get_nickname, load_player},
    utils::neutral_colour,
    GLOBAL_RUNTIME,
};
use async_trait::async_trait;
use pumpkin::plugin::{
    player::{join::PlayerJoinEventImpl, PlayerEvent, PlayerJoinEvent},
    EventHandler,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;

pub struct JoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEventImpl> for JoinHandler {
    async fn handle_blocking(&self, event: &mut PlayerJoinEventImpl) {
        let np = match load_player(&event.get_player()).await {
            Ok(np) => np,
            Err(e) => panic!("Failed to load player: {}", e),
        };

        if np {
            // Teleport player to spawn
            event.set_join_message(
                TextComponent::text(format!(
                    "Welcome, {}!",
                    get_nickname(&event.get_player().gameprofile.id.to_string(),)
                ))
                .color_rgb(neutral_colour()),
            );
        } else {
            event.set_join_message(
                TextComponent::text(format!(
                    "Welcome back, {}!",
                    get_nickname(&event.get_player().gameprofile.id.to_string(),)
                ))
                .color_rgb(neutral_colour()),
            );
        }
    }
}

use crate::{
    cache::{get_nickname, resolve_player},
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

pub struct LeaveHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEventImpl> for LeaveHandler {
    async fn handle_blocking(&self, event: &mut PlayerJoinEventImpl) {
        event.set_join_message(
            TextComponent::text(format!(
                "Goodbye, {}!",
                get_nickname(&event.get_player().gameprofile.id.to_string(),)
            ))
            .color_rgb(neutral_colour()),
        );

        if let Err(e) = resolve_player(&event.get_player().gameprofile.id.to_string()).await {
            panic!("Failed to resolve player: {}", e);
        }
    }
}

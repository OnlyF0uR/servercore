use crate::{
    cache::{get_nickname, resolve_player},
    utils::neutral_colour,
    GLOBAL_RUNTIME,
};
use async_trait::async_trait;
use pumpkin::plugin::{
    player::{leave::PlayerLeaveEventImpl, PlayerEvent, PlayerLeaveEvent},
    EventHandler,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::TextComponent;

pub struct LeaveHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerLeaveEventImpl> for LeaveHandler {
    async fn handle_blocking(&self, event: &mut PlayerLeaveEventImpl) {
        let p = event.get_player();
        let nn = get_nickname(&p.gameprofile.id.to_string());

        // This also deletes the player from cache
        if let Err(e) = resolve_player(&event.get_player().gameprofile.id.to_string()).await {
            panic!("Failed to resolve player: {}", e);
        }

        // Lastly set the leave message
        event.set_leave_message(
            TextComponent::text(format!("Goodbye, {}!", nn)).color_rgb(neutral_colour()),
        );
    }
}

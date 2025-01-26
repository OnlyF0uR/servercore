use crate::GLOBAL_RUNTIME;
use async_trait::async_trait;
use pumpkin::plugin::{
    player::{join::PlayerJoinEventImpl, PlayerEvent, PlayerJoinEvent},
    EventHandler,
};
use pumpkin_api_macros::with_runtime;
use pumpkin_util::text::{color::NamedColor, TextComponent};

pub struct JoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEventImpl> for JoinHandler {
    async fn handle_blocking(&self, event: &mut PlayerJoinEventImpl) {
        event.set_join_message(
            TextComponent::text(format!("Welcome, {}!", event.get_player().gameprofile.name))
                .color_named(NamedColor::Green),
        );

        // TODO: Check if user exists
    }
}

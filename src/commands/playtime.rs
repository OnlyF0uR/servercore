use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{players::PlayersArgumentConsumer, Arg, ConsumedArgs},
        dispatcher::CommandError,
        tree::{
            builder::{argument, require},
            CommandTree,
        },
        CommandExecutor, CommandSender,
    },
    server::Server,
};
use pumpkin_util::{text::TextComponent, PermissionLvl};

use crate::{cache::get_playtime_display, utils::neutral_colour};

const NAMES: [&str; 2] = ["playtime", "pt"];
const DESCRIPTION: &str = "Inspect player playtime.";

const ARG_PLAYER: &str = "player";

struct PlaytimeExecutor;

#[async_trait]
impl CommandExecutor for PlaytimeExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

        let player = &targets[0];
        let pt_s = get_playtime_display(&player.gameprofile.id.to_string());
        let pt_s = format!("{}'s playtime: {}", player.gameprofile.name, pt_s);

        sender
            .send_message(TextComponent::text(pt_s).color_rgb(neutral_colour()))
            .await;

        Ok(())
    }
}

struct PlaytimeExecutorSelf;

#[async_trait]
impl CommandExecutor for PlaytimeExecutorSelf {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let player = sender.as_player().unwrap();

        let pt_s = get_playtime_display(&player.gameprofile.id.to_string());
        let pt_s = format!("Your playtime: {}", pt_s);

        player
            .send_system_message(&TextComponent::text(pt_s).color_rgb(neutral_colour()))
            .await;

        Ok(())
    }
}

// TODO: Move to a proper consumer instead of SimpleArgConsumer for f64s
pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(
            argument(ARG_PLAYER, PlayersArgumentConsumer).then(
                // Viewing playtime from another player requires permission level 1
                require(|sender| sender.has_permission_lvl(PermissionLvl::One))
                    .execute(PlaytimeExecutor),
            ),
        )
        .then(require(|sender| sender.is_player()).execute(PlaytimeExecutorSelf))
}

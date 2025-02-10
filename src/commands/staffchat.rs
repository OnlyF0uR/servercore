use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{message::MsgArgConsumer, Arg, ConsumedArgs},
        dispatcher::CommandError,
        tree::{builder::argument, CommandTree},
        CommandExecutor, CommandSender,
    },
    server::Server,
};
use pumpkin_util::{text::TextComponent, PermissionLvl};

use crate::utils::mark_colour;

const NAMES: [&str; 2] = ["staffchat", "sc"];
const DESCRIPTION: &str = "Send a message to all staffmembers online.";

const ARG_MESSAGE: &str = "message";

struct StaffchatExecutor;

#[async_trait]
impl CommandExecutor for StaffchatExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Msg(msg)) = args.get(ARG_MESSAGE) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_MESSAGE.into())));
        };

        let name: String;
        if sender.is_player() {
            name = sender.as_player().unwrap().gameprofile.name.clone();
        } else {
            name = "Server".to_string();
        }

        let msg = format!("[SC] {}: {}", name, msg);
        let tc = TextComponent::text(msg).color_rgb(mark_colour());

        let players = server.get_all_players().await;
        for player in players.iter() {
            if player.permission_lvl.load().ge(&PermissionLvl::One) {
                player.send_system_message(&tc).await;
            }
        }

        Ok(())
    }
}

pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_MESSAGE, MsgArgConsumer).execute(StaffchatExecutor))
}

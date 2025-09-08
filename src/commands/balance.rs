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
use pumpkin_util::text::TextComponent;

use crate::{cache::get_balance, config::get_config, utils::neutral_colour};

const NAMES: [&str; 2] = ["balance", "bal"];
const DESCRIPTION: &str = "Inspect player balance.";

const ARG_PLAYER: &str = "player";

// TODO: Add support for offline players

struct BalanceExecutor;

#[async_trait]
impl CommandExecutor for BalanceExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

        let player = &targets[0];

        let balance = get_balance(&player.gameprofile.id.to_string());
        let symbol = &get_config().await.value.eco_symbol;

        let msg = format!(
            "{}'s balance: {}{}",
            player.gameprofile.name, symbol, balance
        );

        sender
            .send_message(TextComponent::text(msg).color_rgb(neutral_colour()))
            .await;

        Ok(())
    }
}

struct BalanceExecutorSelf;

#[async_trait]
impl CommandExecutor for BalanceExecutorSelf {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let player = sender.as_player().unwrap();

        let balance = get_balance(&player.gameprofile.id.to_string());
        let symbol = &get_config().await.value.eco_symbol;

        let msg = format!("Your balance: {}{}", symbol, balance);

        sender
            .send_message(TextComponent::text(msg).color_rgb(neutral_colour()))
            .await;

        Ok(())
    }
}

// TODO: Move to a proper consumer instead of SimpleArgConsumer for f64s
pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_PLAYER, PlayersArgumentConsumer).execute(BalanceExecutor))
        .then(require(|sender| sender.is_player()).execute(BalanceExecutorSelf))
}

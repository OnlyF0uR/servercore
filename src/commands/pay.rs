use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{players::PlayersArgumentConsumer, simple::SimpleArgConsumer, Arg, ConsumedArgs},
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

use crate::{
    cache::{get_balance, update_balance},
    config::get_config,
    utils::{neutral_colour, success_colour},
};

const NAMES: [&str; 1] = ["pay"];
const DESCRIPTION: &str = "Pay someone from your balance.";

const ARG_PLAYER: &str = "player";
const ARG_AMOUNT: &str = "amount";

// TODO: Add support for offline players

struct PayExecutor;

#[async_trait]
impl CommandExecutor for PayExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

        if targets.len() != 1 {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        }

        let Some(Arg::Simple(amount)) = args.get(&ARG_AMOUNT) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        };

        // We need to parse the amount to a f64
        let amount = amount
            .parse::<f64>()
            .map_err(|_| CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())))?;

        // We need to check if the amount is negative
        if amount < 0.0 {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        }

        let player = sender.as_player().unwrap();
        let payer_balance = get_balance(&player.gameprofile.id.to_string());

        // Check if the player can afford to pay
        if payer_balance < amount {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        }

        let target = targets.first().unwrap();
        let target_balance = get_balance(&target.gameprofile.id.to_string());

        // Update the balances
        update_balance(&player.gameprofile.id.to_string(), payer_balance - amount);
        update_balance(&target.gameprofile.id.to_string(), target_balance + amount);

        let symbol = get_config().await.value.eco_symbol.clone();

        let sent_msg = format!(
            "You paid {}{} to {}.",
            symbol, amount, target.gameprofile.name
        );
        let received_msg = format!("{} paid you {}{}.", player.gameprofile.name, symbol, amount);

        player
            .send_system_message(&TextComponent::text(sent_msg).color_rgb(success_colour()))
            .await;

        target
            .send_system_message(&TextComponent::text(received_msg).color_rgb(neutral_colour()))
            .await;

        Ok(())
    }
}

// TODO: Move to a proper consumer instead of SimpleArgConsumer for f64s
pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        require(|sender| sender.is_player()).then(
            argument(ARG_PLAYER, PlayersArgumentConsumer)
                .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(PayExecutor)),
        ),
    )
}

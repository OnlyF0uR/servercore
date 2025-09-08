use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{players::PlayersArgumentConsumer, simple::SimpleArgConsumer, Arg, ConsumedArgs},
        dispatcher::CommandError,
        tree::{
            builder::{argument, literal},
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
    utils::success_colour,
};

const NAMES: [&str; 2] = ["economy", "eco"];
const DESCRIPTION: &str = "Manage player economy.";

const ARG_PLAYER: &str = "player";
const ARG_AMOUNT: &str = "amount";

// TODO: Add support for offline players

struct EcoSetExecutor;

#[async_trait]
impl CommandExecutor for EcoSetExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

        let Some(Arg::Simple(amount)) = args.get(&ARG_AMOUNT) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        };

        // We need to parse the amount to a f64
        let amount = amount
            .parse::<f64>()
            .map_err(|_| CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())))?;

        // We need to add the amount to the player's balance
        for target in targets {
            update_balance(&target.gameprofile.id.to_string(), amount);
        }

        let msg = format!("Balance{} set.", if targets.len() == 1 { "" } else { "s" });
        sender
            .send_message(TextComponent::text(msg).color_rgb(success_colour()))
            .await;

        Ok(())
    }
}

struct EcoAddExecutor;

#[async_trait]
impl CommandExecutor for EcoAddExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

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

        // We need to add the amount to the player's balance
        for target in targets {
            let target_balance = get_balance(&target.gameprofile.id.to_string());
            update_balance(&target.gameprofile.id.to_string(), target_balance + amount);
        }

        let msg = format!(
            "Balance{} increased.",
            if targets.len() == 1 { "" } else { "s" }
        );
        sender
            .send_message(TextComponent::text(msg).color_rgb(success_colour()))
            .await;

        Ok(())
    }
}

struct EcoRemoveExecutor;

#[async_trait]
impl CommandExecutor for EcoRemoveExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

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

        // We need to add the amount to the player's balance
        for target in targets {
            let target_balance = get_balance(&target.gameprofile.id.to_string());
            update_balance(&target.gameprofile.id.to_string(), target_balance - amount);
        }

        let msg = format!(
            "Balance{} reduced.",
            if targets.len() == 1 { "" } else { "s" }
        );
        sender
            .send_message(TextComponent::text(msg).color_rgb(success_colour()))
            .await;

        Ok(())
    }
}

struct EcoResetExecutor;

#[async_trait]
impl CommandExecutor for EcoResetExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

        let default_amount = get_config().await.value.eco_starting_balance;

        // We need to add the amount to the player's balance
        for target in targets {
            update_balance(&target.gameprofile.id.to_string(), default_amount);
        }

        let msg = format!(
            "Balance{} reset.",
            if targets.len() == 1 { "" } else { "s" }
        );
        sender
            .send_message(TextComponent::text(msg).color_rgb(success_colour()))
            .await;

        Ok(())
    }
}

// TODO: Move to a proper consumer instead of SimpleArgConsumer for f64s
pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(
            literal("set").then(
                argument(ARG_PLAYER, PlayersArgumentConsumer)
                    .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(EcoSetExecutor)),
            ),
        )
        .then(
            literal("add").then(
                argument(ARG_PLAYER, PlayersArgumentConsumer)
                    .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(EcoAddExecutor)),
            ),
        )
        .then(
            literal("remove").then(
                argument(ARG_PLAYER, PlayersArgumentConsumer)
                    .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(EcoRemoveExecutor)),
            ),
        )
        .then(
            literal("reset")
                .then(argument(ARG_PLAYER, PlayersArgumentConsumer).execute(EcoResetExecutor)),
        )
}
